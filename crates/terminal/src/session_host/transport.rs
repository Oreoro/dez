use std::{fmt, io, path::Path};

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
const TERMINAL_HOST_RECONNECT_ATTEMPTS: usize = 8;
const TERMINAL_HOST_RECONNECT_INTERVAL: std::time::Duration = std::time::Duration::from_millis(250);
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

struct QueuedTerminalHostCommand {
    command: TerminalSessionCommand,
    response_tx:
        Option<futures::channel::oneshot::Sender<Result<TerminalHostResponse, anyhow::Error>>>,
}

/// Shared, ordered command path for all terminal surfaces attached to one
/// authenticated helper connection.
pub struct TerminalHostConnection {
    host_id: TerminalHostId,
    capabilities: TerminalHostCapabilities,
    socket_path: std::path::PathBuf,
    auth_token: TerminalHostAuthToken,
    command_tx: async_channel::Sender<QueuedTerminalHostCommand>,
    _transport_task: Task<()>,
}

struct GlobalTerminalHostConnection(std::sync::Arc<TerminalHostConnection>);

impl Global for GlobalTerminalHostConnection {}

struct GlobalTerminalHostSnapshotStore(Entity<TerminalHostSnapshotStore>);

impl Global for GlobalTerminalHostSnapshotStore {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TerminalHostStartupState {
    Disabled,
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
                        Ok(TerminalHostResponse::Sessions { .. }) => {
                            needs_full_snapshot = !(supports_event_stream || supports_event_cursor);
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
                        _ => {}
                    }
                    let next_poll_interval = store
                        .update(cx, |store, cx| {
                            let previous_snapshots = store.snapshots.clone();
                            let previous_error = store.last_error.clone();
                            let mut poll_interval;
                            match response {
                                Ok(TerminalHostResponse::Sessions { sessions }) => {
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
                                Ok(_) => {
                                    store.last_error = Some(
                                        "terminal host returned an invalid snapshot response"
                                            .to_owned(),
                                    );
                                    poll_interval = TERMINAL_HOST_ERROR_POLL_INTERVAL;
                                }
                                Err(error) => {
                                    let message = format!("{error:#}");
                                    store.last_error = Some(message.clone());
                                    TerminalHostStartupStatus::set(
                                        TerminalHostStartupState::Reconnecting { message },
                                        cx,
                                    );
                                    for snapshot in &mut store.snapshots {
                                        if snapshot.state.may_be_live() {
                                            snapshot.state =
                                                super::TerminalSessionState::Reconnecting;
                                        }
                                    }
                                    poll_interval = TERMINAL_HOST_ERROR_POLL_INTERVAL;
                                }
                            }
                            if store.snapshots != previous_snapshots
                                || store.last_error != previous_error
                            {
                                cx.notify();
                                TerminalHostSnapshotRevision::bump(cx);
                            }
                            if supports_event_stream && store.last_error.is_none() {
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
        socket_path: &Path,
        host_id: TerminalHostId,
        auth_token: TerminalHostAuthToken,
        background_executor: &BackgroundExecutor,
    ) -> Result<Self, TerminalHostTransportError> {
        let client =
            TerminalHostTransportClient::connect(socket_path, host_id, auth_token.clone()).await?;
        let capabilities = client.capabilities();
        let (command_tx, command_rx) = async_channel::unbounded::<QueuedTerminalHostCommand>();
        let event_socket_path = socket_path.to_path_buf();
        let event_auth_token = auth_token.clone();
        let command_socket_path = socket_path.to_path_buf();
        let executor = background_executor.clone();
        let transport_task = background_executor.spawn(async move {
            let mut client = Some(client);
            while let Ok(queued) = command_rx.recv().await {
                let mut reconnect_error = None;
                for attempt in 0..TERMINAL_HOST_RECONNECT_ATTEMPTS {
                    if client.is_some() {
                        break;
                    }
                    match TerminalHostTransportClient::connect(
                        &command_socket_path,
                        host_id,
                        auth_token.clone(),
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
                                executor.timer(TERMINAL_HOST_RECONNECT_INTERVAL).await;
                            }
                        }
                    }
                }
                let result = match client.as_mut() {
                    Some(client) => client
                        .command(queued.command)
                        .await
                        .map_err(anyhow::Error::from),
                    None => match reconnect_error {
                        Some(error) => Err(anyhow::anyhow!(
                            "terminal host connection is still unavailable: {error}"
                        )),
                        None => Err(anyhow::anyhow!("terminal host connection unavailable")),
                    },
                };
                if result.is_err() {
                    // The request may have reached the helper, so never replay
                    // it automatically. Reconnect before the next ordered
                    // command and let callers reconcile through snapshots.
                    client = None;
                }
                if let Some(response_tx) = queued.response_tx {
                    if response_tx.send(result).is_err() {
                        log::debug!("terminal host command response receiver was dropped");
                    }
                } else if let Err(error) = result {
                    log::warn!("terminal host command failed: {error:#}");
                }
            }
        });
        Ok(Self {
            host_id,
            capabilities,
            socket_path: event_socket_path,
            auth_token: event_auth_token,
            command_tx,
            _transport_task: transport_task,
        })
    }

    pub fn host_id(&self) -> TerminalHostId {
        self.host_id
    }

    pub fn capabilities(&self) -> TerminalHostCapabilities {
        self.capabilities
    }

    async fn event_stream(
        &self,
        after_cursor: Option<u64>,
    ) -> Result<TerminalHostEventStream, TerminalHostTransportError> {
        TerminalHostEventStream::connect(
            &self.socket_path,
            self.host_id,
            self.auth_token.clone(),
            after_cursor,
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
        stream
            .as_mut()
            .expect("event stream is connected")
            .next()
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn command(
        &self,
        command: TerminalSessionCommand,
    ) -> anyhow::Result<TerminalHostResponse> {
        let (response_tx, response_rx) = futures::channel::oneshot::channel();
        self.command_tx
            .send(QueuedTerminalHostCommand {
                command,
                response_tx: Some(response_tx),
            })
            .await
            .map_err(|_| anyhow::anyhow!("terminal host connection closed"))?;
        response_rx
            .await
            .map_err(|_| anyhow::anyhow!("terminal host command response was dropped"))?
    }

    pub fn controller(
        &self,
        session_id: super::TerminalSessionId,
    ) -> std::sync::Arc<dyn HostedTerminalController> {
        std::sync::Arc::new(TransportHostedTerminalController {
            session_id,
            command_tx: self.command_tx.clone(),
        })
    }

    pub fn acknowledge_agent_attention(&self, session_id: super::TerminalSessionId) {
        if let Err(error) = self.command_tx.try_send(QueuedTerminalHostCommand {
            command: TerminalSessionCommand::AcknowledgeAgentAttention { session_id },
            response_tx: None,
        }) {
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
                TerminalEvent::TitleChanged | TerminalEvent::BreadcrumbsChanged
            ) {
                return;
            }
            let terminal = terminal.read(cx);
            let command = TerminalSessionCommand::UpdateMetadata {
                session_id,
                title: Some(terminal.title(true)),
                working_directory: terminal.working_directory(),
            };
            if let Err(error) = connection.command_tx.try_send(QueuedTerminalHostCommand {
                command,
                response_tx: None,
            }) {
                log::debug!("failed to queue hosted terminal metadata update: {error}");
            }
        })
        .detach();
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
                    Ok(response) => {
                        if reconnecting {
                            terminal.update(cx, |terminal, cx| {
                                terminal.write_output(
                                    b"\r\n[Dez: terminal host reconnected]\r\n",
                                    cx,
                                );
                            })?;
                            reconnecting = false;
                        }
                        response
                    }
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

                let replay_was_truncated = attachment.replay_was_truncated;
                let state = attachment.snapshot.state;
                let dimensions = attachment.snapshot.dimensions;
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
                    terminal.finish_hosted_replay(dimensions, cx);
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
                    .timer(std::time::Duration::from_millis(32))
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

struct TransportHostedTerminalController {
    session_id: super::TerminalSessionId,
    command_tx: async_channel::Sender<QueuedTerminalHostCommand>,
}

impl TransportHostedTerminalController {
    fn enqueue(&self, command: TerminalSessionCommand) -> anyhow::Result<()> {
        self.command_tx
            .try_send(QueuedTerminalHostCommand {
                command,
                response_tx: None,
            })
            .map_err(|_| anyhow::anyhow!("terminal host connection closed"))
    }
}

impl HostedTerminalController for TransportHostedTerminalController {
    fn input(&self, bytes: Vec<u8>) -> anyhow::Result<()> {
        for chunk in bytes.chunks(MAX_TERMINAL_HOST_INPUT_CHUNK_BYTES) {
            self.enqueue(TerminalSessionCommand::Input {
                session_id: self.session_id,
                bytes: chunk.to_vec(),
            })?;
        }
        Ok(())
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

    fn terminate(&self) -> anyhow::Result<()> {
        self.enqueue(TerminalSessionCommand::Terminate {
            session_id: self.session_id,
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

    #[test]
    fn hosted_input_is_split_into_frame_safe_commands() -> anyhow::Result<()> {
        block_on(async {
            let session_id = super::super::TerminalSessionId::new();
            let (command_tx, command_rx) = async_channel::unbounded();
            let controller = TransportHostedTerminalController {
                session_id,
                command_tx,
            };
            controller.input(vec![7; MAX_TERMINAL_HOST_INPUT_CHUNK_BYTES + 1])?;

            let first = command_rx.recv().await?;
            let second = command_rx.recv().await?;
            let TerminalSessionCommand::Input {
                session_id: first_session_id,
                bytes: first_bytes,
            } = first.command
            else {
                anyhow::bail!("first queued command was not terminal input");
            };
            let TerminalSessionCommand::Input {
                session_id: second_session_id,
                bytes: second_bytes,
            } = second.command
            else {
                anyhow::bail!("second queued command was not terminal input");
            };
            anyhow::ensure!(first_session_id == session_id);
            anyhow::ensure!(second_session_id == session_id);
            anyhow::ensure!(first_bytes.len() == MAX_TERMINAL_HOST_INPUT_CHUNK_BYTES);
            anyhow::ensure!(second_bytes.len() == 1);
            anyhow::Ok(())
        })
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
