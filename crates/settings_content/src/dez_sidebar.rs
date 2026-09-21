use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

#[with_fallible_options]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct DezSidebarSettingsContent {
    /// Whether the dez workspace sidebar should open when a window is
    /// created. The sidebar can always be toggled with
    /// `multi_workspace::ToggleWorkspaceSidebar`.
    ///
    /// Default: true
    pub starts_open: Option<bool>,
}
