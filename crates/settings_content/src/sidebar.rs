use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

/// Where to position the sidebar.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum SidebarDockPosition {
    /// Always show the sidebar on the left side.
    #[default]
    Left,
    /// Always show the sidebar on the right side.
    Right,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum SidebarSide {
    #[default]
    Left,
    Right,
}

#[with_fallible_options]
#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, Default)]
pub struct SidebarSettingsContent {
    /// Where to position the sidebar.
    ///
    /// Default: left
    pub side: Option<SidebarDockPosition>,
    /// Whether the sidebar starts open in new windows.
    ///
    /// Default: true
    pub starts_open: Option<bool>,
}

impl SidebarSettingsContent {
    pub fn set_side(&mut self, position: SidebarDockPosition) {
        self.side = Some(position);
    }
}
