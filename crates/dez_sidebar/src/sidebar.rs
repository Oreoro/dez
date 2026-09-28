use std::cmp::Reverse;

use agent_threads::{SidebarTerminal, SidebarThread, SidebarThreadStatus};
use gpui::{
    AnyElement, App, ClickEvent, Context, Entity, EventEmitter, FocusHandle, Focusable,
    IntoElement, Pixels, Render, SharedString, Styled, Subscription, WeakEntity, Window, div, px,
};
use serde::{Deserialize, Serialize};
use ui::{IconButton, IconButtonShape, Indicator, Tooltip, prelude::*};
use workspace::{
    MultiWorkspace, ProjectGroup, ProjectGroupKey, Sidebar, SidebarEvent, SidebarSide,
};

const DEFAULT_SIDEBAR_WIDTH: Pixels = px(240.0);
const MIN_SIDEBAR_WIDTH: Pixels = px(180.0);
const MAX_SIDEBAR_WIDTH: Pixels = px(420.0);

/// A compact read of a workspace's git status, used to pick the icon and color
/// shown beside the workspace header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ChangeIndicator {
    Conflict,
    Modified,
    Added,
    Deleted,
    Clean,
}

impl ChangeIndicator {
    fn from_counts(conflict: usize, modified: usize, added: usize, deleted: usize) -> Self {
        if conflict > 0 {
            Self::Conflict
        } else if modified > 0 {
            Self::Modified
        } else if added > 0 {
            Self::Added
        } else if deleted > 0 {
            Self::Deleted
        } else {
            Self::Clean
        }
    }

    fn icon(self) -> IconName {
        match self {
            Self::Conflict => IconName::Warning,
            Self::Modified => IconName::SquareDot,
            Self::Added => IconName::SquarePlus,
            Self::Deleted => IconName::SquareMinus,
            Self::Clean => IconName::GitBranch,
        }
    }

    fn color(self) -> Color {
        match self {
            Self::Conflict => Color::VersionControlConflict,
            Self::Modified => Color::VersionControlModified,
            Self::Added => Color::VersionControlAdded,
            Self::Deleted => Color::VersionControlDeleted,
            Self::Clean => Color::Muted,
        }
    }
}

/// The per-workspace chrome summary: project name, active branch, and the
/// changed-file count with its indicator.
struct WorkspaceSummary {
    branch: Option<SharedString>,
    changed: usize,
    indicator: ChangeIndicator,
}

fn summarize_workspace(workspace: &Entity<workspace::Workspace>, cx: &App) -> WorkspaceSummary {
    let project = workspace.read(cx).project().read(cx);

    let (branch, changed, indicator) = match project.active_repository(cx) {
        Some(repository) => {
            let repository = repository.read(cx);
            let branch = repository
                .branch
                .as_ref()
                .map(|branch| SharedString::from(branch.name().to_string()));
            let summary = repository.status_summary();
            let tracked = summary.index + summary.worktree;
            let indicator = ChangeIndicator::from_counts(
                summary.conflict,
                tracked.modified,
                tracked.added,
                tracked.deleted,
            );
            (branch, summary.count, indicator)
        }
        None => (None, 0, ChangeIndicator::Clean),
    };

    WorkspaceSummary {
        branch,
        changed,
        indicator,
    }
}

/// The rail's status color for a live session. Shares the Agent Threads
/// panel's palette so the two surfaces agree on what each state looks like:
/// blocked is the alarm, finished-and-unchecked is worth a look, and
/// finished-and-already-checked is calm.
fn status_color(status: SidebarThreadStatus) -> Color {
    match status {
        SidebarThreadStatus::Running => Color::Success,
        SidebarThreadStatus::NeedsInput => Color::Error,
        SidebarThreadStatus::Finished => Color::Warning,
        SidebarThreadStatus::Idle => Color::Muted,
    }
}

/// One workspace group's contents in the rail: the workspaces themselves plus
/// every live session tied to them, agent threads and plain shells alike.
///
/// Sessions are collected per workspace and never pooled across groups, so a
/// session running in one workspace can never appear under another
/// workspace's header -- that isolation is dez's product guarantee.
struct RailGroup {
    group: ProjectGroup,
    threads: Vec<SidebarThread>,
    terminals: Vec<SidebarTerminal>,
}

/// Sidebar-specific state persisted alongside the workspace.
///
/// `active_view` used to be persisted here while the rail still had Files /
/// Git / Settings tabs. It is deliberately not read back -- the rail is a
/// single surface now, and serde ignores that stale field in state saved by an
/// older build.
#[derive(Default, Serialize, Deserialize)]
struct SerializedDezSidebar {
    #[serde(default)]
    width: Option<f32>,
}

/// The dez workspace rail.
///
/// Implements [`workspace::Sidebar`], so `MultiWorkspace` owns placement,
/// resizing, and persistence while this type owns its render.
pub struct DezSidebar {
    multi_workspace: WeakEntity<MultiWorkspace>,
    focus_handle: FocusHandle,
    width: Option<Pixels>,
    _subscriptions: Vec<Subscription>,
}

impl DezSidebar {
    pub fn new(multi_workspace: WeakEntity<MultiWorkspace>, cx: &mut Context<Self>) -> Self {
        let mut subscriptions = Vec::new();
        if let Some(store) = agent_threads::AgentThreadStore::try_global(cx) {
            // `observe` rather than `subscribe`: the store calls `notify()` for
            // changes that aren't emitted events, and the rail wants all of
            // them rather than just the ones the panel happens to care about.
            subscriptions.push(cx.observe(&store, |_this, _store, cx| cx.notify()));
        }
        // Plain shells are classified live from their own screen tail, so the
        // rail has to repaint on terminal activity too, not only on store
        // changes.
        if let Some(subscription) = agent_threads::observe_terminal_activity(cx) {
            subscriptions.push(subscription);
        }

        Self {
            multi_workspace,
            focus_handle: cx.focus_handle(),
            width: None,
            _subscriptions: subscriptions,
        }
    }

    fn effective_width(&self) -> Pixels {
        self.width.unwrap_or(DEFAULT_SIDEBAR_WIDTH)
    }

    fn project_groups(&self, cx: &App) -> Vec<ProjectGroup> {
        self.multi_workspace
            .read_with(cx, |multi_workspace, cx| multi_workspace.project_groups(cx))
            .unwrap_or_default()
    }

    fn active_workspace_id(&self, cx: &App) -> Option<gpui::EntityId> {
        self.multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().entity_id()
            })
            .ok()
    }

    fn activate_workspace(
        &mut self,
        workspace: Entity<workspace::Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.multi_workspace
            .update(cx, |multi_workspace, cx| {
                multi_workspace.activate(workspace, None, window, cx);
            })
            .ok();
    }

    /// The rail's rollup over session statuses: how many rows are asking for
    /// the user. Defined once here so the rail header, each workspace group's
    /// badge, and the status bar's sidebar notification all count the same
    /// rows the same way.
    fn attention_from_rows(
        threads: impl Iterator<Item = SidebarThreadStatus>,
        terminals: impl Iterator<Item = SidebarThreadStatus>,
    ) -> usize {
        threads
            .chain(terminals)
            .filter(|status| status.needs_attention())
            .count()
    }

    /// How many live sessions across every open workspace are asking for the
    /// user. Same projection the rail renders, without building rows -- this
    /// backs the status bar's sidebar notification, which consults the
    /// sidebar even while it is closed.
    fn rail_attention_count(&self, cx: &App) -> usize {
        let store = agent_threads::AgentThreadStore::try_global(cx);
        self.project_groups(cx)
            .iter()
            .flat_map(|group| group.workspaces.iter())
            .map(|workspace| {
                let threads = store
                    .as_ref()
                    .map(|store| store.read(cx).sidebar_threads_for_workspace(workspace))
                    .unwrap_or_default();
                let terminals =
                    agent_threads::sidebar_terminals_for_workspace_readonly(workspace, cx);
                Self::attention_from_rows(
                    threads.iter().map(|thread| thread.status),
                    terminals.iter().map(|terminal| terminal.status),
                )
            })
            .sum()
    }

    /// The rail's contents: every open workspace group paired with the live
    /// sessions tied to its workspaces -- agent threads and plain shells both,
    /// each list most urgent first.
    fn rail_groups(&self, cx: &mut App) -> Vec<RailGroup> {
        let store = agent_threads::AgentThreadStore::try_global(cx);
        self.project_groups(cx)
            .into_iter()
            .map(|group| {
                let mut threads: Vec<SidebarThread> = store
                    .as_ref()
                    .map(|store| {
                        let store = store.read(cx);
                        group
                            .workspaces
                            .iter()
                            .flat_map(|workspace| store.sidebar_threads_for_workspace(workspace))
                            .collect()
                    })
                    .unwrap_or_default();
                threads.sort_by_key(|thread| (Reverse(thread.status), Reverse(thread.launched_at)));

                let mut terminals: Vec<SidebarTerminal> = group
                    .workspaces
                    .iter()
                    .flat_map(|workspace| {
                        agent_threads::sidebar_terminals_for_workspace(workspace, cx)
                    })
                    .collect();
                terminals.sort_by_key(|terminal| Reverse(terminal.status));

                RailGroup {
                    group,
                    threads,
                    terminals,
                }
            })
            .collect()
    }

    /// Routes a rail click through the store's own focus path, so the rail and
    /// the Agent Threads panel can never disagree about what activating a
    /// session does.
    fn focus_thread(
        &mut self,
        terminal_item_id: gpui::EntityId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(store) = agent_threads::AgentThreadStore::try_global(cx) else {
            return;
        };
        store.update(cx, |store, cx| {
            let _ = store.focus_thread(terminal_item_id, window, cx);
        });
        cx.notify();
    }

    /// The rail's rollup header: the section title, how many sessions across
    /// every open workspace are asking for the user, and the one action that
    /// starts a new one.
    fn render_header(&self, attention: usize, cx: &mut Context<Self>) -> AnyElement {
        h_flex()
            .w_full()
            .items_center()
            .justify_between()
            .gap_2()
            .px_3()
            .py_2()
            .child(
                Label::new("Workspaces")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(
                h_flex()
                    .items_center()
                    .gap_1()
                    .when(attention > 0, |el| {
                        el.child(
                            h_flex()
                                .items_center()
                                .gap_1()
                                .child(Indicator::dot().color(Color::Error).into_any_element())
                                .child(
                                    Label::new(attention.to_string())
                                        .size(LabelSize::XSmall)
                                        .color(Color::Error),
                                ),
                        )
                    })
                    // Opinionated default, on purpose: Codex is the flow dez
                    // is built around (the other kinds stay in the app menu),
                    // and the rail must be able to start a session, not just
                    // list one.
                    .child(
                        IconButton::new("dez-sidebar-new-session", IconName::Plus)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("New Codex session"))
                            .on_click(cx.listener(|_this, _, window, cx| {
                                window.dispatch_action(Box::new(agent_threads::NewCodexThread), cx);
                            })),
                    ),
            )
            .into_any_element()
    }

    /// The rail: one scrollable list of workspace groups, each followed by the
    /// live agent sessions tied to it. This is dez's single sidebar -- sessions
    /// are supervised here instead of in a separate panel, so the shell has
    /// exactly one place that answers "what is running, and where".
    ///
    /// The rail deliberately has no Files / Git / Settings tabs. Those tools
    /// stay in Zed's docked panels; a tab that only toggled a panel beside the
    /// rail is exactly what produced two competing left columns.
    fn render_rail(&self, groups: Vec<RailGroup>, cx: &mut Context<Self>) -> AnyElement {
        let active_id = self.active_workspace_id(cx);

        if groups.is_empty() {
            return v_flex()
                .id("dez-sidebar-rail")
                .flex_1()
                .w_full()
                .child(self.render_hint("No workspaces", "Open a folder to get started.", cx))
                .into_any_element();
        }

        // Built before the group rows for the same borrow reason as
        // `render_group`: one `&mut Context` user at a time.
        let history_row = self.render_thread_history_row(cx);

        v_flex()
            .id("dez-sidebar-rail")
            .flex_1()
            .w_full()
            .overflow_y_scroll()
            .py_1()
            .children(
                groups
                    .into_iter()
                    .map(|group| self.render_group(group, active_id, cx)),
            )
            .child(history_row)
            .into_any_element()
    }

    /// The rail supervises *live* sessions. Resuming or archiving a historical
    /// one is still the Agent Threads panel's job, so this stays the single,
    /// deliberately quiet path to it rather than the rail's default action.
    fn render_thread_history_row(&self, cx: &mut Context<Self>) -> AnyElement {
        let hover_bg = cx.theme().colors().element_hover;

        h_flex()
            .id("dez-sidebar-thread-history")
            .w_full()
            .items_center()
            .gap_2()
            .px_3()
            .py_1()
            .cursor_pointer()
            .hover(move |style| style.bg(hover_bg))
            .child(
                Icon::new(IconName::HistoryRerun)
                    .size(IconSize::Small)
                    .color(Color::Muted),
            )
            .child(
                Label::new("Agent thread history")
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
            )
            .on_click(cx.listener(|_this, _, window, cx| {
                window.dispatch_action(Box::new(dez_actions::agent_threads::ToggleFocus), cx);
            }))
            .into_any_element()
    }

    fn render_group(
        &self,
        group: RailGroup,
        active_id: Option<gpui::EntityId>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let RailGroup {
            group,
            threads,
            terminals,
        } = group;
        let attention = Self::attention_from_rows(
            threads.iter().map(|thread| thread.status),
            terminals.iter().map(|terminal| terminal.status),
        );
        let header = self.group_header(&group, attention, cx);

        // Each row list is collected before the next one is built: every
        // `render_*_row` needs `&mut Context`, so two row closures alive at the
        // same time wouldn't borrow-check.
        let mut sessions: Vec<AnyElement> = threads
            .into_iter()
            .map(|thread| self.render_thread_row(thread, cx))
            .collect();
        sessions.extend(
            terminals
                .into_iter()
                .map(|terminal| self.render_terminal_row(terminal, cx)),
        );
        let workspaces: Vec<AnyElement> = group
            .workspaces
            .into_iter()
            .map(|workspace| self.render_workspace_row(workspace, active_id, cx))
            .collect();

        v_flex()
            .w_full()
            .child(header)
            .children(sessions)
            .children(workspaces)
            .into_any_element()
    }

    fn group_header(&self, group: &ProjectGroup, attention: usize, cx: &App) -> AnyElement {
        let (name, branch, changed, indicator) = match group.workspaces.first() {
            Some(workspace) => {
                let project = workspace.read(cx).project().read(cx);
                let name = ProjectGroupKey::from_project(project, cx)
                    .display_name(&std::collections::HashMap::default());
                let summary = summarize_workspace(workspace, cx);
                (name, summary.branch, summary.changed, summary.indicator)
            }
            None => (
                SharedString::from("Empty Workspace"),
                None,
                0,
                ChangeIndicator::Clean,
            ),
        };

        h_flex()
            .w_full()
            .items_center()
            .justify_between()
            .gap_2()
            .px_3()
            .py_1()
            .child(Label::new(name).size(LabelSize::Small))
            .child(
                h_flex()
                    .items_center()
                    .gap_1()
                    .when(attention > 0, |el| {
                        el.child(
                            Label::new(attention.to_string())
                                .size(LabelSize::XSmall)
                                .color(Color::Error),
                        )
                    })
                    .when_some(branch, |el, branch| {
                        el.child(
                            Label::new(branch)
                                .size(LabelSize::XSmall)
                                .color(Color::Muted),
                        )
                    })
                    .child(
                        Icon::new(indicator.icon())
                            .size(IconSize::XSmall)
                            .color(indicator.color()),
                    )
                    .when(changed > 0, |el| {
                        el.child(
                            Label::new(changed.to_string())
                                .size(LabelSize::XSmall)
                                .color(indicator.color()),
                        )
                    }),
            )
            .into_any_element()
    }

    fn render_workspace_row(
        &self,
        workspace: Entity<workspace::Workspace>,
        active_id: Option<gpui::EntityId>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let is_active = active_id == Some(workspace.entity_id());
        let label: SharedString = workspace
            .read(cx)
            .project()
            .read(cx)
            .worktree_paths(cx)
            .main_worktree_path_list()
            .ordered_paths()
            .last()
            .and_then(|path| path.file_name())
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| "Workspace".to_string())
            .into();

        h_flex()
            .id(("dez-sidebar-workspace", workspace.entity_id()))
            .w_full()
            .items_center()
            .gap_2()
            .px_3()
            .py_1()
            .cursor_pointer()
            .when(is_active, |el| el.bg(cx.theme().colors().element_selected))
            .hover(|style| style.bg(cx.theme().colors().element_hover))
            .child(
                Icon::new(IconName::Folder)
                    .size(IconSize::Small)
                    .color(Color::Muted),
            )
            .child(Label::new(label).size(LabelSize::Small))
            .on_click(cx.listener(move |this, _, window, cx| {
                this.activate_workspace(workspace.clone(), window, cx);
            }))
            .into_any_element()
    }

    /// One live agent session in the rail. Rendered directly under its
    /// workspace group's header, which is what lets the rail answer "what is
    /// running, and where" without a second panel.
    fn render_thread_row(&self, thread: SidebarThread, cx: &mut Context<Self>) -> AnyElement {
        let status = thread.status;
        let terminal_item_id = thread.terminal_item_id;
        let on_click = cx.listener(move |this, _, window, cx| {
            this.focus_thread(terminal_item_id, window, cx);
        });

        self.render_session_row(
            ("dez-sidebar-thread", terminal_item_id.as_u64()),
            Indicator::dot()
                .color(status_color(status))
                .into_any_element(),
            thread.title,
            Some(thread.kind_id),
            status,
            on_click,
            cx,
        )
    }

    /// One live plain terminal in the rail. A shell has no agent kind, so it
    /// carries the terminal glyph instead of an actor label; otherwise its row
    /// is identical to an agent session's.
    fn render_terminal_row(&self, terminal: SidebarTerminal, cx: &mut Context<Self>) -> AnyElement {
        let status = terminal.status;
        let terminal_item_id = terminal.terminal_item_id;
        let on_click = cx.listener(move |_this, _, window, cx| {
            let _ = agent_threads::focus_priority_terminal(terminal_item_id, window, cx);
        });

        self.render_session_row(
            ("dez-sidebar-terminal", terminal_item_id.as_u64()),
            Icon::new(IconName::Terminal)
                .size(IconSize::Indicator)
                .color(status_color(status))
                .into_any_element(),
            terminal.title,
            None,
            status,
            on_click,
            cx,
        )
    }

    /// One live session row, shared by agent sessions and plain terminals so
    /// the two kinds can't drift apart visually. `leading` is the kind marker:
    /// a status dot for an agent thread, a terminal glyph for a shell.
    fn render_session_row(
        &self,
        id: (&'static str, u64),
        leading: AnyElement,
        title: SharedString,
        kind: Option<&'static str>,
        status: SidebarThreadStatus,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let color = status_color(status);

        h_flex()
            .id(id)
            .w_full()
            .items_center()
            .gap_2()
            .pl_6()
            .pr_3()
            .py_1()
            .cursor_pointer()
            .hover(|style| style.bg(cx.theme().colors().element_hover))
            .child(leading)
            // The title owns all shrinking (`min_w_0` + `flex_1`, the same
            // pattern as the Agent Threads panel's rows): a long title
            // ellipsizes instead of squeezing the status and kind labels out
            // of the row.
            .child(
                div()
                    .min_w_0()
                    .flex_1()
                    .overflow_hidden()
                    .child(
                        Label::new(title)
                            .size(LabelSize::Small)
                            .color(color)
                            .truncate(),
                    ),
            )
            .children(kind.map(|kind| {
                Label::new(kind)
                    .size(LabelSize::XSmall)
                    .color(Color::Muted)
                    .flex_shrink_0()
            }))
            .child(
                Label::new(status.state_label())
                    .size(LabelSize::XSmall)
                    .color(Color::Muted)
                    .flex_shrink_0(),
            )
            .on_click(on_click)
            .into_any_element()
    }

    fn render_hint(&self, title: &str, detail: &str, _cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .flex_1()
            .w_full()
            .items_center()
            .justify_center()
            .gap_1()
            .px_3()
            .child(Label::new(title.to_string()).size(LabelSize::Small))
            .child(
                Label::new(detail.to_string())
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
            )
            .into_any_element()
    }
}

impl Focusable for DezSidebar {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<SidebarEvent> for DezSidebar {}

impl Sidebar for DezSidebar {
    fn width(&self, _cx: &App) -> Pixels {
        self.effective_width()
    }

    fn set_width(&mut self, width: Option<Pixels>, cx: &mut Context<Self>) {
        self.width = width.map(|width| width.clamp(MIN_SIDEBAR_WIDTH, MAX_SIDEBAR_WIDTH));
        cx.emit(SidebarEvent::SerializeNeeded);
        cx.notify();
    }

    /// Mirrors the rail header's rollup: agent threads *and* plain shells
    /// that need attention count, so the status bar can't disagree with what
    /// the rail shows. Read-only on purpose -- the status bar consults this
    /// on its own renders, even while the rail is closed.
    fn has_notifications(&self, cx: &App) -> bool {
        self.rail_attention_count(cx) > 0
    }

    fn side(&self, _cx: &App) -> SidebarSide {
        SidebarSide::Left
    }

    /// The rail always presents the sessions list, so "is the threads list the
    /// active view" is unconditionally true here. Workspace code uses this to
    /// decide whether thread navigation keys apply; they always do.
    fn is_threads_list_view_active(&self) -> bool {
        true
    }

    fn serialized_state(&self, _cx: &App) -> Option<String> {
        let state = SerializedDezSidebar {
            width: self.width.map(|width| f32::from(width)),
        };
        serde_json::to_string(&state).ok()
    }

    fn restore_serialized_state(
        &mut self,
        state: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Ok(state) = serde_json::from_str::<SerializedDezSidebar>(state) else {
            return;
        };
        self.width = state
            .width
            .map(|width| px(width).clamp(MIN_SIDEBAR_WIDTH, MAX_SIDEBAR_WIDTH));
        cx.notify();
    }
}

impl Render for DezSidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let rail_groups = self.rail_groups(cx);
        let attention = Self::attention_from_rows(
            rail_groups
                .iter()
                .flat_map(|group| group.threads.iter().map(|thread| thread.status)),
            rail_groups
                .iter()
                .flat_map(|group| group.terminals.iter().map(|terminal| terminal.status)),
        );
        let header = self.render_header(attention, cx);
        let rail = self.render_rail(rail_groups, cx);

        v_flex()
            .key_context("DezSidebar")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .child(header)
            .child(
                div()
                    .w_full()
                    .border_t_1()
                    .border_color(cx.theme().colors().border),
            )
            .child(rail)
    }
}

#[cfg(test)]
mod tests {
    use super::{ChangeIndicator, status_color};
    use agent_threads::SidebarThreadStatus;
    use ui::Color;

    #[test]
    fn session_status_color_is_alarming_only_when_it_should_be() {
        assert!(matches!(
            status_color(SidebarThreadStatus::Running),
            Color::Success
        ));
        assert!(matches!(
            status_color(SidebarThreadStatus::NeedsInput),
            Color::Error
        ));
        assert!(matches!(
            status_color(SidebarThreadStatus::Finished),
            Color::Warning
        ));
        assert!(matches!(
            status_color(SidebarThreadStatus::Idle),
            Color::Muted
        ));
    }

    #[test]
    fn change_indicator_prefers_conflict_then_modified_then_added_then_deleted() {
        assert_eq!(
            ChangeIndicator::from_counts(1, 2, 3, 4),
            ChangeIndicator::Conflict
        );
        assert_eq!(
            ChangeIndicator::from_counts(0, 2, 3, 4),
            ChangeIndicator::Modified
        );
        assert_eq!(
            ChangeIndicator::from_counts(0, 0, 3, 4),
            ChangeIndicator::Added
        );
        assert_eq!(
            ChangeIndicator::from_counts(0, 0, 0, 4),
            ChangeIndicator::Deleted
        );
        assert_eq!(
            ChangeIndicator::from_counts(0, 0, 0, 0),
            ChangeIndicator::Clean
        );
    }
}
