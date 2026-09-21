use gpui::{
    AnyElement, App, ClickEvent, Context, Entity, EventEmitter, FocusHandle, Focusable,
    IntoElement, Pixels, Render, SharedString, Styled, Subscription, WeakEntity, Window, div, px,
};
use serde::{Deserialize, Serialize};
use ui::prelude::*;
use workspace::{
    MultiWorkspace, ProjectGroup, ProjectGroupKey, Sidebar, SidebarEvent, SidebarSide,
};

const DEFAULT_SIDEBAR_WIDTH: Pixels = px(240.0);
const MIN_SIDEBAR_WIDTH: Pixels = px(180.0);
const MAX_SIDEBAR_WIDTH: Pixels = px(420.0);

/// Which surface the sidebar is currently presenting. This is dez's
/// browser-like view model: each variant is a navigator over a different part
/// of the workspace.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum DezSidebarView {
    #[default]
    Home,
    Files,
    Git,
    Settings,
}

impl DezSidebarView {
    fn icon(self) -> IconName {
        match self {
            DezSidebarView::Home => IconName::ListTree,
            DezSidebarView::Files => IconName::FileTree,
            DezSidebarView::Git => IconName::GitBranch,
            DezSidebarView::Settings => IconName::Settings,
        }
    }

    fn label(self) -> &'static str {
        match self {
            DezSidebarView::Home => "Home",
            DezSidebarView::Files => "Files",
            DezSidebarView::Git => "Git",
            DezSidebarView::Settings => "Settings",
        }
    }
}

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
    name: SharedString,
    branch: Option<SharedString>,
    changed: usize,
    indicator: ChangeIndicator,
}

fn summarize_workspace(workspace: &Entity<workspace::Workspace>, cx: &App) -> WorkspaceSummary {
    let project = workspace.read(cx).project().read(cx);
    let name: SharedString = project
        .worktree_paths(cx)
        .main_worktree_path_list()
        .ordered_paths()
        .last()
        .and_then(|path| path.file_name())
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "Workspace".to_string())
        .into();

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
        name,
        branch,
        changed,
        indicator,
    }
}

/// Sidebar-specific state persisted alongside the workspace.
#[derive(Default, Serialize, Deserialize)]
struct SerializedDezSidebar {
    #[serde(default)]
    width: Option<f32>,
    #[serde(default)]
    active_view: DezSidebarView,
}

/// The dez workspace navigator.
///
/// Implements [`workspace::Sidebar`], so `MultiWorkspace` owns placement,
/// resizing, and persistence while this type owns its view state and render.
pub struct DezSidebar {
    multi_workspace: WeakEntity<MultiWorkspace>,
    focus_handle: FocusHandle,
    width: Option<Pixels>,
    active_view: DezSidebarView,
    _subscriptions: Vec<Subscription>,
}

impl DezSidebar {
    pub fn new(multi_workspace: WeakEntity<MultiWorkspace>, cx: &mut Context<Self>) -> Self {
        let mut subscriptions = Vec::new();
        if let Some(store) = agent_threads::AgentThreadStore::try_global(cx) {
            subscriptions.push(cx.subscribe(&store, |_this, _store, _event, cx| {
                cx.notify();
            }));
        }

        Self {
            multi_workspace,
            focus_handle: cx.focus_handle(),
            width: None,
            active_view: DezSidebarView::default(),
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

    fn attention_count(&self, cx: &App) -> usize {
        agent_threads::AgentThreadStore::try_global(cx)
            .map(|store| store.read(cx).attention_count())
            .unwrap_or(0)
    }

    fn render_header(&self, cx: &mut Context<Self>) -> AnyElement {
        let attention = self.attention_count(cx);

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
            .when(attention > 0, |el| {
                el.child(
                    Label::new(attention.to_string())
                        .size(LabelSize::XSmall)
                        .color(Color::Accent),
                )
            })
            .into_any_element()
    }

    fn render_tabs(&self, cx: &mut Context<Self>) -> AnyElement {
        h_flex()
            .w_full()
            .items_center()
            .gap_1()
            .px_2()
            .py_1()
            .children(
                [
                    DezSidebarView::Home,
                    DezSidebarView::Files,
                    DezSidebarView::Git,
                    DezSidebarView::Settings,
                ]
                .into_iter()
                .map(|view| self.render_tab(view, cx)),
            )
            .into_any_element()
    }

    fn render_tab(&self, view: DezSidebarView, cx: &mut Context<Self>) -> AnyElement {
        let selected = self.active_view == view;
        h_flex()
            .id(("dez-sidebar-tab", view as usize))
            .flex_1()
            .justify_center()
            .gap_1()
            .py_1()
            .rounded_sm()
            .cursor_pointer()
            .when(selected, |el| el.bg(cx.theme().colors().element_selected))
            .hover(|style| style.bg(cx.theme().colors().element_hover))
            .child(
                Icon::new(view.icon())
                    .size(IconSize::Small)
                    .color(if selected {
                        Color::Default
                    } else {
                        Color::Muted
                    }),
            )
            .child(
                Label::new(view.label())
                    .size(LabelSize::XSmall)
                    .color(if selected {
                        Color::Default
                    } else {
                        Color::Muted
                    }),
            )
            .on_click(cx.listener(move |this, _, window, cx| {
                this.active_view = view;
                match view {
                    DezSidebarView::Files => {
                        window.dispatch_action(Box::new(dez_actions::project_panel::Toggle), cx)
                    }
                    DezSidebarView::Git => {
                        window.dispatch_action(Box::new(git_ui::git_panel::Toggle), cx)
                    }
                    DezSidebarView::Settings => {
                        window.dispatch_action(Box::new(dez_actions::OpenSettings), cx)
                    }
                    DezSidebarView::Home => {}
                }
                cx.notify();
            }))
            .into_any_element()
    }

    fn render_body(&self, cx: &mut Context<Self>) -> AnyElement {
        match self.active_view {
            DezSidebarView::Home => self.render_home(cx),
            DezSidebarView::Files => self.render_files(cx),
            DezSidebarView::Git => self.render_git(cx),
            DezSidebarView::Settings => self.render_settings(cx),
        }
    }

    fn render_files(&self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .flex_1()
            .w_full()
            .child(self.render_action_row(
                "dez-sidebar-files-project-panel",
                "Project panel",
                "Browse the files in this workspace",
                IconName::FileTree,
                |_, window, cx| {
                    window.dispatch_action(Box::new(dez_actions::project_panel::Toggle), cx)
                },
                cx,
            ))
            .into_any_element()
    }

    fn render_git(&self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .flex_1()
            .w_full()
            .child(self.render_action_row(
                "dez-sidebar-git-changes",
                "Changes",
                "Review the working tree",
                IconName::GitBranch,
                |_, window, cx| window.dispatch_action(Box::new(git_ui::git_panel::Toggle), cx),
                cx,
            ))
            .child(self.render_action_row(
                "dez-sidebar-git-history",
                "History",
                "Browse commits in the Git Graph",
                IconName::HistoryRerun,
                |_, window, cx| window.dispatch_action(Box::new(git_ui::git_graph::Open), cx),
                cx,
            ))
            .into_any_element()
    }

    fn render_settings(&self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .flex_1()
            .w_full()
            .child(self.render_action_row(
                "dez-sidebar-settings-open",
                "Open settings",
                "Configure dez and its extensions",
                IconName::Settings,
                |_, window, cx| window.dispatch_action(Box::new(dez_actions::OpenSettings), cx),
                cx,
            ))
            .into_any_element()
    }

    fn render_home(&self, cx: &mut Context<Self>) -> AnyElement {
        let groups = self.project_groups(cx);
        let active_id = self.active_workspace_id(cx);

        let mut body = v_flex()
            .id("dez-sidebar-home")
            .flex_1()
            .w_full()
            .overflow_y_scroll()
            .child(self.render_activity_row(cx));

        if groups.is_empty() {
            body =
                body.child(self.render_hint("No workspaces", "Open a folder to get started.", cx));
        }

        body.children(
            groups
                .into_iter()
                .map(|group| self.render_group(group, active_id, cx)),
        )
        .into_any_element()
    }

    /// A compact chrome row for agent activity. Clicking it opens the Agent
    /// Threads panel; the count mirrors the panel's attention rollup.
    fn render_activity_row(&self, cx: &mut Context<Self>) -> AnyElement {
        let attention = self.attention_count(cx);
        let hover_bg = cx.theme().colors().element_hover;

        h_flex()
            .id("dez-sidebar-activity")
            .w_full()
            .items_center()
            .justify_between()
            .gap_2()
            .px_3()
            .py_1()
            .cursor_pointer()
            .hover(move |style| style.bg(hover_bg))
            .child(
                h_flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Icon::new(IconName::Sparkle)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(Label::new("Activity").size(LabelSize::Small)),
            )
            .when(attention > 0, |el| {
                el.child(
                    Label::new(attention.to_string())
                        .size(LabelSize::XSmall)
                        .color(Color::Accent),
                )
            })
            .on_click(cx.listener(|_this, _, window, cx| {
                window.dispatch_action(Box::new(dez_actions::agent_threads::ToggleFocus), cx);
            }))
            .into_any_element()
    }

    fn render_group(
        &self,
        group: ProjectGroup,
        active_id: Option<gpui::EntityId>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let header = self.group_header(&group, cx);

        v_flex()
            .w_full()
            .child(header)
            .children(
                group
                    .workspaces
                    .into_iter()
                    .map(|workspace| self.render_workspace_row(workspace, active_id, cx)),
            )
            .into_any_element()
    }

    fn group_header(&self, group: &ProjectGroup, cx: &App) -> AnyElement {
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

    fn render_action_row(
        &self,
        id: &'static str,
        title: &'static str,
        detail: &'static str,
        icon: IconName,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let hover_bg = cx.theme().colors().element_hover;

        h_flex()
            .id(id)
            .w_full()
            .items_center()
            .gap_2()
            .px_3()
            .py_1()
            .cursor_pointer()
            .hover(move |style| style.bg(hover_bg))
            .child(Icon::new(icon).size(IconSize::Small).color(Color::Muted))
            .child(
                v_flex()
                    .gap_0p5()
                    .child(Label::new(title).size(LabelSize::Small))
                    .child(
                        Label::new(detail)
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    ),
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

    fn has_notifications(&self, cx: &App) -> bool {
        self.attention_count(cx) > 0
    }

    fn side(&self, _cx: &App) -> SidebarSide {
        SidebarSide::Left
    }

    fn is_threads_list_view_active(&self) -> bool {
        self.active_view == DezSidebarView::Home
    }

    fn serialized_state(&self, _cx: &App) -> Option<String> {
        let state = SerializedDezSidebar {
            width: self.width.map(|width| f32::from(width)),
            active_view: self.active_view,
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
        self.active_view = state.active_view;
        cx.notify();
    }
}

impl Render for DezSidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("DezSidebar")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .child(self.render_header(cx))
            .child(self.render_tabs(cx))
            .child(
                div()
                    .w_full()
                    .border_t_1()
                    .border_color(cx.theme().colors().border),
            )
            .child(self.render_body(cx))
    }
}

#[cfg(test)]
mod tests {
    use super::ChangeIndicator;

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
