use settings::{RegisterSetting, Settings};

/// Settings for the dez workspace sidebar.
#[derive(Clone, Debug, RegisterSetting)]
pub struct DezSidebarSettings {
    /// Whether the sidebar opens when a window is created.
    pub starts_open: bool,
}

impl Settings for DezSidebarSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let content = content.dez_sidebar.clone().unwrap_or_default();
        Self {
            starts_open: content.starts_open.unwrap_or(true),
        }
    }
}
