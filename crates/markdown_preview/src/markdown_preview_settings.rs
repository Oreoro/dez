use gpui::{Pixels, px};
use settings::{RegisterSetting, Settings};
use workspace::DesignSystemSettings;

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
        let markdown_preview = content.markdown_preview.clone().unwrap_or_default();
        let default_open_mode = markdown_preview.default_open_mode.unwrap_or_default();
        let max_width = if markdown_preview.limit_content_width.unwrap_or(true) {
            markdown_preview
                .max_width
                .map(px)
                .or_else(|| canvas_content_width(content))
        } else {
            None
        };
        let show_edit_source_action = markdown_preview.show_edit_source_action.unwrap_or(true);
        Self {
            default_open_mode,
            max_width,
            show_edit_source_action,
        }
    }
}

fn canvas_content_width(content: &settings::SettingsContent) -> Option<Pixels> {
    let content_width = content
        .design_system
        .as_ref()
        .and_then(|design_system| design_system.content_width)
        .unwrap_or_default();

    DesignSystemSettings::content_width_pixels_for(content_width)
}
