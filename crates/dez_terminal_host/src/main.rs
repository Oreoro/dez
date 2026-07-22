use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, Read as _},
    path::PathBuf,
    sync::{Arc, Mutex, MutexGuard},
};

use anyhow::{Context as _, Result};
use clap::{Parser, Subcommand};
use net::async_net::{UnixListener, UnixStream};
use terminal::session_host::{
    InProcessTerminalHost, TERMINAL_SESSION_PROTOCOL_VERSION, TerminalAgentEventKind,
    TerminalAgentState, TerminalAgentUpdate, TerminalHostCapabilities, TerminalHostEventEnvelope,
    TerminalHostId, TerminalHostPtyEvent, TerminalHostPtyHandle, TerminalSessionCommand,
    TerminalSessionEvent, TerminalSessionId, TerminalSessionSnapshot, TerminalSessionState,
    transport::{
        TERMINAL_HOST_ID_ENV, TERMINAL_HOST_SOCKET_ENV, TERMINAL_HOST_TOKEN_FILE_ENV,
        TERMINAL_SESSION_ID_ENV, TerminalHostAuthToken, TerminalHostClientMessage,
        TerminalHostHandshakeRejection, TerminalHostResponse, TerminalHostServerMessage,
        TerminalHostTransportClient, TerminalHostTransportError, read_frame, write_frame,
    },
};

// JSON represents byte vectors as integer arrays, so keep raw replay well
// below the 1 MiB transport frame ceiling even for worst-case byte values.
const DEFAULT_REPLAY_LIMIT_BYTES: usize = 128 * 1024;
const MAX_HOST_EVENTS: usize = 512;
const MAX_EVENTS_PER_RESPONSE: usize = 8;

#[derive(Clone, Default)]
struct TerminalHostEventNotifier {
    subscribers: Arc<Mutex<Vec<async_channel::Sender<()>>>>,
}

impl TerminalHostEventNotifier {
    fn subscribe(&self) -> async_channel::Receiver<()> {
        let (sender, receiver) = async_channel::bounded(1);
        match self.subscribers.lock() {
            Ok(mut subscribers) => subscribers.push(sender),
            Err(poisoned) => poisoned.into_inner().push(sender),
        }
        receiver
    }

    fn notify(&self) {
        let mut subscribers = match self.subscribers.lock() {
            Ok(subscribers) => subscribers,
            Err(poisoned) => poisoned.into_inner(),
        };
        subscribers.retain(|subscriber| match subscriber.try_send(()) {
            Ok(()) | Err(async_channel::TrySendError::Full(())) => true,
            Err(async_channel::TrySendError::Closed(())) => false,
        });
    }
}

#[derive(Debug, Parser)]
#[command(name = "dez-terminal-host", disable_version_flag = true)]
struct Arguments {
    #[command(subcommand)]
    command: HostCommand,
}

#[derive(Debug, Subcommand)]
enum HostCommand {
    /// Own local terminal processes and serve authenticated GUI clients.
    Serve(ServeArguments),
    /// Read one structured terminal-agent event from stdin.
    AgentEvent,
}

#[derive(Debug, clap::Args)]
struct ServeArguments {
    #[arg(long)]
    socket: PathBuf,
    #[arg(long)]
    token_file: PathBuf,
    #[arg(long)]
    host_id: TerminalHostId,
    #[arg(long, default_value_t = DEFAULT_REPLAY_LIMIT_BYTES)]
    replay_limit_bytes: usize,
}

fn main() -> Result<()> {
    match Arguments::parse().command {
        HostCommand::Serve(arguments) => run_server(arguments),
        HostCommand::AgentEvent => report_agent_event(),
    }
}

fn run_server(arguments: ServeArguments) -> Result<()> {
    anyhow::ensure!(
        arguments.replay_limit_bytes <= DEFAULT_REPLAY_LIMIT_BYTES,
        "terminal replay limit exceeds the frame-safe maximum of {DEFAULT_REPLAY_LIMIT_BYTES} bytes"
    );
    let auth_token = read_auth_token(&arguments.token_file)?;
    validate_private_parent(&arguments.socket)?;
    let listener = UnixListener::bind(&arguments.socket)
        .with_context(|| format!("bind terminal host socket {}", arguments.socket.display()))?;
    make_socket_private(&arguments.socket)?;
    let socket_guard = BoundSocketGuard::new(arguments.socket)?;
    let result = smol::block_on(serve(
        listener,
        arguments.host_id,
        auth_token,
        arguments.replay_limit_bytes,
    ));
    drop(socket_guard);
    result
}

const MAX_AGENT_HOOK_BYTES: u64 = 256 * 1024;
const MAX_AGENT_FIELD_BYTES: usize = 4096;
const MAX_AGENT_FILE_TARGETS: usize = 64;

#[derive(Debug, serde::Deserialize)]
struct CodexHookEvent {
    session_id: String,
    cwd: PathBuf,
    hook_event_name: String,
    #[serde(default)]
    turn_id: Option<String>,
    #[serde(default)]
    tool_name: Option<String>,
    #[serde(default)]
    tool_input: Option<serde_json::Value>,
    #[serde(default)]
    tool_response: Option<serde_json::Value>,
}

fn report_agent_event() -> Result<()> {
    let mut input = Vec::new();
    std::io::stdin()
        .take(MAX_AGENT_HOOK_BYTES + 1)
        .read_to_end(&mut input)
        .context("read terminal-agent hook event")?;
    anyhow::ensure!(
        input.len() <= MAX_AGENT_HOOK_BYTES as usize,
        "terminal-agent hook event exceeds {MAX_AGENT_HOOK_BYTES} bytes"
    );
    let event: CodexHookEvent = serde_json::from_slice(&input).context("parse Codex hook event")?;
    let update = codex_hook_update(&event)?;

    let socket = required_env_path(TERMINAL_HOST_SOCKET_ENV)?;
    let token_file = required_env_path(TERMINAL_HOST_TOKEN_FILE_ENV)?;
    let host_id = required_env(TERMINAL_HOST_ID_ENV)?
        .parse::<TerminalHostId>()
        .context("parse terminal host identity from environment")?;
    let session_id = required_env(TERMINAL_SESSION_ID_ENV)?
        .parse::<TerminalSessionId>()
        .context("parse terminal session identity from environment")?;
    let auth_token = read_auth_token(&token_file)?;

    smol::block_on(async move {
        let mut client = TerminalHostTransportClient::connect(&socket, host_id, auth_token)
            .await
            .context("connect structured terminal-agent hook to Dez host")?;
        match client
            .command(TerminalSessionCommand::UpdateAgent { session_id, update })
            .await
            .context("send structured terminal-agent update")?
        {
            TerminalHostResponse::Snapshot { .. } => Ok(()),
            TerminalHostResponse::Error { message }
            | TerminalHostResponse::Unsupported { message } => {
                anyhow::bail!("terminal host rejected agent update: {message}")
            }
            TerminalHostResponse::Sessions { .. }
            | TerminalHostResponse::Attachment { .. }
            | TerminalHostResponse::Heartbeat { .. }
            | TerminalHostResponse::Events { .. } => {
                anyhow::bail!("terminal host returned an invalid agent-update response")
            }
        }
    })
}

fn required_env(name: &str) -> Result<String> {
    std::env::var(name).with_context(|| format!("{name} is not available in this terminal"))
}

fn required_env_path(name: &str) -> Result<PathBuf> {
    required_env(name).map(PathBuf::from)
}

fn codex_hook_update(event: &CodexHookEvent) -> Result<TerminalAgentUpdate> {
    anyhow::ensure!(
        !event.session_id.is_empty() && event.session_id.len() <= 256,
        "Codex hook session identity is invalid"
    );
    let tool_name = event
        .tool_name
        .as_deref()
        .filter(|name| !name.is_empty())
        .unwrap_or("tool");
    let (state, attention_required, event_kind, summary) = match event.hook_event_name.as_str() {
        "SessionStart" => (
            TerminalAgentState::Starting,
            false,
            TerminalAgentEventKind::SessionStarted,
            "Codex session started".to_owned(),
        ),
        "UserPromptSubmit" => (
            TerminalAgentState::Running,
            false,
            TerminalAgentEventKind::PromptSubmitted,
            "User submitted a Codex turn".to_owned(),
        ),
        "PermissionRequest" => (
            TerminalAgentState::WaitingForPermission,
            true,
            TerminalAgentEventKind::PermissionRequested,
            format!("Codex requested permission for {tool_name}"),
        ),
        "PreToolUse" => (
            TerminalAgentState::Running,
            false,
            TerminalAgentEventKind::ToolStarted,
            format!("Codex started {tool_name}"),
        ),
        "PostToolUse" => (
            TerminalAgentState::Running,
            false,
            TerminalAgentEventKind::ToolFinished,
            format!("Codex finished {tool_name}"),
        ),
        "Stop" => (
            TerminalAgentState::Completed,
            true,
            TerminalAgentEventKind::TurnCompleted,
            "Codex turn completed and is ready for review".to_owned(),
        ),
        other => anyhow::bail!("unsupported Codex hook event {other}"),
    };
    let command = matches!(event.hook_event_name.as_str(), "PreToolUse" | "PostToolUse")
        .then(|| event.tool_input.as_ref().and_then(extract_command))
        .flatten();
    let exit_code = (event.hook_event_name == "PostToolUse")
        .then(|| event.tool_response.as_ref().and_then(extract_exit_code))
        .flatten();
    let (file_targets, file_targets_truncated) = extract_file_targets(event);
    let cwd = bounded_text(event.cwd.to_string_lossy().as_ref());
    let turn = event
        .turn_id
        .as_deref()
        .map(bounded_text)
        .unwrap_or_else(|| "unknown".to_owned());

    Ok(TerminalAgentUpdate {
        adapter: "codex-hooks-v1".to_owned(),
        actor: "Codex".to_owned(),
        capabilities: terminal::session_host::TerminalAgentCapabilities::codex_hooks_v1(),
        provider_session_id: Some(event.session_id.clone()),
        state,
        attention_required,
        resumable: true,
        event_kind,
        summary: bounded_text(&format!("{summary} · turn {turn} · {cwd}")),
        working_directory: Some(PathBuf::from(cwd)),
        file_targets,
        file_targets_truncated,
        command,
        exit_code,
    })
}

fn extract_command(value: &serde_json::Value) -> Option<String> {
    value
        .get("command")
        .and_then(serde_json::Value::as_str)
        .map(bounded_text)
}

fn extract_exit_code(value: &serde_json::Value) -> Option<i32> {
    value
        .get("exit_code")
        .or_else(|| value.get("exitCode"))
        .and_then(serde_json::Value::as_i64)
        .and_then(|code| i32::try_from(code).ok())
}

fn extract_file_targets(event: &CodexHookEvent) -> (Vec<PathBuf>, bool) {
    if event.hook_event_name != "PostToolUse" {
        return (Vec::new(), false);
    }
    let tool_name = event
        .tool_name
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();
    let reports_file_mutation = [
        "apply_patch",
        "patch",
        "write",
        "edit",
        "create_file",
        "delete_file",
    ]
    .iter()
    .any(|marker| tool_name.contains(marker));
    if !reports_file_mutation {
        return (Vec::new(), false);
    }

    let mut raw_paths = Vec::new();
    if let Some(tool_input) = &event.tool_input {
        collect_file_target_fields(tool_input, &mut raw_paths);
        collect_patch_file_targets(tool_input, &mut raw_paths);
    }
    let mut paths = raw_paths
        .into_iter()
        .filter_map(|path| resolve_file_target(&event.cwd, &path))
        .collect::<Vec<_>>();
    paths.sort();
    paths.dedup();
    let truncated = paths.len() > MAX_AGENT_FILE_TARGETS;
    paths.truncate(MAX_AGENT_FILE_TARGETS);
    (paths, truncated)
}

fn collect_file_target_fields(value: &serde_json::Value, paths: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(fields) => {
            for (key, value) in fields {
                if matches!(key.as_str(), "file_path" | "filePath" | "path")
                    && let Some(path) = value.as_str()
                {
                    paths.push(path.to_owned());
                } else {
                    collect_file_target_fields(value, paths);
                }
            }
        }
        serde_json::Value::Array(values) => {
            for value in values {
                collect_file_target_fields(value, paths);
            }
        }
        _ => {}
    }
}

fn collect_patch_file_targets(value: &serde_json::Value, paths: &mut Vec<String>) {
    match value {
        serde_json::Value::String(value) => {
            for line in value.lines() {
                let path = [
                    "*** Add File: ",
                    "*** Update File: ",
                    "*** Delete File: ",
                    "*** Move to: ",
                    "+++ b/",
                    "--- a/",
                ]
                .iter()
                .find_map(|prefix| line.strip_prefix(prefix));
                if let Some(path) = path {
                    paths.push(path.to_owned());
                }
            }
        }
        serde_json::Value::Object(fields) => {
            for value in fields.values() {
                collect_patch_file_targets(value, paths);
            }
        }
        serde_json::Value::Array(values) => {
            for value in values {
                collect_patch_file_targets(value, paths);
            }
        }
        _ => {}
    }
}

fn resolve_file_target(cwd: &std::path::Path, value: &str) -> Option<PathBuf> {
    let value = value
        .trim()
        .trim_matches(|character| matches!(character, '\'' | '"'));
    if value.is_empty()
        || value == "/dev/null"
        || value.len() > MAX_AGENT_FIELD_BYTES
        || value
            .chars()
            .any(|character| matches!(character, '\0' | '\r' | '\n'))
    {
        return None;
    }
    let path = PathBuf::from(value);
    Some(if path.is_absolute() {
        path
    } else {
        cwd.join(path)
    })
}

fn bounded_text(value: &str) -> String {
    if value.len() <= MAX_AGENT_FIELD_BYTES {
        return value.to_owned();
    }
    let mut end = MAX_AGENT_FIELD_BYTES;
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}…", &value[..end])
}

struct BoundSocketGuard {
    path: PathBuf,
    device: u64,
    inode: u64,
}

impl BoundSocketGuard {
    fn new(path: PathBuf) -> Result<Self> {
        use std::os::unix::fs::{FileTypeExt as _, MetadataExt as _};

        let metadata = std::fs::symlink_metadata(&path)
            .with_context(|| format!("inspect bound terminal host socket {}", path.display()))?;
        anyhow::ensure!(
            metadata.file_type().is_socket(),
            "bound terminal host path is not a socket"
        );
        Ok(Self {
            path,
            device: metadata.dev(),
            inode: metadata.ino(),
        })
    }
}

impl Drop for BoundSocketGuard {
    fn drop(&mut self) {
        use std::os::unix::fs::{FileTypeExt as _, MetadataExt as _};

        let metadata = match std::fs::symlink_metadata(&self.path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return,
            Err(error) => {
                eprintln!(
                    "failed to inspect terminal host socket {} during cleanup: {error}",
                    self.path.display()
                );
                return;
            }
        };
        if !metadata.file_type().is_socket()
            || metadata.dev() != self.device
            || metadata.ino() != self.inode
        {
            eprintln!(
                "refusing to remove replaced terminal host socket path {}",
                self.path.display()
            );
            return;
        }
        if let Err(error) = std::fs::remove_file(&self.path) {
            eprintln!(
                "failed to remove terminal host socket {}: {error}",
                self.path.display()
            );
        }
    }
}

fn read_auth_token(path: &std::path::Path) -> Result<TerminalHostAuthToken> {
    validate_private_file(path)?;
    let token = std::fs::read_to_string(path)
        .with_context(|| format!("read terminal host token file {}", path.display()))?;
    TerminalHostAuthToken::parse(token.trim().to_owned()).context("parse terminal host token")
}

#[cfg(unix)]
fn validate_private_file(path: &std::path::Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt as _;

    let metadata = std::fs::symlink_metadata(path)
        .with_context(|| format!("inspect terminal host token file {}", path.display()))?;
    anyhow::ensure!(
        metadata.file_type().is_file(),
        "terminal host token path is not a regular file"
    );
    anyhow::ensure!(
        metadata.permissions().mode() & 0o077 == 0,
        "terminal host token file must not be accessible by group or other users"
    );
    Ok(())
}

#[cfg(not(unix))]
fn validate_private_file(path: &std::path::Path) -> Result<()> {
    anyhow::ensure!(path.is_file(), "terminal host token path is not a file");
    Ok(())
}

#[cfg(unix)]
fn validate_private_parent(socket: &std::path::Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt as _;

    let parent = socket
        .parent()
        .context("terminal host socket must have a parent directory")?;
    let metadata = std::fs::symlink_metadata(parent).with_context(|| {
        format!(
            "inspect terminal host socket directory {}",
            parent.display()
        )
    })?;
    anyhow::ensure!(
        metadata.file_type().is_dir(),
        "terminal host socket parent is not a real directory"
    );
    anyhow::ensure!(
        metadata.permissions().mode() & 0o077 == 0,
        "terminal host socket directory must not be accessible by group or other users"
    );
    Ok(())
}

#[cfg(not(unix))]
fn validate_private_parent(socket: &std::path::Path) -> Result<()> {
    let parent = socket
        .parent()
        .context("terminal host socket must have a parent directory")?;
    anyhow::ensure!(
        parent.is_dir(),
        "terminal host socket parent is not a directory"
    );
    Ok(())
}

#[cfg(unix)]
fn make_socket_private(socket: &std::path::Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt as _;

    std::fs::set_permissions(socket, std::fs::Permissions::from_mode(0o600))
        .with_context(|| format!("secure terminal host socket {}", socket.display()))
}

#[cfg(not(unix))]
fn make_socket_private(_socket: &std::path::Path) -> Result<()> {
    Ok(())
}

async fn serve(
    listener: UnixListener,
    host_id: TerminalHostId,
    auth_token: TerminalHostAuthToken,
    replay_limit_bytes: usize,
) -> Result<()> {
    let notifier = TerminalHostEventNotifier::default();
    let host = Arc::new(Mutex::new(TerminalHostService::new(
        host_id,
        replay_limit_bytes,
        notifier,
    )));
    loop {
        let (stream, ()) = listener
            .accept()
            .await
            .context("accept terminal host client")?;
        let host = host.clone();
        let auth_token = auth_token.clone();
        smol::spawn(async move {
            if let Err(error) = serve_client(stream, host, host_id, &auth_token).await
                && !is_client_disconnect(&error)
            {
                eprintln!("terminal host client failed: {error:#}");
            }
        })
        .detach();
    }
}

fn is_client_disconnect(error: &anyhow::Error) -> bool {
    error
        .downcast_ref::<TerminalHostTransportError>()
        .is_some_and(|error| {
            matches!(
                error,
                TerminalHostTransportError::Io(error)
                    if matches!(
                        error.kind(),
                        io::ErrorKind::UnexpectedEof
                            | io::ErrorKind::BrokenPipe
                            | io::ErrorKind::ConnectionReset
                    )
            )
        })
}

async fn serve_client(
    mut stream: UnixStream,
    host: Arc<Mutex<TerminalHostService>>,
    host_id: TerminalHostId,
    expected_auth_token: &TerminalHostAuthToken,
) -> Result<()> {
    let hello = read_frame::<_, TerminalHostClientMessage>(&mut stream).await?;
    let capabilities = match validate_hello(hello, host_id, expected_auth_token) {
        Ok(capabilities) => capabilities,
        Err(rejection) => {
            write_frame(
                &mut stream,
                &TerminalHostServerMessage::HelloRejected { rejection },
            )
            .await?;
            return Ok(());
        }
    };
    write_frame(
        &mut stream,
        &TerminalHostServerMessage::accepted(host_id, capabilities),
    )
    .await?;

    loop {
        match read_frame::<_, TerminalHostClientMessage>(&mut stream).await? {
            TerminalHostClientMessage::Command {
                request_id,
                command,
            } => {
                let (response, changed, notifier) = {
                    let mut host = lock_host_service(&host);
                    let previous_cursor = host.next_event_cursor;
                    let response = handle_command(&mut host, command);
                    host.capture_snapshot_events();
                    (
                        response,
                        host.next_event_cursor != previous_cursor,
                        host.notifier.clone(),
                    )
                };
                if changed {
                    notifier.notify();
                }
                write_frame(
                    &mut stream,
                    &TerminalHostServerMessage::Response {
                        request_id,
                        response,
                    },
                )
                .await?;
            }
            TerminalHostClientMessage::SubscribeEvents { after_cursor } => {
                anyhow::ensure!(
                    capabilities.event_stream,
                    "terminal host client requested an unnegotiated event stream"
                );
                return serve_event_stream(stream, host, after_cursor).await;
            }
            TerminalHostClientMessage::Hello { .. } => {
                anyhow::bail!("terminal host client sent a second handshake");
            }
        }
    }
}

async fn serve_event_stream(
    mut stream: UnixStream,
    host: Arc<Mutex<TerminalHostService>>,
    mut after_cursor: Option<u64>,
) -> Result<()> {
    let notifications = lock_host_service(&host).notifier.subscribe();
    let mut send_initial_position = true;
    loop {
        let response = {
            let mut host = lock_host_service(&host);
            host.events_after(after_cursor, MAX_EVENTS_PER_RESPONSE as u32)
        };
        let TerminalHostResponse::Events {
            events,
            oldest_cursor,
            latest_cursor,
            truncated,
        } = response
        else {
            unreachable!("events_after always returns an event response")
        };
        let has_events = !events.is_empty();
        if send_initial_position || has_events || truncated {
            write_frame(
                &mut stream,
                &TerminalHostServerMessage::EventBatch {
                    events,
                    oldest_cursor,
                    latest_cursor,
                    truncated,
                },
            )
            .await?;
            send_initial_position = false;
            after_cursor = Some(latest_cursor);
        }
        if has_events {
            continue;
        }
        notifications
            .recv()
            .await
            .context("terminal host event notifier closed")?;
    }
}

fn lock_host_service(host: &Mutex<TerminalHostService>) -> MutexGuard<'_, TerminalHostService> {
    match host.lock() {
        Ok(host) => host,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn validate_hello(
    message: TerminalHostClientMessage,
    host_id: TerminalHostId,
    expected_auth_token: &TerminalHostAuthToken,
) -> Result<TerminalHostCapabilities, TerminalHostHandshakeRejection> {
    match message {
        TerminalHostClientMessage::Hello {
            protocol_version,
            host_id: client_host_id,
            auth_token,
            capabilities,
        } => {
            if !auth_token.authenticated_eq(expected_auth_token) {
                Err(TerminalHostHandshakeRejection::AuthenticationFailed)
            } else if client_host_id != host_id {
                Err(TerminalHostHandshakeRejection::HostMismatch)
            } else if protocol_version != TERMINAL_SESSION_PROTOCOL_VERSION {
                Err(TerminalHostHandshakeRejection::ProtocolMismatch {
                    host_protocol: TERMINAL_SESSION_PROTOCOL_VERSION,
                    client_protocol: protocol_version,
                })
            } else {
                Ok(TerminalHostCapabilities::current().negotiate(capabilities))
            }
        }
        TerminalHostClientMessage::Command { .. } => {
            Err(TerminalHostHandshakeRejection::AuthenticationFailed)
        }
        TerminalHostClientMessage::SubscribeEvents { .. } => {
            Err(TerminalHostHandshakeRejection::AuthenticationFailed)
        }
    }
}

fn handle_command(
    host: &mut TerminalHostService,
    command: TerminalSessionCommand,
) -> TerminalHostResponse {
    host.reap_exited_ptys();
    match command {
        TerminalSessionCommand::Heartbeat { nonce } => TerminalHostResponse::Heartbeat {
            nonce,
            observed_at_unix_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
                .try_into()
                .unwrap_or(u64::MAX),
        },
        TerminalSessionCommand::Events {
            after_cursor,
            limit,
        } => host.events_after(after_cursor, limit),
        TerminalSessionCommand::List => TerminalHostResponse::Sessions {
            sessions: host.lock_model().list(),
        },
        TerminalSessionCommand::Create {
            session_id,
            working_directory,
            shell,
            environment,
            columns,
            rows,
        } => host.create(
            session_id,
            working_directory,
            shell,
            environment.into_iter().collect(),
            columns,
            rows,
        ),
        TerminalSessionCommand::Attach {
            session_id,
            replay_after_sequence,
        } => TerminalHostResponse::Attachment {
            attachment: host.lock_model().attach(
                session_id,
                TERMINAL_SESSION_PROTOCOL_VERSION,
                replay_after_sequence,
            ),
        },
        TerminalSessionCommand::Detach { session_id } => TerminalHostResponse::Snapshot {
            snapshot: host.lock_model().detach(session_id),
        },
        TerminalSessionCommand::Input { session_id, bytes } => host.input(session_id, bytes),
        TerminalSessionCommand::Resize {
            session_id,
            columns,
            rows,
        } => host.resize(session_id, columns, rows),
        TerminalSessionCommand::UpdateMetadata {
            session_id,
            title,
            working_directory,
        } => host.update_metadata(session_id, title, working_directory),
        TerminalSessionCommand::UpdateAgent { session_id, update } => {
            host.update_agent(session_id, update)
        }
        TerminalSessionCommand::AcknowledgeAgentAttention { session_id } => {
            TerminalHostResponse::Snapshot {
                snapshot: host.lock_model().acknowledge_agent_attention(session_id),
            }
        }
        TerminalSessionCommand::Terminate { session_id } => host.terminate(session_id),
    }
}

struct TerminalHostService {
    model: Arc<Mutex<InProcessTerminalHost>>,
    ptys: HashMap<TerminalSessionId, TerminalHostPtyHandle>,
    events: VecDeque<TerminalHostEventEnvelope>,
    next_event_cursor: u64,
    last_event_snapshots: HashMap<TerminalSessionId, TerminalSessionSnapshot>,
    notifier: TerminalHostEventNotifier,
}

impl TerminalHostService {
    fn new(
        host_id: TerminalHostId,
        replay_limit_bytes: usize,
        notifier: TerminalHostEventNotifier,
    ) -> Self {
        Self {
            model: Arc::new(Mutex::new(InProcessTerminalHost::new(
                host_id,
                replay_limit_bytes,
            ))),
            ptys: HashMap::new(),
            events: VecDeque::new(),
            next_event_cursor: 1,
            last_event_snapshots: HashMap::new(),
            notifier,
        }
    }

    fn capture_snapshot_events(&mut self) {
        let snapshots = self.lock_model().list();
        let observed_at_unix_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            .try_into()
            .unwrap_or(u64::MAX);
        for snapshot in &snapshots {
            let unchanged = self
                .last_event_snapshots
                .get(&snapshot.session_id)
                .is_some_and(|previous| previous == snapshot);
            if unchanged {
                continue;
            }
            self.events.push_back(TerminalHostEventEnvelope {
                cursor: self.next_event_cursor,
                observed_at_unix_ms,
                event: TerminalSessionEvent::Snapshot {
                    snapshot: snapshot.clone(),
                },
            });
            self.next_event_cursor = self.next_event_cursor.saturating_add(1);
            while self.events.len() > MAX_HOST_EVENTS {
                self.events.pop_front();
            }
        }
        self.last_event_snapshots = snapshots
            .into_iter()
            .map(|snapshot| (snapshot.session_id, snapshot))
            .collect();
    }

    fn events_after(&mut self, after_cursor: Option<u64>, limit: u32) -> TerminalHostResponse {
        self.capture_snapshot_events();
        let oldest_cursor = self
            .events
            .front()
            .map_or(self.next_event_cursor, |event| event.cursor);
        let truncated = after_cursor.is_some_and(|cursor| cursor.saturating_add(1) < oldest_cursor);
        let effective_cursor = if truncated {
            oldest_cursor.saturating_sub(1)
        } else {
            after_cursor.unwrap_or(0)
        };
        let limit = usize::try_from(limit)
            .unwrap_or(MAX_EVENTS_PER_RESPONSE)
            .clamp(1, MAX_EVENTS_PER_RESPONSE);
        let events = self
            .events
            .iter()
            .filter(|event| event.cursor > effective_cursor)
            .take(limit)
            .cloned()
            .collect::<Vec<_>>();
        let latest_cursor = events.last().map_or(effective_cursor, |event| event.cursor);
        TerminalHostResponse::Events {
            events,
            oldest_cursor,
            latest_cursor,
            truncated,
        }
    }

    fn lock_model(&self) -> MutexGuard<'_, InProcessTerminalHost> {
        match self.model.lock() {
            Ok(model) => model,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    fn reap_exited_ptys(&mut self) {
        let live_session_ids = self
            .lock_model()
            .list()
            .into_iter()
            .filter(|snapshot| snapshot.state.may_be_live())
            .map(|snapshot| snapshot.session_id)
            .collect::<HashSet<_>>();
        self.ptys
            .retain(|session_id, _pty| live_session_ids.contains(session_id));
    }

    fn create(
        &mut self,
        session_id: TerminalSessionId,
        working_directory: Option<PathBuf>,
        shell: Option<terminal::session_host::TerminalSessionShell>,
        environment: HashMap<String, String>,
        columns: u16,
        rows: u16,
    ) -> TerminalHostResponse {
        if let Err(error) = self
            .lock_model()
            .create(session_id, working_directory.clone())
        {
            return TerminalHostResponse::Error {
                message: error.to_string(),
            };
        }
        let model = self.model.clone();
        let notifier = self.notifier.clone();
        let pty = TerminalHostPtyHandle::spawn(
            working_directory,
            shell,
            environment,
            columns,
            rows,
            move |event| handle_pty_event(&model, &notifier, session_id, event),
        );
        match pty {
            Ok(pty) => {
                let process_id = pty.process_id();
                self.lock_model().set_process_id(session_id, process_id);
                self.ptys.insert(session_id, pty);
                TerminalHostResponse::Snapshot {
                    snapshot: self.lock_model().snapshot(session_id),
                }
            }
            Err(error) => {
                self.lock_model()
                    .set_state(session_id, TerminalSessionState::Exited { exit_code: None });
                TerminalHostResponse::Error {
                    message: format!("failed to start terminal process: {error}"),
                }
            }
        }
    }

    fn input(&self, session_id: TerminalSessionId, bytes: Vec<u8>) -> TerminalHostResponse {
        let Some(pty) = self.ptys.get(&session_id) else {
            return TerminalHostResponse::Error {
                message: format!("terminal session {session_id} has no live PTY"),
            };
        };
        if let Err(error) = pty.input(bytes) {
            return TerminalHostResponse::Error {
                message: error.to_string(),
            };
        }
        TerminalHostResponse::Snapshot {
            snapshot: self.lock_model().snapshot(session_id),
        }
    }

    fn resize(
        &self,
        session_id: TerminalSessionId,
        columns: u16,
        rows: u16,
    ) -> TerminalHostResponse {
        let Some(pty) = self.ptys.get(&session_id) else {
            return TerminalHostResponse::Error {
                message: format!("terminal session {session_id} has no live PTY"),
            };
        };
        if let Err(error) = pty.resize(columns, rows) {
            return TerminalHostResponse::Error {
                message: error.to_string(),
            };
        }
        TerminalHostResponse::Snapshot {
            snapshot: self.lock_model().snapshot(session_id),
        }
    }

    fn terminate(&mut self, session_id: TerminalSessionId) -> TerminalHostResponse {
        if let Some(pty) = self.ptys.remove(&session_id)
            && let Err(error) = pty.terminate()
        {
            eprintln!("failed to terminate terminal session {session_id}: {error}");
        }
        TerminalHostResponse::Snapshot {
            snapshot: self.lock_model().terminate(session_id, None),
        }
    }

    fn update_metadata(
        &self,
        session_id: TerminalSessionId,
        title: Option<String>,
        working_directory: Option<PathBuf>,
    ) -> TerminalHostResponse {
        let mut model = self.lock_model();
        model.set_title(session_id, title);
        model.set_working_directory(session_id, working_directory);
        TerminalHostResponse::Snapshot {
            snapshot: model.snapshot(session_id),
        }
    }

    fn update_agent(
        &self,
        session_id: TerminalSessionId,
        update: TerminalAgentUpdate,
    ) -> TerminalHostResponse {
        match self.lock_model().update_agent(session_id, update) {
            Ok(snapshot) => TerminalHostResponse::Snapshot { snapshot },
            Err(error) => TerminalHostResponse::Error {
                message: error.to_string(),
            },
        }
    }
}

fn handle_pty_event(
    model: &Mutex<InProcessTerminalHost>,
    notifier: &TerminalHostEventNotifier,
    session_id: TerminalSessionId,
    event: TerminalHostPtyEvent,
) {
    let mut model = match model.lock() {
        Ok(model) => model,
        Err(poisoned) => poisoned.into_inner(),
    };
    match event {
        TerminalHostPtyEvent::Output(bytes) => {
            if let Err(error) = model.push_output(session_id, bytes) {
                eprintln!("failed to retain output for terminal session {session_id}: {error}");
            }
        }
        TerminalHostPtyEvent::Exited { exit_code } => {
            model.set_state(session_id, TerminalSessionState::Exited { exit_code });
            model.set_process_id(session_id, None);
        }
        TerminalHostPtyEvent::Failed(error) => {
            eprintln!("terminal session {session_id} PTY failed: {error}");
            model.set_state(session_id, TerminalSessionState::Exited { exit_code: None });
            model.set_process_id(session_id, None);
        }
    }
    drop(model);
    notifier.notify();
}

#[cfg(test)]
mod tests {
    use terminal::session_host::{TerminalSessionId, TerminalSessionState};

    use super::*;

    fn token(value: char) -> Result<TerminalHostAuthToken> {
        TerminalHostAuthToken::parse(value.to_string().repeat(32)).map_err(Into::into)
    }

    fn codex_event(name: &str) -> CodexHookEvent {
        CodexHookEvent {
            session_id: "codex-session-7".to_owned(),
            cwd: PathBuf::from("/workspace"),
            hook_event_name: name.to_owned(),
            turn_id: Some("turn-3".to_owned()),
            tool_name: None,
            tool_input: None,
            tool_response: None,
        }
    }

    #[test]
    fn event_notifications_are_broadcast_and_coalesced() {
        let notifier = TerminalHostEventNotifier::default();
        let first = notifier.subscribe();
        let second = notifier.subscribe();

        notifier.notify();
        notifier.notify();

        assert_eq!(first.try_recv(), Ok(()));
        assert_eq!(second.try_recv(), Ok(()));
        assert!(matches!(
            first.try_recv(),
            Err(async_channel::TryRecvError::Empty)
        ));
    }

    #[test]
    fn codex_hooks_map_to_structured_attention_without_terminal_scraping() -> Result<()> {
        let mut permission = codex_event("PermissionRequest");
        permission.tool_name = Some("Bash".to_owned());
        let permission = codex_hook_update(&permission)?;
        assert_eq!(permission.state, TerminalAgentState::WaitingForPermission);
        assert!(permission.attention_required);
        assert_eq!(
            permission.event_kind,
            TerminalAgentEventKind::PermissionRequested
        );

        let completed = codex_hook_update(&codex_event("Stop"))?;
        assert_eq!(completed.state, TerminalAgentState::Completed);
        assert!(completed.attention_required);
        Ok(())
    }

    #[test]
    fn codex_post_tool_hook_retains_bounded_command_evidence() -> Result<()> {
        let mut event = codex_event("PostToolUse");
        event.tool_name = Some("Bash".to_owned());
        event.tool_input = Some(serde_json::json!({ "command": "cargo test -p terminal" }));
        event.tool_response = Some(serde_json::json!({ "exit_code": 0 }));

        let update = codex_hook_update(&event)?;

        assert_eq!(update.command.as_deref(), Some("cargo test -p terminal"));
        assert_eq!(update.exit_code, Some(0));
        assert_eq!(
            update.working_directory.as_deref(),
            Some(std::path::Path::new("/workspace"))
        );
        assert!(!update.attention_required);
        Ok(())
    }

    #[test]
    fn codex_edit_hooks_retain_bounded_file_targets_without_claiming_success() -> Result<()> {
        let mut event = codex_event("PostToolUse");
        event.tool_name = Some("apply_patch".to_owned());
        event.tool_input = Some(serde_json::json!({
            "patch": "*** Update File: src/lib.rs\n*** Add File: /tmp/new.rs\n",
            "file_path": "src/lib.rs"
        }));

        let update = codex_hook_update(&event)?;

        assert_eq!(
            update.file_targets,
            vec![
                PathBuf::from("/tmp/new.rs"),
                PathBuf::from("/workspace/src/lib.rs")
            ]
        );
        assert!(!update.file_targets_truncated);
        assert!(update.capabilities.file_targets);
        Ok(())
    }

    #[test]
    fn handshake_fails_closed() -> Result<()> {
        let host_id = TerminalHostId::from_stable_key("helper-test");
        let expected_token = token('a')?;
        let wrong_token = token('b')?;
        let message = TerminalHostClientMessage::Hello {
            protocol_version: TERMINAL_SESSION_PROTOCOL_VERSION,
            host_id,
            auth_token: wrong_token,
            capabilities: TerminalHostCapabilities::current(),
        };
        assert_eq!(
            validate_hello(message, host_id, &expected_token),
            Err(TerminalHostHandshakeRejection::AuthenticationFailed)
        );

        let message = TerminalHostClientMessage::Hello {
            protocol_version: TERMINAL_SESSION_PROTOCOL_VERSION,
            host_id: TerminalHostId::from_stable_key("other-host"),
            auth_token: expected_token.clone(),
            capabilities: TerminalHostCapabilities::current(),
        };
        assert_eq!(
            validate_hello(message, host_id, &expected_token),
            Err(TerminalHostHandshakeRejection::HostMismatch)
        );

        let message = TerminalHostClientMessage::Hello {
            protocol_version: TERMINAL_SESSION_PROTOCOL_VERSION + 1,
            host_id,
            auth_token: expected_token.clone(),
            capabilities: TerminalHostCapabilities::current(),
        };
        assert!(matches!(
            validate_hello(message, host_id, &expected_token),
            Err(TerminalHostHandshakeRejection::ProtocolMismatch { .. })
        ));
        Ok(())
    }

    #[test]
    fn lifecycle_commands_preserve_detach_and_terminate_distinction() {
        let host_id = TerminalHostId::from_stable_key("helper-lifecycle-test");
        let session_id = TerminalSessionId::new();
        let mut host =
            TerminalHostService::new(host_id, 1024, TerminalHostEventNotifier::default());
        let created = handle_command(
            &mut host,
            TerminalSessionCommand::Create {
                session_id,
                working_directory: None,
                shell: None,
                environment: Vec::new(),
                columns: 80,
                rows: 24,
            },
        );
        assert!(matches!(created, TerminalHostResponse::Snapshot { .. }));

        let detached = handle_command(&mut host, TerminalSessionCommand::Detach { session_id });
        assert!(matches!(
            detached,
            TerminalHostResponse::Snapshot {
                snapshot: terminal::session_host::TerminalSessionSnapshot {
                    state: TerminalSessionState::Detached,
                    ..
                }
            }
        ));

        let terminated =
            handle_command(&mut host, TerminalSessionCommand::Terminate { session_id });
        assert!(matches!(
            terminated,
            TerminalHostResponse::Snapshot {
                snapshot: terminal::session_host::TerminalSessionSnapshot {
                    state: TerminalSessionState::Exited { .. },
                    ..
                }
            }
        ));
    }

    #[test]
    fn heartbeat_echoes_nonce_without_mutating_sessions() {
        let host_id = TerminalHostId::from_stable_key("helper-heartbeat-test");
        let mut host =
            TerminalHostService::new(host_id, 1024, TerminalHostEventNotifier::default());
        let response = handle_command(&mut host, TerminalSessionCommand::Heartbeat { nonce: 42 });
        assert!(matches!(
            response,
            TerminalHostResponse::Heartbeat {
                nonce: 42,
                observed_at_unix_ms: _
            }
        ));
        assert!(host.lock_model().list().is_empty());
    }

    #[test]
    fn snapshot_events_resume_after_the_last_delivered_cursor() {
        let host_id = TerminalHostId::from_stable_key("helper-event-cursor-test");
        let session_id = TerminalSessionId::new();
        let mut host =
            TerminalHostService::new(host_id, 1024, TerminalHostEventNotifier::default());
        host.lock_model()
            .create(session_id, None)
            .expect("in-process session creation should succeed");

        let first = host.events_after(None, 8);
        let TerminalHostResponse::Events {
            events,
            latest_cursor,
            truncated,
            ..
        } = first
        else {
            panic!("event request should return a bounded event response");
        };
        assert!(!truncated);
        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0].event,
            TerminalSessionEvent::Snapshot { snapshot }
                if snapshot.session_id == session_id
        ));

        let unchanged = host.events_after(Some(latest_cursor), 8);
        assert!(matches!(
            unchanged,
            TerminalHostResponse::Events { ref events, .. } if events.is_empty()
        ));

        host.lock_model()
            .set_state(session_id, TerminalSessionState::Detached);
        let changed = host.events_after(Some(latest_cursor), 8);
        assert!(matches!(
            changed,
            TerminalHostResponse::Events { ref events, .. }
                if matches!(
                    events.as_slice(),
                    [TerminalHostEventEnvelope {
                        event: TerminalSessionEvent::Snapshot { snapshot },
                        ..
                    }] if snapshot.state == TerminalSessionState::Detached
                )
        ));
    }
}
