use crate::{
    DesignSystemSettings, ItemHandle, MultiWorkspace, Pane, SidebarSide, ToggleSidebar,
    sidebar_header_control_metrics, sidebar_side_context_menu,
};
use gpui::{
    Anchor, AnyView, App, Context, Decorations, Entity, FocusHandle, Focusable, IntoElement,
    ParentElement, Pixels, Render, Role, SharedString, Styled, Subscription, WeakEntity, Window,
};
use paths::APP_NAME;
use settings::{Settings as _, SettingsContent, update_settings_file};
use std::{any::TypeId, sync::Arc};
use theme::CLIENT_SIDE_DECORATION_ROUNDING;
use ui::{ContextMenu, Divider, IconPosition, Indicator, Tooltip, prelude::*, right_click_menu};

const WORKSPACE_STATUS_LABEL_MIN_VIEWPORT_WIDTH: Pixels = px(760.0);
const WORKSPACE_STATUS_NAME_MIN_VIEWPORT_WIDTH: Pixels = px(960.0);

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
    has_notifications: bool,
    show_toggle: bool,
    workspace_name: Option<SharedString>,
}

impl SidebarStatus {
    fn query(multi_workspace: &Option<WeakEntity<MultiWorkspace>>, cx: &App) -> Self {
        multi_workspace
            .as_ref()
            .and_then(|mw| mw.upgrade())
            .map(|mw| {
                let mw = mw.read(cx);
                let enabled = mw.multi_workspace_enabled(cx);
                let workspace_name = mw
                    .workspace()
                    .read(cx)
                    .project_group_key(cx)
                    .path_list()
                    .ordered_paths()
                    .next()
                    .and_then(|path| path.file_name())
                    .map(|name| SharedString::from(name.to_string_lossy().into_owned()));
                Self {
                    open: mw.sidebar_open() && enabled,
                    side: mw.sidebar_side(cx),
                    has_notifications: mw.sidebar_has_notifications(cx),
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
    multi_workspace: Option<WeakEntity<MultiWorkspace>>,
    focus_handle: FocusHandle,
    _observe_active_pane: Subscription,
}

fn status_bar_label(app_name: &str) -> &'static str {
    if app_name == "Zed" {
        "Status bar"
    } else {
        "Workspace status and navigation"
    }
}

fn status_bar_height(app_name: &str, density: settings::CanvasDensity) -> Option<Pixels> {
    (app_name != "Zed").then(|| {
        px(match density {
            settings::CanvasDensity::Compact => 24.0,
            settings::CanvasDensity::Balanced => 26.0,
            settings::CanvasDensity::Spacious => 30.0,
        })
    })
}

fn sidebar_toggle_label(app_name: &str, open: bool) -> &'static str {
    match (app_name == "Zed", open) {
        (true, true) => "Hide Sessions",
        (true, false) => "Open Sessions",
        (false, true) => "Hide Workspaces",
        (false, false) => "Open Workspaces",
    }
}

fn sidebar_toggle_accessibility_label(
    app_name: &str,
    open: bool,
    has_notifications: bool,
) -> &'static str {
    match (app_name == "Zed", open, has_notifications) {
        (true, true, true) => "Hide Sessions, attention needed",
        (true, true, false) => "Hide Sessions",
        (true, false, true) => "Open Sessions, attention needed",
        (true, false, false) => "Open Sessions",
        (false, true, true) => "Hide Workspaces, attention needed",
        (false, true, false) => "Hide Workspaces",
        (false, false, true) => "Open Workspaces, attention needed",
        (false, false, false) => "Open Workspaces",
    }
}

fn sidebar_toggle_visible_label(
    app_name: &str,
    workspace_name: Option<&SharedString>,
    viewport_width: Pixels,
) -> Option<SharedString> {
    if app_name == "Zed" || viewport_width < WORKSPACE_STATUS_LABEL_MIN_VIEWPORT_WIDTH {
        return None;
    }

    if viewport_width >= WORKSPACE_STATUS_NAME_MIN_VIEWPORT_WIDTH
        && let Some(workspace_name) = workspace_name
    {
        Some(format!("Workspaces · {workspace_name}").into())
    } else {
        Some("Workspaces".into())
    }
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
        let status_bar_height =
            status_bar_height(APP_NAME, DesignSystemSettings::get_global(cx).density);
        let viewport_width = window.viewport_size().width;

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
    fn dez_status_bar_names_its_workspace_scope() {
        assert_eq!(status_bar_label("Dez"), "Workspace status and navigation");
        assert_eq!(status_bar_label("Zed"), "Status bar");
        assert_eq!(sidebar_toggle_label("Dez", false), "Open Workspaces");
        assert_eq!(sidebar_toggle_label("Dez", true), "Hide Workspaces");
        assert_eq!(sidebar_toggle_label("Zed", false), "Open Sessions");
        assert_eq!(sidebar_toggle_label("Zed", true), "Hide Sessions");
        assert_eq!(
            sidebar_toggle_accessibility_label("Dez", false, true),
            "Open Workspaces, attention needed"
        );
        assert_eq!(
            sidebar_toggle_accessibility_label("Dez", true, false),
            "Hide Workspaces"
        );
        let workspace_name: SharedString = "paykit".into();
        assert_eq!(
            sidebar_toggle_visible_label("Dez", Some(&workspace_name), px(1200.0)),
            Some("Workspaces · paykit".into())
        );
        assert_eq!(
            sidebar_toggle_visible_label("Dez", None, px(1200.0)),
            Some("Workspaces".into())
        );
        assert_eq!(
            sidebar_toggle_visible_label("Dez", Some(&workspace_name), px(600.0)),
            None,
            "compact windows should keep the native recovery action without crowding status context"
        );
        assert_eq!(
            sidebar_toggle_visible_label("Dez", Some(&workspace_name), px(760.0)),
            Some("Workspaces".into()),
            "medium windows should preserve navigation identity before Workspace metadata"
        );
        assert_eq!(
            sidebar_toggle_visible_label("Dez", Some(&workspace_name), px(959.0)),
            Some("Workspaces".into())
        );
        assert_eq!(
            sidebar_toggle_visible_label("Dez", Some(&workspace_name), px(960.0)),
            Some("Workspaces · paykit".into())
        );
        assert_eq!(sidebar_toggle_visible_label("Zed", None, px(1200.0)), None);
        assert_eq!(
            status_bar_height("Dez", settings::CanvasDensity::Compact),
            Some(px(24.0))
        );
        assert_eq!(
            status_bar_height("Dez", settings::CanvasDensity::Balanced),
            Some(px(26.0))
        );
        assert_eq!(
            status_bar_height("Dez", settings::CanvasDensity::Spacious),
            Some(px(30.0))
        );
        assert_eq!(
            status_bar_height("Zed", settings::CanvasDensity::Balanced),
            None
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
        let has_notifications = sidebar.has_notifications;
        let indicator_border = cx.theme().colors().status_bar_background;
        let toggle_label = sidebar_toggle_label(APP_NAME, open);
        let accessibility_label =
            sidebar_toggle_accessibility_label(APP_NAME, open, has_notifications);
        let visible_label =
            sidebar_toggle_visible_label(APP_NAME, sidebar.workspace_name.as_ref(), viewport_width);
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
                    Button::new("toggle-workspace-sidebar", visible_label)
                        .start_icon(Icon::new(icon).size(icon_size).color(if has_notifications {
                            Color::Accent
                        } else {
                            Color::Muted
                        }))
                        .size(control_size)
                        .label_size(LabelSize::Small)
                        .when(has_notifications, |this| {
                            this.end_icon(
                                Icon::new(IconName::Circle)
                                    .size(IconSize::XSmall)
                                    .color(Color::Accent),
                            )
                        })
                        .tab_index(0isize)
                        .aria_label(accessibility_label)
                        .aria_expanded(open)
                        .tooltip(move |_, cx| Tooltip::for_action(toggle_label, &ToggleSidebar, cx))
                        .on_click(move |_, window, cx| {
                            toggle_workspace_sidebar(window, cx);
                        })
                        .into_any_element()
                } else {
                    IconButton::new("toggle-workspace-sidebar", icon)
                        .size(control_size)
                        .icon_size(icon_size)
                        .tab_index(0isize)
                        .aria_label(accessibility_label)
                        .aria_expanded(open)
                        .when(has_notifications, |this| {
                            this.indicator(Indicator::dot().color(Color::Accent))
                                .indicator_border_color(Some(indicator_border))
                        })
                        .tooltip(move |_, cx| Tooltip::for_action(toggle_label, &ToggleSidebar, cx))
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
    pub fn new(
        active_pane: &Entity<Pane>,
        multi_workspace: Option<WeakEntity<MultiWorkspace>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self {
            left_items: Default::default(),
            right_items: Default::default(),
            active_pane: active_pane.clone(),
            multi_workspace,
            focus_handle: cx.focus_handle(),
            _observe_active_pane: cx.observe_in(active_pane, window, |this, _, window, cx| {
                this.update_active_pane_item(window, cx)
            }),
        };
        this.update_active_pane_item(window, cx);
        this
    }

    pub fn set_multi_workspace(
        &mut self,
        multi_workspace: WeakEntity<MultiWorkspace>,
        cx: &mut Context<Self>,
    ) {
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
