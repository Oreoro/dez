use editor::{Editor, EditorSettings};
use gpui::{App, FocusHandle};
use paths::APP_NAME;
use settings::Settings as _;
use ui::{Button, ButtonCommon, Clickable, Context, Render, Tooltip, Window, prelude::*};
use workspace::{HideStatusItem, ItemHandle, StatusItemView};

pub const SEARCH_ICON: IconName = IconName::MagnifyingGlass;

fn workspace_search_label(app_name: &str) -> &'static str {
    if app_name == "Zed" {
        "Project Search"
    } else {
        "Search Workspace Files"
    }
}

fn workspace_search_visible_label(app_name: &str, has_active_editor: bool) -> &'static str {
    if app_name != "Zed" && !has_active_editor {
        "Search files"
    } else {
        ""
    }
}

pub struct SearchButton {
    pane_item_focus_handle: Option<FocusHandle>,
    has_active_editor: bool,
}

impl SearchButton {
    pub fn new() -> Self {
        Self {
            pane_item_focus_handle: None,
            has_active_editor: false,
        }
    }
}

impl Render for SearchButton {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let button = div();

        if !EditorSettings::get_global(cx).search.button {
            return button.hidden();
        }

        let focus_handle = self.pane_item_focus_handle.clone();
        let search_label = workspace_search_label(APP_NAME);
        let visible_label = workspace_search_visible_label(APP_NAME, self.has_active_editor);
        let search_button = if visible_label.is_empty() {
            IconButton::new("project-search-indicator", SEARCH_ICON)
                .icon_size(IconSize::Small)
                .tab_index(0isize)
                .aria_label(search_label)
                .tooltip(move |_window, cx| {
                    if let Some(focus_handle) = &focus_handle {
                        Tooltip::for_action_in(
                            search_label,
                            &workspace::DeploySearch::default(),
                            focus_handle,
                            cx,
                        )
                    } else {
                        Tooltip::for_action(search_label, &workspace::DeploySearch::default(), cx)
                    }
                })
                .on_click(cx.listener(|_this, _, window, cx| {
                    window.dispatch_action(Box::new(workspace::DeploySearch::default()), cx);
                }))
                .into_any_element()
        } else {
            Button::new("project-search-indicator", visible_label)
                .label_size(LabelSize::Small)
                .start_icon(Icon::new(SEARCH_ICON).size(IconSize::Small))
                .tab_index(0isize)
                .aria_label(search_label)
                .tooltip(move |_window, cx| {
                    if let Some(focus_handle) = &focus_handle {
                        Tooltip::for_action_in(
                            search_label,
                            &workspace::DeploySearch::default(),
                            focus_handle,
                            cx,
                        )
                    } else {
                        Tooltip::for_action(search_label, &workspace::DeploySearch::default(), cx)
                    }
                })
                .on_click(cx.listener(|_this, _, window, cx| {
                    window.dispatch_action(Box::new(workspace::DeploySearch::default()), cx);
                }))
                .into_any_element()
        };

        button.child(search_button)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_search_uses_product_language() {
        assert_eq!(workspace_search_label("Dez"), "Search Workspace Files");
        assert_eq!(workspace_search_label("Zed"), "Project Search");
    }

    #[test]
    fn dez_names_search_when_the_active_surface_is_not_an_editor() {
        assert_eq!(workspace_search_visible_label("Dez", false), "Search files");
        assert_eq!(workspace_search_visible_label("Dez", true), "");
        assert_eq!(workspace_search_visible_label("Zed", false), "");
    }
}

impl StatusItemView for SearchButton {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.pane_item_focus_handle = active_pane_item.map(|item| item.item_focus_handle(cx));
        self.has_active_editor = active_pane_item
            .and_then(|item| item.downcast::<Editor>())
            .is_some();
        cx.notify();
    }

    fn hide_setting(&self, _: &App) -> Option<HideStatusItem> {
        Some(HideStatusItem::new(|settings| {
            settings.editor.search.get_or_insert_default().button = Some(false);
        }))
    }
}
