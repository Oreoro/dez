use crate::{
    DesignSystemSettings, Event as WorkspaceEvent, ItemHandle, MultiWorkspace, Pane, SidebarSide,
    ToggleSidebar, Workspace, WorkspaceAccessState, sidebar_header_control_metrics,
    sidebar_side_context_menu, workspace_access_state,
};
use gpui::{
    Anchor, AnyElement, AnyView, App, Context, Decorations, Entity, FocusHandle, Focusable,
    IntoElement, ParentElement, Pixels, Render, Role, SharedString, Styled, Subscription,
    WeakEntity, Window,
};
use paths::APP_NAME;
use project::git_store::{GitStoreEvent, RepositoryEvent};
use settings::{Settings as _, SettingsContent, SettingsStore, update_settings_file};
use std::{any::TypeId, path::Path, sync::Arc};
use theme::CLIENT_SIDE_DECORATION_ROUNDING;
use ui::{ContextMenu, Divider, IconPosition, Indicator, Tooltip, prelude::*, right_click_menu};

const WORKSPACE_STATUS_LABEL_MIN_VIEWPORT_WIDTH: Pixels = px(760.0);
const WORKSPACE_STATUS_NAME_MIN_VIEWPORT_WIDTH: Pixels = px(960.0);
const WORKSPACE_STATUS_NAME_MAX_CHARACTERS: usize = 24;
const REPOSITORY_STATUS_MIN_VIEWPORT_WIDTH: Pixels = px(1200.0);

/// Describes how a status-bar item can be hidden by the user.
///
/// Every [`StatusItemView`] must either provide this (so that the user gets a
/// "Hide Button" entry in the right-click menu) or explicitly return `None`
/// to opt out. Returning `None` should be reserved for items that are
/// already conditional on some other setting exposed elsewhere (e.g., the
/// activity indicator, which disappears on its own once there's no work to
/// display).
#[derive(Clone)]
pub struct HideStatusItem {
    hide: Arc<dyn Fn(&mut SettingsContent) + Send + Sync>,
}

impl HideStatusItem {
    pub fn new(hide: impl Fn(&mut SettingsContent) + Send + Sync + 'static) -> Self {
        Self {
            hide: Arc::new(hide),
        }
    }

    /// Persists the hide by updating the user settings file.
    pub fn apply(&self, cx: &App) {
        let hide = self.hide.clone();
        let fs = <dyn fs::Fs>::global(cx);
        update_settings_file(fs, cx, move |settings, _cx| (hide)(settings));
    }
}

pub trait StatusItemView: Render {
    /// Event callback that is triggered when the active pane item changes.
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn crate::ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    );

    /// Returns metadata describing how this item can be hidden from the
    /// status bar by writing to the user settings file.
    ///
    /// Implementors that return `None` must be inherently conditional on
    /// another user-exposed setting; otherwise, they should return `Some` so
    /// that the status bar can show a "Hide Button" entry in its
    /// right-click menu.
    fn hide_setting(&self, cx: &App) -> Option<HideStatusItem>;
}

trait StatusItemViewHandle: Send {
    fn to_any(&self) -> AnyView;
    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut App,
    );
    fn item_type(&self) -> TypeId;
    fn hide_setting(&self, cx: &App) -> Option<HideStatusItem>;
}

#[derive(Default)]
struct SidebarStatus {
    open: bool,
    side: SidebarSide,
    attention_count: usize,
    access_required: bool,
    show_toggle: bool,
    workspace_name: Option<SharedString>,
}

fn workspace_roots_require_access<'a>(
    is_local: bool,
    workspace_roots: impl IntoIterator<Item = &'a Path>,
    access_state: &WorkspaceAccessState,
) -> bool {
    if !is_local {
        return false;
    }
    let WorkspaceAccessState::AccessRequired { roots } = access_state else {
        return false;
    };

    workspace_roots.into_iter().any(|workspace_root| {
        roots.iter().any(|blocked_root| {
            blocked_root.starts_with(workspace_root) || workspace_root.starts_with(blocked_root)
        })
    })
}

impl SidebarStatus {
    fn query(multi_workspace: &Option<WeakEntity<MultiWorkspace>>, cx: &App) -> Self {
        multi_workspace
            .as_ref()
            .and_then(|mw| mw.upgrade())
            .map(|mw| {
                let mw = mw.read(cx);
                let enabled = mw.multi_workspace_enabled(cx);
                let project_group_key = mw.workspace().read(cx).project_group_key(cx);
                let workspace_name = project_group_key
                    .path_list()
                    .ordered_paths()
                    .next()
                    .and_then(|path| path.file_name())
                    .map(|name| SharedString::from(name.to_string_lossy().into_owned()));
                let access_required = workspace_roots_require_access(
                    project_group_key.host().is_none(),
                    project_group_key
                        .path_list()
                        .ordered_paths()
                        .map(|path| path.as_path()),
                    &workspace_access_state(cx),
                );
                Self {
                    open: mw.sidebar_open() && enabled,
                    side: mw.sidebar_side(cx),
                    attention_count: mw.sidebar_attention_count(cx),
                    access_required,
                    show_toggle: enabled,
                    workspace_name,
                }
            })
            .unwrap_or_default()
    }
}

pub struct StatusBar {
    left_items: Vec<Box<dyn StatusItemViewHandle>>,
    right_items: Vec<Box<dyn StatusItemViewHandle>>,
    active_pane: Entity<Pane>,
    workspace: WeakEntity<Workspace>,
    multi_workspace: Option<WeakEntity<MultiWorkspace>>,
    focus_handle: FocusHandle,
    _observe_active_pane: Subscription,
    _workspace_subscriptions: Vec<Subscription>,
    _observe_multi_workspace: Option<Subscription>,
    _settings_subscription: Subscription,
}

fn status_bar_label(app_name: &str) -> &'static str {
    if app_name == "Zed" {
        "Status bar"
    } else {
        "Workspace status and navigation"
    }
}

fn status_bar_height(
    app_name: &str,
    density: settings::CanvasDensity,
    interface_scale: f32,
) -> Option<Pixels> {
    (app_name != "Zed").then(|| {
        let base_height = match density {
            settings::CanvasDensity::Compact => 24.0,
            settings::CanvasDensity::Balanced => 28.0,
            settings::CanvasDensity::Spacious => 32.0,
        };
        px(base_height * interface_scale)
    })
}

fn status_bar_responsive_viewport_width(
    app_name: &str,
    viewport_width: Pixels,
    interface_scale: f32,
) -> Pixels {
    if app_name == "Zed" {
        viewport_width
    } else {
        viewport_width * interface_scale.max(f32::EPSILON).recip()
    }
}

fn sidebar_toggle_label(app_name: &str, open: bool) -> &'static str {
    match (app_name == "Zed", open) {
        (true, true) => "Hide Sessions",
        (true, false) => "Open Sessions",
        (false, true) => "Hide Workspaces",
        (false, false) => "Open Workspaces",
    }
}

fn sidebar_toggle_tooltip_label(
    app_name: &str,
    open: bool,
    access_required: bool,
    attention_count: usize,
) -> SharedString {
    let base_label = sidebar_toggle_label(app_name, open);
    if app_name == "Zed" {
        return base_label.into();
    }

    match (access_required, attention_count) {
        (true, 0) => format!("{base_label} · Access required").into(),
        (true, count) => format!("{base_label} · Access required · Attention {count}").into(),
        (false, 0) => base_label.into(),
        (false, count) => format!("{base_label} · Attention {count}").into(),
    }
}

fn sidebar_toggle_accessibility_label(
    app_name: &str,
    open: bool,
    workspace_name: Option<&SharedString>,
    access_required: bool,
    attention_count: usize,
) -> SharedString {
    let base_label = match (app_name == "Zed", open) {
        (true, true) => "Hide Sessions",
        (true, false) => "Open Sessions",
        (false, true) => "Hide Workspaces",
        (false, false) => "Open Workspaces",
    };
    let workspace_context = if app_name == "Zed" {
        String::new()
    } else {
        workspace_name
            .map(|workspace_name| format!(", current Workspace {workspace_name}"))
            .unwrap_or_default()
    };
    let access_context = if app_name != "Zed" && access_required {
        ", Workspace access required"
    } else {
        ""
    };
    if attention_count == 0 {
        return format!("{base_label}{workspace_context}{access_context}").into();
    }

    let attention_noun = match (app_name == "Zed", attention_count == 1) {
        (true, true) => "session",
        (true, false) => "sessions",
        (false, true) => "item",
        (false, false) => "items",
    };
    let attention_verb = if attention_count == 1 {
        "needs"
    } else {
        "need"
    };
    format!(
        "{base_label}{workspace_context}{access_context}, {attention_count} {attention_noun} {attention_verb} attention"
    )
    .into()
}

fn sidebar_toggle_visible_label(
    app_name: &str,
    workspace_name: Option<&SharedString>,
    access_required: bool,
    attention_count: usize,
    viewport_width: Pixels,
) -> Option<SharedString> {
    if app_name == "Zed" || viewport_width < WORKSPACE_STATUS_LABEL_MIN_VIEWPORT_WIDTH {
        return None;
    }

    let base_label = if viewport_width >= WORKSPACE_STATUS_NAME_MIN_VIEWPORT_WIDTH
        && let Some(workspace_name) = workspace_name
    {
        let workspace_name =
            util::truncate_and_trailoff(workspace_name, WORKSPACE_STATUS_NAME_MAX_CHARACTERS);
        format!("Workspaces · {workspace_name}")
    } else {
        "Workspaces".to_owned()
    };

    let base_label = if access_required {
        format!("{base_label} · Access required")
    } else {
        base_label
    };

    Some(if attention_count > 0 {
        format!("{base_label} · Attention {attention_count}").into()
    } else {
        base_label.into()
    })
}

fn focused_pane_status_label(
    app_name: &str,
    pane_index: usize,
    pane_count: usize,
    viewport_width: Pixels,
) -> Option<SharedString> {
    if app_name == "Zed"
        || pane_count <= 1
        || pane_index >= pane_count
        || viewport_width < WORKSPACE_STATUS_LABEL_MIN_VIEWPORT_WIDTH
    {
        return None;
    }

    if viewport_width >= WORKSPACE_STATUS_NAME_MIN_VIEWPORT_WIDTH {
        Some(format!("Pane {} of {pane_count}", pane_index + 1).into())
    } else {
        Some(format!("Pane {}/{pane_count}", pane_index + 1).into())
    }
}

fn focused_pane_accessibility_label(pane_index: usize, pane_count: usize) -> SharedString {
    format!("Focused pane {} of {pane_count}", pane_index + 1).into()
}

fn repository_status_label(
    app_name: &str,
    branch_label: Option<&str>,
    changed_file_count: usize,
    viewport_width: Pixels,
) -> Option<SharedString> {
    if app_name == "Zed" || viewport_width < REPOSITORY_STATUS_MIN_VIEWPORT_WIDTH {
        return None;
    }

    match (branch_label, changed_file_count) {
        (Some(branch), 0) => Some(branch.to_owned().into()),
        (Some(branch), 1) => Some(format!("{branch} · 1 change").into()),
        (Some(branch), count) => Some(format!("{branch} · {count} changes").into()),
        (None, 0) => Some("Detached".into()),
        (None, 1) => Some("Detached · 1 change".into()),
        (None, count) => Some(format!("Detached · {count} changes").into()),
    }
}

fn repository_status_accessibility_label(repository_status: &str) -> SharedString {
    format!("Git repository: {repository_status}").into()
}

fn toggle_workspace_sidebar(window: &mut Window, cx: &mut App) {
    if let Some(multi_workspace) = window.root::<MultiWorkspace>().flatten() {
        multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.toggle_sidebar(window, cx);
        });
    }
}

impl Focusable for StatusBar {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for StatusBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sidebar = SidebarStatus::query(&self.multi_workspace, cx);
        let interface_scale = crate::interface_scale(cx);
        let status_bar_height = status_bar_height(
            APP_NAME,
            DesignSystemSettings::get_global(cx).density,
            interface_scale,
        );
        let viewport_width = status_bar_responsive_viewport_width(
            APP_NAME,
            window.viewport_size().width,
            interface_scale,
        );

        h_flex()
            .id("status-bar")
            .track_focus(&self.focus_handle)
            .key_context("StatusBar")
            // Expose the status bar as an ARIA toolbar so assistive technology
            // announces it as a toolbar and region navigation can reach its
            // controls. The controls inside form a tab group: region navigation
            // lands on the first control (per the ARIA toolbar pattern), Tab
            // steps through them, and arrow keys move between them once focus is
            // inside.
            .role(Role::Toolbar)
            .aria_label(status_bar_label(APP_NAME))
            .tab_group()
            .on_key_down(
                cx.listener(|status_bar, event: &gpui::KeyDownEvent, window, cx| {
                    if event.keystroke.modifiers.modified() {
                        return;
                    }
                    match event.keystroke.key.as_str() {
                        "right" => {
                            status_bar.move_item_focus(true, window, cx);
                            cx.stop_propagation();
                        }
                        "left" => {
                            status_bar.move_item_focus(false, window, cx);
                            cx.stop_propagation();
                        }
                        _ => {}
                    }
                }),
            )
            .w_full()
            .justify_between()
            .when(APP_NAME == "Zed", |this| {
                this.gap(DynamicSpacing::Base08.rems(cx))
                    .p(DynamicSpacing::Base04.rems(cx))
            })
            .when_some(status_bar_height, |this, height| {
                this.h(height).gap_1().px_1().py_0()
            })
            .bg(cx.theme().colors().status_bar_background)
            .map(|el| match window.window_decorations() {
                Decorations::Server => el,
                Decorations::Client { tiling, .. } => el
                    .when(
                        !(tiling.bottom || tiling.right)
                            && !(sidebar.open && sidebar.side == SidebarSide::Right),
                        |el| el.rounded_br(CLIENT_SIDE_DECORATION_ROUNDING),
                    )
                    .when(
                        !(tiling.bottom || tiling.left)
                            && !(sidebar.open && sidebar.side == SidebarSide::Left),
                        |el| el.rounded_bl(CLIENT_SIDE_DECORATION_ROUNDING),
                    )
                    // This border is to avoid a transparent gap in the rounded corners
                    .mb(px(-1.))
                    .mt({
                        #[cfg(target_os = "linux")]
                        let needs_gap_fix = {
                            // Running on Wayland and using some scaling levels other than 100% causes a
                            // 1px gap above the status bar; adding a margin avoids this.
                            gpui::guess_compositor() == "Wayland" && window.scale_factor() != 1.0
                        };
                        #[cfg(not(target_os = "linux"))]
                        let needs_gap_fix = false;
                        if needs_gap_fix { px(-1.) } else { px(0.) }
                    })
                    .border_b(px(1.0))
                    .border_color(cx.theme().colors().status_bar_background),
            })
            .child(self.render_left_tools(&sidebar, viewport_width, cx))
            .child(self.render_right_tools(&sidebar, viewport_width, cx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_access_status_matches_only_the_owning_local_workspace() {
        let access_state = WorkspaceAccessState::AccessRequired {
            roots: vec![std::path::PathBuf::from("/workspace/paykit/private")].into(),
        };

        assert!(workspace_roots_require_access(
            true,
            [Path::new("/workspace/paykit")],
            &access_state,
        ));
        let parent_access_state = WorkspaceAccessState::AccessRequired {
            roots: vec![std::path::PathBuf::from("/workspace")].into(),
        };
        assert!(workspace_roots_require_access(
            true,
            [Path::new("/workspace/paykit")],
            &parent_access_state,
        ));
        assert!(!workspace_roots_require_access(
            true,
            [Path::new("/workspace/infra")],
            &access_state,
        ));
        assert!(!workspace_roots_require_access(
            false,
            [Path::new("/workspace/paykit")],
            &access_state,
        ));
        assert!(!workspace_roots_require_access(
            true,
            [Path::new("/workspace/paykit")],
            &WorkspaceAccessState::Available,
        ));
    }

    #[test]
    fn dez_status_bar_names_its_workspace_scope() {
        assert_eq!(status_bar_label("Dez"), "Workspace status and navigation");
        assert_eq!(status_bar_label("Zed"), "Status bar");
        assert_eq!(sidebar_toggle_label("Dez", false), "Open Workspaces");
        assert_eq!(sidebar_toggle_label("Dez", true), "Hide Workspaces");
        assert_eq!(sidebar_toggle_label("Zed", false), "Open Sessions");
        assert_eq!(sidebar_toggle_label("Zed", true), "Hide Sessions");
        assert_eq!(
            sidebar_toggle_tooltip_label("Dez", false, true, 2),
            "Open Workspaces · Access required · Attention 2"
        );
        assert_eq!(
            sidebar_toggle_tooltip_label("Dez", true, true, 0),
            "Hide Workspaces · Access required"
        );
        assert_eq!(
            sidebar_toggle_tooltip_label("Dez", false, false, 2),
            "Open Workspaces · Attention 2"
        );
        assert_eq!(
            sidebar_toggle_tooltip_label("Zed", false, true, 2),
            "Open Sessions"
        );
        let workspace_name: SharedString = "paykit".into();
        assert_eq!(
            sidebar_toggle_accessibility_label("Dez", false, Some(&workspace_name), false, 2),
            "Open Workspaces, current Workspace paykit, 2 items need attention"
        );
        assert_eq!(
            sidebar_toggle_accessibility_label("Dez", true, Some(&workspace_name), true, 0),
            "Hide Workspaces, current Workspace paykit, Workspace access required"
        );
        assert_eq!(
            sidebar_toggle_accessibility_label("Dez", true, Some(&workspace_name), false, 0),
            "Hide Workspaces, current Workspace paykit"
        );
        assert_eq!(
            sidebar_toggle_accessibility_label("Dez", false, None, false, 0),
            "Open Workspaces"
        );
        assert_eq!(
            sidebar_toggle_accessibility_label("Zed", true, Some(&workspace_name), true, 1),
            "Hide Sessions, 1 session needs attention"
        );
        assert_eq!(
            sidebar_toggle_visible_label("Dez", Some(&workspace_name), false, 0, px(1200.0)),
            Some("Workspaces · paykit".into())
        );
        assert_eq!(
            sidebar_toggle_visible_label("Dez", Some(&workspace_name), false, 2, px(1200.0)),
            Some("Workspaces · paykit · Attention 2".into())
        );
        assert_eq!(
            sidebar_toggle_visible_label("Dez", Some(&workspace_name), true, 2, px(1200.0)),
            Some("Workspaces · paykit · Access required · Attention 2".into())
        );
        let long_workspace_name: SharedString = "this-is-a-very-long-workspace-name".into();
        assert_eq!(
            sidebar_toggle_visible_label("Dez", Some(&long_workspace_name), false, 2, px(1200.0),),
            Some("Workspaces · this-is-a-very-long-work… · Attention 2".into()),
            "secondary Workspace identity should truncate before the recovery action or attention count"
        );
        assert_eq!(
            sidebar_toggle_visible_label("Dez", None, false, 0, px(1200.0)),
            Some("Workspaces".into())
        );
        assert_eq!(
            sidebar_toggle_visible_label("Dez", Some(&workspace_name), true, 2, px(600.0)),
            None,
            "compact windows should keep the native recovery action without crowding status context"
        );
        assert_eq!(
            sidebar_toggle_visible_label("Dez", Some(&workspace_name), true, 2, px(760.0)),
            Some("Workspaces · Access required · Attention 2".into()),
            "medium windows should preserve recovery and attention before Workspace metadata"
        );
        assert_eq!(
            sidebar_toggle_visible_label("Dez", Some(&workspace_name), false, 0, px(959.0)),
            Some("Workspaces".into())
        );
        assert_eq!(
            sidebar_toggle_visible_label("Dez", Some(&workspace_name), true, 0, px(960.0)),
            Some("Workspaces · paykit · Access required".into())
        );
        assert_eq!(
            sidebar_toggle_visible_label("Dez", Some(&workspace_name), false, 0, px(960.0)),
            Some("Workspaces · paykit".into())
        );
        assert_eq!(
            sidebar_toggle_visible_label("Zed", None, true, 2, px(1200.0)),
            None
        );
        assert_eq!(
            focused_pane_status_label("Dez", 0, 2, px(1200.0)),
            Some("Pane 1 of 2".into())
        );
        assert_eq!(
            focused_pane_status_label("Dez", 1, 3, px(800.0)),
            Some("Pane 2/3".into())
        );
        assert_eq!(
            focused_pane_status_label("Dez", 0, 2, px(759.0)),
            None,
            "narrow windows should preserve the Workspaces recovery icon before pane metadata"
        );
        assert_eq!(
            focused_pane_status_label("Dez", 0, 1, px(1200.0)),
            None,
            "single-pane identity is already implicit in the Main Work Area"
        );
        assert_eq!(focused_pane_status_label("Zed", 0, 2, px(1200.0)), None);
        assert_eq!(
            focused_pane_accessibility_label(1, 3),
            "Focused pane 2 of 3"
        );
        assert_eq!(
            repository_status_label("Dez", Some("main"), 2, px(1200.0)),
            Some("main · 2 changes".into())
        );
        assert_eq!(
            repository_status_label("Dez", Some("main"), 0, px(1199.0)),
            None,
            "repository context should yield to Workspace and pane identity"
        );
        assert_eq!(
            repository_status_label("Dez", None, 1, px(1200.0)),
            Some("Detached · 1 change".into())
        );
        assert_eq!(
            repository_status_label("Zed", Some("main"), 2, px(1200.0)),
            None
        );
        assert_eq!(
            repository_status_accessibility_label("main · 2 changes"),
            "Git repository: main · 2 changes"
        );
        assert_eq!(
            status_bar_height("Dez", settings::CanvasDensity::Compact, 1.0),
            Some(px(24.0))
        );
        assert_eq!(
            status_bar_height("Dez", settings::CanvasDensity::Balanced, 1.0),
            Some(px(28.0))
        );
        assert_eq!(
            status_bar_height("Dez", settings::CanvasDensity::Spacious, 1.0),
            Some(px(32.0))
        );
        assert_eq!(
            status_bar_height("Zed", settings::CanvasDensity::Balanced, 1.5),
            None
        );
        assert_eq!(
            status_bar_height("Dez", settings::CanvasDensity::Spacious, 1.5),
            Some(px(48.0))
        );
        assert_eq!(
            status_bar_responsive_viewport_width("Dez", px(1200.0), 1.5),
            px(800.0),
            "whole-interface zoom must collapse secondary status text before native actions"
        );
        assert_eq!(
            status_bar_responsive_viewport_width("Dez", px(570.0), 0.75),
            px(760.0),
            "zooming out may reveal metadata when equivalent status width is available"
        );
        assert_eq!(
            status_bar_responsive_viewport_width("Zed", px(1200.0), 1.5),
            px(1200.0),
            "official Zed keeps its inherited physical viewport breakpoints"
        );
    }
}

impl StatusBar {
    fn render_left_tools(
        &self,
        sidebar: &SidebarStatus,
        viewport_width: Pixels,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let repository_status = self.render_repository_status(viewport_width, cx);
        let focused_pane_status = self.render_focused_pane_status(viewport_width, cx);

        h_flex()
            .flex_1()
            .gap_1()
            .min_w_0()
            .overflow_x_hidden()
            .when(
                sidebar.show_toggle && sidebar.side == SidebarSide::Left,
                |this| {
                    this.child(div().flex_none().child(self.render_sidebar_toggle(
                        sidebar,
                        viewport_width,
                        cx,
                    )))
                },
            )
            .when_some(repository_status, |this, status| this.child(status))
            .when_some(focused_pane_status, |this, status| this.child(status))
            .child(
                h_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_1()
                    .overflow_x_hidden()
                    .children(self.left_items.iter().enumerate().map(|(index, item)| {
                        render_hideable_item("status-bar-left", index, item.as_ref(), cx)
                    })),
            )
    }

    fn render_repository_status(&self, viewport_width: Pixels, cx: &App) -> Option<AnyElement> {
        let workspace = self.workspace.upgrade()?;
        let project = workspace.read(cx).project().clone();
        let project = project.read(cx);
        let repository = if let Some(repository) = project.active_repository(cx) {
            repository
        } else if project.repositories(cx).len() == 1 {
            project.repositories(cx).values().next()?.clone()
        } else {
            return None;
        };
        let repository = repository.read(cx);
        let visible_label = repository_status_label(
            APP_NAME,
            repository.branch.as_ref().map(|branch| branch.name()),
            repository.status_summary().count,
            viewport_width,
        )?;
        let accessibility_label = repository_status_accessibility_label(visible_label.as_ref());

        Some(
            h_flex()
                .id("repository-status")
                .role(Role::Status)
                .aria_label(accessibility_label)
                .min_w_0()
                .max_w(px(240.0))
                .gap_0p5()
                .px_1()
                .overflow_hidden()
                .child(
                    Icon::new(IconName::GitBranch)
                        .size(IconSize::XSmall)
                        .color(Color::Muted),
                )
                .child(
                    Label::new(visible_label)
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                        .truncate(),
                )
                .into_any_element(),
        )
    }

    fn render_focused_pane_status(&self, viewport_width: Pixels, cx: &App) -> Option<AnyElement> {
        let workspace = self.workspace.upgrade()?;
        let workspace = workspace.read(cx);
        let mut pane_index = None;
        let mut pane_count = 0;
        for pane in workspace.panes() {
            if !pane.read(cx).is_visible() {
                continue;
            }
            if pane == &self.active_pane {
                pane_index = Some(pane_count);
            }
            pane_count += 1;
        }
        let pane_index = pane_index?;
        let visible_label =
            focused_pane_status_label(APP_NAME, pane_index, pane_count, viewport_width)?;
        let accessibility_label = focused_pane_accessibility_label(pane_index, pane_count);

        Some(
            h_flex()
                .id("focused-pane-status")
                .role(Role::Status)
                .aria_label(accessibility_label)
                .flex_none()
                .px_1()
                .child(
                    Label::new(visible_label)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element(),
        )
    }

    fn render_right_tools(
        &self,
        sidebar: &SidebarStatus,
        viewport_width: Pixels,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        h_flex()
            .flex_shrink_0()
            .gap_1()
            .overflow_x_hidden()
            .children(
                self.right_items
                    .iter()
                    .enumerate()
                    .rev()
                    .map(|(index, item)| {
                        render_hideable_item("status-bar-right", index, item.as_ref(), cx)
                    }),
            )
            .when(
                sidebar.show_toggle && sidebar.side == SidebarSide::Right,
                |this| {
                    this.child(div().flex_none().child(self.render_sidebar_toggle(
                        sidebar,
                        viewport_width,
                        cx,
                    )))
                },
            )
    }

    fn render_sidebar_toggle(
        &self,
        sidebar: &SidebarStatus,
        viewport_width: Pixels,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let on_right = sidebar.side == SidebarSide::Right;
        let open = sidebar.open;
        let attention_count = sidebar.attention_count;
        let access_required = APP_NAME != "Zed" && sidebar.access_required;
        let has_attention = attention_count > 0;
        let has_status = access_required || has_attention;
        let status_color = if access_required {
            Color::Warning
        } else if has_attention {
            Color::Accent
        } else {
            Color::Muted
        };
        let indicator_border = cx.theme().colors().status_bar_background;
        let tooltip_label =
            sidebar_toggle_tooltip_label(APP_NAME, open, access_required, attention_count);
        let accessibility_label = sidebar_toggle_accessibility_label(
            APP_NAME,
            open,
            sidebar.workspace_name.as_ref(),
            access_required,
            attention_count,
        );
        let visible_label = sidebar_toggle_visible_label(
            APP_NAME,
            sidebar.workspace_name.as_ref(),
            access_required,
            attention_count,
            viewport_width,
        );
        let (control_size, icon_size) =
            sidebar_header_control_metrics(APP_NAME, DesignSystemSettings::get_global(cx).density);

        let toggle = sidebar_side_context_menu("sidebar-status-toggle-menu", cx)
            .anchor(if on_right {
                Anchor::BottomRight
            } else {
                Anchor::BottomLeft
            })
            .attach(if on_right {
                Anchor::TopRight
            } else {
                Anchor::TopLeft
            })
            .trigger(move |_is_active, _window, _cx| {
                let icon = match (open, on_right) {
                    (true, true) => IconName::SidebarRightOpen,
                    (true, false) => IconName::SidebarLeftOpen,
                    (false, true) => IconName::SidebarRightClosed,
                    (false, false) => IconName::SidebarLeftClosed,
                };

                if let Some(visible_label) = visible_label {
                    let tooltip_label = tooltip_label.clone();
                    Button::new("toggle-workspace-sidebar", visible_label)
                        .truncate(true)
                        .start_icon(Icon::new(icon).size(icon_size).color(status_color))
                        .size(control_size)
                        .label_size(LabelSize::Small)
                        .when(has_status, |this| {
                            this.end_icon(
                                Icon::new(IconName::Circle)
                                    .size(IconSize::XSmall)
                                    .color(status_color),
                            )
                        })
                        .tab_index(0isize)
                        .aria_label(accessibility_label)
                        .aria_expanded(open)
                        .tooltip(move |_, cx| {
                            Tooltip::for_action(tooltip_label.clone(), &ToggleSidebar, cx)
                        })
                        .on_click(move |_, window, cx| {
                            toggle_workspace_sidebar(window, cx);
                        })
                        .into_any_element()
                } else {
                    let tooltip_label = tooltip_label.clone();
                    IconButton::new("toggle-workspace-sidebar", icon)
                        .size(control_size)
                        .icon_size(icon_size)
                        .tab_index(0isize)
                        .aria_label(accessibility_label)
                        .aria_expanded(open)
                        .when(has_status, |this| {
                            this.indicator(Indicator::dot().color(status_color))
                                .indicator_border_color(Some(indicator_border))
                        })
                        .tooltip(move |_, cx| {
                            Tooltip::for_action(tooltip_label.clone(), &ToggleSidebar, cx)
                        })
                        .on_click(move |_, window, cx| {
                            toggle_workspace_sidebar(window, cx);
                        })
                        .into_any_element()
                }
            });

        h_flex()
            .gap_0p5()
            .when(on_right, |this| {
                this.child(Divider::vertical().color(ui::DividerColor::Border))
            })
            .child(toggle)
            .when(!on_right, |this| {
                this.child(Divider::vertical().color(ui::DividerColor::Border))
            })
    }
}

fn render_hideable_item(
    side: &'static str,
    index: usize,
    item: &dyn StatusItemViewHandle,
    cx: &App,
) -> impl IntoElement {
    let view = item.to_any();
    let Some(hide) = item.hide_setting(cx) else {
        return view.into_any_element();
    };

    let menu_id: SharedString = format!("{side}-item-menu-{index}").into();
    right_click_menu(menu_id)
        .trigger(move |_is_active, _window, _cx| view)
        .menu(move |window, cx| {
            let hide = hide.clone();
            ContextMenu::build(window, cx, move |menu, _window, _cx| {
                add_hide_button_entry(menu, hide)
            })
        })
        .into_any_element()
}

/// Appends a "Hide Button" entry aligned with surrounding toggleable entries.
pub fn add_hide_button_entry(menu: ContextMenu, hide: HideStatusItem) -> ContextMenu {
    menu.toggleable_entry(
        "Hide Button",
        false,
        IconPosition::Start,
        None,
        move |_window, cx| hide.apply(cx),
    )
}

impl StatusBar {
    fn observe_workspace(
        workspace: &WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Vec<Subscription> {
        let Some(workspace) = workspace.upgrade() else {
            return Vec::new();
        };
        let git_store = workspace.read(cx).project().read(cx).git_store().clone();
        vec![
            cx.observe(&workspace, |_, _, cx| cx.notify()),
            cx.subscribe(&workspace, |_, _, event: &WorkspaceEvent, cx| {
                if matches!(
                    event,
                    WorkspaceEvent::PaneAdded(_) | WorkspaceEvent::PaneRemoved
                ) {
                    cx.notify();
                }
            }),
            cx.subscribe(&git_store, |_, _, event, cx| {
                if matches!(
                    event,
                    GitStoreEvent::ActiveRepositoryChanged(_)
                        | GitStoreEvent::RepositoryAdded
                        | GitStoreEvent::RepositoryRemoved(_)
                        | GitStoreEvent::RepositoryUpdated(
                            _,
                            RepositoryEvent::StatusesChanged
                                | RepositoryEvent::HeadChanged
                                | RepositoryEvent::BranchListChanged
                                | RepositoryEvent::GitDirectoryChanged,
                            true,
                        )
                ) {
                    cx.notify();
                }
            }),
        ]
    }

    fn observe_multi_workspace(
        multi_workspace: &WeakEntity<MultiWorkspace>,
        cx: &mut Context<Self>,
    ) -> Option<Subscription> {
        let multi_workspace = multi_workspace.upgrade()?;
        Some(cx.observe(&multi_workspace, |_, _, cx| cx.notify()))
    }

    pub fn new(
        active_pane: &Entity<Pane>,
        workspace: WeakEntity<Workspace>,
        multi_workspace: Option<WeakEntity<MultiWorkspace>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let settings_subscription = cx.observe_global::<SettingsStore>(|_, cx| cx.notify());
        let workspace_subscriptions = Self::observe_workspace(&workspace, cx);
        let observe_multi_workspace = multi_workspace
            .as_ref()
            .and_then(|multi_workspace| Self::observe_multi_workspace(multi_workspace, cx));
        let mut this = Self {
            left_items: Default::default(),
            right_items: Default::default(),
            active_pane: active_pane.clone(),
            workspace,
            multi_workspace,
            focus_handle: cx.focus_handle(),
            _observe_active_pane: cx.observe_in(active_pane, window, |this, _, window, cx| {
                this.update_active_pane_item(window, cx)
            }),
            _workspace_subscriptions: workspace_subscriptions,
            _observe_multi_workspace: observe_multi_workspace,
            _settings_subscription: settings_subscription,
        };
        this.update_active_pane_item(window, cx);
        this
    }

    pub fn set_multi_workspace(
        &mut self,
        multi_workspace: WeakEntity<MultiWorkspace>,
        cx: &mut Context<Self>,
    ) {
        self._observe_multi_workspace = Self::observe_multi_workspace(&multi_workspace, cx);
        self.multi_workspace = Some(multi_workspace);
        cx.notify();
    }

    pub fn add_left_item<T>(&mut self, item: Entity<T>, window: &mut Window, cx: &mut Context<Self>)
    where
        T: 'static + StatusItemView,
    {
        let active_pane_item = self.active_pane.read(cx).active_item();
        item.set_active_pane_item(active_pane_item.as_deref(), window, cx);

        self.left_items.push(Box::new(item));
        cx.notify();
    }

    pub fn item_of_type<T: StatusItemView>(&self) -> Option<Entity<T>> {
        self.left_items
            .iter()
            .chain(self.right_items.iter())
            .find_map(|item| item.to_any().downcast().ok())
    }

    pub fn position_of_item<T>(&self) -> Option<usize>
    where
        T: StatusItemView,
    {
        for (index, item) in self.left_items.iter().enumerate() {
            if item.item_type() == TypeId::of::<T>() {
                return Some(index);
            }
        }
        for (index, item) in self.right_items.iter().enumerate() {
            if item.item_type() == TypeId::of::<T>() {
                return Some(index + self.left_items.len());
            }
        }
        None
    }

    pub fn insert_item_after<T>(
        &mut self,
        position: usize,
        item: Entity<T>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) where
        T: 'static + StatusItemView,
    {
        let active_pane_item = self.active_pane.read(cx).active_item();
        item.set_active_pane_item(active_pane_item.as_deref(), window, cx);

        if position < self.left_items.len() {
            self.left_items.insert(position + 1, Box::new(item))
        } else {
            self.right_items
                .insert(position + 1 - self.left_items.len(), Box::new(item))
        }
        cx.notify()
    }

    pub fn remove_item_at(&mut self, position: usize, cx: &mut Context<Self>) {
        if position < self.left_items.len() {
            self.left_items.remove(position);
        } else {
            self.right_items.remove(position - self.left_items.len());
        }
        cx.notify();
    }

    pub fn add_right_item<T>(
        &mut self,
        item: Entity<T>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) where
        T: 'static + StatusItemView,
    {
        let active_pane_item = self.active_pane.read(cx).active_item();
        item.set_active_pane_item(active_pane_item.as_deref(), window, cx);

        self.right_items.push(Box::new(item));
        cx.notify();
    }

    pub fn set_active_pane(
        &mut self,
        active_pane: &Entity<Pane>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.active_pane = active_pane.clone();
        self._observe_active_pane = cx.observe_in(active_pane, window, |this, _, window, cx| {
            this.update_active_pane_item(window, cx)
        });
        self.update_active_pane_item(window, cx);
    }

    fn update_active_pane_item(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let active_pane_item = self.active_pane.read(cx).active_item();
        for item in self.left_items.iter().chain(&self.right_items) {
            item.set_active_pane_item(active_pane_item.as_deref(), window, cx);
        }
        cx.notify();
    }

    /// Moves focus between the interactive controls within the status bar in
    /// response to arrow keys. Navigation is clamped to the status bar so
    /// arrows move between items and stop at the ends (ARIA toolbar semantics);
    /// Tab is still used to leave the toolbar.
    fn move_item_focus(&mut self, forward: bool, window: &mut Window, cx: &mut Context<Self>) {
        let previous = window.focused(cx);
        if forward {
            window.focus_next(cx);
        } else {
            window.focus_prev(cx);
        }
        let landed_in_status_bar = window
            .focused(cx)
            .is_some_and(|handle| self.focus_handle.contains(&handle, window));
        if !landed_in_status_bar && let Some(previous) = previous {
            window.focus(&previous, cx);
        }
        cx.notify();
    }
}

impl<T: StatusItemView> StatusItemViewHandle for Entity<T> {
    fn to_any(&self) -> AnyView {
        self.clone().into()
    }

    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.update(cx, |this, cx| {
            this.set_active_pane_item(active_pane_item, window, cx)
        });
    }

    fn item_type(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn hide_setting(&self, cx: &App) -> Option<HideStatusItem> {
        self.read(cx).hide_setting(cx)
    }
}

impl From<&dyn StatusItemViewHandle> for AnyView {
    fn from(val: &dyn StatusItemViewHandle) -> Self {
        val.to_any()
    }
}
