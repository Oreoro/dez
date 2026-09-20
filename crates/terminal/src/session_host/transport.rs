use std::{
    fmt,
    future::Future,
    io,
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use futures_lite::io::{AsyncRead, AsyncReadExt as _, AsyncWrite, AsyncWriteExt as _};
use gpui::{
    App, AppContext as _, BackgroundExecutor, BorrowAppContext as _, Entity, Global, Task,
    WeakEntity,
};
use net::async_net::UnixStream;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use thiserror::Error;

use super::{
    TERMINAL_SESSION_PROTOCOL_VERSION, TerminalAttachment, TerminalHostCapabilities,
    TerminalHostEventEnvelope, TerminalHostId, TerminalSessionCommand, TerminalSessionEvent,
    TerminalSessionSnapshot,
};
use crate::{Event as TerminalEvent, HostedTerminalController, Terminal};

pub const MAX_TERMINAL_HOST_FRAME_BYTES: usize = 1024 * 1024;
pub const EXPERIMENTAL_TERMINAL_HOST_ENV: &str = "DEZ_EXPERIMENTAL_TERMINAL_HOST";
pub const TERMINAL_HOST_BIN_ENV: &str = "DEZ_TERMINAL_HOST_BIN";
pub const TERMINAL_HOST_SOCKET_ENV: &str = "DEZ_TERMINAL_HOST_SOCKET";
pub const TERMINAL_HOST_TOKEN_FILE_ENV: &str = "DEZ_TERMINAL_HOST_TOKEN_FILE";
pub const TERMINAL_HOST_ID_ENV: &str = "DEZ_TERMINAL_HOST_ID";
pub const TERMINAL_SESSION_ID_ENV: &str = "DEZ_TERMINAL_SESSION_ID";
const MAX_TERMINAL_HOST_INPUT_CHUNK_BYTES: usize = 64 * 1024;
const MAX_TERMINAL_HOST_INPUT_BATCH_BYTES: usize = super::pty_process::MAX_PTY_QUEUED_INPUT_BYTES;
const MAX_TERMINAL_HOST_QUEUED_INPUT_BYTES: usize = 16 * 1024 * 1024;
const TERMINAL_HOST_COMMAND_QUEUE_CAPACITY: usize = 256;
const TERMINAL_HOST_COMMAND_ENQUEUE_TIMEOUT: std::time::Duration =
    std::time::Duration::from_millis(250);
const TERMINAL_HOST_RECONNECT_ATTEMPTS: usize = 8;
const TERMINAL_HOST_RECONNECT_INTERVAL: std::time::Duration = std::time::Duration::from_millis(250);
const TERMINAL_HOST_CONNECT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(3);
const TERMINAL_HOST_COMMAND_CYCLE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(12);
const TERMINAL_HOST_EVENT_READ_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);
const TERMINAL_HOST_REPLAY_ACTIVE_INTERVAL: std::time::Duration =
    std::time::Duration::from_millis(32);
const TERMINAL_HOST_REPLAY_IDLE_INTERVAL: std::time::Duration =
    std::time::Duration::from_millis(125);
const TERMINAL_HOST_ACTIVE_POLL_INTERVAL: std::time::Duration =
    std::time::Duration::from_millis(250);
const TERMINAL_HOST_IDLE_POLL_INTERVAL: std::time::Duration = std::time::Duration::from_secs(1);
const TERMINAL_HOST_ERROR_POLL_INTERVAL: std::time::Duration =
    std::time::Duration::from_millis(500);

pub fn terminal_host_executable_path() -> io::Result<std::path::PathBuf> {
    let helper_name = if cfg!(windows) {
        "dez-terminal-host.exe"
    } else {
        "dez-terminal-host"
    };
    Ok(std::env::current_exe()?.with_file_name(helper_name))
}

fn terminal_host_enablement(
    app_name: &str,
    override_value: Option<&str>,
    helper_is_installed: bool,
) -> bool {
    match override_value.map(str::trim).map(str::to_ascii_lowercase) {
        Some(value) if matches!(value.as_str(), "1" | "true" | "on") => true,
        Some(value) if matches!(value.as_str(), "0" | "false" | "off") => false,
        Some(_) => false,
        None => app_name != "Zed" && helper_is_installed,
    }
}

/// Returns whether this application should create host-owned local terminals.
///
/// Packaged Dez builds install `dez-terminal-host` next to the GUI, so durable
/// terminal ownership is the default there. Source and partial installations
/// without the helper keep the ordinary in-process terminal path, and official
/// Zed compatibility remains unchanged. The legacy environment override stays
/// available for diagnostics and development.
pub fn terminal_host_enabled_for_app(app_name: &str) -> bool {
    let helper_is_installed = terminal_host_executable_path().is_ok_and(|helper| helper.is_file());
    terminal_host_enablement(
        app_name,
        std::env::var(EXPERIMENTAL_TERMINAL_HOST_ENV)
            .ok()
            .as_deref(),
        helper_is_installed,
    )
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TerminalHostAuthToken(String);

impl TerminalHostAuthToken {
    pub fn parse(value: impl Into<String>) -> Result<Self, TerminalHostAuthTokenError> {
        let value = value.into();
        if !(32..=256).contains(&value.len()) {
            return Err(TerminalHostAuthTokenError::InvalidLength);
        }
        if value.chars().any(char::is_whitespace) {
            return Err(TerminalHostAuthTokenError::ContainsWhitespace);
        }
        Ok(Self(value))
    }

    pub fn authenticated_eq(&self, other: &Self) -> bool {
        let left = self.0.as_bytes();
        let right = other.0.as_bytes();
        let mut difference = left.len() ^ right.len();
        for index in 0..left.len().max(right.len()) {
            difference |= usize::from(left.get(index).copied().unwrap_or_default())
                ^ usize::from(right.get(index).copied().unwrap_or_default());
        }
        difference == 0
    }
}

impl fmt::Debug for TerminalHostAuthToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("TerminalHostAuthToken([REDACTED])")
    }
}

#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum TerminalHostAuthTokenError {
    #[error("terminal host authentication token must contain 32 to 256 bytes")]
    InvalidLength,
    #[error("terminal host authentication token must not contain whitespace")]
    ContainsWhitespace,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "message", rename_all = "snake_case")]
pub enum TerminalHostClientMessage {
    Hello {
        protocol_version: u32,
        host_id: TerminalHostId,
        auth_token: TerminalHostAuthToken,
        #[serde(default)]
        capabilities: TerminalHostCapabilities,
    },
    Command {
        request_id: u64,
        command: TerminalSessionCommand,
    },
    /// Converts this authenticated connection into a server-pushed event
    /// stream. Commands continue to use a separate ordered connection.
    SubscribeEvents {
        #[serde(default)]
        after_cursor: Option<u64>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "reason", rename_all = "snake_case")]
pub enum TerminalHostHandshakeRejection {
    AuthenticationFailed,
    HostMismatch,
    ProtocolMismatch {
        host_protocol: u32,
        client_protocol: u32,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "response", rename_all = "snake_case")]
pub enum TerminalHostResponse {
    Heartbeat {
        nonce: u64,
        observed_at_unix_ms: u64,
    },
    Events {
        events: Vec<TerminalHostEventEnvelope>,
        oldest_cursor: u64,
        latest_cursor: u64,
        truncated: bool,
    },
    Sessions {
        sessions: Vec<TerminalSessionSnapshot>,
        #[serde(default)]
        latest_event_cursor: Option<u64>,
    },
    Snapshot {
        snapshot: TerminalSessionSnapshot,
    },
    Attachment {
        attachment: TerminalAttachment,
    },
    Unsupported {
        message: String,
    },
    Error {
        message: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "message", rename_all = "snake_case")]
pub enum TerminalHostServerMessage {
    HelloAccepted {
        protocol_version: u32,
        host_id: TerminalHostId,
        #[serde(default)]
        capabilities: TerminalHostCapabilities,
    },
    HelloRejected {
        rejection: TerminalHostHandshakeRejection,
    },
    Response {
        request_id: u64,
        response: TerminalHostResponse,
    },
    EventBatch {
        events: Vec<TerminalHostEventEnvelope>,
        oldest_cursor: u64,
        latest_cursor: u64,
        truncated: bool,
    },
}

impl TerminalHostServerMessage {
    pub fn accepted(host_id: TerminalHostId, capabilities: TerminalHostCapabilities) -> Self {
        Self::HelloAccepted {
            protocol_version: TERMINAL_SESSION_PROTOCOL_VERSION,
            host_id,
            capabilities,
        }
    }
}

#[derive(Debug, Error)]
pub enum TerminalHostTransportError {
    #[error("terminal host transport I/O failed")]
    Io(#[from] io::Error),
    #[error("terminal host frame is {actual} bytes; maximum is {maximum}")]
    FrameTooLarge { actual: usize, maximum: usize },
    #[error("terminal host frame serialization failed")]
    Serialize(#[source] serde_json::Error),
    #[error("terminal host frame deserialization failed")]
    Deserialize(#[source] serde_json::Error),
    #[error("terminal host rejected the handshake: {0:?}")]
    HandshakeRejected(TerminalHostHandshakeRejection),
    #[error("terminal host responded with an unexpected protocol message")]
    UnexpectedMessage,
    #[error("terminal host accepted a different host identity")]
    HostMismatch,
    #[error(
        "terminal host protocol {host_protocol} is incompatible with client protocol {client_protocol}"
    )]
    ProtocolMismatch {
        host_protocol: u32,
        client_protocol: u32,
    },
    #[error("terminal host request identity space was exhausted")]
    RequestIdExhausted,
    #[error("terminal host does not support server-pushed events")]
    EventStreamUnsupported,
    #[error("terminal host {operation} timed out after {timeout:?}")]
    TimedOut {
        operation: &'static str,
        timeout: std::time::Duration,
    },
}

async fn run_bounded_terminal_host_operation<F, T>(
    future: F,
    executor: &BackgroundExecutor,
    operation: &'static str,
    timeout: std::time::Duration,
) -> Result<T, TerminalHostTransportError>
where
    F: Future<Output = Result<T, TerminalHostTransportError>>,
{
    let timer = executor.timer(timeout);
    futures::pin_mut!(future, timer);
    match futures::future::select(future, timer).await {
        futures::future::Either::Left((result, _)) => result,
        futures::future::Either::Right(_) => {
            Err(TerminalHostTransportError::TimedOut { operation, timeout })
        }
    }
}

async fn run_bounded_terminal_host_command_cycle<F, T>(
    future: F,
    executor: &BackgroundExecutor,
) -> anyhow::Result<T>
where
    F: Future<Output = anyhow::Result<T>>,
{
    let timer = executor.timer(TERMINAL_HOST_COMMAND_CYCLE_TIMEOUT);
    futures::pin_mut!(future, timer);
    match futures::future::select(future, timer).await {
        futures::future::Either::Left((result, _)) => result,
        futures::future::Either::Right(_) => anyhow::bail!(
            "terminal host command cycle timed out after {:?}",
            TERMINAL_HOST_COMMAND_CYCLE_TIMEOUT
        ),
    }
}

async fn run_bounded_terminal_host_command_request<F, T>(
    future: F,
    executor: &BackgroundExecutor,
    operation: &'static str,
) -> anyhow::Result<T>
where
    F: Future<Output = anyhow::Result<T>>,
{
    let timer = executor.timer(TERMINAL_HOST_COMMAND_CYCLE_TIMEOUT);
    futures::pin_mut!(future, timer);
    match futures::future::select(future, timer).await {
        futures::future::Either::Left((result, _)) => result,
        futures::future::Either::Right(_) => anyhow::bail!(
            "terminal host {operation} timed out after {:?}; the request was canceled",
            TERMINAL_HOST_COMMAND_CYCLE_TIMEOUT
        ),
    }
}

/// Authenticated, sequential command client for the local terminal helper.
///
/// Server-pushed observations use [`TerminalHostEventStream`] on a dedicated
/// authenticated socket. This keeps mutating commands ordered without adding
/// request multiplexing, while cursor resume makes reconnects loss-aware.
pub struct TerminalHostTransportClient {
    stream: UnixStream,
    next_request_id: u64,
    capabilities: TerminalHostCapabilities,
}

/// Dedicated authenticated stream of bounded, cursor-addressed host events.
pub struct TerminalHostEventStream {
    stream: UnixStream,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerminalHostEndpoint {
    socket_path: std::path::PathBuf,
    token_file_path: std::path::PathBuf,
    generation: &'static str,
}

impl TerminalHostEndpoint {
    pub fn new(
        socket_path: std::path::PathBuf,
        token_file_path: std::path::PathBuf,
        generation: &'static str,
    ) -> Self {
        Self {
            socket_path,
            token_file_path,
            generation,
        }
    }

    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }

    pub fn token_file_path(&self) -> &Path {
        &self.token_file_path
    }

    pub fn generation(&self) -> &'static str {
        self.generation
    }
}

struct QueuedTerminalHostCommand {
    commands: Vec<TerminalSessionCommand>,
    response_tx:
        Option<futures::channel::oneshot::Sender<Result<TerminalHostResponse, anyhow::Error>>>,
    _input_reservations: Vec<TerminalHostInputReservation>,
}

struct TerminalHostInputBudget {
    queued_bytes: AtomicUsize,
    maximum_queued_bytes: usize,
}

impl TerminalHostInputBudget {
    fn new(maximum_queued_bytes: usize) -> Self {
        Self {
            queued_bytes: AtomicUsize::new(0),
            maximum_queued_bytes,
        }
    }

    fn reserve(
        self: &Arc<Self>,
        byte_count: usize,
    ) -> anyhow::Result<TerminalHostInputReservation> {
        self.queued_bytes
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |queued_bytes| {
                queued_bytes
                    .checked_add(byte_count)
                    .filter(|total| *total <= self.maximum_queued_bytes)
            })
            .map_err(|_| {
                anyhow::anyhow!(
                    "terminal host input queue is full; wait for pending input to drain and retry"
                )
            })?;
        Ok(TerminalHostInputReservation {
            budget: self.clone(),
            byte_count,
        })
    }

    #[cfg(test)]
    fn queued_bytes(&self) -> usize {
        self.queued_bytes.load(Ordering::Acquire)
    }
}

struct TerminalHostInputReservation {
    budget: Arc<TerminalHostInputBudget>,
    byte_count: usize,
}

impl Drop for TerminalHostInputReservation {
    fn drop(&mut self) {
        self.budget
            .queued_bytes
            .fetch_sub(self.byte_count, Ordering::AcqRel);
    }
}

impl QueuedTerminalHostCommand {
    fn single(
        command: TerminalSessionCommand,
        response_tx: Option<
            futures::channel::oneshot::Sender<Result<TerminalHostResponse, anyhow::Error>>,
        >,
    ) -> Self {
        Self {
            commands: vec![command],
            response_tx,
            _input_reservations: Vec::new(),
        }
    }

    fn input(
        session_id: super::TerminalSessionId,
        bytes: Vec<u8>,
        input_budget: &Arc<TerminalHostInputBudget>,
    ) -> anyhow::Result<Option<Self>> {
        if bytes.is_empty() {
            return Ok(None);
        }
        anyhow::ensure!(
            bytes.len() <= MAX_TERMINAL_HOST_INPUT_BATCH_BYTES,
            "terminal input batch is {} bytes; the terminal host limit is {} bytes",
            bytes.len(),
            MAX_TERMINAL_HOST_INPUT_BATCH_BYTES
        );
        let input_reservation = input_budget.reserve(bytes.len())?;

        // The reservation and command chunks share one queue item, so the GUI
        // either accepts the complete batch or rejects it before transport.
        Ok(Some(Self {
            commands: bytes
                .chunks(MAX_TERMINAL_HOST_INPUT_CHUNK_BYTES)
                .map(|chunk| TerminalSessionCommand::Input {
                    session_id,
                    bytes: chunk.to_vec(),
                })
                .collect(),
            response_tx: None,
            _input_reservations: vec![input_reservation],
        }))
    }

    fn input_session_and_byte_count(&self) -> Option<(super::TerminalSessionId, usize)> {
        if self.response_tx.is_some() {
            return None;
        }

        let mut session_id = None;
        let mut byte_count = 0usize;
        for command in &self.commands {
            let TerminalSessionCommand::Input {
                session_id: command_session_id,
                bytes,
            } = command
            else {
                return None;
            };
            if session_id.is_some_and(|session_id| session_id != *command_session_id) {
                return None;
            }
            session_id = Some(*command_session_id);
            byte_count = byte_count.checked_add(bytes.len())?;
        }
        session_id.map(|session_id| (session_id, byte_count))
    }

    fn try_append_adjacent_input(&mut self, mut adjacent: Self) -> Result<(), Self> {
        let Some((session_id, byte_count)) = self.input_session_and_byte_count() else {
            return Err(adjacent);
        };
        let Some((adjacent_session_id, adjacent_byte_count)) =
            adjacent.input_session_and_byte_count()
        else {
            return Err(adjacent);
        };
        if session_id != adjacent_session_id
            || byte_count
                .checked_add(adjacent_byte_count)
                .is_none_or(|byte_count| byte_count > MAX_TERMINAL_HOST_INPUT_BATCH_BYTES)
        {
            return Err(adjacent);
        }

        let adjacent_commands = std::mem::take(&mut adjacent.commands);
        for command in adjacent_commands {
            let TerminalSessionCommand::Input { session_id, bytes } = command else {
                log::error!("validated terminal input batch contained another command");
                return Err(adjacent);
            };
            if let Some(TerminalSessionCommand::Input {
                session_id: previous_session_id,
                bytes: previous_bytes,
            }) = self.commands.last_mut()
                && *previous_session_id == session_id
                && previous_bytes.len().saturating_add(bytes.len())
                    <= MAX_TERMINAL_HOST_INPUT_CHUNK_BYTES
            {
                previous_bytes.extend(bytes);
            } else {
                self.commands
                    .push(TerminalSessionCommand::Input { session_id, bytes });
            }
        }
        self._input_reservations
            .append(&mut adjacent._input_reservations);
        Ok(())
    }
}

/// Shared, ordered command path for all terminal surfaces attached to one
/// authenticated helper connection.
pub struct TerminalHostConnection {
    host_id: TerminalHostId,
    capabilities: TerminalHostCapabilities,
    endpoint: TerminalHostEndpoint,
    auth_token: TerminalHostAuthToken,
    background_executor: BackgroundExecutor,
    command_tx: async_channel::Sender<QueuedTerminalHostCommand>,
    input_budget: Arc<TerminalHostInputBudget>,
    _transport_task: Task<()>,
}

struct GlobalTerminalHostConnection(std::sync::Arc<TerminalHostConnection>);

impl Global for GlobalTerminalHostConnection {}

struct GlobalTerminalHostSnapshotStore(Entity<TerminalHostSnapshotStore>);

impl Global for GlobalTerminalHostSnapshotStore {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TerminalHostStartupState {
    Disabled,
    InstallationRequired { message: String },
    Connecting,
    Connected { host_id: TerminalHostId },
    Reconnecting { message: String },
    Failed { message: String },
}

pub struct TerminalHostStartupStatus {
    state: TerminalHostStartupState,
}

impl Global for TerminalHostStartupStatus {}

impl TerminalHostStartupStatus {
    pub fn init(cx: &mut App) {
        if cx.try_global::<Self>().is_none() {
            cx.set_global(Self {
                state: TerminalHostStartupState::Disabled,
            });
        }
    }

    pub fn set(state: TerminalHostStartupState, cx: &mut App) {
        Self::init(cx);
        if cx
            .try_global::<Self>()
            .is_some_and(|status| status.state == state)
        {
            return;
        }
        cx.update_global::<Self, _>(|status, _cx| status.state = state);
    }

    pub fn state(cx: &App) -> TerminalHostStartupState {
        cx.try_global::<Self>()
            .map(|status| status.state.clone())
            .unwrap_or(TerminalHostStartupState::Disabled)
    }
}

/// Lightweight notification global observed by UI projections. The snapshot
/// store may be installed after those projections are constructed, so they
/// cannot rely on a one-time entity subscription.
pub struct TerminalHostSnapshotRevision(u64);

impl Global for TerminalHostSnapshotRevision {}

impl TerminalHostSnapshotRevision {
    pub fn init(cx: &mut App) {
        if cx.try_global::<Self>().is_none() {
            cx.set_global(Self(0));
        }
    }

    fn bump(cx: &mut App) {
        if cx.try_global::<Self>().is_some() {
            cx.update_global::<Self, _>(|revision, _cx| {
                revision.0 = revision.0.wrapping_add(1);
            });
        }
    }
}

pub struct TerminalHostSnapshotStore {
    snapshots: Vec<TerminalSessionSnapshot>,
    last_error: Option<String>,
    _poll_task: Task<()>,
}

fn require_authoritative_snapshot(event_cursor: &mut Option<u64>, needs_full_snapshot: &mut bool) {
    // A restarted helper owns a new cursor namespace, and an invalid response
    // makes the current stream's authority unknown. Reusing the old high
    // cursor could make a valid empty event batch look authoritative forever.
    *event_cursor = None;
    *needs_full_snapshot = true;
}

fn mark_snapshot_store_reconnecting(
    store: &mut TerminalHostSnapshotStore,
    message: String,
    cx: &mut App,
) {
    store.last_error = Some(message.clone());
    TerminalHostStartupStatus::set(TerminalHostStartupState::Reconnecting { message }, cx);
    for snapshot in &mut store.snapshots {
        if snapshot.state.may_be_live() {
            snapshot.state = super::TerminalSessionState::Reconnecting;
        }
    }
}

impl TerminalHostSnapshotStore {
    fn apply_event(&mut self, event: TerminalSessionEvent) {
        match event {
            TerminalSessionEvent::Snapshot { snapshot } => {
                if let Some(existing) = self
                    .snapshots
                    .iter_mut()
                    .find(|existing| existing.session_id == snapshot.session_id)
                {
                    *existing = snapshot;
                } else {
                    self.snapshots.push(snapshot);
                }
            }
            TerminalSessionEvent::WorkingDirectoryChanged {
                session_id,
                working_directory,
            } => {
                if let Some(snapshot) = self
                    .snapshots
                    .iter_mut()
                    .find(|snapshot| snapshot.session_id == session_id)
                {
                    snapshot.working_directory = working_directory;
                }
            }
            TerminalSessionEvent::TitleChanged { session_id, title } => {
                if let Some(snapshot) = self
                    .snapshots
                    .iter_mut()
                    .find(|snapshot| snapshot.session_id == session_id)
                {
                    snapshot.title = title;
                }
            }
            TerminalSessionEvent::StateChanged { session_id, state } => {
                if let Some(snapshot) = self
                    .snapshots
                    .iter_mut()
                    .find(|snapshot| snapshot.session_id == session_id)
                {
                    snapshot.state = state;
                }
            }
            TerminalSessionEvent::Output { .. } => {}
        }
    }

    fn init(connection: std::sync::Arc<TerminalHostConnection>, cx: &mut App) -> Entity<Self> {
        let host_id = connection.host_id();
        let store = cx.new(|_| Self {
            snapshots: Vec::new(),
            last_error: None,
            _poll_task: Task::ready(()),
        });
        let poll_task = cx.spawn({
            let store = store.downgrade();
            async move |cx| {
                let supports_event_cursor = connection.capabilities().event_cursor_resume;
                let supports_event_stream = connection.capabilities().event_stream;
                let mut event_cursor = None;
                let mut event_stream = None;
                // Always establish one authoritative baseline. Cursors then
                // carry only changes and can survive transport reconnects.
                let mut needs_full_snapshot = true;
                loop {
                    let response = if needs_full_snapshot {
                        connection.command(TerminalSessionCommand::List).await
                    } else if supports_event_stream {
                        let response = connection
                            .next_event_batch(&mut event_stream, event_cursor)
                            .await;
                        if response.is_err() {
                            event_stream = None;
                            require_authoritative_snapshot(
                                &mut event_cursor,
                                &mut needs_full_snapshot,
                            );
                        }
                        response
                    } else {
                        connection
                            .command(TerminalSessionCommand::Events {
                                after_cursor: event_cursor,
                                limit: 8,
                            })
                            .await
                    };
                    match &response {
                        Ok(TerminalHostResponse::Sessions {
                            latest_event_cursor,
                            ..
                        }) => {
                            event_cursor = *latest_event_cursor;
                            needs_full_snapshot =
                                !(supports_event_cursor && event_cursor.is_some());
                        }
                        Ok(TerminalHostResponse::Events {
                            latest_cursor,
                            truncated,
                            ..
                        }) => {
                            if *truncated {
                                needs_full_snapshot = true;
                                event_cursor = None;
                            } else {
                                event_cursor = Some(*latest_cursor);
                            }
                        }
                        Ok(_) => {
                            event_stream = None;
                            require_authoritative_snapshot(
                                &mut event_cursor,
                                &mut needs_full_snapshot,
                            );
                        }
                        _ => {}
                    }
                    let next_poll_interval = store
                        .update(cx, |store, cx| {
                            let previous_snapshots = store.snapshots.clone();
                            let previous_error = store.last_error.clone();
                            let mut poll_interval;
                            match response {
                                Ok(TerminalHostResponse::Sessions { sessions, .. }) => {
                                    store.snapshots = sessions;
                                    store.last_error = None;
                                    poll_interval = if store.snapshots == previous_snapshots
                                        && previous_error.is_none()
                                    {
                                        TERMINAL_HOST_IDLE_POLL_INTERVAL
                                    } else {
                                        TERMINAL_HOST_ACTIVE_POLL_INTERVAL
                                    };
                                    if previous_error.is_some() {
                                        TerminalHostStartupStatus::set(
                                            TerminalHostStartupState::Connected { host_id },
                                            cx,
                                        );
                                    }
                                }
                                Ok(TerminalHostResponse::Events {
                                    events, truncated, ..
                                }) => {
                                    store.last_error = None;
                                    if truncated {
                                        poll_interval = TERMINAL_HOST_ACTIVE_POLL_INTERVAL;
                                    } else {
                                        for envelope in events {
                                            store.apply_event(envelope.event);
                                        }
                                        store.snapshots.sort_by_key(|snapshot| {
                                            snapshot.session_id.to_string()
                                        });
                                        poll_interval = if store.snapshots == previous_snapshots
                                            && previous_error.is_none()
                                        {
                                            TERMINAL_HOST_IDLE_POLL_INTERVAL
                                        } else {
                                            TERMINAL_HOST_ACTIVE_POLL_INTERVAL
                                        };
                                        if previous_error.is_some() {
                                            TerminalHostStartupStatus::set(
                                                TerminalHostStartupState::Connected { host_id },
                                                cx,
                                            );
                                        }
                                    }
                                }
                                Ok(response) => {
                                    let message = terminal_host_response_rejection(&response)
                                        .map(|message| {
                                            format!(
                                                "terminal host rejected snapshot refresh: {message}"
                                            )
                                        })
                                        .unwrap_or_else(|| {
                                            "terminal host returned an invalid snapshot response"
                                                .to_owned()
                                        });
                                    mark_snapshot_store_reconnecting(store, message, cx);
                                    poll_interval = TERMINAL_HOST_ERROR_POLL_INTERVAL;
                                }
                                Err(error) => {
                                    let message = format!("{error:#}");
                                    mark_snapshot_store_reconnecting(store, message, cx);
                                    poll_interval = TERMINAL_HOST_ERROR_POLL_INTERVAL;
                                }
                            }
                            if store.snapshots != previous_snapshots
                                || store.last_error != previous_error
                            {
                                cx.notify();
                                TerminalHostSnapshotRevision::bump(cx);
                            }
                            if supports_event_stream
                                && event_cursor.is_some()
                                && !needs_full_snapshot
                                && store.last_error.is_none()
                            {
                                // The next stream read sleeps in the helper
                                // until a new authoritative event is ready.
                                poll_interval = std::time::Duration::ZERO;
                            }
                            poll_interval
                        })
                        .ok();
                    let Some(next_poll_interval) = next_poll_interval else {
                        return;
                    };
                    cx.background_executor().timer(next_poll_interval).await;
                }
            }
        });
        store.update(cx, |store, _cx| store._poll_task = poll_task);
        cx.set_global(GlobalTerminalHostSnapshotStore(store.clone()));
        store
    }

    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalTerminalHostSnapshotStore>()
            .map(|store| store.0.clone())
    }

    pub fn snapshots(&self) -> &[TerminalSessionSnapshot] {
        &self.snapshots
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }
}

impl TerminalHostConnection {
    pub fn set_global(connection: std::sync::Arc<Self>, cx: &mut App) {
        TerminalHostSnapshotRevision::init(cx);
        TerminalHostStartupStatus::set(
            TerminalHostStartupState::Connected {
                host_id: connection.host_id(),
            },
            cx,
        );
        cx.set_global(GlobalTerminalHostConnection(connection.clone()));
        TerminalHostSnapshotStore::init(connection, cx);
    }

    pub fn try_global(cx: &App) -> Option<std::sync::Arc<Self>> {
        cx.try_global::<GlobalTerminalHostConnection>()
            .map(|connection| connection.0.clone())
    }

    pub async fn connect(
        endpoint: &TerminalHostEndpoint,
        host_id: TerminalHostId,
        auth_token: TerminalHostAuthToken,
        background_executor: &BackgroundExecutor,
    ) -> Result<Self, TerminalHostTransportError> {
        let client = run_bounded_terminal_host_operation(
            TerminalHostTransportClient::connect(
                endpoint.socket_path(),
                host_id,
                auth_token.clone(),
            ),
            background_executor,
            "connection",
            TERMINAL_HOST_CONNECT_TIMEOUT,
        )
        .await?;
        let capabilities = client.capabilities();
        let (command_tx, command_rx) = async_channel::bounded::<QueuedTerminalHostCommand>(
            TERMINAL_HOST_COMMAND_QUEUE_CAPACITY,
        );
        let input_budget = Arc::new(TerminalHostInputBudget::new(
            MAX_TERMINAL_HOST_QUEUED_INPUT_BYTES,
        ));
        let endpoint = endpoint.clone();
        let event_auth_token = auth_token.clone();
        let command_socket_path = endpoint.socket_path().to_path_buf();
        let executor = background_executor.clone();
        let transport_task = background_executor.spawn(async move {
            let mut client = Some(client);
            let mut deferred_queued = None;
            loop {
                let mut queued = match deferred_queued.take() {
                    Some(queued) => queued,
                    None => {
                        let Ok(queued) = command_rx.recv().await else {
                            break;
                        };
                        queued
                    }
                };
                if queued.input_session_and_byte_count().is_some() {
                    while let Ok(adjacent) = command_rx.try_recv() {
                        if let Err(adjacent) = queued.try_append_adjacent_input(adjacent) {
                            deferred_queued = Some(adjacent);
                            break;
                        }
                    }
                }
                let QueuedTerminalHostCommand {
                    commands,
                    response_tx,
                    _input_reservations,
                } = queued;
                if response_tx
                    .as_ref()
                    .is_some_and(|response_tx| response_tx.is_canceled())
                {
                    continue;
                }

                let command_cycle = async {
                    let mut reconnect_error = None;
                    for attempt in 0..TERMINAL_HOST_RECONNECT_ATTEMPTS {
                        if response_tx
                            .as_ref()
                            .is_some_and(|response_tx| response_tx.is_canceled())
                        {
                            return Ok(None);
                        }
                        if client.is_some() {
                            break;
                        }
                        match run_bounded_terminal_host_operation(
                            TerminalHostTransportClient::connect(
                                &command_socket_path,
                                host_id,
                                auth_token.clone(),
                            ),
                            &executor,
                            "reconnection",
                            TERMINAL_HOST_CONNECT_TIMEOUT,
                        )
                        .await
                        {
                            Ok(reconnected) => client = Some(reconnected),
                            Err(error) => {
                                log::debug!("terminal host reconnect failed: {error}");
                                let permanent = reconnect_error_is_permanent(&error);
                                reconnect_error = Some(error);
                                if permanent {
                                    break;
                                }
                                if attempt + 1 < TERMINAL_HOST_RECONNECT_ATTEMPTS {
                                    if response_tx
                                        .as_ref()
                                        .is_some_and(|response_tx| response_tx.is_canceled())
                                    {
                                        return Ok(None);
                                    }
                                    executor.timer(TERMINAL_HOST_RECONNECT_INTERVAL).await;
                                }
                            }
                        }
                    }
                    if response_tx
                        .as_ref()
                        .is_some_and(|response_tx| response_tx.is_canceled())
                    {
                        return Ok(None);
                    }
                    match client.as_mut() {
                        Some(client) => {
                            let mut last_response = None;
                            for command in commands {
                                if response_tx
                                    .as_ref()
                                    .is_some_and(|response_tx| response_tx.is_canceled())
                                {
                                    return Ok(None);
                                }
                                let response = client
                                    .command(command)
                                    .await
                                    .map_err(anyhow::Error::from)?;
                                let rejected = terminal_host_response_rejection(&response).is_some();
                                last_response = Some(response);
                                if rejected {
                                    break;
                                }
                            }
                            Ok(last_response)
                        }
                        None => match reconnect_error {
                            Some(error) => Err(anyhow::anyhow!(
                                "terminal host connection is still unavailable: {error}"
                            )),
                            None => Err(anyhow::anyhow!("terminal host connection unavailable")),
                        },
                    }
                };
                let result =
                    run_bounded_terminal_host_command_cycle(command_cycle, &executor).await;
                if result.is_err() {
                    // The request may have reached the helper, so never replay
                    // it automatically. Any work already queued behind the
                    // failed request is stale by definition and must fail too.
                    client = None;
                }
                let failure_message = result.as_ref().err().map(|error| format!("{error:#}"));
                if let Some(failure_message) = failure_message.as_ref() {
                    let queued_at_failure = command_rx.len();
                    let mut discarded_count = 0usize;
                    let mut discard = |pending: QueuedTerminalHostCommand| {
                        discarded_count = discarded_count.saturating_add(1);
                        if let Some(response_tx) = pending.response_tx
                            && !response_tx.is_canceled()
                            && response_tx
                                .send(Err(anyhow::anyhow!(
                                    "terminal host discarded queued work after a transport failure: {failure_message}"
                                )))
                                .is_err()
                        {
                            log::debug!(
                                "terminal host queued command response receiver was dropped"
                            );
                        }
                    };
                    if let Some(pending) = deferred_queued.take() {
                        discard(pending);
                    }
                    for _ in 0..queued_at_failure {
                        let Ok(pending) = command_rx.try_recv() else {
                            break;
                        };
                        discard(pending);
                    }
                    if discarded_count > 0 {
                        log::warn!(
                            "discarded {discarded_count} stale terminal host commands after a transport failure"
                        );
                    }
                }
                if let Some(response_tx) = response_tx {
                    let result = result.and_then(|response| {
                        response.ok_or_else(|| anyhow::anyhow!("terminal host command canceled"))
                    });
                    if response_tx.send(result).is_err() {
                        log::debug!("terminal host command response receiver was dropped");
                    }
                } else {
                    match result {
                        Err(error) => log::warn!("terminal host command failed: {error:#}"),
                        Ok(Some(response)) => {
                            if let Some(message) = terminal_host_response_rejection(&response) {
                                log::warn!("terminal host rejected queued command: {message}");
                            }
                        }
                        Ok(None) => {}
                    }
                }
                drop(_input_reservations);
            }
        });
        Ok(Self {
            host_id,
            capabilities,
            endpoint,
            auth_token: event_auth_token,
            background_executor: background_executor.clone(),
            command_tx,
            input_budget,
            _transport_task: transport_task,
        })
    }

    pub fn host_id(&self) -> TerminalHostId {
        self.host_id
    }

    pub fn capabilities(&self) -> TerminalHostCapabilities {
        self.capabilities
    }

    pub fn endpoint(&self) -> &TerminalHostEndpoint {
        &self.endpoint
    }

    async fn event_stream(
        &self,
        after_cursor: Option<u64>,
    ) -> Result<TerminalHostEventStream, TerminalHostTransportError> {
        run_bounded_terminal_host_operation(
            TerminalHostEventStream::connect(
                self.endpoint.socket_path(),
                self.host_id,
                self.auth_token.clone(),
                after_cursor,
            ),
            &self.background_executor,
            "event connection",
            TERMINAL_HOST_CONNECT_TIMEOUT,
        )
        .await
    }

    async fn next_event_batch(
        &self,
        stream: &mut Option<TerminalHostEventStream>,
        after_cursor: Option<u64>,
    ) -> anyhow::Result<TerminalHostResponse> {
        if stream.is_none() {
            *stream = Some(self.event_stream(after_cursor).await?);
        }
        let event_stream = stream
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("terminal host event stream was not connected"))?;
        run_bounded_terminal_host_operation(
            event_stream.next(),
            &self.background_executor,
            "event heartbeat",
            TERMINAL_HOST_EVENT_READ_TIMEOUT,
        )
        .await
        .map_err(anyhow::Error::from)
    }

    pub async fn command(
        &self,
        command: TerminalSessionCommand,
    ) -> anyhow::Result<TerminalHostResponse> {
        request_terminal_host_command(
            &self.command_tx,
            command,
            &self.background_executor,
            "command",
        )
        .await
    }

    pub fn controller(
        &self,
        session_id: super::TerminalSessionId,
    ) -> std::sync::Arc<dyn HostedTerminalController> {
        std::sync::Arc::new(TransportHostedTerminalController {
            session_id,
            command_tx: self.command_tx.clone(),
            input_budget: self.input_budget.clone(),
            background_executor: self.background_executor.clone(),
        })
    }

    pub fn acknowledge_agent_attention(&self, session_id: super::TerminalSessionId) {
        if let Err(error) = self.command_tx.try_send(QueuedTerminalHostCommand::single(
            TerminalSessionCommand::AcknowledgeAgentAttention { session_id },
            None,
        )) {
            log::debug!("failed to queue terminal-agent attention acknowledgement: {error}");
        }
    }

    /// Mirrors display-derived metadata back to the owning host so detached
    /// sessions remain recognizable in projections such as the Session Rail.
    pub fn observe_terminal(
        self: &std::sync::Arc<Self>,
        terminal: &Entity<Terminal>,
        cx: &mut App,
    ) {
        let session_id = terminal.read(cx).session_id();
        let connection = self.clone();
        cx.subscribe(terminal, move |terminal, event: &TerminalEvent, cx| {
            if !matches!(
                event,
                TerminalEvent::TitleChanged
                    | TerminalEvent::BreadcrumbsChanged
                    | TerminalEvent::ProcessInfoChanged
            ) {
                return;
            }
            let terminal = terminal.read(cx);
            let command = TerminalSessionCommand::UpdateMetadata {
                session_id,
                title: Some(terminal.title(false)),
                working_directory: terminal.working_directory(),
                workspace_id: None,
            };
            if let Err(error) = connection
                .command_tx
                .try_send(QueuedTerminalHostCommand::single(command, None))
            {
                log::debug!("failed to queue hosted terminal metadata update: {error}");
            }
        })
        .detach();
    }

    /// Associates a hosted Session with its durable Workspace without adding
    /// Workspace ownership to the terminal emulator or Project stores.
    pub fn associate_workspace(&self, terminal: &Entity<Terminal>, workspace_id: i64, cx: &App) {
        let terminal = terminal.read(cx);
        let command = TerminalSessionCommand::UpdateMetadata {
            session_id: terminal.session_id(),
            title: Some(terminal.title(false)),
            working_directory: terminal.working_directory(),
            workspace_id: Some(workspace_id),
        };
        if let Err(error) = self
            .command_tx
            .try_send(QueuedTerminalHostCommand::single(command, None))
        {
            log::debug!("failed to queue hosted terminal Workspace association: {error}");
        }
    }

    pub fn follow_session(
        self: std::sync::Arc<Self>,
        terminal: WeakEntity<Terminal>,
        session_id: super::TerminalSessionId,
        cx: &App,
    ) -> Task<anyhow::Result<()>> {
        cx.spawn(async move |cx| {
            let mut latest_sequence = 0;
            let mut reconnecting = false;
            loop {
                let response = match self
                    .command(TerminalSessionCommand::Attach {
                        session_id,
                        replay_after_sequence: Some(latest_sequence),
                    })
                    .await
                {
                    Ok(response) => response,
                    Err(error) => {
                        if !reconnecting {
                            log::warn!("terminal host connection lost: {error:#}");
                            terminal.update(cx, |terminal, cx| {
                                terminal.write_output(
                                    b"\r\n[Dez: terminal host connection lost; process left untouched, reconnecting]\r\n",
                                    cx,
                                );
                            })?;
                            reconnecting = true;
                        }
                        cx.background_executor()
                            .timer(std::time::Duration::from_millis(250))
                            .await;
                        continue;
                    }
                };
                let attachment = match response {
                    TerminalHostResponse::Attachment { attachment } => attachment,
                    TerminalHostResponse::Error { message }
                    | TerminalHostResponse::Unsupported { message } => {
                        anyhow::bail!("terminal host attach failed: {message}");
                    }
                    TerminalHostResponse::Sessions { .. }
                    | TerminalHostResponse::Snapshot { .. }
                    | TerminalHostResponse::Heartbeat { .. }
                    | TerminalHostResponse::Events { .. } => {
                        anyhow::bail!("terminal host returned a non-attachment response");
                    }
                };

                if reconnecting {
                    terminal.update(cx, |terminal, cx| {
                        terminal.write_output(b"\r\n[Dez: terminal host reconnected]\r\n", cx);
                    })?;
                    reconnecting = false;
                }

                let replay_was_truncated = attachment.replay_was_truncated;
                let replay_is_active = !attachment.replay.is_empty();
                let state = attachment.snapshot.state;
                let dimensions = attachment.snapshot.dimensions;
                let foreground_command = attachment.snapshot.foreground_command.clone();
                let snapshot_latest_sequence = attachment.snapshot.latest_replay_sequence;
                let replay = attachment.replay;
                terminal.update(cx, |terminal, cx| {
                    if replay_was_truncated {
                        terminal.write_output(
                            b"\r\n[Dez: earlier terminal output was evicted from bounded replay]\r\n",
                            cx,
                        );
                    }
                    for chunk in replay {
                        latest_sequence = latest_sequence.max(chunk.sequence);
                        terminal.write_hosted_replay(&chunk, cx);
                    }
                    let terminal_bounds = terminal.last_content().terminal_bounds;
                    let dimensions_changed = terminal_bounds.num_columns()
                        != usize::from(dimensions.columns)
                        || terminal_bounds.num_lines() != usize::from(dimensions.rows);
                    if replay_is_active || replay_was_truncated || dimensions_changed {
                        terminal.finish_hosted_replay(dimensions, cx);
                    }
                    terminal.set_hosted_foreground_command(foreground_command, cx);
                    match state {
                        super::TerminalSessionState::Exited { exit_code } => {
                            terminal.hosted_process_exited(exit_code, cx);
                        }
                        super::TerminalSessionState::Missing => {
                            terminal.write_output(
                                b"\r\n[Dez: hosted terminal session is missing; no replacement was started]\r\n",
                                cx,
                            );
                            terminal.hosted_process_exited(None, cx);
                        }
                        super::TerminalSessionState::Incompatible { .. } => {
                            terminal.write_output(
                                b"\r\n[Dez: terminal host protocol is incompatible; no replacement was started]\r\n",
                                cx,
                            );
                            terminal.hosted_process_exited(None, cx);
                        }
                        super::TerminalSessionState::Starting
                        | super::TerminalSessionState::Attached
                        | super::TerminalSessionState::Detached
                        | super::TerminalSessionState::Reconnecting => {}
                    }
                })?;
                latest_sequence = latest_sequence.max(snapshot_latest_sequence);

                if !state.may_be_live() {
                    return Ok(());
                }
                cx.background_executor()
                    .timer(if replay_is_active {
                        TERMINAL_HOST_REPLAY_ACTIVE_INTERVAL
                    } else {
                        TERMINAL_HOST_REPLAY_IDLE_INTERVAL
                    })
                    .await;
            }
        })
    }
}

fn reconnect_error_is_permanent(error: &TerminalHostTransportError) -> bool {
    matches!(
        error,
        TerminalHostTransportError::HandshakeRejected(
            TerminalHostHandshakeRejection::AuthenticationFailed
                | TerminalHostHandshakeRejection::HostMismatch
                | TerminalHostHandshakeRejection::ProtocolMismatch { .. }
        ) | TerminalHostTransportError::HostMismatch
            | TerminalHostTransportError::ProtocolMismatch { .. }
    )
}

fn terminal_host_response_rejection(response: &TerminalHostResponse) -> Option<&str> {
    match response {
        TerminalHostResponse::Error { message } | TerminalHostResponse::Unsupported { message } => {
            Some(message)
        }
        TerminalHostResponse::Heartbeat { .. }
        | TerminalHostResponse::Events { .. }
        | TerminalHostResponse::Sessions { .. }
        | TerminalHostResponse::Snapshot { .. }
        | TerminalHostResponse::Attachment { .. } => None,
    }
}

fn terminal_host_command_queue_error(
    error: async_channel::TrySendError<QueuedTerminalHostCommand>,
) -> anyhow::Error {
    match error {
        async_channel::TrySendError::Full(_) => {
            anyhow::anyhow!("terminal host is busy; wait for pending terminal operations and retry")
        }
        async_channel::TrySendError::Closed(_) => {
            anyhow::anyhow!("terminal host connection closed")
        }
    }
}

async fn enqueue_terminal_host_command(
    command_tx: &async_channel::Sender<QueuedTerminalHostCommand>,
    queued_command: QueuedTerminalHostCommand,
    executor: &BackgroundExecutor,
    operation: &'static str,
) -> anyhow::Result<()> {
    let send = command_tx.send(queued_command);
    let timer = executor.timer(TERMINAL_HOST_COMMAND_ENQUEUE_TIMEOUT);
    futures::pin_mut!(send, timer);
    match futures::future::select(send, timer).await {
        futures::future::Either::Left((result, _)) => result.map_err(|_| {
            anyhow::anyhow!("terminal host connection closed while queuing {operation}")
        }),
        futures::future::Either::Right(_) => anyhow::bail!(
            "terminal host is busy; {operation} was not queued within {:?}",
            TERMINAL_HOST_COMMAND_ENQUEUE_TIMEOUT
        ),
    }
}

async fn request_terminal_host_command(
    command_tx: &async_channel::Sender<QueuedTerminalHostCommand>,
    command: TerminalSessionCommand,
    executor: &BackgroundExecutor,
    operation: &'static str,
) -> anyhow::Result<TerminalHostResponse> {
    let (response_tx, response_rx) = futures::channel::oneshot::channel();
    let request = async {
        enqueue_terminal_host_command(
            command_tx,
            QueuedTerminalHostCommand::single(command, Some(response_tx)),
            executor,
            operation,
        )
        .await?;
        response_rx
            .await
            .map_err(|_| anyhow::anyhow!("terminal host {operation} response was dropped"))?
    };
    run_bounded_terminal_host_command_request(request, executor, operation).await
}

struct TransportHostedTerminalController {
    session_id: super::TerminalSessionId,
    command_tx: async_channel::Sender<QueuedTerminalHostCommand>,
    input_budget: Arc<TerminalHostInputBudget>,
    background_executor: BackgroundExecutor,
}

impl TransportHostedTerminalController {
    fn enqueue(&self, command: TerminalSessionCommand) -> anyhow::Result<()> {
        self.command_tx
            .try_send(QueuedTerminalHostCommand::single(command, None))
            .map_err(terminal_host_command_queue_error)
    }
}

impl HostedTerminalController for TransportHostedTerminalController {
    fn input(&self, bytes: Vec<u8>) -> anyhow::Result<()> {
        let Some(command) =
            QueuedTerminalHostCommand::input(self.session_id, bytes, &self.input_budget)?
        else {
            return Ok(());
        };
        self.command_tx
            .try_send(command)
            .map_err(terminal_host_command_queue_error)
    }

    fn resize(&self, columns: u16, rows: u16) -> anyhow::Result<()> {
        self.enqueue(TerminalSessionCommand::Resize {
            session_id: self.session_id,
            columns,
            rows,
        })
    }

    fn detach(&self) -> anyhow::Result<()> {
        self.enqueue(TerminalSessionCommand::Detach {
            session_id: self.session_id,
        })
    }

    fn terminate(&self) -> futures::future::BoxFuture<'static, anyhow::Result<()>> {
        let command_tx = self.command_tx.clone();
        let background_executor = self.background_executor.clone();
        let session_id = self.session_id;
        Box::pin(async move {
            let response = request_terminal_host_command(
                &command_tx,
                TerminalSessionCommand::Terminate { session_id },
                &background_executor,
                "termination",
            )
            .await?;
            match response {
                TerminalHostResponse::Snapshot { snapshot }
                    if matches!(snapshot.state, super::TerminalSessionState::Exited { .. }) =>
                {
                    Ok(())
                }
                TerminalHostResponse::Error { message }
                | TerminalHostResponse::Unsupported { message } => {
                    anyhow::bail!("terminal host could not end the session: {message}")
                }
                TerminalHostResponse::Snapshot { snapshot } => {
                    anyhow::bail!(
                        "terminal host did not confirm termination; current state is {:?}",
                        snapshot.state
                    )
                }
                TerminalHostResponse::Sessions { .. }
                | TerminalHostResponse::Attachment { .. }
                | TerminalHostResponse::Heartbeat { .. }
                | TerminalHostResponse::Events { .. } => {
                    anyhow::bail!("terminal host returned an invalid termination response")
                }
            }
        })
    }
}

impl TerminalHostTransportClient {
    pub async fn connect(
        socket_path: &Path,
        host_id: TerminalHostId,
        auth_token: TerminalHostAuthToken,
    ) -> Result<Self, TerminalHostTransportError> {
        let mut stream = UnixStream::connect(socket_path).await?;
        write_frame(
            &mut stream,
            &TerminalHostClientMessage::Hello {
                protocol_version: TERMINAL_SESSION_PROTOCOL_VERSION,
                host_id,
                auth_token,
                capabilities: TerminalHostCapabilities::current(),
            },
        )
        .await?;
        let capabilities = match read_frame::<_, TerminalHostServerMessage>(&mut stream).await? {
            TerminalHostServerMessage::HelloAccepted {
                protocol_version,
                host_id: accepted_host_id,
                capabilities,
            } => {
                if protocol_version != TERMINAL_SESSION_PROTOCOL_VERSION {
                    return Err(TerminalHostTransportError::ProtocolMismatch {
                        host_protocol: protocol_version,
                        client_protocol: TERMINAL_SESSION_PROTOCOL_VERSION,
                    });
                }
                if accepted_host_id != host_id {
                    return Err(TerminalHostTransportError::HostMismatch);
                }
                TerminalHostCapabilities::current().negotiate(capabilities)
            }
            TerminalHostServerMessage::HelloRejected { rejection } => {
                return Err(TerminalHostTransportError::HandshakeRejected(rejection));
            }
            TerminalHostServerMessage::Response { .. } => {
                return Err(TerminalHostTransportError::UnexpectedMessage);
            }
            TerminalHostServerMessage::EventBatch { .. } => {
                return Err(TerminalHostTransportError::UnexpectedMessage);
            }
        };
        Ok(Self {
            stream,
            next_request_id: 1,
            capabilities,
        })
    }

    pub fn capabilities(&self) -> TerminalHostCapabilities {
        self.capabilities
    }

    pub async fn command(
        &mut self,
        command: TerminalSessionCommand,
    ) -> Result<TerminalHostResponse, TerminalHostTransportError> {
        let request_id = self.next_request_id;
        self.next_request_id = request_id
            .checked_add(1)
            .ok_or(TerminalHostTransportError::RequestIdExhausted)?;
        write_frame(
            &mut self.stream,
            &TerminalHostClientMessage::Command {
                request_id,
                command,
            },
        )
        .await?;
        match read_frame::<_, TerminalHostServerMessage>(&mut self.stream).await? {
            TerminalHostServerMessage::Response {
                request_id: response_request_id,
                response,
            } if response_request_id == request_id => Ok(response),
            _ => Err(TerminalHostTransportError::UnexpectedMessage),
        }
    }
}

impl TerminalHostEventStream {
    pub async fn connect(
        socket_path: &Path,
        host_id: TerminalHostId,
        auth_token: TerminalHostAuthToken,
        after_cursor: Option<u64>,
    ) -> Result<Self, TerminalHostTransportError> {
        let client = TerminalHostTransportClient::connect(socket_path, host_id, auth_token).await?;
        if !client.capabilities.event_stream {
            return Err(TerminalHostTransportError::EventStreamUnsupported);
        }
        let mut stream = client.stream;
        write_frame(
            &mut stream,
            &TerminalHostClientMessage::SubscribeEvents { after_cursor },
        )
        .await?;
        Ok(Self { stream })
    }

    pub async fn next(&mut self) -> Result<TerminalHostResponse, TerminalHostTransportError> {
        match read_frame::<_, TerminalHostServerMessage>(&mut self.stream).await? {
            TerminalHostServerMessage::EventBatch {
                events,
                oldest_cursor,
                latest_cursor,
                truncated,
            } => Ok(TerminalHostResponse::Events {
                events,
                oldest_cursor,
                latest_cursor,
                truncated,
            }),
            _ => Err(TerminalHostTransportError::UnexpectedMessage),
        }
    }
}

pub async fn write_frame<W, T>(
    writer: &mut W,
    message: &T,
) -> Result<(), TerminalHostTransportError>
where
    W: AsyncWrite + Unpin,
    T: Serialize,
{
    let payload = serde_json::to_vec(message).map_err(TerminalHostTransportError::Serialize)?;
    if payload.len() > MAX_TERMINAL_HOST_FRAME_BYTES {
        return Err(TerminalHostTransportError::FrameTooLarge {
            actual: payload.len(),
            maximum: MAX_TERMINAL_HOST_FRAME_BYTES,
        });
    }
    let payload_length =
        u32::try_from(payload.len()).map_err(|_| TerminalHostTransportError::FrameTooLarge {
            actual: payload.len(),
            maximum: MAX_TERMINAL_HOST_FRAME_BYTES,
        })?;
    writer.write_all(&payload_length.to_be_bytes()).await?;
    writer.write_all(&payload).await?;
    writer.flush().await?;
    Ok(())
}

pub async fn read_frame<R, T>(reader: &mut R) -> Result<T, TerminalHostTransportError>
where
    R: AsyncRead + Unpin,
    T: DeserializeOwned,
{
    let mut length_bytes = [0; size_of::<u32>()];
    reader.read_exact(&mut length_bytes).await?;
    let payload_length = u32::from_be_bytes(length_bytes) as usize;
    if payload_length > MAX_TERMINAL_HOST_FRAME_BYTES {
        return Err(TerminalHostTransportError::FrameTooLarge {
            actual: payload_length,
            maximum: MAX_TERMINAL_HOST_FRAME_BYTES,
        });
    }
    let mut payload = vec![0; payload_length];
    reader.read_exact(&mut payload).await?;
    serde_json::from_slice(&payload).map_err(TerminalHostTransportError::Deserialize)
}

#[cfg(test)]
mod tests {
    use futures_lite::{future::block_on, io::Cursor};

    use super::*;

    fn token(value: char) -> Result<TerminalHostAuthToken, TerminalHostAuthTokenError> {
        TerminalHostAuthToken::parse(value.to_string().repeat(32))
    }

    #[test]
    fn packaged_dez_enables_host_owned_terminals_by_default() {
        assert!(terminal_host_enablement("Dez", None, true));
        assert!(!terminal_host_enablement("Dez", None, false));
        assert!(!terminal_host_enablement("Zed", None, true));
        assert!(terminal_host_enablement("Dez", Some("on"), false));
        assert!(terminal_host_enablement("Zed", Some("1"), false));
        assert!(!terminal_host_enablement("Dez", Some("off"), true));
        assert!(!terminal_host_enablement("Dez", Some("unexpected"), true));
    }

    #[test]
    fn auth_token_is_redacted_and_compared_without_early_exit() -> anyhow::Result<()> {
        let first = token('a')?;
        let same = token('a')?;
        let different = token('b')?;
        assert!(first.authenticated_eq(&same));
        assert!(!first.authenticated_eq(&different));
        assert_eq!(format!("{first:?}"), "TerminalHostAuthToken([REDACTED])");
        Ok(())
    }

    #[test]
    fn frame_round_trips() -> anyhow::Result<()> {
        block_on(async {
            let host_id = TerminalHostId::from_stable_key("transport-test");
            let message =
                TerminalHostServerMessage::accepted(host_id, TerminalHostCapabilities::current());
            let mut bytes = Cursor::new(Vec::new());
            write_frame(&mut bytes, &message).await?;
            bytes.set_position(0);
            let restored: TerminalHostServerMessage = read_frame(&mut bytes).await?;
            anyhow::ensure!(restored == message);
            anyhow::Ok(())
        })
    }

    #[test]
    fn event_subscription_and_batch_round_trip() -> anyhow::Result<()> {
        block_on(async {
            let subscribe = TerminalHostClientMessage::SubscribeEvents {
                after_cursor: Some(41),
            };
            let mut bytes = Cursor::new(Vec::new());
            write_frame(&mut bytes, &subscribe).await?;
            bytes.set_position(0);
            let restored: TerminalHostClientMessage = read_frame(&mut bytes).await?;
            anyhow::ensure!(restored == subscribe);

            let batch = TerminalHostServerMessage::EventBatch {
                events: Vec::new(),
                oldest_cursor: 42,
                latest_cursor: 41,
                truncated: false,
            };
            let mut bytes = Cursor::new(Vec::new());
            write_frame(&mut bytes, &batch).await?;
            bytes.set_position(0);
            let restored: TerminalHostServerMessage = read_frame(&mut bytes).await?;
            anyhow::ensure!(restored == batch);
            anyhow::Ok(())
        })
    }

    #[test]
    fn event_stream_failure_requires_a_fresh_authoritative_snapshot() {
        let mut event_cursor = Some(8_192);
        let mut needs_full_snapshot = false;

        require_authoritative_snapshot(&mut event_cursor, &mut needs_full_snapshot);

        assert_eq!(event_cursor, None);
        assert!(needs_full_snapshot);
    }

    #[gpui::test]
    fn snapshot_poll_failure_sets_reconnecting_state(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            TerminalHostStartupStatus::init(cx);
            let store = cx.new(|_| TerminalHostSnapshotStore {
                snapshots: Vec::new(),
                last_error: None,
                _poll_task: Task::ready(()),
            });
            store.update(cx, |store, cx| {
                mark_snapshot_store_reconnecting(store, "invalid snapshot response".to_owned(), cx);
            });

            assert_eq!(
                TerminalHostStartupStatus::state(cx),
                TerminalHostStartupState::Reconnecting {
                    message: "invalid snapshot response".to_owned(),
                }
            );
            assert_eq!(
                store.read(cx).last_error(),
                Some("invalid snapshot response")
            );
        });
    }

    #[test]
    fn old_session_list_without_an_event_cursor_stays_compatible() -> anyhow::Result<()> {
        let response: TerminalHostResponse =
            serde_json::from_str(r#"{"response":"sessions","sessions":[]}"#)?;
        let TerminalHostResponse::Sessions {
            latest_event_cursor,
            ..
        } = response
        else {
            anyhow::bail!("old response should still deserialize as a session list");
        };
        anyhow::ensure!(latest_event_cursor.is_none());
        Ok(())
    }

    #[test]
    fn oversized_frame_is_rejected_before_payload_allocation() -> anyhow::Result<()> {
        block_on(async {
            let announced_length = u32::try_from(MAX_TERMINAL_HOST_FRAME_BYTES + 1)?;
            let mut bytes = Cursor::new(announced_length.to_be_bytes().to_vec());
            let Err(error) = read_frame::<_, TerminalHostClientMessage>(&mut bytes).await else {
                anyhow::bail!("oversized frame should be rejected");
            };
            anyhow::ensure!(matches!(
                error,
                TerminalHostTransportError::FrameTooLarge { .. }
            ));
            anyhow::Ok(())
        })
    }

    #[gpui::test]
    async fn hosted_input_is_split_into_one_atomic_frame_safe_batch(
        background_executor: BackgroundExecutor,
    ) {
        let session_id = super::super::TerminalSessionId::new();
        let (command_tx, command_rx) = async_channel::bounded(1);
        let input_budget = Arc::new(TerminalHostInputBudget::new(
            MAX_TERMINAL_HOST_QUEUED_INPUT_BYTES,
        ));
        let controller = TransportHostedTerminalController {
            session_id,
            command_tx,
            input_budget,
            background_executor,
        };
        if let Err(error) = controller.input(vec![7; MAX_TERMINAL_HOST_INPUT_CHUNK_BYTES + 1]) {
            panic!("input batch should fit in an empty queue: {error:#}");
        }

        let Ok(queued) = command_rx.recv().await else {
            panic!("input batch should remain queued");
        };
        assert!(queued.response_tx.is_none());
        assert_eq!(queued.commands.len(), 2);
        let mut commands = queued.commands.into_iter();
        let Some(first) = commands.next() else {
            panic!("input batch did not contain its first chunk");
        };
        let Some(second) = commands.next() else {
            panic!("input batch did not contain its second chunk");
        };
        assert!(commands.next().is_none());
        let TerminalSessionCommand::Input {
            session_id: first_session_id,
            bytes: first_bytes,
        } = first
        else {
            panic!("first queued command was not terminal input");
        };
        let TerminalSessionCommand::Input {
            session_id: second_session_id,
            bytes: second_bytes,
        } = second
        else {
            panic!("second queued command was not terminal input");
        };
        assert_eq!(first_session_id, session_id);
        assert_eq!(second_session_id, session_id);
        assert_eq!(first_bytes.len(), MAX_TERMINAL_HOST_INPUT_CHUNK_BYTES);
        assert_eq!(second_bytes.len(), 1);
    }

    #[test]
    fn adjacent_hosted_input_for_one_session_coalesces_without_reordering() {
        let session_id = super::super::TerminalSessionId::new();
        let input_budget = Arc::new(TerminalHostInputBudget::new(32));
        let Some(mut first) =
            QueuedTerminalHostCommand::input(session_id, vec![0x1b, b'[', b'M'], &input_budget)
                .expect("first input should fit the queue budget")
        else {
            panic!("non-empty first input should create a queued command");
        };
        let Some(second) = QueuedTerminalHostCommand::input(session_id, vec![b'k'], &input_budget)
            .expect("second input should fit the queue budget")
        else {
            panic!("non-empty second input should create a queued command");
        };

        if first.try_append_adjacent_input(second).is_err() {
            panic!("adjacent input for one session should coalesce");
        }
        assert_eq!(first.commands.len(), 1);
        assert_eq!(first._input_reservations.len(), 2);
        assert_eq!(input_budget.queued_bytes(), 4);
        let Some(TerminalSessionCommand::Input {
            session_id: queued_session_id,
            bytes,
        }) = first.commands.first()
        else {
            panic!("coalesced command should remain terminal input");
        };
        assert_eq!(*queued_session_id, session_id);
        assert_eq!(bytes, &[0x1b, b'[', b'M', b'k']);

        drop(first);
        assert_eq!(input_budget.queued_bytes(), 0);
    }

    #[test]
    fn hosted_input_coalescing_preserves_session_and_command_boundaries() {
        let session_id = super::super::TerminalSessionId::new();
        let other_session_id = super::super::TerminalSessionId::new();
        let input_budget = Arc::new(TerminalHostInputBudget::new(32));
        let Some(mut first) =
            QueuedTerminalHostCommand::input(session_id, vec![b'a'], &input_budget)
                .expect("first input should fit the queue budget")
        else {
            panic!("non-empty first input should create a queued command");
        };
        let Some(other_session) =
            QueuedTerminalHostCommand::input(other_session_id, vec![b'b'], &input_budget)
                .expect("other-session input should fit the queue budget")
        else {
            panic!("non-empty other-session input should create a queued command");
        };

        let Err(other_session) = first.try_append_adjacent_input(other_session) else {
            panic!("input from another session must remain a separate queue item");
        };
        assert_eq!(first.commands.len(), 1);
        assert_eq!(other_session.commands.len(), 1);

        let resize = QueuedTerminalHostCommand::single(
            TerminalSessionCommand::Resize {
                session_id,
                columns: 120,
                rows: 40,
            },
            None,
        );
        let Err(resize) = first.try_append_adjacent_input(resize) else {
            panic!("resize must remain an ordering boundary for terminal input");
        };
        assert!(matches!(
            resize.commands.first(),
            Some(TerminalSessionCommand::Resize {
                session_id: resize_session_id,
                columns: 120,
                rows: 40,
            }) if *resize_session_id == session_id
        ));
    }

    #[gpui::test]
    async fn hosted_input_is_not_partially_enqueued_when_the_queue_is_full(
        background_executor: BackgroundExecutor,
    ) {
        let session_id = super::super::TerminalSessionId::new();
        let (command_tx, command_rx) = async_channel::bounded(1);
        let input_budget = Arc::new(TerminalHostInputBudget::new(
            MAX_TERMINAL_HOST_QUEUED_INPUT_BYTES,
        ));
        if command_tx
            .try_send(QueuedTerminalHostCommand::single(
                TerminalSessionCommand::Detach { session_id },
                None,
            ))
            .is_err()
        {
            panic!("empty queue should accept the retained command");
        }
        let controller = TransportHostedTerminalController {
            session_id,
            command_tx,
            input_budget: input_budget.clone(),
            background_executor,
        };

        let error = controller
            .input(vec![7; MAX_TERMINAL_HOST_INPUT_CHUNK_BYTES + 1])
            .expect_err("a full queue should reject the complete input batch");
        assert!(error.to_string().contains("terminal host is busy"));
        let Ok(retained) = command_rx.try_recv() else {
            panic!("the original queued command should be retained");
        };
        assert_eq!(retained.commands.len(), 1);
        assert!(matches!(
            retained.commands.first(),
            Some(TerminalSessionCommand::Detach { session_id: retained_id })
                if *retained_id == session_id
        ));
        assert!(matches!(
            command_rx.try_recv(),
            Err(async_channel::TryRecvError::Empty)
        ));
        assert_eq!(input_budget.queued_bytes(), 0);
    }

    #[gpui::test]
    async fn hosted_input_rejects_a_batch_larger_than_the_pty_budget(
        background_executor: BackgroundExecutor,
    ) {
        let session_id = super::super::TerminalSessionId::new();
        let (command_tx, command_rx) = async_channel::bounded(1);
        let input_budget = Arc::new(TerminalHostInputBudget::new(
            MAX_TERMINAL_HOST_QUEUED_INPUT_BYTES,
        ));
        let controller = TransportHostedTerminalController {
            session_id,
            command_tx,
            input_budget,
            background_executor,
        };

        let error = controller
            .input(vec![0; MAX_TERMINAL_HOST_INPUT_BATCH_BYTES + 1])
            .expect_err("an oversized input batch should be rejected before enqueue");
        assert!(error.to_string().contains("terminal host limit"));
        assert!(matches!(
            command_rx.try_recv(),
            Err(async_channel::TryRecvError::Empty)
        ));
    }

    #[test]
    fn hosted_input_budget_bounds_all_queued_batches_and_releases_on_drop() {
        let session_id = super::super::TerminalSessionId::new();
        let input_budget = Arc::new(TerminalHostInputBudget::new(3));
        let first = QueuedTerminalHostCommand::input(session_id, vec![1, 2], &input_budget)
            .expect("first input should fit the aggregate budget")
            .expect("non-empty input should create a queued command");
        assert_eq!(input_budget.queued_bytes(), 2);

        let Err(error) = QueuedTerminalHostCommand::input(session_id, vec![3, 4], &input_budget)
        else {
            panic!("aggregate input above the budget should be rejected");
        };
        assert!(error.to_string().contains("input queue is full"));
        assert_eq!(input_budget.queued_bytes(), 2);

        drop(first);
        assert_eq!(input_budget.queued_bytes(), 0);
        let replacement =
            QueuedTerminalHostCommand::input(session_id, vec![5, 6, 7], &input_budget)
                .expect("released budget should be reusable")
                .expect("non-empty input should create a queued command");
        assert_eq!(input_budget.queued_bytes(), 3);
        drop(replacement);
        assert_eq!(input_budget.queued_bytes(), 0);
    }

    #[gpui::test]
    async fn awaited_command_enqueue_has_a_deadline(background_executor: BackgroundExecutor) {
        let session_id = super::super::TerminalSessionId::new();
        let (command_tx, command_rx) = async_channel::bounded(1);
        if command_tx
            .try_send(QueuedTerminalHostCommand::single(
                TerminalSessionCommand::Detach { session_id },
                None,
            ))
            .is_err()
        {
            panic!("empty queue should accept the retained command");
        }

        let error = enqueue_terminal_host_command(
            &command_tx,
            QueuedTerminalHostCommand::single(
                TerminalSessionCommand::AcknowledgeAgentAttention { session_id },
                None,
            ),
            &background_executor,
            "test command",
        )
        .await
        .expect_err("a saturated command queue should time out");
        assert!(error.to_string().contains("was not queued"));
        let Ok(retained) = command_rx.try_recv() else {
            panic!("the original queued command should be retained");
        };
        assert_eq!(retained.commands.len(), 1);
        assert!(matches!(
            command_rx.try_recv(),
            Err(async_channel::TryRecvError::Empty)
        ));
    }

    #[gpui::test]
    async fn command_request_deadline_includes_time_waiting_for_a_response(
        background_executor: BackgroundExecutor,
    ) {
        let session_id = super::super::TerminalSessionId::new();
        let (command_tx, command_rx) = async_channel::bounded(1);
        let error = request_terminal_host_command(
            &command_tx,
            TerminalSessionCommand::Detach { session_id },
            &background_executor,
            "test command",
        )
        .await
        .expect_err("a command without a transport worker should time out");
        assert!(error.to_string().contains("request was canceled"));

        let Ok(queued) = command_rx.try_recv() else {
            panic!("timed-out request should remain queued for cancellation");
        };
        let Some(response_tx) = queued.response_tx else {
            panic!("request did not include a response channel");
        };
        assert!(response_tx.is_canceled());
    }

    #[test]
    fn permanent_reconnect_failures_do_not_spin() {
        assert!(reconnect_error_is_permanent(
            &TerminalHostTransportError::HandshakeRejected(
                TerminalHostHandshakeRejection::AuthenticationFailed,
            ),
        ));
        assert!(reconnect_error_is_permanent(
            &TerminalHostTransportError::ProtocolMismatch {
                host_protocol: 2,
                client_protocol: 3,
            },
        ));
        assert!(!reconnect_error_is_permanent(
            &TerminalHostTransportError::Io(io::Error::new(
                io::ErrorKind::ConnectionRefused,
                "helper is restarting",
            )),
        ));
    }
}
