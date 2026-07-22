use std::collections::HashMap;

use gpui::{App, AppContext as _, Context, Entity, Global};

use crate::{Event, Terminal};

use super::{
    InProcessTerminalHost, TERMINAL_SESSION_PROTOCOL_VERSION, TerminalAttachment, TerminalHostId,
    TerminalSessionId, TerminalSessionRef, TerminalSessionSnapshot,
};

struct GlobalLocalTerminalHost(Entity<LocalTerminalHost>);

impl Global for GlobalLocalTerminalHost {}

/// An attachment to an actual terminal retained by the local in-process host.
pub struct AttachedLocalTerminal {
    pub attachment: TerminalAttachment,
    pub terminal: Option<Entity<Terminal>>,
}

/// GPUI adapter that retains local terminal computation independently from a
/// `TerminalView` or pane.
///
/// This is intentionally an in-process owner. It proves detach/reattach
/// semantics against the real terminal entity, but it does not survive a GUI
/// process exit; that requires the later helper-process transport.
pub struct LocalTerminalHost {
    model: InProcessTerminalHost,
    terminals: HashMap<TerminalSessionId, Entity<Terminal>>,
}

impl LocalTerminalHost {
    pub fn init(host_id: TerminalHostId, cx: &mut App) -> Entity<Self> {
        if let Some(host) = Self::try_global(cx) {
            return host;
        }

        let host = cx.new(|_| Self {
            model: InProcessTerminalHost::new(host_id, 1024 * 1024),
            terminals: HashMap::new(),
        });
        cx.set_global(GlobalLocalTerminalHost(host.clone()));
        host
    }

    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalLocalTerminalHost>()
            .map(|host| host.0.clone())
    }

    pub fn register(
        &mut self,
        terminal: Entity<Terminal>,
        cx: &mut Context<Self>,
    ) -> TerminalSessionId {
        let session_id = terminal.read(cx).session_id();
        if self.terminals.contains_key(&session_id) {
            return session_id;
        }

        let working_directory = terminal.read(cx).working_directory();
        if self.model.create(session_id, working_directory).is_err() {
            return session_id;
        }
        let title = terminal.read(cx).title(true);
        self.model.set_title(session_id, Some(title));
        self.model.set_process_id(
            session_id,
            terminal
                .read(cx)
                .pid()
                .map(|process_id| process_id.as_u32()),
        );
        self.model
            .attach(session_id, TERMINAL_SESSION_PROTOCOL_VERSION, None);
        self.terminals.insert(session_id, terminal.clone());

        cx.subscribe(&terminal, move |this, terminal, event, cx| match event {
            Event::TitleChanged | Event::BreadcrumbsChanged | Event::Wakeup => {
                let terminal = terminal.read(cx);
                this.model.set_title(session_id, Some(terminal.title(true)));
                this.model
                    .set_working_directory(session_id, terminal.working_directory());
            }
            Event::CloseTerminal => {
                let exit_code = terminal.read(cx).exit_code();
                this.model.terminate(session_id, exit_code);
                this.model.set_process_id(session_id, None);
                this.terminals.remove(&session_id);
                cx.notify();
            }
            Event::ProcessExited { exit_code } => {
                this.model.terminate(session_id, *exit_code);
                this.model.set_process_id(session_id, None);
                this.terminals.remove(&session_id);
                cx.notify();
            }
            Event::Bell
            | Event::BlinkChanged(_)
            | Event::SelectionsChanged
            | Event::NewNavigationTarget(_)
            | Event::Open(_) => {}
        })
        .detach();

        session_id
    }

    pub fn list(&self) -> Vec<TerminalSessionSnapshot> {
        self.model.list()
    }

    pub fn host_id(&self) -> TerminalHostId {
        self.model.host_id()
    }

    /// Returns a durable reference only when this host actually owns the
    /// session. Display-only and remote terminals must not be persisted as
    /// locally reattachable computation.
    pub fn session_ref_if_registered(
        &self,
        session_id: TerminalSessionId,
    ) -> Option<TerminalSessionRef> {
        self.terminals
            .contains_key(&session_id)
            .then_some(TerminalSessionRef {
                host_id: self.model.host_id(),
                session_id,
            })
    }

    pub fn detach(&mut self, session_id: TerminalSessionId) -> TerminalSessionSnapshot {
        self.model.detach(session_id)
    }

    pub fn attach(
        &mut self,
        session_id: TerminalSessionId,
        replay_after_sequence: Option<u64>,
    ) -> AttachedLocalTerminal {
        let attachment = self.model.attach(
            session_id,
            TERMINAL_SESSION_PROTOCOL_VERSION,
            replay_after_sequence,
        );
        let terminal = attachment
            .snapshot
            .state
            .accepts_input()
            .then(|| self.terminals.get(&session_id).cloned())
            .flatten();
        AttachedLocalTerminal {
            attachment,
            terminal,
        }
    }

    pub fn terminate(
        &mut self,
        session_id: TerminalSessionId,
        cx: &mut Context<Self>,
    ) -> TerminalSessionSnapshot {
        let snapshot = self.model.terminate(session_id, None);
        if let Some(terminal) = self.terminals.remove(&session_id) {
            terminal.update(cx, |terminal, cx| terminal.terminate_process(cx));
        }
        snapshot
    }
}
