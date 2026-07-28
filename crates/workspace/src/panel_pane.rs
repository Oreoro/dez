use crate::{
    Panel,
    dock::PanelHandle,
    item::Item,
    pane::{Pane, PaneKind},
};
use gpui::{
    App, Context, EventEmitter, FocusHandle, Focusable, IntoElement, Render, SharedString, Window,
};
use std::sync::Arc;
use ui::Icon;

pub const PROJECT_TOOL_PANEL_KEYS: &[&str] =
    &["ProjectPanel", "GitPanel", "OutlinePanel", "DebugPanel"];

const AGENT_PANEL_KEY: &str = "agent_panel";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PanelPaneKind {
    Project,
    Agent,
}

impl PanelPaneKind {
    pub fn for_panel_key(panel_key: &str) -> Option<Self> {
        Self::for_panel_key_and_app(panel_key, paths::APP_NAME)
    }

    fn for_panel_key_and_app(panel_key: &str, app_name: &str) -> Option<Self> {
        if matches!(panel_key, "TerminalPanel" | "CollaborationPanel") {
            (app_name == "Zed").then_some(Self::Project)
        } else if PROJECT_TOOL_PANEL_KEYS.contains(&panel_key) {
            Some(Self::Project)
        } else if panel_key == AGENT_PANEL_KEY {
            Some(Self::Agent)
        } else {
            None
        }
    }

    pub fn pane_kind(self) -> PaneKind {
        match self {
            Self::Project => PaneKind::Project,
            Self::Agent => PaneKind::Agent,
        }
    }
}

pub struct PanelItem {
    panel: Arc<dyn PanelHandle>,
}

impl PanelItem {
    pub fn new(panel: Arc<dyn PanelHandle>) -> Self {
        Self { panel }
    }

    pub fn panel(&self) -> Arc<dyn PanelHandle> {
        self.panel.clone()
    }

    pub fn panel_id(&self) -> gpui::EntityId {
        self.panel.panel_id()
    }

    pub fn panel_key(&self) -> &'static str {
        self.panel.panel_key()
    }

    pub fn is_panel<T: Panel>(&self) -> bool {
        self.panel.to_any().downcast::<T>().is_ok()
    }

    fn tab_label(&self) -> &'static str {
        match self.panel.panel_key() {
            "ProjectPanel" if paths::APP_NAME == "Zed" => "Project",
            "ProjectPanel" => "Files",
            "GitPanel" => "Git",
            "OutlinePanel" => "Outline",
            "DebugPanel" => "Debug",
            "TerminalPanel" => "Terminals",
            "CollaborationPanel" => "Collab",
            AGENT_PANEL_KEY if paths::APP_NAME == "Zed" => "Agent",
            AGENT_PANEL_KEY => "Built-in Agent",
            _ => self.panel.persistent_name(),
        }
    }
}

impl Focusable for PanelItem {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.panel.panel_focus_handle(cx)
    }
}

impl EventEmitter<()> for PanelItem {}

impl Render for PanelItem {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.panel.to_any()
    }
}

impl Item for PanelItem {
    type Event = ();

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        self.tab_label().into()
    }

    fn tab_icon(&self, window: &Window, cx: &App) -> Option<Icon> {
        self.panel.icon(window, cx).map(Icon::new)
    }

    fn activated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.panel.set_active(true, window, cx);
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.panel.set_active(false, window, cx);
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn include_in_nav_history() -> bool {
        false
    }
}

fn agent_pane_uses_outer_tab_bar(app_name: &str) -> bool {
    app_name == "Zed"
}

pub fn configure_project_pane(pane: &mut Pane, cx: &mut Context<Pane>) {
    pane.set_pane_kind(PaneKind::Project, cx);
    pane.set_close_pane_if_empty(true, cx);
    pane.set_should_display_tab_bar(|_, _| true);
}

pub fn configure_agent_pane(pane: &mut Pane, cx: &mut Context<Pane>) {
    pane.set_pane_kind(PaneKind::Agent, cx);
    pane.set_close_pane_if_empty(true, cx);
    pane.set_should_display_tab_bar(|_, _| agent_pane_uses_outer_tab_bar(paths::APP_NAME));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn developer_tool_panels_are_routed_to_the_project_surface() {
        for panel_key in ["ProjectPanel", "GitPanel", "OutlinePanel", "DebugPanel"] {
            assert_eq!(
                PanelPaneKind::for_panel_key(panel_key),
                Some(PanelPaneKind::Project),
                "{panel_key} must remain reachable when legacy docks are hidden"
            );
        }

        assert_eq!(
            PanelPaneKind::for_panel_key_and_app("TerminalPanel", "Dez"),
            None,
            "Dez terminals belong in the main work area, not a second tool surface"
        );
        assert_eq!(
            PanelPaneKind::for_panel_key_and_app("TerminalPanel", "Zed"),
            Some(PanelPaneKind::Project),
            "official Zed keeps its inherited Terminal Panel behavior"
        );
        assert_eq!(
            PanelPaneKind::for_panel_key_and_app("CollaborationPanel", "Dez"),
            None,
            "Dez Workspace Tools must not regain the removed Collaboration surface"
        );
        assert_eq!(
            PanelPaneKind::for_panel_key_and_app("CollaborationPanel", "Zed"),
            Some(PanelPaneKind::Project),
            "official Zed keeps its inherited Collaboration Panel behavior"
        );
        assert_eq!(
            PanelPaneKind::for_panel_key("agent_panel"),
            Some(PanelPaneKind::Agent)
        );
        assert_eq!(PanelPaneKind::for_panel_key("UnknownPanel"), None);
    }

    #[test]
    fn dez_agent_uses_its_native_toolbar_instead_of_a_second_tab_bar() {
        assert!(!agent_pane_uses_outer_tab_bar("Dez"));
        assert!(agent_pane_uses_outer_tab_bar("Zed"));
    }
}
