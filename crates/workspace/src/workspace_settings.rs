use std::{num::NonZeroUsize, time::Duration};

use crate::DockPosition;
use collections::HashMap;
use gpui::{App, Pixels, Subscription, px, set_pending_input_timeout};
use serde::Deserialize;
pub use settings::{
    AutosaveSetting, EncodingDisplayOptions, InactiveOpacity, PaneSplitDirectionHorizontal,
    PaneSplitDirectionVertical, RegisterSetting, RestoreOnStartupBehavior, Settings,
    SidebarDockPosition, SidebarSide,
};
use settings::{CommandAliasTarget, SettingsStore};

#[derive(RegisterSetting)]
pub struct WorkspaceSettings {
    pub active_pane_modifiers: ActivePanelModifiers,
    pub pane_split_direction_horizontal: settings::PaneSplitDirectionHorizontal,
    pub pane_split_direction_vertical: settings::PaneSplitDirectionVertical,
    pub centered_layout: settings::CenteredLayoutSettings,
    pub card_gap: f32,
    pub confirm_quit: bool,
    pub show_call_status_icon: bool,
    pub autosave: AutosaveSetting,
    pub restore_on_startup: settings::RestoreOnStartupBehavior,
    pub cli_default_open_behavior: settings::CliDefaultOpenBehavior,
    pub default_open_behavior: settings::DefaultOpenBehavior,
    pub restore_on_file_reopen: bool,
    pub drop_target_size: f32,
    pub use_system_path_prompts: bool,
    pub use_system_prompts: bool,
    pub accessible_mode: bool,
    pub command_aliases: HashMap<String, CommandAliasTarget>,
    pub max_tabs: Option<NonZeroUsize>,
    pub when_closing_with_no_tabs: settings::CloseWindowWhenNoItems,
    pub on_last_window_closed: settings::OnLastWindowClosed,
    pub text_rendering_mode: settings::TextRenderingMode,
    pub resize_all_panels_in_dock: Vec<DockPosition>,
    pub close_on_file_delete: bool,
    pub close_panel_on_toggle: bool,
    pub use_system_window_tabs: bool,
    pub zoomed_padding: bool,
    pub window_decorations: settings::WindowDecorations,
    pub focus_follows_mouse: FocusFollowsMouse,
}

#[derive(Copy, Clone, Deserialize)]
pub struct FocusFollowsMouse {
    pub enabled: bool,
    pub debounce: Duration,
}

#[derive(Clone, Debug, RegisterSetting)]
pub struct SidebarSettings {
    pub side: SidebarDockPosition,
    pub starts_open: bool,
    pub always_open: bool,
    pub show_project_pane_button: bool,
}

impl SidebarSettings {
    pub fn side(&self) -> SidebarSide {
        match self.side {
            SidebarDockPosition::Left => SidebarSide::Left,
            SidebarDockPosition::Right => SidebarSide::Right,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct ActivePanelModifiers {
    /// Size of the border surrounding the active pane.
    /// When set to 0, the active pane doesn't have any border.
    /// The border is drawn inset.
    ///
    /// Default: `0.0`
    // TODO: make this not an option, it is never None
    pub border_size: Option<f32>,
    /// Opacity of inactive panels.
    /// When set to 1.0, the inactive panes have the same opacity as the active one.
    /// If set to 0, the inactive panes content will not be visible at all.
    /// Values are clamped to the [0.0, 1.0] range.
    ///
    /// Default: `1.0`
    // TODO: make this not an option, it is never None
    pub inactive_opacity: Option<InactiveOpacity>,
}

#[derive(Clone, Debug)]
pub struct PaneGridResponsiveProfile {
    pub narrow_width: f32,
    pub portrait_ratio: f32,
    pub ultrawide_width: f32,
    pub ultrawide_ratio: f32,
}

impl PaneGridResponsiveProfile {
    pub fn narrow_width(&self) -> f32 {
        self.narrow_width.max(1.)
    }

    pub fn portrait_ratio(&self) -> f32 {
        self.portrait_ratio.max(0.1)
    }

    pub fn ultrawide_width(&self) -> f32 {
        self.ultrawide_width.max(1.)
    }

    pub fn ultrawide_ratio(&self) -> f32 {
        self.ultrawide_ratio.max(0.1)
    }
}

#[derive(Clone, Debug)]
pub struct PaneGridResponsiveProfileOverride {
    pub narrow_width: Option<f32>,
    pub portrait_ratio: Option<f32>,
    pub ultrawide_width: Option<f32>,
    pub ultrawide_ratio: Option<f32>,
}

#[derive(Clone, Debug, RegisterSetting)]
pub struct DesignSystemSettings {
    pub family: String,
    pub density: settings::CanvasDensity,
    pub radius: settings::CanvasRadius,
    pub motion: settings::CanvasMotion,
    pub contrast: settings::CanvasContrast,
    pub content_width: settings::CanvasContentWidth,
    pub icon_style: settings::CanvasIconStyle,
    pub show_labels: settings::CanvasLabelVisibility,
}

impl DesignSystemSettings {
    pub fn is_high_contrast(&self) -> bool {
        self.contrast == settings::CanvasContrast::High
    }

    pub fn is_low_contrast(&self) -> bool {
        self.contrast == settings::CanvasContrast::Low
    }

    pub fn show_contextual_labels(&self) -> bool {
        self.show_labels != settings::CanvasLabelVisibility::Hidden
    }

    pub fn content_width_pixels(&self) -> Option<Pixels> {
        Self::content_width_pixels_for(self.content_width)
    }

    pub fn content_width_pixels_for(content_width: settings::CanvasContentWidth) -> Option<Pixels> {
        match content_width {
            settings::CanvasContentWidth::Narrow => Some(px(680.)),
            settings::CanvasContentWidth::Comfortable => Some(px(800.)),
            settings::CanvasContentWidth::Wide => Some(px(1040.)),
            settings::CanvasContentWidth::Full => None,
        }
    }
}

#[derive(Clone, Debug, RegisterSetting)]
pub struct PaneGridSettings {
    pub auto_reflow: bool,
    pub layout_history: bool,
    pub show_legacy_docks: bool,
    pub focus_indicator: settings::PaneGridFocusIndicator,
    pub panel_surface: settings::CanvasPanelSurface,
    pub draggable_panel_tabs: bool,
    pub attention_ring: bool,
    pub tab_overflow: settings::TabOverflowBehavior,
    pub auto_hide_single_tab_bar: bool,
    pub responsive_narrow_width: f32,
    pub responsive_portrait_ratio: f32,
    pub responsive_ultrawide_width: f32,
    pub responsive_ultrawide_ratio: f32,
    pub responsive_recipe_overrides: HashMap<String, PaneGridResponsiveProfileOverride>,
}

impl PaneGridSettings {
    pub fn panels_as_pane_tabs(&self) -> bool {
        !self.show_legacy_docks
            && self.draggable_panel_tabs
            && matches!(self.panel_surface, settings::CanvasPanelSurface::PaneTab)
    }

    pub fn shows_active_pane_border(&self) -> bool {
        self.attention_ring
            && matches!(
                self.focus_indicator,
                settings::PaneGridFocusIndicator::Border
                    | settings::PaneGridFocusIndicator::BorderAndTitle
                    | settings::PaneGridFocusIndicator::Ring
            )
    }

    pub fn responsive_profile(&self, recipe_id: &str) -> PaneGridResponsiveProfile {
        let override_profile = self.responsive_recipe_overrides.get(recipe_id);
        PaneGridResponsiveProfile {
            narrow_width: override_profile
                .and_then(|profile| profile.narrow_width)
                .unwrap_or(self.responsive_narrow_width),
            portrait_ratio: override_profile
                .and_then(|profile| profile.portrait_ratio)
                .unwrap_or(self.responsive_portrait_ratio),
            ultrawide_width: override_profile
                .and_then(|profile| profile.ultrawide_width)
                .unwrap_or(self.responsive_ultrawide_width),
            ultrawide_ratio: override_profile
                .and_then(|profile| profile.ultrawide_ratio)
                .unwrap_or(self.responsive_ultrawide_ratio),
        }
    }
}

#[derive(Clone, Debug, RegisterSetting)]
pub struct MultiplexerSettings {
    pub prefix_mode: bool,
    pub prefix: String,
    pub prefix_timeout: Option<Duration>,
    pub layout_cycle: Vec<String>,
    pub broadcast_confirmation: settings::BroadcastConfirmation,
}

#[derive(Deserialize, RegisterSetting)]
pub struct TabBarSettings {
    pub show: bool,
    pub show_nav_history_buttons: bool,
    pub show_tab_bar_buttons: bool,
    pub show_pinned_tabs_in_separate_row: bool,
}

#[derive(Deserialize, RegisterSetting)]
pub struct ToolbarSettings {
    pub compact_mode: bool,
}

impl Settings for WorkspaceSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let workspace = &content.workspace;
        Self {
            active_pane_modifiers: ActivePanelModifiers {
                border_size: Some(
                    workspace
                        .active_pane_modifiers
                        .unwrap()
                        .border_size
                        .unwrap(),
                ),
                inactive_opacity: Some(
                    workspace
                        .active_pane_modifiers
                        .unwrap()
                        .inactive_opacity
                        .unwrap(),
                ),
            },
            pane_split_direction_horizontal: workspace.pane_split_direction_horizontal.unwrap(),
            pane_split_direction_vertical: workspace.pane_split_direction_vertical.unwrap(),
            centered_layout: workspace.centered_layout.unwrap(),
            card_gap: workspace.card_gap.unwrap(),
            confirm_quit: workspace.confirm_quit.unwrap(),
            show_call_status_icon: workspace.show_call_status_icon.unwrap(),
            autosave: workspace.autosave.unwrap(),
            restore_on_startup: workspace.restore_on_startup.unwrap(),
            cli_default_open_behavior: workspace.cli_default_open_behavior.unwrap(),
            default_open_behavior: workspace.default_open_behavior.unwrap(),
            restore_on_file_reopen: workspace.restore_on_file_reopen.unwrap(),
            drop_target_size: workspace.drop_target_size.unwrap(),
            use_system_path_prompts: workspace.use_system_path_prompts.unwrap(),
            use_system_prompts: workspace.use_system_prompts.unwrap(),
            accessible_mode: workspace.accessible_mode.unwrap(),
            command_aliases: workspace.command_aliases.clone(),
            max_tabs: workspace.max_tabs,
            when_closing_with_no_tabs: workspace.when_closing_with_no_tabs.unwrap(),
            on_last_window_closed: workspace.on_last_window_closed.unwrap(),
            text_rendering_mode: workspace.text_rendering_mode.unwrap(),
            resize_all_panels_in_dock: workspace
                .resize_all_panels_in_dock
                .clone()
                .unwrap()
                .into_iter()
                .map(Into::into)
                .collect(),
            close_on_file_delete: workspace.close_on_file_delete.unwrap(),
            close_panel_on_toggle: workspace.close_panel_on_toggle.unwrap(),
            use_system_window_tabs: workspace.use_system_window_tabs.unwrap(),
            zoomed_padding: workspace.zoomed_padding.unwrap(),
            window_decorations: workspace.window_decorations.unwrap(),
            focus_follows_mouse: FocusFollowsMouse {
                enabled: workspace
                    .focus_follows_mouse
                    .unwrap()
                    .enabled
                    .unwrap_or(false),
                debounce: Duration::from_millis(
                    workspace
                        .focus_follows_mouse
                        .unwrap()
                        .debounce_ms
                        .unwrap_or(250),
                ),
            },
        }
    }
}

impl Settings for SidebarSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let sidebar = content.sidebar.clone().unwrap();
        let session_rail_visibility = content
            .session_rail
            .as_ref()
            .and_then(|session_rail| session_rail.visibility)
            .unwrap_or_default();
        let session_rail_mode = content
            .session_rail
            .as_ref()
            .and_then(|session_rail| session_rail.mode)
            .unwrap_or_default();
        let session_rail_hidden = session_rail_visibility == settings::CanvasVisibility::Hidden
            || session_rail_mode == settings::CanvasVisibility::Hidden;
        let session_rail_always_open = !session_rail_hidden
            && (session_rail_visibility == settings::CanvasVisibility::Always
                || session_rail_mode == settings::CanvasVisibility::Always);
        let session_rail_side = content
            .session_rail
            .as_ref()
            .and_then(|session_rail| session_rail.position)
            .map(|side| match side {
                settings::CanvasSide::Left => SidebarDockPosition::Left,
                settings::CanvasSide::Right => SidebarDockPosition::Right,
            });
        Self {
            side: session_rail_side.unwrap_or_else(|| sidebar.side.unwrap()),
            starts_open: sidebar.starts_open.unwrap() || session_rail_always_open,
            always_open: session_rail_always_open,
            show_project_pane_button: sidebar.show_project_pane_button.unwrap(),
        }
    }
}

impl Settings for DesignSystemSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let design_system = content.design_system.clone().unwrap();
        Self {
            family: design_system.family.clone().unwrap(),
            density: design_system.density.unwrap(),
            radius: design_system.radius.unwrap(),
            motion: design_system.motion.unwrap(),
            contrast: design_system.contrast.unwrap(),
            content_width: design_system.content_width.unwrap(),
            icon_style: design_system.icon_style.unwrap(),
            show_labels: design_system.show_labels.unwrap(),
        }
    }
}

impl Settings for PaneGridSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let pane_grid = content.pane_grid.clone().unwrap();
        Self {
            auto_reflow: pane_grid.auto_reflow.unwrap(),
            layout_history: pane_grid.layout_history.unwrap(),
            show_legacy_docks: pane_grid.show_legacy_docks.unwrap(),
            focus_indicator: pane_grid.focus_indicator.unwrap(),
            panel_surface: pane_grid.panel_surface.unwrap(),
            draggable_panel_tabs: pane_grid.draggable_panel_tabs.unwrap(),
            attention_ring: pane_grid.attention_ring.unwrap(),
            tab_overflow: pane_grid.tab_overflow.unwrap(),
            auto_hide_single_tab_bar: pane_grid.auto_hide_single_tab_bar.unwrap(),
            responsive_narrow_width: pane_grid.responsive_narrow_width.unwrap(),
            responsive_portrait_ratio: pane_grid.responsive_portrait_ratio.unwrap(),
            responsive_ultrawide_width: pane_grid.responsive_ultrawide_width.unwrap(),
            responsive_ultrawide_ratio: pane_grid.responsive_ultrawide_ratio.unwrap(),
            responsive_recipe_overrides: pane_grid
                .responsive_recipe_overrides
                .unwrap()
                .into_iter()
                .map(|(recipe_id, profile)| {
                    (
                        recipe_id,
                        PaneGridResponsiveProfileOverride {
                            narrow_width: profile.narrow_width,
                            portrait_ratio: profile.portrait_ratio,
                            ultrawide_width: profile.ultrawide_width,
                            ultrawide_ratio: profile.ultrawide_ratio,
                        },
                    )
                })
                .collect(),
        }
    }
}

impl Settings for MultiplexerSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let multiplexer = content.multiplexer.clone().unwrap();
        let prefix_timeout_ms = multiplexer.prefix_timeout_ms.unwrap();
        Self {
            prefix_mode: multiplexer.prefix_mode.unwrap(),
            prefix: multiplexer.prefix.clone().unwrap(),
            prefix_timeout: (prefix_timeout_ms > 0)
                .then(|| Duration::from_millis(prefix_timeout_ms)),
            layout_cycle: multiplexer.layout_cycle.clone().unwrap(),
            broadcast_confirmation: multiplexer.broadcast_confirmation.unwrap(),
        }
    }
}

pub fn apply_multiplexer_settings(cx: &mut App) {
    set_pending_input_timeout(MultiplexerSettings::get_global(cx).prefix_timeout, cx);
}

/// Provides convenient access to whether "accessible mode" is enabled, mirroring
/// [`theme::ActiveTheme`] for the active theme. Import this trait to call
/// `cx.accessible_mode()`.
pub trait AccessibleMode {
    /// Returns whether accessible mode is enabled.
    fn accessible_mode(&self) -> bool;
}

impl AccessibleMode for App {
    fn accessible_mode(&self) -> bool {
        WorkspaceSettings::get_global(self).accessible_mode
    }
}

/// Observes changes to the accessible-mode setting, invoking `callback` with the
/// new value whenever it changes. Mirrors the common
/// `cx.observe_global::<SettingsStore>` pattern, but only fires when the value
/// actually changes. The returned [`Subscription`] must be retained for the
/// callback to keep firing.
pub fn observe_accessible_mode(
    cx: &mut App,
    mut callback: impl FnMut(bool, &mut App) + 'static,
) -> Subscription {
    let mut last = cx.accessible_mode();
    cx.observe_global::<SettingsStore>(move |cx| {
        let current = cx.accessible_mode();
        if current != last {
            last = current;
            callback(current, cx);
        }
    })
}

impl Settings for TabBarSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let tab_bar = content.tab_bar.clone().unwrap();
        TabBarSettings {
            show: tab_bar.show.unwrap(),
            show_nav_history_buttons: tab_bar.show_nav_history_buttons.unwrap(),
            show_tab_bar_buttons: tab_bar.show_tab_bar_buttons.unwrap(),
            show_pinned_tabs_in_separate_row: tab_bar.show_pinned_tabs_in_separate_row.unwrap(),
        }
    }
}

impl Settings for ToolbarSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let toolbar = content.editor.toolbar.as_ref().unwrap();
        ToolbarSettings {
            compact_mode: toolbar.compact_mode.unwrap(),
        }
    }
}

#[derive(Deserialize, RegisterSetting)]
pub struct StatusBarSettings {
    pub show: bool,
    pub show_active_file: bool,
    pub active_language_button: bool,
    pub cursor_position_button: bool,
    pub line_endings_button: bool,
    pub active_encoding_button: EncodingDisplayOptions,
}

impl Settings for StatusBarSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let status_bar = content.status_bar.clone().unwrap();
        StatusBarSettings {
            show: status_bar.show.unwrap(),
            show_active_file: status_bar.show_active_file.unwrap(),
            active_language_button: status_bar.active_language_button.unwrap(),
            cursor_position_button: status_bar.cursor_position_button.unwrap(),
            line_endings_button: status_bar.line_endings_button.unwrap(),
            active_encoding_button: status_bar.active_encoding_button.unwrap(),
        }
    }
}
