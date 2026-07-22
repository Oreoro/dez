//! Host-owned terminal session protocol primitives.
//!
//! These types define the stable identity and lifecycle boundary used by both
//! the in-process compatibility adapter and the out-of-process Dez terminal
//! host. The host owns PTYs independently from [`crate::Terminal`] views, so
//! detach and terminate remain distinct across GUI restarts.

use std::{fmt, path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod in_process;
mod local;
mod pty_process;
pub mod transport;

pub use in_process::{
    InProcessTerminalHost, TerminalAttachment, TerminalHostError, TerminalReplayChunk,
};
pub use local::{AttachedLocalTerminal, LocalTerminalHost};
pub use pty_process::{TerminalHostPtyEvent, TerminalHostPtyHandle};

/// First version of the terminal host protocol understood by this client.
pub const TERMINAL_SESSION_PROTOCOL_VERSION: u32 = 4;

/// Cell dimensions under which a Host produced terminal output.
///
/// Terminal byte streams may contain cursor positioning derived from the PTY
/// size. Retaining dimensions with replay is therefore required to rebuild the
/// same screen after a GUI restart instead of applying wide-screen output to a
/// default 80x24 grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalDimensions {
    pub columns: u16,
    pub rows: u16,
}

impl TerminalDimensions {
    pub const DEFAULT: Self = Self {
        columns: 80,
        rows: 24,
    };

    pub const fn new(columns: u16, rows: u16) -> Self {
        Self {
            columns: if columns == 0 { 1 } else { columns },
            rows: if rows == 0 { 1 } else { rows },
        }
    }
}

impl Default for TerminalDimensions {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Features implemented by both sides of one authenticated Host connection.
///
/// Capabilities are additive within a protocol version. New fields must
/// default to `false` so older peers fail closed instead of advertising an
/// operation they cannot honor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalHostCapabilities {
    #[serde(default)]
    pub session_lifecycle: bool,
    #[serde(default)]
    pub bounded_replay: bool,
    #[serde(default)]
    pub metadata_updates: bool,
    #[serde(default)]
    pub structured_agent_updates: bool,
    #[serde(default)]
    pub attention_acknowledgement: bool,
    #[serde(default)]
    pub heartbeat: bool,
    #[serde(default)]
    pub event_stream: bool,
    #[serde(default)]
    pub event_cursor_resume: bool,
}

impl TerminalHostCapabilities {
    pub const fn current() -> Self {
        Self {
            session_lifecycle: true,
            bounded_replay: true,
            metadata_updates: true,
            structured_agent_updates: true,
            attention_acknowledgement: true,
            heartbeat: true,
            event_stream: true,
            event_cursor_resume: true,
        }
    }

    pub const fn negotiate(self, peer: Self) -> Self {
        Self {
            session_lifecycle: self.session_lifecycle && peer.session_lifecycle,
            bounded_replay: self.bounded_replay && peer.bounded_replay,
            metadata_updates: self.metadata_updates && peer.metadata_updates,
            structured_agent_updates: self.structured_agent_updates
                && peer.structured_agent_updates,
            attention_acknowledgement: self.attention_acknowledgement
                && peer.attention_acknowledgement,
            heartbeat: self.heartbeat && peer.heartbeat,
            event_stream: self.event_stream && peer.event_stream,
            event_cursor_resume: self.event_cursor_resume && peer.event_cursor_resume,
        }
    }
}

/// Stable identity for a machine or execution environment that owns sessions.
///
/// A local host ID should be derived from persisted installation material with
/// [`Self::from_stable_key`], not generated again on every launch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TerminalHostId(Uuid);

impl TerminalHostId {
    pub fn from_stable_key(key: &str) -> Self {
        Self(Uuid::new_v5(&Uuid::NAMESPACE_OID, key.as_bytes()))
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl fmt::Display for TerminalHostId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for TerminalHostId {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(value).map(Self)
    }
}

/// Stable identity for one computation owned by a terminal host.
///
/// It is independent of pane, item, entity, workspace, and window IDs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TerminalSessionId(Uuid);

impl TerminalSessionId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for TerminalSessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TerminalSessionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for TerminalSessionId {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(value).map(Self)
    }
}

/// Durable reference stored by terminal items and metadata rows.
///
/// A reference is not proof that the session still exists. Clients must
/// reconcile it against the owning host and surface Missing or Incompatible
/// instead of silently creating replacement computation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TerminalSessionRef {
    pub host_id: TerminalHostId,
    pub session_id: TerminalSessionId,
}

/// Provider-neutral state projected by an agent adapter attached to a normal
/// terminal session. Critical states enter through structured adapter events;
/// process-name detection alone must never synthesize them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalAgentState {
    Starting,
    Running,
    WaitingForPermission,
    WaitingForInput,
    Idle,
    Completed,
    Failed,
    Disconnected,
    Resumable,
    Exited,
}

impl TerminalAgentState {
    pub fn label(self) -> &'static str {
        match self {
            Self::Starting => "Starting",
            Self::Running => "Running",
            Self::WaitingForPermission => "Waiting for permission",
            Self::WaitingForInput => "Waiting for input",
            Self::Idle => "Idle",
            Self::Completed => "Ready for review",
            Self::Failed => "Failed",
            Self::Disconnected => "Disconnected",
            Self::Resumable => "Resumable",
            Self::Exited => "Exited",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalAgentEventKind {
    SessionStarted,
    PromptSubmitted,
    PermissionRequested,
    ToolStarted,
    ToolFinished,
    TurnCompleted,
}

/// Evidence and recovery semantics an adapter can truthfully provide.
///
/// The UI must consult these flags instead of deriving support from an adapter
/// name. This keeps future providers honest and makes partial adapters useful.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalAgentCapabilities {
    #[serde(default)]
    pub structured_state: bool,
    #[serde(default)]
    pub attention: bool,
    #[serde(default)]
    pub activity_events: bool,
    #[serde(default)]
    pub command_evidence: bool,
    #[serde(default)]
    pub check_results: bool,
    #[serde(default)]
    pub file_targets: bool,
    #[serde(default)]
    pub resumability: bool,
    /// The adapter can provide a scoped, auditable permission response
    /// contract. Attention events alone do not imply this capability.
    #[serde(default)]
    pub permission_responses: bool,
    /// The adapter can submit bounded user input to the owning provider
    /// session without terminal scraping or synthetic keystrokes.
    #[serde(default)]
    pub input_responses: bool,
}

impl TerminalAgentCapabilities {
    pub const fn codex_hooks_v1() -> Self {
        Self {
            structured_state: true,
            attention: true,
            activity_events: true,
            command_evidence: true,
            check_results: true,
            file_targets: true,
            resumability: true,
            permission_responses: false,
            input_responses: false,
        }
    }
}

/// One bounded, structured event supplied by an agent adapter.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalAgentEvent {
    pub sequence: u64,
    #[serde(default)]
    pub observed_at_unix_ms: u64,
    pub kind: TerminalAgentEventKind,
    pub summary: String,
    #[serde(default)]
    pub working_directory: Option<PathBuf>,
    #[serde(default)]
    pub file_targets: Vec<PathBuf>,
    #[serde(default)]
    pub file_targets_truncated: bool,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub exit_code: Option<i32>,
}

/// Last-known adapter truth retained by the terminal host and therefore able
/// to survive a GUI disconnect with the owning terminal computation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalAgentSnapshot {
    pub adapter: String,
    pub actor: String,
    #[serde(default)]
    pub capabilities: TerminalAgentCapabilities,
    #[serde(default)]
    pub provider_session_id: Option<String>,
    pub state: TerminalAgentState,
    pub attention_required: bool,
    pub resumable: bool,
    #[serde(default)]
    pub events: Vec<TerminalAgentEvent>,
}

/// Atomic adapter update. The host assigns event sequence numbers and bounds
/// retained history; clients cannot claim an arbitrary ordering.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalAgentUpdate {
    pub adapter: String,
    pub actor: String,
    #[serde(default)]
    pub capabilities: TerminalAgentCapabilities,
    #[serde(default)]
    pub provider_session_id: Option<String>,
    pub state: TerminalAgentState,
    pub attention_required: bool,
    pub resumable: bool,
    pub event_kind: TerminalAgentEventKind,
    pub summary: String,
    #[serde(default)]
    pub working_directory: Option<PathBuf>,
    #[serde(default)]
    pub file_targets: Vec<PathBuf>,
    #[serde(default)]
    pub file_targets_truncated: bool,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub exit_code: Option<i32>,
}

/// Truthful last-known lifecycle state of a host-owned terminal session.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum TerminalSessionState {
    Starting,
    Attached,
    Detached,
    Reconnecting,
    Exited {
        exit_code: Option<i32>,
    },
    Missing,
    Incompatible {
        host_protocol: u32,
        client_protocol: u32,
    },
}

impl TerminalSessionState {
    /// Whether the host may still own a live process for this session.
    pub fn may_be_live(self) -> bool {
        matches!(
            self,
            Self::Starting | Self::Attached | Self::Detached | Self::Reconnecting
        )
    }

    /// Whether this client can currently send input to the session.
    pub fn accepts_input(self) -> bool {
        matches!(self, Self::Attached)
    }

    /// Whether an attach attempt is meaningful without creating a new shell.
    pub fn can_attach(self) -> bool {
        matches!(self, Self::Detached | Self::Reconnecting)
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Starting => "Starting",
            Self::Attached => "Attached",
            Self::Detached => "Detached",
            Self::Reconnecting => "Reconnecting",
            Self::Exited { .. } => "Exited",
            Self::Missing => "Missing",
            Self::Incompatible { .. } => "Incompatible",
        }
    }
}

/// Bounded, serializable metadata returned by a terminal host.
///
/// This intentionally excludes unbounded output and secrets. Output replay is
/// streamed separately and addressed by monotonically increasing sequence
/// numbers.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalSessionSnapshot {
    pub protocol_version: u32,
    pub host_id: TerminalHostId,
    pub session_id: TerminalSessionId,
    pub state: TerminalSessionState,
    pub title: Option<String>,
    pub working_directory: Option<PathBuf>,
    #[serde(default)]
    pub process_id: Option<u32>,
    #[serde(default)]
    pub agent: Option<TerminalAgentSnapshot>,
    #[serde(default)]
    pub dimensions: TerminalDimensions,
    pub earliest_replay_sequence: u64,
    pub latest_replay_sequence: u64,
}

/// Commands sent from a GUI client to the terminal host.
///
/// `Detach` releases this client while preserving computation. `Terminate` is
/// the explicit destructive operation that asks the host to end it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum TerminalSessionCommand {
    /// Lightweight authenticated liveness probe. The nonce prevents callers
    /// from mistaking a delayed response for the current probe.
    Heartbeat {
        nonce: u64,
    },
    /// Retrieve bounded Host events after a durable cursor. A stale cursor is
    /// reported as truncated so clients can rebuild from authoritative state.
    Events {
        after_cursor: Option<u64>,
        limit: u32,
    },
    List,
    Create {
        session_id: TerminalSessionId,
        working_directory: Option<PathBuf>,
        #[serde(default)]
        shell: Option<TerminalSessionShell>,
        #[serde(default)]
        environment: Vec<(String, String)>,
        columns: u16,
        rows: u16,
    },
    Attach {
        session_id: TerminalSessionId,
        replay_after_sequence: Option<u64>,
    },
    Detach {
        session_id: TerminalSessionId,
    },
    Input {
        session_id: TerminalSessionId,
        bytes: Vec<u8>,
    },
    Resize {
        session_id: TerminalSessionId,
        columns: u16,
        rows: u16,
    },
    UpdateMetadata {
        session_id: TerminalSessionId,
        title: Option<String>,
        working_directory: Option<PathBuf>,
    },
    UpdateAgent {
        session_id: TerminalSessionId,
        update: TerminalAgentUpdate,
    },
    AcknowledgeAgentAttention {
        session_id: TerminalSessionId,
    },
    Terminate {
        session_id: TerminalSessionId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalSessionShell {
    pub program: String,
    pub args: Vec<String>,
}

/// Events streamed from a terminal host to its clients.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum TerminalSessionEvent {
    Snapshot {
        snapshot: TerminalSessionSnapshot,
    },
    Output {
        session_id: TerminalSessionId,
        sequence: u64,
        bytes: Vec<u8>,
    },
    WorkingDirectoryChanged {
        session_id: TerminalSessionId,
        working_directory: Option<PathBuf>,
    },
    TitleChanged {
        session_id: TerminalSessionId,
        title: Option<String>,
    },
    StateChanged {
        session_id: TerminalSessionId,
        state: TerminalSessionState,
    },
}

/// One versioned Host observation addressable across client reconnects.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalHostEventEnvelope {
    pub cursor: u64,
    pub observed_at_unix_ms: u64,
    pub event: TerminalSessionEvent,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_host_identity_round_trips() {
        let host = TerminalHostId::from_stable_key("local-installation-42");
        let Ok(parsed) = host.to_string().parse::<TerminalHostId>() else {
            panic!("generated terminal host ID should parse");
        };
        assert_eq!(parsed, host);
        assert_eq!(
            TerminalHostId::from_stable_key("local-installation-42"),
            host
        );
        assert_ne!(
            TerminalHostId::from_stable_key("local-installation-43"),
            host
        );
    }

    #[test]
    fn lifecycle_predicates_do_not_confuse_saved_with_live() {
        assert!(TerminalSessionState::Attached.may_be_live());
        assert!(TerminalSessionState::Attached.accepts_input());
        assert!(!TerminalSessionState::Detached.accepts_input());
        assert!(TerminalSessionState::Detached.can_attach());
        assert!(!TerminalSessionState::Exited { exit_code: Some(0) }.may_be_live());
        assert!(!TerminalSessionState::Missing.may_be_live());
    }

    #[test]
    fn detach_and_terminate_are_distinct_commands() {
        let session_id = TerminalSessionId::new();
        assert_ne!(
            TerminalSessionCommand::Detach { session_id },
            TerminalSessionCommand::Terminate { session_id }
        );
    }

    #[test]
    fn capability_negotiation_fails_closed_per_feature() {
        let peer = TerminalHostCapabilities {
            bounded_replay: true,
            metadata_updates: true,
            ..TerminalHostCapabilities::default()
        };
        let negotiated = TerminalHostCapabilities::current().negotiate(peer);
        assert!(negotiated.bounded_replay);
        assert!(negotiated.metadata_updates);
        assert!(!negotiated.session_lifecycle);
        assert!(!negotiated.structured_agent_updates);
        assert!(!negotiated.heartbeat);
    }
}
