use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

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
pub enum CanvasDensity {
    Compact,
    #[default]
    Balanced,
    Spacious,
}

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
pub enum CanvasMotion {
    Reduced,
    #[default]
    System,
    Full,
}

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
pub enum CanvasVisibility {
    Hidden,
    Icon,
    Compact,
    Detailed,
    Overlay,
    Always,
    #[default]
    Auto,
}

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
pub enum CanvasSide {
    #[default]
    Left,
    Right,
}

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
pub enum CanvasRadius {
    None,
    #[default]
    Subtle,
    Rounded,
}

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
pub enum CanvasContrast {
    Low,
    #[default]
    Standard,
    High,
}

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
pub enum CanvasContentWidth {
    Narrow,
    #[default]
    Comfortable,
    Wide,
    Full,
}

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
pub enum CanvasIconStyle {
    #[default]
    Outline,
    Filled,
}

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
pub enum CanvasLabelVisibility {
    Hidden,
    #[default]
    Contextual,
    Always,
}

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
pub enum WorkspaceBarHeight {
    Minimal,
    #[default]
    Compact,
    Comfortable,
}

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
pub enum SessionRailGrouping {
    #[default]
    Project,
    Repository,
    Worktree,
    RemoteHost,
    AgentProvider,
    Status,
    Manual,
}

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
pub enum SessionRailSorting {
    #[default]
    Manual,
    Attention,
    RecentActivity,
    Project,
    AgentState,
    CreationTime,
}

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
pub enum PaneGridFocusIndicator {
    Border,
    Title,
    #[default]
    BorderAndTitle,
    Ring,
}

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
pub enum CanvasPanelSurface {
    Dock,
    #[default]
    PaneTab,
}

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
pub enum TabOverflowBehavior {
    Scroll,
    #[default]
    Searchable,
    Stack,
}

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
pub enum AgentPresentation {
    Chat,
    #[default]
    Document,
    Compact,
}

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
pub enum AgentEventVerbosity {
    Summary,
    #[default]
    Normal,
    Verbose,
}

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
pub enum AgentFleetView {
    #[default]
    Lanes,
    Matrix,
    List,
}

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
pub enum BroadcastConfirmation {
    Always,
    #[default]
    Risky,
    Never,
}

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
pub enum VisibleFocusStrength {
    Standard,
    #[default]
    Strong,
}

#[with_fallible_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct DesignSystemSettingsContent {
    /// The active design system family.
    ///
    /// Default: zed_canvas
    pub family: Option<String>,
    /// Overall UI density profile.
    ///
    /// Default: balanced
    pub density: Option<CanvasDensity>,
    /// Global radius treatment for Canvas surfaces.
    ///
    /// Default: subtle
    pub radius: Option<CanvasRadius>,
    /// Motion policy for workspace transitions.
    ///
    /// Default: system
    pub motion: Option<CanvasMotion>,
    /// Contrast policy for Canvas components.
    ///
    /// Default: standard
    pub contrast: Option<CanvasContrast>,
    /// Default readable width for prose-oriented workspace items.
    ///
    /// Default: comfortable
    pub content_width: Option<CanvasContentWidth>,
    /// Preferred icon treatment.
    ///
    /// Default: outline
    pub icon_style: Option<CanvasIconStyle>,
    /// When to show labels for icon-forward controls.
    ///
    /// Default: contextual
    pub show_labels: Option<CanvasLabelVisibility>,
}

#[with_fallible_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct WorkspaceBarSettingsContent {
    /// Workspace bar visibility.
    ///
    /// Default: always
    pub visibility: Option<CanvasVisibility>,
    /// Workspace bar height profile.
    ///
    /// Default: compact
    pub height: Option<WorkspaceBarHeight>,
    /// Whether the center area should expose command search.
    ///
    /// Default: true
    pub center_command_search: Option<bool>,
    /// Whether to show the active layout.
    ///
    /// Default: true
    pub show_layout: Option<bool>,
    /// Whether to surface agent attention in the workspace bar.
    ///
    /// Default: true
    pub show_agent_attention: Option<bool>,
}

#[with_fallible_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct SessionRailSettingsContent {
    /// Session rail visibility.
    ///
    /// Default: auto
    pub visibility: Option<CanvasVisibility>,
    /// Session rail display mode.
    ///
    /// Default: compact
    pub mode: Option<CanvasVisibility>,
    /// Which side hosts the rail.
    ///
    /// Default: left
    pub position: Option<CanvasSide>,
    /// Primary grouping mode.
    ///
    /// Default: project
    pub group_by: Option<SessionRailGrouping>,
    /// Primary sorting mode.
    ///
    /// Default: manual
    pub sort_by: Option<SessionRailSorting>,
    /// Metadata fields to show on rail entries.
    ///
    /// Default: ["branch", "worktree", "agent_state", "layout", "latest_attention"]
    pub metadata: Option<Vec<String>>,
}

#[with_fallible_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct PaneGridSettingsContent {
    /// Whether pane layouts can reflow responsively.
    ///
    /// Default: true
    pub auto_reflow: Option<bool>,
    /// Whether structural layout changes are tracked for undo and restore.
    ///
    /// Default: true
    pub layout_history: Option<bool>,
    /// How to indicate the active pane.
    ///
    /// Default: border_and_title
    pub focus_indicator: Option<PaneGridFocusIndicator>,
    /// Where traditional panel surfaces are hosted by default.
    ///
    /// Default: pane_tab
    pub panel_surface: Option<CanvasPanelSurface>,
    /// Whether legacy left, right, or bottom dock chrome is visible by default.
    ///
    /// Default: false
    pub show_legacy_docks: Option<bool>,
    /// Whether project, agent, Git, outline, and collaboration panel tabs can be
    /// dragged and arranged like regular pane tabs.
    ///
    /// Default: true
    pub draggable_panel_tabs: Option<bool>,
    /// Whether pane attention rings are enabled.
    ///
    /// Default: true
    pub attention_ring: Option<bool>,
    /// How tab bars handle overflow.
    ///
    /// Default: searchable
    pub tab_overflow: Option<TabOverflowBehavior>,
    /// Whether to hide the tab bar when a pane has one tab.
    ///
    /// Default: false
    pub auto_hide_single_tab_bar: Option<bool>,
}

#[with_fallible_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct AgentUiSettingsContent {
    /// How agent conversations are presented.
    ///
    /// Default: document
    pub presentation: Option<AgentPresentation>,
    /// Default verbosity for agent event timelines.
    ///
    /// Default: normal
    pub event_verbosity: Option<AgentEventVerbosity>,
    /// Whether repeated tool calls are grouped.
    ///
    /// Default: true
    pub group_tool_calls: Option<bool>,
    /// Whether failures remain expanded by default.
    ///
    /// Default: true
    pub keep_failures_expanded: Option<bool>,
    /// Whether permission requests remain expanded by default.
    ///
    /// Default: true
    pub keep_permissions_expanded: Option<bool>,
    /// Whether terminal-agent detection confidence is shown.
    ///
    /// Default: true
    pub show_detection_confidence: Option<bool>,
    /// Default presentation for many-agent views.
    ///
    /// Default: lanes
    pub fleet_view: Option<AgentFleetView>,
    /// Whether multiple agent surfaces can be visible in one workspace.
    ///
    /// Default: true
    pub allow_multiple_visible_agents: Option<bool>,
    /// Whether terminals should be inspected for known agent processes.
    ///
    /// Default: true
    pub detect_terminal_agents: Option<bool>,
    /// Whether detected terminal agents should surface in the Session Rail.
    ///
    /// Default: true
    pub show_terminal_agents_in_session_rail: Option<bool>,
    /// Whether agent session metadata should be restored on restart.
    ///
    /// Default: true
    pub resume_sessions_on_restart: Option<bool>,
    /// Whether provider and terminal lifecycle hooks are connected when available.
    ///
    /// Default: true
    pub connect_hooks: Option<bool>,
    /// Whether agent attention changes produce notifications.
    ///
    /// Default: true
    pub notify_on_attention: Option<bool>,
}

#[with_fallible_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct MultiplexerSettingsContent {
    /// Whether optional tmux-style prefix mode is enabled.
    ///
    /// Default: false
    pub prefix_mode: Option<bool>,
    /// The prefix key sequence shown in Canvas command hints.
    ///
    /// Default: ctrl-b
    pub prefix: Option<String>,
    /// How long to wait before replaying an incomplete prefix sequence.
    /// Set to 0 to disable prefix timeout replay.
    ///
    /// Default: 1000
    pub prefix_timeout_ms: Option<u64>,
    /// Layout names included when cycling layouts.
    ///
    /// Default: ["even_columns", "even_rows", "main_left", "main_top", "tiled", "agent_control"]
    pub layout_cycle: Option<Vec<String>>,
    /// When broadcast groups require confirmation.
    ///
    /// Default: risky
    pub broadcast_confirmation: Option<BroadcastConfirmation>,
}

#[with_fallible_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct AccessibilitySettingsContent {
    /// Strength of the active focus treatment.
    ///
    /// Default: strong
    pub visible_focus: Option<VisibleFocusStrength>,
    /// Reduced motion policy.
    ///
    /// Default: system
    pub reduced_motion: Option<CanvasMotion>,
    /// Whether agent streaming text should be announced continuously.
    ///
    /// Default: false
    pub announce_agent_streaming: Option<bool>,
    /// Whether agent attention changes should be announced.
    ///
    /// Default: true
    pub announce_agent_attention: Option<bool>,
    /// Minimum target size policy.
    ///
    /// Default: system
    pub minimum_target_size: Option<String>,
}
