use std::collections::{HashMap, VecDeque};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{
    TERMINAL_SESSION_PROTOCOL_VERSION, TerminalAgentEvent, TerminalAgentSnapshot,
    TerminalAgentState, TerminalAgentUpdate, TerminalHostId, TerminalSessionId,
    TerminalSessionSnapshot, TerminalSessionState,
};

const MAX_AGENT_EVENTS: usize = 32;

/// One bounded output fragment retained by an in-process terminal host.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalReplayChunk {
    pub sequence: u64,
    pub bytes: Vec<u8>,
}

/// Result of resolving a client attachment against the host's current truth.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalAttachment {
    pub snapshot: TerminalSessionSnapshot,
    pub replay: Vec<TerminalReplayChunk>,
    /// `true` when output older than the first returned sequence was evicted.
    pub replay_was_truncated: bool,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum TerminalHostError {
    #[error("terminal session {0} already exists")]
    AlreadyExists(TerminalSessionId),
    #[error("terminal session {0} is not accepting output")]
    NotRunning(TerminalSessionId),
}

struct HostedSession {
    snapshot: TerminalSessionSnapshot,
    replay: VecDeque<TerminalReplayChunk>,
    replay_bytes: usize,
}

/// In-process reference implementation of Host/Session lifecycle and replay.
///
/// This registry is intentionally independent from GPUI entities. It does not
/// yet own a PTY and cannot survive process exit; a later adapter will move PTY
/// ownership behind this boundary without changing its client-visible truth.
pub struct InProcessTerminalHost {
    host_id: TerminalHostId,
    replay_limit_bytes: usize,
    sessions: HashMap<TerminalSessionId, HostedSession>,
}

impl InProcessTerminalHost {
    pub fn new(host_id: TerminalHostId, replay_limit_bytes: usize) -> Self {
        Self {
            host_id,
            replay_limit_bytes,
            sessions: HashMap::new(),
        }
    }

    pub fn host_id(&self) -> TerminalHostId {
        self.host_id
    }

    pub fn create(
        &mut self,
        session_id: TerminalSessionId,
        working_directory: Option<std::path::PathBuf>,
    ) -> Result<TerminalSessionSnapshot, TerminalHostError> {
        if self.sessions.contains_key(&session_id) {
            return Err(TerminalHostError::AlreadyExists(session_id));
        }

        let snapshot = TerminalSessionSnapshot {
            protocol_version: TERMINAL_SESSION_PROTOCOL_VERSION,
            host_id: self.host_id,
            session_id,
            state: TerminalSessionState::Starting,
            title: None,
            working_directory,
            process_id: None,
            agent: None,
            earliest_replay_sequence: 0,
            latest_replay_sequence: 0,
        };
        self.sessions.insert(
            session_id,
            HostedSession {
                snapshot: snapshot.clone(),
                replay: VecDeque::new(),
                replay_bytes: 0,
            },
        );
        Ok(snapshot)
    }

    pub fn list(&self) -> Vec<TerminalSessionSnapshot> {
        let mut sessions = self
            .sessions
            .values()
            .map(|session| session.snapshot.clone())
            .collect::<Vec<_>>();
        sessions.sort_by_key(|session| session.session_id.as_uuid());
        sessions
    }

    pub fn snapshot(&self, session_id: TerminalSessionId) -> TerminalSessionSnapshot {
        self.sessions
            .get(&session_id)
            .map(|session| session.snapshot.clone())
            .unwrap_or_else(|| self.missing_snapshot(session_id))
    }

    pub fn attach(
        &mut self,
        session_id: TerminalSessionId,
        client_protocol: u32,
        replay_after_sequence: Option<u64>,
    ) -> TerminalAttachment {
        if client_protocol != TERMINAL_SESSION_PROTOCOL_VERSION {
            let mut snapshot = self.snapshot(session_id);
            snapshot.state = TerminalSessionState::Incompatible {
                host_protocol: TERMINAL_SESSION_PROTOCOL_VERSION,
                client_protocol,
            };
            return TerminalAttachment {
                snapshot,
                replay: Vec::new(),
                replay_was_truncated: false,
            };
        }

        let Some(session) = self.sessions.get_mut(&session_id) else {
            return TerminalAttachment {
                snapshot: self.missing_snapshot(session_id),
                replay: Vec::new(),
                replay_was_truncated: false,
            };
        };

        if session.snapshot.state.may_be_live() {
            session.snapshot.state = TerminalSessionState::Attached;
        }

        let requested_after = replay_after_sequence.unwrap_or(0);
        let first_available = session.replay.front().map(|chunk| chunk.sequence);
        let replay_was_truncated = first_available.map_or_else(
            || session.snapshot.latest_replay_sequence > requested_after,
            |first| requested_after.saturating_add(1) < first,
        );
        let replay = session
            .replay
            .iter()
            .filter(|chunk| chunk.sequence > requested_after)
            .cloned()
            .collect();

        TerminalAttachment {
            snapshot: session.snapshot.clone(),
            replay,
            replay_was_truncated,
        }
    }

    pub fn detach(&mut self, session_id: TerminalSessionId) -> TerminalSessionSnapshot {
        let Some(session) = self.sessions.get_mut(&session_id) else {
            return self.missing_snapshot(session_id);
        };
        if session.snapshot.state.may_be_live() {
            session.snapshot.state = TerminalSessionState::Detached;
        }
        session.snapshot.clone()
    }

    pub fn terminate(
        &mut self,
        session_id: TerminalSessionId,
        exit_code: Option<i32>,
    ) -> TerminalSessionSnapshot {
        let Some(session) = self.sessions.get_mut(&session_id) else {
            return self.missing_snapshot(session_id);
        };
        session.snapshot.state = TerminalSessionState::Exited { exit_code };
        if let Some(agent) = &mut session.snapshot.agent {
            agent.state = TerminalAgentState::Exited;
            agent.attention_required = true;
        }
        session.snapshot.clone()
    }

    pub fn set_state(
        &mut self,
        session_id: TerminalSessionId,
        state: TerminalSessionState,
    ) -> TerminalSessionSnapshot {
        let Some(session) = self.sessions.get_mut(&session_id) else {
            return self.missing_snapshot(session_id);
        };
        session.snapshot.state = state;
        if matches!(state, TerminalSessionState::Exited { .. })
            && let Some(agent) = &mut session.snapshot.agent
        {
            agent.state = TerminalAgentState::Exited;
            agent.attention_required = true;
        }
        session.snapshot.clone()
    }

    pub fn set_title(
        &mut self,
        session_id: TerminalSessionId,
        title: Option<String>,
    ) -> TerminalSessionSnapshot {
        let Some(session) = self.sessions.get_mut(&session_id) else {
            return self.missing_snapshot(session_id);
        };
        session.snapshot.title = title;
        session.snapshot.clone()
    }

    pub fn set_working_directory(
        &mut self,
        session_id: TerminalSessionId,
        working_directory: Option<std::path::PathBuf>,
    ) -> TerminalSessionSnapshot {
        let Some(session) = self.sessions.get_mut(&session_id) else {
            return self.missing_snapshot(session_id);
        };
        session.snapshot.working_directory = working_directory;
        session.snapshot.clone()
    }

    pub fn set_process_id(
        &mut self,
        session_id: TerminalSessionId,
        process_id: Option<u32>,
    ) -> TerminalSessionSnapshot {
        let Some(session) = self.sessions.get_mut(&session_id) else {
            return self.missing_snapshot(session_id);
        };
        session.snapshot.process_id = process_id;
        session.snapshot.clone()
    }

    pub fn update_agent(
        &mut self,
        session_id: TerminalSessionId,
        update: TerminalAgentUpdate,
    ) -> Result<TerminalSessionSnapshot, TerminalHostError> {
        let Some(session) = self.sessions.get_mut(&session_id) else {
            return Err(TerminalHostError::NotRunning(session_id));
        };
        if !session.snapshot.state.may_be_live() {
            return Err(TerminalHostError::NotRunning(session_id));
        }

        let retain_existing_events = session.snapshot.agent.as_ref().is_some_and(|agent| {
            agent.adapter == update.adapter
                && agent.provider_session_id == update.provider_session_id
        });
        let mut events = if retain_existing_events {
            session
                .snapshot
                .agent
                .take()
                .map(|agent| agent.events)
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        let sequence = events
            .last()
            .map_or(1, |event| event.sequence.saturating_add(1));
        events.push(TerminalAgentEvent {
            sequence,
            observed_at_unix_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
                .try_into()
                .unwrap_or(u64::MAX),
            kind: update.event_kind,
            summary: update.summary,
            working_directory: update.working_directory,
            file_targets: update.file_targets,
            file_targets_truncated: update.file_targets_truncated,
            command: update.command,
            exit_code: update.exit_code,
        });
        if events.len() > MAX_AGENT_EVENTS {
            events.drain(..events.len() - MAX_AGENT_EVENTS);
        }
        session.snapshot.agent = Some(TerminalAgentSnapshot {
            adapter: update.adapter,
            actor: update.actor,
            capabilities: update.capabilities,
            provider_session_id: update.provider_session_id,
            state: update.state,
            attention_required: update.attention_required,
            resumable: update.resumable,
            events,
        });
        Ok(session.snapshot.clone())
    }

    pub fn acknowledge_agent_attention(
        &mut self,
        session_id: TerminalSessionId,
    ) -> TerminalSessionSnapshot {
        let Some(session) = self.sessions.get_mut(&session_id) else {
            return self.missing_snapshot(session_id);
        };
        if let Some(agent) = &mut session.snapshot.agent {
            agent.attention_required = false;
        }
        session.snapshot.clone()
    }

    pub fn record_output(
        &mut self,
        session_id: TerminalSessionId,
        bytes: Vec<u8>,
    ) -> Result<Option<TerminalReplayChunk>, TerminalHostError> {
        let Some(session) = self.sessions.get_mut(&session_id) else {
            return Err(TerminalHostError::NotRunning(session_id));
        };
        if !session.snapshot.state.may_be_live() {
            return Err(TerminalHostError::NotRunning(session_id));
        }
        if bytes.is_empty() || self.replay_limit_bytes == 0 {
            return Ok(None);
        }

        let sequence = session.snapshot.latest_replay_sequence.saturating_add(1);
        let chunk = TerminalReplayChunk { sequence, bytes };
        session.replay_bytes = session.replay_bytes.saturating_add(chunk.bytes.len());
        session.replay.push_back(chunk.clone());
        session.snapshot.latest_replay_sequence = sequence;

        while session.replay_bytes > self.replay_limit_bytes {
            let Some(removed) = session.replay.pop_front() else {
                break;
            };
            session.replay_bytes = session.replay_bytes.saturating_sub(removed.bytes.len());
        }

        session.snapshot.earliest_replay_sequence = session
            .replay
            .front()
            .map_or(sequence.saturating_add(1), |chunk| chunk.sequence);
        Ok(Some(chunk))
    }

    fn missing_snapshot(&self, session_id: TerminalSessionId) -> TerminalSessionSnapshot {
        TerminalSessionSnapshot {
            protocol_version: TERMINAL_SESSION_PROTOCOL_VERSION,
            host_id: self.host_id,
            session_id,
            state: TerminalSessionState::Missing,
            title: None,
            working_directory: None,
            process_id: None,
            agent: None,
            earliest_replay_sequence: 0,
            latest_replay_sequence: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn host(replay_limit_bytes: usize) -> InProcessTerminalHost {
        InProcessTerminalHost::new(
            TerminalHostId::from_stable_key("test-installation"),
            replay_limit_bytes,
        )
    }

    fn agent_update(state: TerminalAgentState, attention_required: bool) -> TerminalAgentUpdate {
        TerminalAgentUpdate {
            adapter: "codex-hooks-v1".to_owned(),
            actor: "Codex".to_owned(),
            capabilities: super::super::TerminalAgentCapabilities::codex_hooks_v1(),
            provider_session_id: Some("codex-session-7".to_owned()),
            state,
            attention_required,
            resumable: true,
            event_kind: super::super::TerminalAgentEventKind::TurnCompleted,
            summary: "Codex turn completed".to_owned(),
            working_directory: Some(std::path::PathBuf::from("/workspace")),
            file_targets: Vec::new(),
            file_targets_truncated: false,
            command: None,
            exit_code: None,
        }
    }

    #[test]
    fn structured_agent_attention_survives_detach_until_acknowledged()
    -> Result<(), TerminalHostError> {
        let mut host = host(1024);
        let session_id = TerminalSessionId::new();
        host.create(session_id, None)?;
        host.update_agent(
            session_id,
            agent_update(TerminalAgentState::Completed, true),
        )?;

        let detached = host.detach(session_id);
        assert!(
            detached
                .agent
                .as_ref()
                .is_some_and(|agent| agent.attention_required)
        );
        let acknowledged = host.acknowledge_agent_attention(session_id);
        assert!(
            acknowledged
                .agent
                .as_ref()
                .is_some_and(|agent| !agent.attention_required)
        );
        Ok(())
    }

    #[test]
    fn detach_reattach_and_terminate_preserve_session_identity() -> Result<(), TerminalHostError> {
        let mut host = host(1024);
        let session_id = TerminalSessionId::new();
        let created = host.create(session_id, None)?;
        assert_eq!(created.state, TerminalSessionState::Starting);

        let attached = host.attach(session_id, TERMINAL_SESSION_PROTOCOL_VERSION, None);
        assert_eq!(attached.snapshot.session_id, session_id);
        assert_eq!(attached.snapshot.state, TerminalSessionState::Attached);

        let detached = host.detach(session_id);
        assert_eq!(detached.state, TerminalSessionState::Detached);
        assert!(detached.state.may_be_live());

        let reattached = host.attach(session_id, TERMINAL_SESSION_PROTOCOL_VERSION, None);
        assert_eq!(reattached.snapshot.session_id, session_id);
        assert_eq!(reattached.snapshot.state, TerminalSessionState::Attached);

        let exited = host.terminate(session_id, Some(0));
        assert_eq!(
            exited.state,
            TerminalSessionState::Exited { exit_code: Some(0) }
        );
        assert!(!exited.state.may_be_live());
        Ok(())
    }

    #[test]
    fn metadata_and_process_identity_survive_detach() -> Result<(), TerminalHostError> {
        let mut host = host(1024);
        let session_id = TerminalSessionId::new();
        host.create(session_id, Some(std::path::PathBuf::from("/tmp/old")))?;
        host.set_title(session_id, Some("Codex review".to_owned()));
        host.set_working_directory(session_id, Some(std::path::PathBuf::from("/tmp/new")));
        host.set_process_id(session_id, Some(4242));

        let detached = host.detach(session_id);
        assert_eq!(detached.title.as_deref(), Some("Codex review"));
        assert_eq!(
            detached.working_directory.as_deref(),
            Some(std::path::Path::new("/tmp/new"))
        );
        assert_eq!(detached.process_id, Some(4242));
        assert_eq!(detached.state, TerminalSessionState::Detached);
        Ok(())
    }

    #[test]
    fn replay_is_bounded_and_reports_a_gap() -> Result<(), TerminalHostError> {
        let mut host = host(5);
        let session_id = TerminalSessionId::new();
        host.create(session_id, None)?;
        host.attach(session_id, TERMINAL_SESSION_PROTOCOL_VERSION, None);

        host.record_output(session_id, b"abc".to_vec())?;
        host.record_output(session_id, b"def".to_vec())?;

        let attachment = host.attach(session_id, TERMINAL_SESSION_PROTOCOL_VERSION, Some(0));
        assert!(attachment.replay_was_truncated);
        assert_eq!(attachment.replay.len(), 1);
        assert_eq!(attachment.replay[0].sequence, 2);
        assert_eq!(attachment.replay[0].bytes, b"def");
        assert_eq!(attachment.snapshot.earliest_replay_sequence, 2);
        assert_eq!(attachment.snapshot.latest_replay_sequence, 2);
        Ok(())
    }

    #[test]
    fn missing_and_incompatible_are_explicit() -> Result<(), TerminalHostError> {
        let mut host = host(1024);
        let missing_id = TerminalSessionId::new();
        let missing = host.attach(missing_id, TERMINAL_SESSION_PROTOCOL_VERSION, None);
        assert_eq!(missing.snapshot.state, TerminalSessionState::Missing);

        let session_id = TerminalSessionId::new();
        host.create(session_id, None)?;
        let incompatible = host.attach(session_id, TERMINAL_SESSION_PROTOCOL_VERSION + 1, None);
        assert_eq!(
            incompatible.snapshot.state,
            TerminalSessionState::Incompatible {
                host_protocol: TERMINAL_SESSION_PROTOCOL_VERSION,
                client_protocol: TERMINAL_SESSION_PROTOCOL_VERSION + 1,
            }
        );
        assert_eq!(
            host.snapshot(session_id).state,
            TerminalSessionState::Starting
        );
        Ok(())
    }
}
