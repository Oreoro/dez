use gpui::{
    AnyElement, App, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement, Pixels,
    Render, SharedString, Styled, Subscription, WeakEntity, Window, div, px,
};
use serde::{Deserialize, Serialize};
use ui::prelude::*;
use workspace::{MultiWorkspace, ProjectGroup, ProjectGroupKey, Sidebar, SidebarEvent, SidebarSide};

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

    fn render_header(&self, _cx: &mut Context<Self>) -> AnyElement {
        h_flex()
            .w_full()
            .items_center()
            .gap_2()
            .px_3()
            .py_2()
            .child(
                Label::new("Workspace")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
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
                    (DezSidebarView::Home, "Home"),
                    (DezSidebarView::Files, "Files"),
                    (DezSidebarView::Git, "Git"),
                    (DezSidebarView::Settings, "Settings"),
                ]
                .into_iter()
                .map(|(view, _label)| self.render_tab(view, cx)),
            )
            .into_any_element()
    }

    fn render_tab(&self, view: DezSidebarView, cx: &mut Context<Self>) -> AnyElement {
        let selected = self.active_view == view;
        h_flex()
            .id(("dez-sidebar-tab", view as usize))
            .flex_1()
            .justify_center()
            .py_1()
            .rounded_sm()
            .cursor_pointer()
            .when(selected, |el| {
                el.bg(cx.theme().colors().element_selected)
            })
            .hover(|style| style.bg(cx.theme().colors().element_hover))
            .child(
                Icon::new(view.icon())
                    .size(IconSize::Small)
                    .color(if selected { Color::Default } else { Color::Muted }),
            )
            .on_click(cx.listener(move |this, _, window, cx| {
                this.active_view = view;
                match view {
                    DezSidebarView::Files => window.dispatch_action(
                        Box::new(dez_actions::project_panel::Toggle),
                        cx,
                    ),
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
            DezSidebarView::Files => self.render_hint("Files", "The project panel owns file navigation.", cx),
            DezSidebarView::Git => self.render_hint("Git", "The git panel owns change review.", cx),
            DezSidebarView::Settings => {
                self.render_hint("Settings", "Open settings to configure dez.", cx)
            }
        }
    }

    fn render_home(&self, cx: &mut Context<Self>) -> AnyElement {
        let groups = self.project_groups(cx);
        let active_id = self.active_workspace_id(cx);

        if groups.is_empty() {
            return self.render_hint("No workspaces", "Open a folder to get started.", cx);
        }

        v_flex()
            .id("dez-sidebar-home")
            .flex_1()
            .w_full()
            .overflow_y_scroll()
            .children(
                groups
                    .into_iter()
                    .map(|group| self.render_group(group, active_id, cx)),
            )
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
        let (name, branch) = group
            .workspaces
            .first()
            .map(|workspace| {
                let project = workspace.read(cx).project().read(cx);
                let name = ProjectGroupKey::from_project(project, cx)
                    .display_name(&std::collections::HashMap::default());
                let branch = project
                    .active_repository(cx)
                    .and_then(|repository| {
                        let repository = repository.read(cx);
                        repository.branch.as_ref().map(|branch| branch.name().to_string())
                    })
                    .map(SharedString::from);
                (name, branch)
            })
            .unwrap_or_else(|| ("Empty Workspace".into(), None));

        h_flex()
            .w_full()
            .items_center()
            .justify_between()
            .gap_2()
            .px_3()
            .py_1()
            .child(Label::new(name).size(LabelSize::Small))
            .when_some(branch, |el, branch| {
                el.child(
                    h_flex()
                        .items_center()
                        .gap_1()
                        .child(
                            Icon::new(IconName::GitBranch)
                                .size(IconSize::XSmall)
                                .color(Color::Muted),
                        )
                        .child(
                            Label::new(branch)
                                .size(LabelSize::XSmall)
                                .color(Color::Muted),
                        ),
                )
            })
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
        agent_threads::AgentThreadStore::try_global(cx)
            .map(|store| store.read(cx).attention_count() > 0)
            .unwrap_or(false)
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
            .child(div().w_full().border_t_1().border_color(cx.theme().colors().border))
            .child(self.render_body(cx))
    }
}