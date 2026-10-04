use std::cmp::Reverse;

use agent_threads::{SidebarTerminal, SidebarThread, SidebarThreadStatus};
use editor::{Editor, EditorEvent};
use gpui::{
    AnyElement, App, ClickEvent, Context, Entity, EventEmitter, FocusHandle, Focusable, FontWeight,
    IntoElement, Pixels, Render, SharedString, Styled, Subscription, TextStyleRefinement,
    WeakEntity, Window, div, px,
};
use serde::{Deserialize, Serialize};
use ui::{
    Disclosure, HighlightedLabel, IconButton, IconButtonShape, Indicator, Tooltip, prelude::*,
};
use util::ResultExt as _;
use workspace::{
    FocusWorkspaceSidebarSearch, MultiWorkspace, MultiWorkspaceEvent, ProjectGroup,
    ProjectGroupKey, Sidebar, SidebarEvent, SidebarSide,
};

use crate::filter;

const DEFAULT_SIDEBAR_WIDTH: Pixels = px(240.0);
const MIN_SIDEBAR_WIDTH: Pixels = px(180.0);
const MAX_SIDEBAR_WIDTH: Pixels = px(420.0);

/// Every interactive row in the rail is the same height, so a long title, a
/// status label, or a missing actor label can't make the list jitter.
const ROW_HEIGHT: Pixels = px(26.0);
const GROUP_HEADER_HEIGHT: Pixels = px(28.0);

/// The rail's session search field. Tall enough for the editor's own line
/// height at the font size set below, short enough that the search row does
/// not push the first workspace group off a short window.
const SEARCH_FIELD_HEIGHT: Pixels = px(28.0);

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

    /// Higher means more urgent. Used to fold several roots' indicators into
    /// one group-level indicator without inventing a new ordering.
    fn rank(self) -> u8 {
        match self {
            Self::Conflict => 4,
            Self::Modified => 3,
            Self::Added => 2,
            Self::Deleted => 1,
            Self::Clean => 0,
        }
    }

    fn worst(self, other: Self) -> Self {
        if other.rank() > self.rank() {
            other
        } else {
            self
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

/// A group-level summary. A multi-root group shows the branch and changed
/// count across every root it holds, so the header describes the group rather
/// than only its first workspace. The branch comes from the first root that
/// has one; the indicator is the most urgent across roots.
fn summarize_group(group: &ProjectGroup, cx: &App) -> WorkspaceSummary {
    let mut summary = WorkspaceSummary {
        branch: None,
        changed: 0,
        indicator: ChangeIndicator::Clean,
    };
    for workspace in &group.workspaces {
        let workspace_summary = summarize_workspace(workspace, cx);
        if summary.branch.is_none() {
            summary.branch = workspace_summary.branch;
        }
        summary.changed += workspace_summary.changed;
        summary.indicator = summary.indicator.worst(workspace_summary.indicator);
    }
    summary
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

/// The trailing state label, present only for the two states that ask for the
/// user. `Running` and `Idle` are carried by the status dot alone, which is
/// what keeps a busy rail quiet; every row still spells out its full state in
/// a tooltip.
fn status_label(status: SidebarThreadStatus) -> Option<(&'static str, Color)> {
    match status {
        SidebarThreadStatus::NeedsInput => Some(("Needs input", Color::Error)),
        SidebarThreadStatus::Finished => Some(("Finished", Color::Muted)),
        SidebarThreadStatus::Running | SidebarThreadStatus::Idle => None,
    }
}

/// One workspace group's contents in the rail: the workspaces themselves plus
/// every live session tied to them, agent threads and plain shells alike.
///
/// Sessions are collected per workspace and never pooled across groups, so a
/// session running in one workspace can never appear under another
/// workspace's header -- that isolation is dez's product guarantee.
///
/// Every label carries the byte offsets the rail's search query matched inside
/// it, so a row that survived the filter can highlight what let it through.
/// An empty list means there is nothing to highlight: either the search is off,
/// or the row matched only on a field it does not display, such as an agent
/// kind. Both draw identically, so they are deliberately not distinguished.
struct RailGroup {
    group: ProjectGroup,
    /// The group's display name, resolved once here rather than per render so
    /// the filter can match it against the same string the header will draw.
    name: SharedString,
    name_match: Vec<usize>,
    /// Present only for a multi-root group, which is the only case where the
    /// rail spells a root out separately from the header.
    roots: Vec<FilteredRoot>,
    threads: Vec<FilteredThread>,
    terminals: Vec<FilteredTerminal>,
}

/// A workspace root row and the byte offsets the search query matched in its
/// label.
struct FilteredRoot {
    workspace: Entity<workspace::Workspace>,
    label: SharedString,
    label_match: Vec<usize>,
}

/// One live agent session and the byte offsets the search query matched in its
/// title.
struct FilteredThread {
    thread: SidebarThread,
    title_match: Vec<usize>,
}

/// One live plain shell and the byte offsets the search query matched in its
/// title.
struct FilteredTerminal {
    terminal: SidebarTerminal,
    title_match: Vec<usize>,
}

/// The label a workspace root is listed under in a multi-root group: the last
/// segment of its main worktree path, which is the only name the root has that
/// the group's own header does not already carry.
fn workspace_root_label(workspace: &Entity<workspace::Workspace>, cx: &App) -> SharedString {
    workspace
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
        .into()
}

/// How a rail label is drawn. Spelled out once so the highlighted and plain
/// shapes cannot drift apart: same size, same weight, same truncation, or a
/// filtered rail would visibly restyle every row the moment a query appeared.
#[derive(Clone, Copy)]
struct TitleStyle {
    size: LabelSize,
    weight: FontWeight,
}

impl TitleStyle {
    const GROUP: Self = Self {
        size: LabelSize::Small,
        weight: FontWeight::MEDIUM,
    };
    const ROW: Self = Self {
        size: LabelSize::Small,
        weight: FontWeight::NORMAL,
    };
}

/// A title with the bytes at `positions` highlighted. An empty `positions`
/// draws exactly what a plain label would, so the rail needs only this one
/// shape and a filtered rail cannot restyle its rows the moment a query
/// appears.
fn session_title(title: SharedString, positions: &[usize], style: TitleStyle) -> AnyElement {
    HighlightedLabel::new(title, positions.to_vec())
        .size(style.size)
        .weight(style.weight)
        .truncate()
        .into_any_element()
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
    /// The rail's session search. Built here rather than per render because an
    /// `Editor` needs the window it was created for, and rebuilding it would
    /// throw away the query and the cursor on every keystroke.
    search_editor: Entity<Editor>,
    width: Option<Pixels>,
    _subscriptions: Vec<Subscription>,
}

impl DezSidebar {
    pub fn new(
        multi_workspace: WeakEntity<MultiWorkspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut subscriptions = Vec::new();
        if let Some(store) = agent_threads::AgentThreadStore::try_global(cx) {
            // `observe` rather than `subscribe`: the store calls `notify()` for
            // changes that aren't emitted events, and the rail wants all of
            // them rather than just the ones the panel happens to care about.
            subscriptions.push(cx.observe(&store, |_this, _store, cx| cx.notify()));
            // `observe` covers `notify()` only. Opening and closing a session
            // are *emits* (`ThreadOpened` / `ThreadClosed`), and the store does
            // not also notify, so without this the rail keeps showing a row for
            // a session that has been closed -- and a row the user can click
            // into nothing -- until some unrelated change repaints it.
            subscriptions.push(cx.subscribe(
                &store,
                |_this, _store, _event: &agent_threads::AgentThreadStoreEvent, cx| cx.notify(),
            ));
        }
        // Plain shells are classified live from their own screen tail, so the
        // rail has to repaint on terminal activity too, not only on store
        // changes.
        if let Some(subscription) = agent_threads::observe_terminal_activity(cx) {
            subscriptions.push(subscription);
        }
        // The rail's active-workspace highlight, group order, and collapsed
        // state all come from the MultiWorkspace. Repaint on any of its
        // changes: switching workspaces (including by focusing a session from
        // the rail), adding or removing a workspace, and folding a group.
        if let Some(multi_workspace) = multi_workspace.upgrade() {
            subscriptions.push(cx.subscribe(
                &multi_workspace,
                |_this, _multi_workspace, _event: &MultiWorkspaceEvent, cx| cx.notify(),
            ));
        }

        let search_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Search sessions", window, cx);
            // The default single-line editor is sized for a document, which in
            // a 240px rail would either clip the query or push the first
            // workspace group off a short window. `TextSize::Small` is what
            // `LabelSize::Small` resolves to, so the query matches the rows
            // around it.
            editor.set_text_style_refinement(TextStyleRefinement {
                font_size: Some(
                    ui::TextSize::Small
                        .rems(cx)
                        .to_pixels(window.rem_size())
                        .into(),
                ),
                ..Default::default()
            });
            editor.set_show_gutter(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_use_autoclose(false);
            editor
        });
        // The rail rebuilds its rows during render, so a keystroke has nothing
        // to invalidate except the notification itself.
        subscriptions.push(cx.subscribe_in(
            &search_editor,
            window,
            |_this, _editor, event: &EditorEvent, _window, cx| {
                if matches!(event, EditorEvent::BufferEdited) {
                    cx.notify();
                }
            },
        ));

        Self {
            multi_workspace,
            focus_handle: cx.focus_handle(),
            search_editor,
            width: None,
            _subscriptions: subscriptions,
        }
    }

    /// The rail's search query, or `None` when the search is not narrowing
    /// anything. Blank and whitespace-only both read as "no filter", so a
    /// trailing space never empties the rail mid-word.
    fn search_query(&self, cx: &App) -> Option<String> {
        filter::normalize_query(&self.search_editor.read(cx).text(cx)).map(str::to_string)
    }

    fn has_search_query(&self, cx: &App) -> bool {
        self.search_query(cx).is_some()
    }

    fn focus_search_field(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.search_editor
            .read(cx)
            .focus_handle(cx)
            .focus(window, cx);
    }

    fn clear_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.search_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
        cx.notify();
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
    ///
    /// With a search query active, this also does the filtering: a group stays
    /// when its own name matched or when any of its rows did, and each row
    /// carries the offsets that let it through. Filtering happens here, once,
    /// rather than in the row renderers, so the header's attention rollup counts
    /// exactly the rows the user can see.
    fn rail_groups(&self, cx: &mut App) -> Vec<RailGroup> {
        let store = agent_threads::AgentThreadStore::try_global(cx);
        let query = self.search_query(cx);

        self.project_groups(cx)
            .into_iter()
            .filter_map(|group| {
                let name: SharedString = match group.workspaces.first() {
                    Some(workspace) => {
                        let project = workspace.read(cx).project().read(cx);
                        ProjectGroupKey::from_project(project, cx)
                            .display_name(&std::collections::HashMap::default())
                    }
                    None => "Empty Workspace".into(),
                };
                // `None` here means the group's own name did not match, which
                // is what decides whether its rows are filtered individually
                // and whether the group survives at all.
                let name_matched = query
                    .as_deref()
                    .and_then(|query| filter::match_positions(query, &name));
                let name_match = name_matched.clone().unwrap_or_default();

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

                // A group whose own name matched keeps every row: the user asked
                // for that workspace, and hiding its sessions would answer with
                // less than they asked for. Otherwise each row has to survive the
                // filter on its own.
                let row_query = if name_matched.is_some() {
                    None
                } else {
                    query.as_deref()
                };

                // A session is searched by what its row shows plus the two
                // things it does not: the agent kind behind the row's actor
                // label, and the directory it was started in. Both are how a
                // user remembers a session they cannot see the name of.
                let threads: Vec<FilteredThread> = threads
                    .into_iter()
                    .filter_map(|thread| {
                        let working_directory = thread.worktree_root.to_string_lossy();
                        let title_match = match row_query {
                            Some(query) => filter::match_title(
                                query,
                                &thread.title,
                                &[thread.kind_id, &working_directory],
                            ),
                            None => Some(Vec::new()),
                        }?;
                        Some(FilteredThread {
                            thread,
                            title_match,
                        })
                    })
                    .collect();
                let terminals: Vec<FilteredTerminal> = terminals
                    .into_iter()
                    .filter_map(|terminal| {
                        let title_match = match row_query {
                            Some(query) => filter::match_title(query, &terminal.title, &[]),
                            None => Some(Vec::new()),
                        }?;
                        Some(FilteredTerminal {
                            terminal,
                            title_match,
                        })
                    })
                    .collect();

                // A single-root group is already named by its header, so the
                // per-root row would only repeat it. Multi-root groups need the
                // rows to tell their roots apart, and each one has to survive
                // the filter on its own label rather than inherit its group's.
                let roots: Vec<FilteredRoot> = if group.workspaces.len() > 1 {
                    group
                        .workspaces
                        .iter()
                        .filter_map(|workspace| {
                            let label = workspace_root_label(workspace, cx);
                            let label_match = match row_query {
                                Some(query) => filter::match_positions(query, &label),
                                None => Some(Vec::new()),
                            }?;
                            Some(FilteredRoot {
                                workspace: workspace.clone(),
                                label,
                                label_match,
                            })
                        })
                        .collect()
                } else {
                    Vec::new()
                };

                if name_matched.is_none() && threads.is_empty() && terminals.is_empty() {
                    return None;
                }

                Some(RailGroup {
                    group,
                    name,
                    name_match,
                    roots,
                    threads,
                    terminals,
                })
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
        // Not discarded: these are the failures that make a rail row look
        // inert. A stale `terminal_item_id` (the session's pane was closed and
        // the row has not repainted yet) surfaces here, and swallowing it makes
        // a click indistinguishable from a click that was ignored.
        store.update(cx, |store, cx| {
            store.focus_thread(terminal_item_id, window, cx).log_err();
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

    /// The rail's session search field: type to narrow the rail to the
    /// workspaces and live sessions whose names match, with the matched
    /// characters highlighted where they sit.
    ///
    /// It sits under the rollup header rather than inside it, because the
    /// header's own controls are two fixed-width items and a field between
    /// them would leave nothing for the query.
    fn render_search(&self, cx: &mut Context<Self>) -> AnyElement {
        let has_query = self.has_search_query(cx);

        h_flex()
            .id("dez-sidebar-search")
            .aria_label("Search sessions")
            .flex_none()
            .h(SEARCH_FIELD_HEIGHT)
            .px_2()
            .gap_1()
            .child(
                Icon::new(IconName::MagnifyingGlass)
                    .size(IconSize::XSmall)
                    .color(Color::Muted),
            )
            // The editor owns the shrinking, so a long query ellipsizes
            // instead of pushing the clear button out of the row.
            .child(div().min_w_0().flex_1().child(self.search_editor.clone()))
            // The clear button only exists while there is something to clear,
            // so an idle rail does not carry a permanent affordance.
            .when(has_query, |el| {
                el.child(
                    IconButton::new("dez-sidebar-search-clear", IconName::Close)
                        .shape(IconButtonShape::Square)
                        .icon_size(IconSize::XSmall)
                        .tooltip(Tooltip::text("Clear search"))
                        .on_click(cx.listener(|this, _, window, cx| this.clear_search(window, cx))),
                )
            })
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
            // "Nothing here" and "nothing matched" are different answers: the
            // first is fixed by opening a folder, the second by typing a
            // different query, so they get different copy.
            let (title, detail) = match self.search_query(cx) {
                Some(query) => (
                    "No matches",
                    format!("Nothing in the rail matches \"{query}\"."),
                ),
                None => ("No workspaces", "Open a folder to get started.".to_string()),
            };
            return v_flex()
                .id("dez-sidebar-rail")
                .flex_1()
                .w_full()
                .child(self.render_hint(title, &detail, cx))
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
            .h(ROW_HEIGHT)
            .mx_1()
            .items_center()
            .gap_2()
            .px_2()
            .rounded_md()
            .cursor_pointer()
            .hover(move |style| style.bg(hover_bg))
            .child(
                Icon::new(IconName::HistoryRerun)
                    .size(IconSize::XSmall)
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
            name,
            name_match,
            roots,
            threads,
            terminals,
        } = group;
        let attention = Self::attention_from_rows(
            threads.iter().map(|thread| thread.thread.status),
            terminals.iter().map(|terminal| terminal.terminal.status),
        );
        let header = self.group_header(&group, &name, &name_match, active_id, attention, cx);

        // A collapsed group still shows its header (and its attention rollup)
        // but builds no rows at all, so folding a busy group away is cheap.
        let (workspaces, sessions) = if group.expanded {
            // Each row list is collected before the next one is built: every
            // `render_*_row` needs `&mut Context`, so two row closures alive at
            // the same time wouldn't borrow-check.
            let mut sessions: Vec<AnyElement> = threads
                .into_iter()
                .map(|thread| self.render_thread_row(thread, cx))
                .collect();
            sessions.extend(
                terminals
                    .into_iter()
                    .map(|terminal| self.render_terminal_row(terminal, cx)),
            );
            let workspaces: Vec<AnyElement> = roots
                .into_iter()
                .map(|root| self.render_workspace_row(root, active_id, cx))
                .collect();
            (workspaces, sessions)
        } else {
            (Vec::new(), Vec::new())
        };

        v_flex()
            .w_full()
            .pt_2()
            .child(header)
            .children(workspaces)
            .children(sessions)
            .into_any_element()
    }

    fn group_header(
        &self,
        group: &ProjectGroup,
        name: &SharedString,
        name_match: &[usize],
        active_id: Option<gpui::EntityId>,
        attention: usize,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let (branch, changed, indicator) = match group.workspaces.first() {
            Some(_) => {
                let summary = summarize_group(group, cx);
                (summary.branch, summary.changed, summary.indicator)
            }
            None => (None, 0, ChangeIndicator::Clean),
        };

        // Clicking a header selects the workspace already active in the group,
        // or the first root otherwise, so it never silently switches roots.
        let target = group
            .workspaces
            .iter()
            .find(|workspace| Some(workspace.entity_id()) == active_id)
            .or_else(|| group.workspaces.first())
            .cloned();
        let header_id = group
            .workspaces
            .first()
            .map(|workspace| workspace.entity_id().as_u64())
            .unwrap_or(0);
        let hover_bg = cx.theme().colors().element_hover;

        h_flex()
            .id(("dez-sidebar-group", header_id))
            .w_full()
            .h(GROUP_HEADER_HEIGHT)
            .items_center()
            .justify_between()
            .gap_2()
            .px_3()
            .when(target.is_some(), |el| {
                el.cursor_pointer()
                    .hover(move |style| style.bg(hover_bg))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        if let Some(workspace) = target.clone() {
                            this.activate_workspace(workspace, window, cx);
                        }
                    }))
            })
            .child(
                h_flex()
                    .min_w_0()
                    .flex_1()
                    .items_center()
                    .gap_1()
                    .child(
                        Disclosure::new(
                            ("dez-sidebar-group-disclosure", header_id),
                            group.expanded,
                        )
                        .on_click({
                            let multi_workspace = self.multi_workspace.clone();
                            let key = group.key.clone();
                            move |_, _, cx| {
                                // The header itself activates the group's
                                // workspace; folding must not also do that.
                                cx.stop_propagation();
                                multi_workspace
                                    .update(cx, |multi_workspace, cx| {
                                        multi_workspace.toggle_project_group_expanded(&key, cx);
                                    })
                                    .ok();
                            }
                        }),
                    )
                    .child(
                        div()
                            .min_w_0()
                            .flex_1()
                            .overflow_hidden()
                            .child(session_title(name.clone(), name_match, TitleStyle::GROUP)),
                    ),
            )
            .child(
                h_flex()
                    .flex_shrink_0()
                    .items_center()
                    .gap_1()
                    .when(attention > 0, |el| {
                        el.child(Indicator::dot().color(Color::Error).into_any_element())
                            .child(
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
        root: FilteredRoot,
        active_id: Option<gpui::EntityId>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let FilteredRoot {
            workspace,
            label,
            label_match,
        } = root;
        let is_active = active_id == Some(workspace.entity_id());

        h_flex()
            .id(("dez-sidebar-workspace", workspace.entity_id()))
            .w_full()
            .h(ROW_HEIGHT)
            .mx_1()
            .items_center()
            .gap_2()
            .pl_5()
            .pr_2()
            .rounded_md()
            .cursor_pointer()
            .when(is_active, |el| el.bg(cx.theme().colors().element_selected))
            .hover(|style| style.bg(cx.theme().colors().element_hover))
            .child(
                Icon::new(IconName::Folder)
                    .size(IconSize::XSmall)
                    .color(Color::Muted),
            )
            .child(
                div()
                    .min_w_0()
                    .flex_1()
                    .overflow_hidden()
                    .child(session_title(label, &label_match, TitleStyle::ROW)),
            )
            .on_click(cx.listener(move |this, _, window, cx| {
                this.activate_workspace(workspace.clone(), window, cx);
            }))
            .into_any_element()
    }

    /// One live agent session in the rail. Rendered directly under its
    /// workspace group's header, which is what lets the rail answer "what is
    /// running, and where" without a second panel.
    fn render_thread_row(&self, filtered: FilteredThread, cx: &mut Context<Self>) -> AnyElement {
        let FilteredThread {
            thread,
            title_match,
        } = filtered;
        let status = thread.status;
        let terminal_item_id = thread.terminal_item_id;
        let tooltip: SharedString = format!("{} · {}", thread.kind_id, status.state_label()).into();
        let on_click = cx.listener(move |this, _, window, cx| {
            this.focus_thread(terminal_item_id, window, cx);
        });

        self.render_session_row(
            ("dez-sidebar-thread", terminal_item_id.as_u64()),
            Indicator::dot()
                .color(status_color(status))
                .into_any_element(),
            thread.title,
            &title_match,
            Some(thread.kind_id),
            status,
            tooltip,
            on_click,
            cx,
        )
    }

    /// One live plain terminal in the rail. A shell has no agent kind, so it
    /// carries the terminal glyph instead of an actor label; otherwise its row
    /// is identical to an agent session's.
    fn render_terminal_row(
        &self,
        filtered: FilteredTerminal,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let FilteredTerminal {
            terminal,
            title_match,
        } = filtered;
        let status = terminal.status;
        let terminal_item_id = terminal.terminal_item_id;
        let tooltip: SharedString = format!("Terminal · {}", status.state_label()).into();
        let on_click = cx.listener(move |_this, _, window, cx| {
            agent_threads::focus_priority_terminal(terminal_item_id, window, cx).log_err();
        });

        self.render_session_row(
            ("dez-sidebar-terminal", terminal_item_id.as_u64()),
            Icon::new(IconName::Terminal)
                .size(IconSize::XSmall)
                .color(status_color(status))
                .into_any_element(),
            terminal.title,
            &title_match,
            None,
            status,
            tooltip,
            on_click,
            cx,
        )
    }

    /// One live session row, shared by agent sessions and plain terminals so
    /// the two kinds can't drift apart visually. `leading` is the kind marker:
    /// a status dot for an agent thread, a terminal glyph for a shell. The
    /// title stays in the default text color -- only the marker and the
    /// attention label carry status, which is what keeps a busy rail calm.
    fn render_session_row(
        &self,
        id: (&'static str, u64),
        leading: AnyElement,
        title: SharedString,
        title_match: &[usize],
        kind: Option<&'static str>,
        status: SidebarThreadStatus,
        tooltip: SharedString,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        h_flex()
            .id(id)
            .w_full()
            .h(ROW_HEIGHT)
            .mx_1()
            .items_center()
            .gap_2()
            .pl_5()
            .pr_2()
            .rounded_md()
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
                    .child(session_title(title, title_match, TitleStyle::ROW)),
            )
            .children(kind.map(|kind| {
                Label::new(kind)
                    .size(LabelSize::XSmall)
                    .color(Color::Muted)
                    .flex_shrink_0()
            }))
            .children(status_label(status).map(|(text, color)| {
                Label::new(text)
                    .size(LabelSize::XSmall)
                    .color(color)
                    .flex_shrink_0()
            }))
            .tooltip(Tooltip::text(tooltip))
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

    /// Opening the rail from outside lands on the search field whenever there
    /// is already a query: the user who closed the rail mid-search and came
    /// back wants to keep typing, not to start over from an unfiltered list.
    fn prepare_for_focus(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.has_search_query(cx) {
            self.focus_search_field(window, cx);
        }
    }

    fn focus_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.focus_search_field(window, cx);
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
                .flat_map(|group| group.threads.iter().map(|thread| thread.thread.status)),
            rail_groups.iter().flat_map(|group| {
                group
                    .terminals
                    .iter()
                    .map(|terminal| terminal.terminal.status)
            }),
        );
        let header = self.render_header(attention, cx);
        let search = self.render_search(cx);
        let rail = self.render_rail(rail_groups, cx);

        v_flex()
            .key_context("DezSidebar")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().panel_background)
            // Handled here rather than on the search row so the binding works
            // wherever focus happens to be inside the rail, including on a
            // session row.
            .on_action(cx.listener(
                |this: &mut Self, _: &FocusWorkspaceSidebarSearch, window, cx| {
                    this.focus_search_field(window, cx);
                },
            ))
            .child(header)
            .child(search)
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
