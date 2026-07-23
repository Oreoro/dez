use crate::{
    Panel,
    dock::PanelHandle,
    item::{Item, TabContentParams},
    pane::{Pane, PaneKind},
};
use gpui::{
    App, Context, EventEmitter, FocusHandle, Focusable, IntoElement, Render, SharedString, Window,
};
use std::sync::Arc;
use ui::{Icon, IconSize, Label, LabelCommon, prelude::*};

pub const PROJECT_TOOL_PANEL_KEYS: &[&str] = &[
    "ProjectPanel",
    "GitPanel",
    "OutlinePanel",
    "DebugPanel",
    "TerminalPanel",
    "CollaborationPanel",
];

const AGENT_PANEL_KEY: &str = "agent_panel";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PanelPaneKind {
    Project,
    Agent,
}

impl PanelPaneKind {
    pub fn for_panel_key(panel_key: &str) -> Option<Self> {
        if PROJECT_TOOL_PANEL_KEYS.contains(&panel_key) {
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
            "ProjectPanel" => "Project",
            "GitPanel" => "Git",
            "OutlinePanel" => "Outline",
            "DebugPanel" => "Debug",
            "TerminalPanel" => "Terminals",
            "CollaborationPanel" => "Collab",
            AGENT_PANEL_KEY => "Agent",
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

    fn tab_content(&self, params: TabContentParams, window: &Window, cx: &App) -> gpui::AnyElement {
        h_flex()
            .min_w_0()
            .gap_1()
            .when_some(self.tab_icon(window, cx), |this, icon| {
                this.child(div().flex_none().child(icon.size(IconSize::XSmall)))
            })
            .child(
                Label::new(self.tab_content_text(params.detail.unwrap_or_default(), cx))
                    .single_line()
                    .truncate()
                    .color(params.text_color()),
            )
            .into_any_element()
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

pub fn configure_project_pane(pane: &mut Pane, cx: &mut Context<Pane>) {
    pane.set_pane_kind(PaneKind::Project, cx);
    pane.set_close_pane_if_empty(true, cx);
    pane.set_should_display_tab_bar(|_, _| true);
}

pub fn configure_agent_pane(pane: &mut Pane, cx: &mut Context<Pane>) {
    pane.set_pane_kind(PaneKind::Agent, cx);
    pane.set_close_pane_if_empty(true, cx);
    pane.set_should_display_tab_bar(|_, _| true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn developer_tool_panels_are_routed_to_the_project_surface() {
        for panel_key in [
            "ProjectPanel",
            "GitPanel",
            "OutlinePanel",
            "DebugPanel",
            "TerminalPanel",
        ] {
            assert_eq!(
                PanelPaneKind::for_panel_key(panel_key),
                Some(PanelPaneKind::Project),
                "{panel_key} must remain reachable when legacy docks are hidden"
            );
        }

        assert_eq!(
            PanelPaneKind::for_panel_key("agent_panel"),
            Some(PanelPaneKind::Agent)
        );
        assert_eq!(PanelPaneKind::for_panel_key("UnknownPanel"), None);
    }
}
