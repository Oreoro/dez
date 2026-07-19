use gpui::{Pixels, px};
use settings::{RegisterSetting, Settings};

/// The settings for the markdown preview.
#[derive(Clone, Copy, Debug, Default, RegisterSetting)]
pub struct MarkdownPreviewSettings {
    /// How Markdown files should open by default.
    pub default_open_mode: settings::MarkdownPreviewOpenMode,
    /// The maximum width of the rendered markdown content, or `None` to render
    /// content edge to edge.
    pub max_width: Option<Pixels>,
    /// Whether preview-first surfaces should show a source-edit affordance.
    pub show_edit_source_action: bool,
}

impl Settings for MarkdownPreviewSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let content = content.markdown_preview.clone().unwrap_or_default();
        let default_open_mode = content.default_open_mode.unwrap_or_default();
        let max_width = if content.limit_content_width.unwrap_or(true) {
            content.max_width.map(px)
        } else {
            None
        };
        let show_edit_source_action = content.show_edit_source_action.unwrap_or(true);
        Self {
            default_open_mode,
            max_width,
            show_edit_source_action,
        }
    }
}
