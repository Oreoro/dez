mod thread_switcher;

use acp_thread::ThreadStatus;
use action_log::DiffStats;
use agent::{ThreadStore, ZED_AGENT_ID};
use agent_client_protocol::schema::v1 as acp;
use agent_settings::{AgentSettings, UserAgentsMd};
use agent_ui::terminal_thread_metadata_store::{
    TerminalAgentKind, TerminalAttentionCondition, TerminalAttentionPresentation,
    TerminalAttentionPriority, TerminalAttentionState, TerminalThreadMetadata,
    TerminalThreadMetadataStore, detect_terminal_agent_command, terminal_title_prefix,
};
use agent_ui::thread_metadata_store::{
    ThreadMetadata, ThreadMetadataStore, WorktreePaths, worktree_info_from_thread_paths,
};
use agent_ui::threads_archive_view::{
    ThreadsArchiveView, ThreadsArchiveViewEvent, format_history_entry_timestamp,
    fuzzy_match_positions,
};
use agent_ui::{
    AcpThreadImportOnboarding, Agent, AgentPanel, AgentPanelEvent, AgentThreadItem,
    AgentThreadSource, ArchiveSelectedThread, CanvasAgentUiSettings, ConversationView,
    CrossChannelImportOnboarding, ManageProfiles, NewTerminalThread, ObservedRepositoryEvidence,
    ObservedRunActivity, ObservedRunCheck, ObservedRunCheckStatus, ObservedRunCommand,
    ObservedWorkspaceEvidence, OpenAgentDiff, RenameSelectedThread, RunReviewBrief, RunReviewState,
    TerminalId, ThreadId, ThreadImportModal, ThreadTitleRegenerationResult, ToggleOptionsMenu,
    WorkspaceEvidenceKind, channels_with_threads, connection_store_for_project,
    create_agent_thread_in_workspace, default_agent_session_title,
    import_threads_from_other_channels, open_agent_thread_in_workspace,
};
use agent_ui::{MessageEditorEvent, StateChange, thread_worktree_archive};
use chrono::{DateTime, TimeZone as _, Utc};
use editor::Editor;
use feature_flags::{
    AgentThreadWorktreeLabel, AgentThreadWorktreeLabelFlag, FeatureFlag, FeatureFlagAppExt as _,
};
use gpui::{
    Action as _, AnyElement, App, ClickEvent, ClipboardItem, Context, Decorations, DismissEvent,
    Entity, EntityId, FocusHandle, Focusable, KeyContext, ListState, Modifiers, Pixels,
    PromptLevel, Render, SharedString, Task, TaskExt, WeakEntity, Window,
    WindowBackgroundAppearance, WindowHandle, linear_color_stop, linear_gradient, list, prelude::*,
    px,
};
use itertools::Itertools;
use language_model::LanguageModelRegistry;
use menu::{
    Cancel, Confirm, SelectChild, SelectFirst, SelectLast, SelectNext, SelectParent, SelectPrevious,
};
use notifications::status_toast::StatusToast;
use paths::APP_NAME;
use project::{AgentId, AgentRegistryStore, Event as ProjectEvent, WorktreeId};
use recent_projects::sidebar_recent_projects::SidebarRecentProjects;
use remote::{RemoteConnectionOptions, same_remote_connection_identity};
use serde::{Deserialize, Serialize};
use session::{AppSession, AppSessionEvent, DurableWorkspaceResolution};
use settings::Settings as _;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::mem;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use terminal::session_host::{
    LocalTerminalHost, TerminalAgentEventKind, TerminalAgentSnapshot, TerminalAgentState,
    TerminalSessionCommand, TerminalSessionId, TerminalSessionSnapshot, TerminalSessionState,
    transport::{
        TerminalHostConnection, TerminalHostSnapshotRevision, TerminalHostSnapshotStore,
        TerminalHostStartupState, TerminalHostStartupStatus,
    },
};
use terminal_view::TerminalView;
use theme::{ActiveTheme, CLIENT_SIDE_DECORATION_ROUNDING};
use ui::{
    AgentThreadStatus, Callout, CommonAnimationExt, ContextMenu, ContextMenuEntry, GradientFade,
    HighlightedLabel, PopoverMenu, PopoverMenuHandle, ScrollAxes, Scrollbars, Severity, Tab,
    ThreadItem, ThreadItemContrast, ThreadItemDensity, ThreadItemEvidenceStatus, ThreadItemRadius,
    ThreadItemWorktreeInfo, TintColor, Tooltip, WithScrollbar, prelude::*, render_modifiers,
    right_click_menu,
};
use unicode_segmentation::UnicodeSegmentation as _;
use util::ResultExt as _;
use util::path_list::PathList;
use workspace::{
    CloseWindow, DesignSystemSettings, MultiWorkspace, MultiWorkspaceEvent, NewCenterTerminal,
    NextProject, NextThread, Open, OpenMode, PreviousProject, PreviousThread, ProjectGroupKey,
    SaveIntent, Sidebar as WorkspaceSidebar, SidebarRenderState, SidebarSettings, SidebarSide,
    Toast, ToggleSidebar, Workspace,
    evidence::{
        WorkspaceEvidenceKind as AuthoritativeWorkspaceEvidenceKind, WorkspaceEvidenceLifecycle,
        WorkspaceEvidenceProvenance,
    },
    notifications::NotificationId,
    render_sidebar_header_controls_with_state,
};

use git_ui::worktree_service::{RemoteBranchName, worktree_create_targets};
use zed_actions::agent::OpenSettings;
use zed_actions::assistant::{ManageSkills, OpenGlobalAgentsMdRules, OpenProjectAgentsMdRules};
use zed_actions::editor::{MoveDown, MoveUp};
use zed_actions::{CreateWorktree, NewWorktreeBranchTarget, OpenRecent};

use zed_actions::sidebar::{FocusSidebarFilter, ToggleThreadSwitcher};

use crate::thread_switcher::{
    ThreadSwitcher, ThreadSwitcherEntry, ThreadSwitcherEvent, ThreadSwitcherSelection,
    ThreadSwitcherTerminalEntry, ThreadSwitcherThreadEntry,
};

#[cfg(test)]
mod sidebar_tests;

gpui::actions!(
    sidebar,
    [
        /// Creates a new agent thread in the currently selected or active Workspace.
        NewThreadInGroup,
        /// Creates a new terminal session in the currently selected or active Workspace.
        NewSessionInGroup,
        /// Moves the selected session rail row up within its project group.
        MoveSelectedEntryUp,
        /// Moves the selected session rail row down within its project group.
        MoveSelectedEntryDown,
        /// Toggles between the thread list and the thread history.
        ToggleThreadHistory,
        /// Shows only sessions and agents that currently need attention.
        ToggleAttentionFilter,
        /// Opens a deterministic evidence-backed review brief for the selected Run.
        OpenSelectedReviewBrief,
    ]
);

gpui::actions!(
    dev,
    [
        /// Dumps multi-workspace state (projects, worktrees, active threads) into a new buffer.
        DumpWorkspaceInfo,
    ]
);

const DEFAULT_WIDTH: Pixels = px(300.0);
const COMPACT_MAX_WIDTH: Pixels = px(280.0);
const DETAILED_MIN_WIDTH: Pixels = px(380.0);
const SUPPLEMENTAL_METADATA_MIN_WIDTH: Pixels = px(440.0);
const MIN_WIDTH: Pixels = px(240.0);
const MAX_WIDTH: Pixels = px(800.0);

#[derive(Clone, Debug, settings::RegisterSetting)]
struct SessionRailSettings {
    visibility: settings::CanvasVisibility,
    mode: settings::CanvasVisibility,
    show_worktree_metadata: bool,
    show_agent_state_metadata: bool,
    show_layout_metadata: bool,
    show_latest_attention_metadata: bool,
    sort_by: settings::SessionRailSorting,
}

#[derive(Clone, Debug, settings::RegisterSetting)]
struct WorkspaceBarAttentionSettings {
    show_agent_attention: bool,
}

impl settings::Settings for SessionRailSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let session_rail = content.session_rail.clone().unwrap();
        let metadata = session_rail.metadata.clone().unwrap();
        Self {
            visibility: session_rail.visibility.unwrap(),
            mode: session_rail.mode.unwrap(),
            show_worktree_metadata: session_rail_metadata_contains(&metadata, "worktree")
                || session_rail_metadata_contains(&metadata, "branch"),
            show_agent_state_metadata: session_rail_metadata_contains(&metadata, "agent_state")
                || session_rail_metadata_contains(&metadata, "running_agents"),
            show_layout_metadata: session_rail_metadata_contains(&metadata, "layout")
                || session_rail_metadata_contains(&metadata, "saved_layout"),
            show_latest_attention_metadata: session_rail_metadata_contains(
                &metadata,
                "latest_attention",
            ),
            sort_by: session_rail.sort_by.unwrap(),
        }
    }
}

impl SessionRailSettings {
    fn is_hidden(&self) -> bool {
        self.visibility == settings::CanvasVisibility::Hidden
            || self.mode == settings::CanvasVisibility::Hidden
    }

    fn display_mode(&self) -> settings::CanvasVisibility {
        let requested_mode = match self.visibility {
            settings::CanvasVisibility::Icon
            | settings::CanvasVisibility::Compact
            | settings::CanvasVisibility::Detailed => self.visibility,
            settings::CanvasVisibility::Hidden
            | settings::CanvasVisibility::Overlay
            | settings::CanvasVisibility::Always
            | settings::CanvasVisibility::Auto => self.mode,
        };

        match requested_mode {
            // The legacy 56px rail cannot present Dez's terminal supervision
            // hierarchy, search, evidence, and recovery actions without
            // clipping. Keep reading the compatibility value, but degrade it
            // to the smallest deliberately supported v0.0.1 layout.
            settings::CanvasVisibility::Icon => settings::CanvasVisibility::Compact,
            mode => mode,
        }
    }

    fn width(&self, configured_width: Pixels) -> Pixels {
        let display_mode = self.display_mode();
        if self.is_hidden() {
            Pixels::ZERO
        } else if display_mode == settings::CanvasVisibility::Compact {
            configured_width.min(COMPACT_MAX_WIDTH)
        } else if display_mode == settings::CanvasVisibility::Detailed {
            Pixels::max(configured_width, DETAILED_MIN_WIDTH)
        } else {
            configured_width
        }
    }
}

impl settings::Settings for WorkspaceBarAttentionSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let workspace_bar = content.workspace_bar.clone().unwrap();
        let workspace_bar_visible =
            workspace_bar.visibility.unwrap() != settings::CanvasVisibility::Hidden;
        Self {
            show_agent_attention: workspace_bar_visible
                && workspace_bar.show_agent_attention.unwrap(),
        }
    }
}

fn session_rail_metadata_contains(metadata: &[String], field: &str) -> bool {
    metadata
        .iter()
        .any(|candidate| candidate.eq_ignore_ascii_case(field))
}

fn session_rail_labels_visible(design_system: &DesignSystemSettings) -> bool {
    design_system.show_contextual_labels()
}

fn agent_session_label(
    app_name: &str,
    upstream_thread_label: &'static str,
    dez_session_label: &'static str,
) -> &'static str {
    if app_name == "Zed" {
        upstream_thread_label
    } else {
        dez_session_label
    }
}

fn draft_discard_requires_confirmation(app_name: &str) -> bool {
    app_name != "Zed"
}

#[cfg(test)]
mod agent_session_label_tests {
    use super::*;

    #[test]
    fn preserves_official_zed_thread_language_and_uses_sessions_in_dez() {
        assert_eq!(
            agent_session_label("Zed", "Archive Thread", "Archive Agent Session"),
            "Archive Thread"
        );
        assert_eq!(
            agent_session_label("Dez", "Archive Thread", "Archive Agent Session"),
            "Archive Agent Session"
        );
    }
}

fn session_rail_row_is_compact(width: Pixels) -> bool {
    width < DETAILED_MIN_WIDTH
}

fn session_rail_recency_visible(width: Pixels, has_priority_metadata: bool) -> bool {
    !session_rail_row_is_compact(width) || !has_priority_metadata
}

fn session_rail_supplemental_metadata_visible(width: Pixels) -> bool {
    width >= SUPPLEMENTAL_METADATA_MIN_WIDTH
}

fn session_overview_status_label(
    session_count: usize,
    attention_count: usize,
    workspace_count: usize,
    is_searching: bool,
) -> String {
    let session_noun = if session_count == 1 {
        "session"
    } else {
        "sessions"
    };
    if is_searching {
        format!("{session_count} matching {session_noun}")
    } else if session_count == 0 {
        if workspace_count == 0 {
            return "No sessions yet".to_owned();
        }
        let workspace_noun = if workspace_count == 1 {
            "workspace"
        } else {
            "workspaces"
        };
        format!("{workspace_count} {workspace_noun} ready")
    } else if attention_count > 0 {
        let attention_verb = if attention_count == 1 {
            "needs"
        } else {
            "need"
        };
        format!("{attention_count} {attention_verb} attention · {session_count} total")
    } else {
        format!("{session_count} {session_noun} · caught up")
    }
}

fn session_scope_controls_visible(session_count: usize) -> bool {
    session_count > 0
}

fn all_sessions_accessibility_label(session_count: usize) -> String {
    format!("All sessions, {session_count} total")
}

fn attention_sessions_accessibility_label(attention_count: usize) -> String {
    let attention_verb = if attention_count == 1 {
        "needs"
    } else {
        "need"
    };
    format!("Attention sessions, {attention_count} {attention_verb} attention")
}

fn session_search_visible(session_count: usize, has_query: bool) -> bool {
    session_count > 0 || has_query
}

fn session_overview_visible(_show_start_state: bool) -> bool {
    true
}

fn session_overview_create_action_visible(session_count: usize) -> bool {
    session_count > 0
}

fn session_start_state_copy() -> (&'static str, &'static str, &'static str) {
    (
        "Terminals open in the Main Work Area. Live state and attention return here.",
        "New Terminal",
        "Open Workspace…",
    )
}

fn workspace_new_terminal_action_persistent(is_active: bool, is_menu_open: bool) -> bool {
    is_active || is_menu_open
}

fn workspace_options_action_persistent(is_active: bool, is_menu_open: bool) -> bool {
    is_active || is_menu_open
}

fn workspace_header_accessibility_label(
    workspace_name: &str,
    has_sessions: bool,
    has_running_sessions: bool,
    attention_count: usize,
) -> String {
    let mut label = format!("Workspace {workspace_name}");
    if !has_sessions {
        label.push_str(", ready for a session");
    }
    if has_running_sessions {
        label.push_str(", running work");
    }
    if attention_count == 1 {
        label.push_str(", 1 session needs attention");
    } else if attention_count > 1 {
        label.push_str(&format!(", {attention_count} sessions need attention"));
    }
    label
}

fn workspace_new_terminal_control_label(workspace_name: &str) -> String {
    format!("New Terminal in {workspace_name}")
}

fn workspace_options_control_label(workspace_name: &str) -> String {
    format!("Workspace Options for {workspace_name}")
}

fn merge_unambiguous_branch(
    branches: &mut HashMap<PathBuf, SharedString>,
    ambiguous_paths: &mut HashSet<PathBuf>,
    path: PathBuf,
    branch: SharedString,
) {
    if ambiguous_paths.contains(&path) {
        return;
    }

    if branches
        .get(&path)
        .is_some_and(|existing| existing != &branch)
    {
        branches.remove(&path);
        ambiguous_paths.insert(path);
    } else {
        branches.entry(path).or_insert(branch);
    }
}

fn canvas_thread_item_style(
    thread_item: ThreadItem,
    design_system: &DesignSystemSettings,
) -> ThreadItem {
    let density = match design_system.density {
        settings::CanvasDensity::Compact => ThreadItemDensity::Compact,
        settings::CanvasDensity::Balanced => ThreadItemDensity::Balanced,
        settings::CanvasDensity::Spacious => ThreadItemDensity::Spacious,
    };
    let radius = match design_system.radius {
        settings::CanvasRadius::None => ThreadItemRadius::None,
        settings::CanvasRadius::Subtle => ThreadItemRadius::Subtle,
        settings::CanvasRadius::Rounded => ThreadItemRadius::Rounded,
    };
    let contrast = match design_system.contrast {
        settings::CanvasContrast::Low => ThreadItemContrast::Low,
        settings::CanvasContrast::Standard => ThreadItemContrast::Standard,
        settings::CanvasContrast::High => ThreadItemContrast::High,
    };

    thread_item
        .density(density)
        .radius(radius)
        .contrast(contrast)
}

fn canvas_layout_recipe_label(recipe_id: &str) -> Option<&'static str> {
    Some(match recipe_id {
        "full" => "Full",
        "agent_control" => "Agent Control",
        "editor_focus" => "Focus Editor",
        "even_columns" => "Even Columns",
        "even_rows" => "Even Rows",
        "main_stack" => "Main + Stack",
        "main_top" => "Main Top",
        "golden_split" => "Golden Split",
        "code_run_observe" => "Code, Run, Observe",
        "review" => "Review",
        "debug" => "Debug",
        "documentation_studio" => "Documentation Studio",
        "browser_development" => "Browser Development",
        "agent_operations" => "Agent Operations",
        "four_agent_matrix" => "Four-Agent Matrix",
        "six_agent_supervisor" => "Six-Agent Supervisor",
        "worktree_matrix" => "Worktree Matrix",
        "remote_operations" => "Remote Operations",
        "pair_programming" => "Pair Programming",
        "incident_response" => "Incident Response",
        "portrait_display" => "Portrait Display",
        _ => return None,
    })
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum SerializedSidebarView {
    #[default]
    ThreadList,
    #[serde(alias = "Archive")]
    History,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NewEntryTarget {
    Terminal,
    AgentThread,
}

fn default_new_session_target() -> NewEntryTarget {
    NewEntryTarget::Terminal
}

#[derive(Default, Serialize, Deserialize)]
struct SerializedSidebar {
    #[serde(default)]
    width: Option<f32>,
    #[serde(default)]
    active_view: SerializedSidebarView,
    #[serde(default)]
    manual_entry_order: Vec<ManualEntryOrderKey>,
}

#[derive(Debug, Default)]
enum SidebarView {
    #[default]
    ThreadList,
    Archive(Entity<ThreadsArchiveView>),
}

enum ArchiveWorktreeOutcome {
    Success,
    Cancelled,
}

#[derive(Clone, Debug)]
enum ActiveEntry {
    Thread {
        thread_id: agent_ui::ThreadId,
        /// Stable remote identifier, used for matching when thread_id
        /// differs (e.g. after cross-window activation creates a new
        /// local ThreadId).
        session_id: Option<acp::SessionId>,
        workspace: Entity<Workspace>,
    },
    Terminal {
        terminal_id: TerminalId,
        workspace: Entity<Workspace>,
    },
}

impl ActiveEntry {
    fn workspace(&self) -> &Entity<Workspace> {
        match self {
            ActiveEntry::Thread { workspace, .. } | ActiveEntry::Terminal { workspace, .. } => {
                workspace
            }
        }
    }

    fn is_active_thread(&self, thread_id: &agent_ui::ThreadId) -> bool {
        matches!(self, ActiveEntry::Thread { thread_id: active_thread_id, .. } if active_thread_id == thread_id)
    }

    fn is_active_terminal(&self, terminal_id: TerminalId) -> bool {
        matches!(self, ActiveEntry::Terminal { terminal_id: active_terminal_id, .. } if *active_terminal_id == terminal_id)
    }

    fn matches_entry(&self, entry: &ListEntry) -> bool {
        match (self, entry) {
            (
                ActiveEntry::Thread {
                    thread_id,
                    session_id,
                    ..
                },
                ListEntry::Thread(thread),
            ) => {
                *thread_id == thread.metadata.thread_id
                    || session_id
                        .as_ref()
                        .zip(thread.metadata.session_id.as_ref())
                        .is_some_and(|(a, b)| a == b)
            }
            (ActiveEntry::Terminal { terminal_id, .. }, ListEntry::Terminal(terminal)) => {
                *terminal_id == terminal.metadata.terminal_id
            }
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
struct ActiveThreadInfo {
    session_id: acp::SessionId,
    title: SharedString,
    status: AgentThreadStatus,
    icon: IconName,
    icon_from_external_svg: Option<SharedString>,
    is_background: bool,
    is_title_generating: bool,
    diff_stats: DiffStats,
    changed_files: Vec<PathBuf>,
}

#[derive(Clone)]
enum ThreadEntryWorkspace {
    Open(Entity<Workspace>),
    Closed {
        /// The paths this entry uses (may point to linked worktrees).
        folder_paths: PathList,
        /// The project group this entry belongs to.
        project_group_key: ProjectGroupKey,
    },
}

impl ThreadEntryWorkspace {
    const MAX_REPOSITORY_EVIDENCE: usize = 8;
    const MAX_REPOSITORY_CHANGED_PATHS: usize = 128;

    fn is_remote(&self, cx: &App) -> bool {
        match self {
            ThreadEntryWorkspace::Open(workspace) => {
                !workspace.read(cx).project().read(cx).is_local()
            }
            ThreadEntryWorkspace::Closed {
                project_group_key, ..
            } => project_group_key.host().is_some(),
        }
    }

    fn authoritative_workspace_evidence(
        &self,
        terminal_session_id: Option<&str>,
        cx: &App,
    ) -> Option<(Vec<ObservedWorkspaceEvidence>, bool, bool, bool)> {
        let Self::Open(workspace) = self else {
            return None;
        };
        let workspace = workspace.read(cx);
        let evidence = workspace.evidence_set();
        let user_selected_paths = evidence
            .records()
            .iter()
            .filter(|record| record.kind == AuthoritativeWorkspaceEvidenceKind::UserSelectedPath)
            .map(|record| record.path.as_ref())
            .collect::<HashSet<_>>();
        let records = evidence
            .records()
            .iter()
            .filter(|record| match record.kind {
                AuthoritativeWorkspaceEvidenceKind::WorkspaceRoot
                | AuthoritativeWorkspaceEvidenceKind::UserSelectedPath => true,
                AuthoritativeWorkspaceEvidenceKind::OpenFile => {
                    !user_selected_paths.contains(record.path.as_ref())
                }
                AuthoritativeWorkspaceEvidenceKind::TerminalWorkingDirectory => terminal_session_id
                    .is_some_and(|terminal_session_id| {
                        matches!(
                            &record.provenance,
                            WorkspaceEvidenceProvenance::TerminalSession { session_id }
                                if session_id.as_ref() == terminal_session_id
                        )
                    }),
            })
            .collect::<Vec<_>>();
        let has_stale_evidence = records
            .iter()
            .any(|record| record.lifecycle == WorkspaceEvidenceLifecycle::Stale);
        let has_unresolved_evidence = records
            .iter()
            .any(|record| record.lifecycle == WorkspaceEvidenceLifecycle::Unresolved);
        Some((
            records
                .into_iter()
                .map(|record| ObservedWorkspaceEvidence {
                    kind: match record.kind {
                        AuthoritativeWorkspaceEvidenceKind::WorkspaceRoot => {
                            WorkspaceEvidenceKind::WorkspaceRoot
                        }
                        AuthoritativeWorkspaceEvidenceKind::OpenFile => {
                            WorkspaceEvidenceKind::OpenFile
                        }
                        AuthoritativeWorkspaceEvidenceKind::UserSelectedPath => {
                            WorkspaceEvidenceKind::UserSelectedPath
                        }
                        AuthoritativeWorkspaceEvidenceKind::TerminalWorkingDirectory => {
                            WorkspaceEvidenceKind::TerminalWorkingDirectory
                        }
                    },
                    path: record.path.to_path_buf(),
                })
                .collect(),
            evidence.is_truncated(),
            has_stale_evidence,
            has_unresolved_evidence,
        ))
    }

    fn authoritative_repository_evidence(
        &self,
        terminal_working_directory: Option<&Path>,
        cx: &App,
    ) -> Option<(Vec<ObservedRepositoryEvidence>, bool)> {
        let Self::Open(workspace) = self else {
            return None;
        };
        let project_entity = workspace.read(cx).project().clone();
        let project = project_entity.read(cx);
        let mut repositories = project
            .repositories(cx)
            .values()
            .cloned()
            .collect::<Vec<_>>();

        if let Some(terminal_working_directory) = terminal_working_directory {
            repositories.retain(|repository| {
                repository
                    .read(cx)
                    .abs_path_to_repo_path(terminal_working_directory)
                    .is_some()
            });
            // Nested repositories can both contain a directory. The longest
            // worktree path is the most specific authoritative owner.
            repositories.sort_by(|left, right| {
                right
                    .read(cx)
                    .work_directory_abs_path
                    .components()
                    .count()
                    .cmp(&left.read(cx).work_directory_abs_path.components().count())
            });
            repositories.truncate(1);
        } else {
            repositories.sort_by(|left, right| {
                left.read(cx)
                    .work_directory_abs_path
                    .cmp(&right.read(cx).work_directory_abs_path)
            });
        }

        let repositories_truncated = repositories.len() > Self::MAX_REPOSITORY_EVIDENCE;
        repositories.truncate(Self::MAX_REPOSITORY_EVIDENCE);
        let evidence = repositories
            .into_iter()
            .map(|repository| {
                let repository = repository.read(cx);
                let summary = repository.status_summary();
                let mut changed_paths = repository
                    .cached_status()
                    .filter_map(|entry| {
                        repository
                            .repo_path_to_project_path(&entry.repo_path, cx)
                            .and_then(|path| project.absolute_path(&path, cx))
                    })
                    .collect::<Vec<_>>();
                changed_paths.sort();
                changed_paths.dedup();
                let changed_paths_truncated = changed_paths.len() < summary.count
                    || changed_paths.len() > Self::MAX_REPOSITORY_CHANGED_PATHS;
                changed_paths.truncate(Self::MAX_REPOSITORY_CHANGED_PATHS);
                ObservedRepositoryEvidence {
                    worktree_path: repository.work_directory_abs_path.to_path_buf(),
                    main_worktree_path: repository
                        .main_worktree_abs_path()
                        .filter(|path| *path != repository.work_directory_abs_path.as_ref())
                        .map(Path::to_path_buf),
                    branch: repository
                        .branch
                        .as_ref()
                        .map(|branch| branch.name().to_owned()),
                    changed_paths,
                    changed_path_count: summary.count,
                    conflict_count: summary.conflict,
                    untracked_count: summary.untracked,
                    linked_worktree: repository.is_linked_worktree(),
                    truncated: changed_paths_truncated,
                }
            })
            .collect();
        Some((evidence, repositories_truncated))
    }
}

/// If the title begins with a decorative prefix (such as a leading emoji,
/// spinner glyph, or symbol the agent prefixed the title with), splits that
/// prefix off so a single representative glyph can be displayed in place of the
/// entry's icon.
fn split_leading_icon_char(
    title: &SharedString,
    highlight_positions: &[usize],
) -> Option<(SharedString, SharedString, Vec<usize>)> {
    let prefix = terminal_title_prefix(title)?;
    let icon_char = pick_icon_glyph(prefix)?;

    let stripped_len = prefix.len();
    let trimmed_title = &title[stripped_len..];
    if trimmed_title.is_empty() {
        return None;
    }

    let adjusted_positions = highlight_positions
        .iter()
        .filter(|&&position| position >= stripped_len)
        .map(|&position| position - stripped_len)
        .collect();

    Some((
        icon_char,
        trimmed_title.to_string().into(),
        adjusted_positions,
    ))
}

fn terminal_agent_icon(kind: TerminalAgentKind) -> IconName {
    match kind {
        TerminalAgentKind::Claude => IconName::AiClaude,
        TerminalAgentKind::Codex => IconName::Robot,
        TerminalAgentKind::Copilot => IconName::Copilot,
        TerminalAgentKind::Cursor => IconName::EditorCursor,
        TerminalAgentKind::Gemini
        | TerminalAgentKind::Aider
        | TerminalAgentKind::Agy
        | TerminalAgentKind::OpenCode
        | TerminalAgentKind::Amp
        | TerminalAgentKind::Crush
        | TerminalAgentKind::Devin
        | TerminalAgentKind::Droid
        | TerminalAgentKind::Goose
        | TerminalAgentKind::Grok
        | TerminalAgentKind::OpenHands
        | TerminalAgentKind::Pi
        | TerminalAgentKind::Qwen => IconName::Robot,
    }
}

const CODEX_HOOK_SETUP: &str = include_str!("../../../assets/dez/codex-hooks.json");

fn terminal_agent_state_label(
    agent: Option<&TerminalAgentSnapshot>,
    runtime: Option<&TerminalRuntimeInfo>,
    needs_attention: bool,
    is_snoozed: bool,
    setup_available: bool,
    show_detection_confidence: bool,
) -> SharedString {
    let runtime_state = runtime.map(|runtime| runtime.state);
    let mut state = agent.map_or_else(
        || {
            let transport_state = match runtime_state {
                Some(TerminalRuntimeState::Live) if needs_attention => "Needs attention",
                Some(TerminalRuntimeState::Live) => "Live",
                Some(TerminalRuntimeState::Detached) => "Detached",
                Some(TerminalRuntimeState::Reconnecting) => "Reconnecting",
                Some(TerminalRuntimeState::Exited) => "Exited",
                Some(TerminalRuntimeState::Missing) => "Missing",
                Some(TerminalRuntimeState::Incompatible) => "Incompatible",
                None => "Saved",
            };
            if needs_attention && transport_state != "Needs attention" {
                format!("{transport_state} · Needs attention")
            } else {
                transport_state.to_owned()
            }
        },
        |agent| {
            let mut state = agent.state.label().to_owned();
            if let Some(runtime_state) =
                runtime_state.filter(|state| !matches!(state, TerminalRuntimeState::Live))
            {
                state.push_str(" · ");
                state.push_str(runtime_state.label());
            }
            if needs_attention
                && !matches!(
                    agent.state,
                    TerminalAgentState::WaitingForPermission
                        | TerminalAgentState::WaitingForInput
                        | TerminalAgentState::Failed
                )
            {
                state.push_str(" · Needs attention");
            }
            state
        },
    );
    if is_snoozed {
        state.push_str(" · Snoozed");
    }
    let state = if show_detection_confidence && agent.is_none() {
        format!("Detected · {state}")
    } else {
        state
    };
    if setup_available {
        format!("{state} · Hook setup").into()
    } else {
        state.into()
    }
}

fn observed_run_evidence_label(
    checks: &[ObservedRunCheck],
    command_count: usize,
    evidence_truncated: bool,
) -> Option<(SharedString, ThreadItemEvidenceStatus)> {
    let qualify = |label: String| {
        if evidence_truncated {
            format!("{label} · partial")
        } else {
            label
        }
    };
    let check_count = checks.len();
    if check_count > 0 {
        let failed_count = checks
            .iter()
            .filter(|check| check.status == ObservedRunCheckStatus::Failed)
            .count();
        if failed_count > 0 {
            let label = if check_count == 1 {
                "1 check failed".to_owned()
            } else {
                format!("{failed_count}/{check_count} checks failed")
            };
            return Some((qualify(label).into(), ThreadItemEvidenceStatus::Failed));
        }

        let running_count = checks
            .iter()
            .filter(|check| check.status == ObservedRunCheckStatus::Running)
            .count();
        if running_count > 0 {
            let noun = if running_count == 1 {
                "check"
            } else {
                "checks"
            };
            return Some((
                qualify(format!("{running_count} {noun} running")).into(),
                ThreadItemEvidenceStatus::Neutral,
            ));
        }

        let noun = if check_count == 1 { "check" } else { "checks" };
        return Some((
            qualify(format!("{check_count} {noun} passed")).into(),
            ThreadItemEvidenceStatus::Passed,
        ));
    }

    if command_count > 0 {
        let noun = if command_count == 1 {
            "command"
        } else {
            "commands"
        };
        Some((
            qualify(format!("{command_count} {noun}")).into(),
            ThreadItemEvidenceStatus::Neutral,
        ))
    } else {
        None
    }
}

fn terminal_attention_priority(
    metadata: &TerminalThreadMetadata,
    agent: Option<&TerminalAgentSnapshot>,
) -> TerminalAttentionPriority {
    if agent.is_some_and(|agent| {
        matches!(
            agent.state,
            TerminalAgentState::WaitingForPermission | TerminalAgentState::Failed
        )
    }) {
        TerminalAttentionPriority::Urgent
    } else {
        metadata.attention.priority
    }
}

#[derive(Clone)]
struct TerminalRuntimeInfo {
    state: TerminalRuntimeState,
}

fn terminal_runtime_from_snapshot(
    snapshot: &TerminalSessionSnapshot,
) -> Option<TerminalRuntimeInfo> {
    let state = match snapshot.state {
        TerminalSessionState::Starting | TerminalSessionState::Attached => {
            TerminalRuntimeState::Live
        }
        TerminalSessionState::Detached => TerminalRuntimeState::Detached,
        TerminalSessionState::Reconnecting => TerminalRuntimeState::Reconnecting,
        TerminalSessionState::Exited { .. } => TerminalRuntimeState::Exited,
        TerminalSessionState::Missing => TerminalRuntimeState::Missing,
        TerminalSessionState::Incompatible { .. } => TerminalRuntimeState::Incompatible,
    };
    Some(TerminalRuntimeInfo { state })
}

fn terminal_agent_kind_from_snapshot(
    snapshot: Option<&TerminalAgentSnapshot>,
) -> Option<TerminalAgentKind> {
    snapshot.and_then(|agent| {
        agent
            .adapter
            .starts_with("codex-")
            .then_some(TerminalAgentKind::Codex)
    })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TerminalRuntimeState {
    Live,
    Detached,
    Reconnecting,
    Exited,
    Missing,
    Incompatible,
}

impl TerminalRuntimeState {
    fn label(self) -> &'static str {
        match self {
            Self::Live => "Live",
            Self::Detached => "Detached",
            Self::Reconnecting => "Reconnecting",
            Self::Exited => "Exited",
            Self::Missing => "Missing",
            Self::Incompatible => "Incompatible",
        }
    }
}

fn terminal_row_close_presentation(
    is_host_session: bool,
    runtime_state: Option<TerminalRuntimeState>,
) -> (&'static str, bool) {
    match (is_host_session, runtime_state) {
        (true, Some(TerminalRuntimeState::Live)) => ("Terminate Running Terminal…", true),
        (true, Some(TerminalRuntimeState::Detached)) => ("Terminate Detached Terminal…", true),
        (true, Some(TerminalRuntimeState::Reconnecting)) => {
            ("Terminate Reconnecting Terminal…", true)
        }
        (true, Some(TerminalRuntimeState::Exited)) => ("Remove Exited Terminal", false),
        (true, Some(TerminalRuntimeState::Missing)) => ("Remove Missing Terminal", false),
        (true, Some(TerminalRuntimeState::Incompatible)) => ("Remove Incompatible Terminal", false),
        (true, None) => ("Remove Saved Terminal", false),
        (false, Some(TerminalRuntimeState::Live)) => ("Detach Live Terminal", false),
        (false, Some(TerminalRuntimeState::Detached)) => ("Terminate Detached Terminal…", true),
        (false, Some(TerminalRuntimeState::Reconnecting)) => {
            ("Terminate Reconnecting Terminal…", true)
        }
        (false, Some(TerminalRuntimeState::Exited)) => ("Close Exited Terminal", false),
        (false, Some(TerminalRuntimeState::Missing)) => ("Remove Missing Terminal", false),
        (false, Some(TerminalRuntimeState::Incompatible)) => {
            ("Remove Incompatible Terminal", false)
        }
        (false, None) => ("Remove Saved Terminal", false),
    }
}

fn terminal_termination_confirmation_copy(title: &str) -> (&'static str, String) {
    (
        "Terminate Terminal Session?",
        format!(
            "“{title}” will stop immediately, including its shell and any foreground process. This cannot be undone."
        ),
    )
}

fn terminal_row_owner_label(has_session_ref: bool, is_remote: bool) -> &'static str {
    if has_session_ref {
        "Persistent"
    } else if is_remote {
        "Remote"
    } else {
        "Local"
    }
}

#[cfg(test)]
mod workspace_header_label_tests {
    use super::*;

    #[test]
    fn workspace_controls_name_their_actual_workspace() {
        assert_eq!(
            workspace_new_terminal_control_label("compiler"),
            "New Terminal in compiler"
        );
        assert_eq!(
            workspace_options_control_label("compiler"),
            "Workspace Options for compiler"
        );
        assert!(!workspace_new_terminal_control_label("compiler").contains("header-group"));
        assert!(!workspace_options_control_label("compiler").contains("header-group"));
    }
}

#[cfg(test)]
mod session_start_state_tests {
    use super::*;

    #[test]
    fn start_state_has_one_primary_action_and_workspace_copy() {
        assert!(!session_overview_create_action_visible(0));
        assert!(session_overview_create_action_visible(1));
        assert_eq!(
            session_start_state_copy(),
            (
                "Terminals open in the Main Work Area. Live state and attention return here.",
                "New Terminal",
                "Open Workspace…"
            )
        );
    }
}

#[cfg(test)]
mod terminal_runtime_label_tests {
    use super::*;

    #[test]
    fn host_owned_session_actions_never_present_termination_as_detach() {
        assert_eq!(
            terminal_row_close_presentation(true, Some(TerminalRuntimeState::Live)),
            ("Terminate Running Terminal…", true)
        );
        assert_eq!(
            terminal_row_close_presentation(true, Some(TerminalRuntimeState::Detached)),
            ("Terminate Detached Terminal…", true)
        );
        assert_eq!(
            terminal_row_close_presentation(true, Some(TerminalRuntimeState::Exited)),
            ("Remove Exited Terminal", false)
        );
        assert_eq!(
            terminal_row_close_presentation(false, Some(TerminalRuntimeState::Live)),
            ("Detach Live Terminal", false)
        );
    }

    #[test]
    fn terminal_termination_confirmation_names_the_irreversible_effect() {
        let (heading, detail) = terminal_termination_confirmation_copy("tests");
        assert_eq!(heading, "Terminate Terminal Session?");
        assert!(detail.contains("“tests”"));
        assert!(detail.contains("shell and any foreground process"));
        assert!(detail.contains("cannot be undone"));
        assert!(!detail.contains("durable"));
    }

    #[test]
    fn terminal_row_ownership_is_explicit() {
        assert_eq!(terminal_row_owner_label(true, false), "Persistent");
        assert_eq!(terminal_row_owner_label(false, true), "Remote");
        assert_eq!(terminal_row_owner_label(false, false), "Local");
    }

    #[test]
    fn distinguishes_saved_live_and_attention_terminal_agents() {
        let live = TerminalRuntimeInfo {
            state: TerminalRuntimeState::Live,
        };
        let detached = TerminalRuntimeInfo {
            state: TerminalRuntimeState::Detached,
        };
        let missing = TerminalRuntimeInfo {
            state: TerminalRuntimeState::Missing,
        };
        let incompatible = TerminalRuntimeInfo {
            state: TerminalRuntimeState::Incompatible,
        };

        assert_eq!(
            terminal_agent_state_label(None, None, false, false, false, true),
            "Detected · Saved"
        );
        assert_eq!(
            terminal_agent_state_label(None, None, true, false, false, true),
            "Detected · Saved · Needs attention"
        );
        assert_eq!(
            terminal_agent_state_label(None, Some(&live), false, false, false, true),
            "Detected · Live"
        );
        assert_eq!(
            terminal_agent_state_label(None, Some(&live), true, false, false, true),
            "Detected · Needs attention"
        );
        assert_eq!(
            terminal_agent_state_label(None, Some(&detached), false, false, false, true),
            "Detected · Detached"
        );
        assert_eq!(
            terminal_agent_state_label(None, Some(&detached), true, false, false, true),
            "Detected · Detached · Needs attention"
        );
        assert_eq!(
            terminal_agent_state_label(None, Some(&missing), false, false, false, true),
            "Detected · Missing"
        );
        assert_eq!(
            terminal_agent_state_label(None, Some(&missing), true, false, false, true),
            "Detected · Missing · Needs attention"
        );
        assert_eq!(
            terminal_agent_state_label(None, Some(&incompatible), false, false, false, true),
            "Detected · Incompatible"
        );

        let structured = TerminalAgentSnapshot {
            adapter: "codex-hooks-v1".to_owned(),
            actor: "Codex".to_owned(),
            capabilities: terminal::session_host::TerminalAgentCapabilities::codex_hooks_v1(),
            provider_session_id: Some("session-7".to_owned()),
            state: TerminalAgentState::WaitingForPermission,
            attention_required: true,
            resumable: true,
            events_truncated: false,
            events: Vec::new(),
        };
        assert_eq!(
            terminal_agent_state_label(Some(&structured), Some(&live), true, false, false, true,),
            "Waiting for permission"
        );
        assert_eq!(
            terminal_agent_state_label(
                Some(&TerminalAgentSnapshot {
                    state: TerminalAgentState::Running,
                    ..structured.clone()
                }),
                Some(&detached),
                false,
                false,
                false,
                true,
            ),
            "Running · Detached"
        );
        assert_eq!(
            terminal_agent_state_label(
                Some(&TerminalAgentSnapshot {
                    state: TerminalAgentState::Running,
                    ..structured.clone()
                }),
                Some(&missing),
                true,
                false,
                false,
                true,
            ),
            "Running · Missing · Needs attention"
        );
        assert_eq!(
            terminal_agent_state_label(None, Some(&live), false, true, false, true),
            "Detected · Live · Snoozed"
        );
        assert_eq!(
            terminal_agent_state_label(None, Some(&live), false, false, true, true),
            "Detected · Live · Hook setup"
        );
    }

    #[test]
    fn bundled_codex_hook_setup_covers_structured_lifecycle() {
        let setup: serde_json::Value =
            serde_json::from_str(CODEX_HOOK_SETUP).unwrap_or(serde_json::Value::Null);
        let hooks = setup.get("hooks").and_then(serde_json::Value::as_object);
        assert!(hooks.is_some_and(|hooks| {
            [
                "SessionStart",
                "UserPromptSubmit",
                "PermissionRequest",
                "PreToolUse",
                "PostToolUse",
                "Stop",
            ]
            .iter()
            .all(|event| hooks.contains_key(*event))
        }));
    }

    #[test]
    fn run_evidence_labels_prioritize_check_outcomes() {
        let passed = ObservedRunCheck {
            name: "cargo test".to_owned(),
            status: ObservedRunCheckStatus::Passed,
            source_path: None,
        };
        let failed = ObservedRunCheck {
            name: "cargo clippy".to_owned(),
            status: ObservedRunCheckStatus::Failed,
            source_path: None,
        };

        assert_eq!(
            observed_run_evidence_label(&[], 2, false),
            Some(("2 commands".into(), ThreadItemEvidenceStatus::Neutral))
        );
        assert_eq!(
            observed_run_evidence_label(std::slice::from_ref(&passed), 1, false),
            Some(("1 check passed".into(), ThreadItemEvidenceStatus::Passed))
        );
        assert_eq!(
            observed_run_evidence_label(&[passed, failed], 2, false),
            Some(("1/2 checks failed".into(), ThreadItemEvidenceStatus::Failed))
        );
        assert_eq!(
            observed_run_evidence_label(&[], 32, true),
            Some((
                "32 commands · partial".into(),
                ThreadItemEvidenceStatus::Neutral
            ))
        );
    }

    #[test]
    fn review_state_never_hides_exceptional_transport_truth() {
        let agent = TerminalAgentSnapshot {
            adapter: "codex-hooks-v1".to_owned(),
            actor: "Codex".to_owned(),
            capabilities: terminal::session_host::TerminalAgentCapabilities::codex_hooks_v1(),
            provider_session_id: Some("session-8".to_owned()),
            state: TerminalAgentState::Running,
            attention_required: false,
            resumable: true,
            events_truncated: false,
            events: Vec::new(),
        };
        let detached = TerminalRuntimeInfo {
            state: TerminalRuntimeState::Detached,
        };
        let missing = TerminalRuntimeInfo {
            state: TerminalRuntimeState::Missing,
        };
        let exited = TerminalRuntimeInfo {
            state: TerminalRuntimeState::Exited,
        };

        assert_eq!(
            terminal_run_review_state(Some(&agent), Some(&detached)),
            RunReviewState::Detached
        );
        assert_eq!(
            terminal_run_review_state(Some(&agent), Some(&missing)),
            RunReviewState::Missing
        );
        assert_eq!(
            terminal_run_review_state(Some(&agent), Some(&exited)),
            RunReviewState::Exited
        );
        assert_eq!(
            terminal_run_review_state(
                Some(&TerminalAgentSnapshot {
                    state: TerminalAgentState::Completed,
                    ..agent
                }),
                Some(&exited),
            ),
            RunReviewState::Completed
        );
    }
}

fn standalone_terminal_id(
    _workspace: &Entity<Workspace>,
    terminal_view: &Entity<TerminalView>,
    cx: &App,
) -> TerminalId {
    let terminal_entity = terminal_view.read(cx).terminal().clone();
    let terminal = terminal_entity.read(cx);
    let session_id = terminal.session_id();
    let host_id = if terminal.is_hosted() {
        TerminalHostConnection::try_global(cx).map(|connection| connection.host_id())
    } else {
        LocalTerminalHost::try_global(cx).map(|host| host.read(cx).host_id())
    };
    let terminal_key = host_id
        .map(|host_id| format!("{host_id}:{session_id}"))
        .unwrap_or_else(|| session_id.to_string());
    TerminalId::from_stable_key("terminal-session", &terminal_key)
}

fn standalone_terminal_metadata(
    workspace: &Entity<Workspace>,
    terminal_view: &Entity<TerminalView>,
    created_at: DateTime<Utc>,
    cx: &App,
) -> (
    TerminalThreadMetadata,
    Option<TerminalAgentKind>,
    TerminalRuntimeInfo,
) {
    let terminal_id = standalone_terminal_id(workspace, terminal_view, cx);
    let terminal_view = terminal_view.read(cx);
    let has_attention = terminal_view.has_bell();
    let terminal = terminal_view.terminal().read(cx);
    let title = SharedString::from(terminal.title(false));
    let custom_title = terminal_view
        .custom_title()
        .map(|title| SharedString::from(title.to_string()));
    let working_directory = terminal.working_directory();
    let session_ref = if terminal.is_hosted() {
        TerminalHostConnection::try_global(cx).map(|connection| {
            terminal::session_host::TerminalSessionRef {
                host_id: connection.host_id(),
                session_id: terminal.session_id(),
            }
        })
    } else {
        LocalTerminalHost::try_global(cx).and_then(|host| {
            host.read(cx)
                .session_ref_if_registered(terminal.session_id())
        })
    };
    let detected_agent_command = terminal
        .foreground_process_command_name()
        .as_deref()
        .and_then(detect_terminal_agent_command);
    let runtime = TerminalRuntimeInfo {
        state: if terminal.process_exited() {
            TerminalRuntimeState::Exited
        } else {
            TerminalRuntimeState::Live
        },
    };

    let project = workspace.read(cx).project().clone();
    let project = project.read(cx);
    let mut attention = TerminalAttentionState::default();
    if has_attention {
        attention.raise_observed(Utc::now());
    }
    let metadata = TerminalThreadMetadata {
        terminal_id,
        title,
        custom_title,
        created_at,
        worktree_paths: project.worktree_paths(cx),
        remote_connection: project.remote_connection_options(cx),
        working_directory,
        attention,
        session_ref,
    };

    let detected_agent_kind = metadata.detected_agent_kind().or(detected_agent_command);
    (metadata, detected_agent_kind, runtime)
}

fn workspace_for_local_terminal_session(
    snapshot: &TerminalSessionSnapshot,
    workspaces: &[Entity<Workspace>],
    active_workspace: Option<&Entity<Workspace>>,
    cx: &App,
) -> Option<Entity<Workspace>> {
    let durable_workspace_match = snapshot.workspace_id.and_then(|workspace_id| {
        workspaces
            .iter()
            .find(|workspace| workspace.read(cx).database_id().map(i64::from) == Some(workspace_id))
            .cloned()
    });
    let best_path_match = snapshot.working_directory.as_ref().and_then(|cwd| {
        workspaces
            .iter()
            .filter_map(|workspace| {
                let match_depth = workspace
                    .read(cx)
                    .root_paths(cx)
                    .into_iter()
                    .filter(|root| cwd.starts_with(root))
                    .map(|root| root.components().count())
                    .max()?;
                Some((match_depth, workspace.clone()))
            })
            .max_by_key(|(depth, _)| *depth)
            .map(|(_, workspace)| workspace)
    });

    durable_workspace_match
        .or(best_path_match)
        .or_else(|| active_workspace.cloned())
        .or_else(|| workspaces.first().cloned())
}

/// Picks a single glyph to render as the icon from a detected title prefix.
///
/// We only ever show one glyph, so this makes a best effort to choose a
/// meaningful one by glancing at the leading characters of the prefix:
/// runs of `.` are condensed into a single ellipsis, surrounding ASCII brackets
/// are stripped (so `[!]` yields `!`), and a leading run of the same character
/// is collapsed (so `>>>` yields `>`). The result is the first grapheme cluster
/// of whatever remains, keeping multi-codepoint emoji intact.
fn pick_icon_glyph(prefix: &str) -> Option<SharedString> {
    let prefix = prefix.trim();
    if prefix.is_empty() {
        return None;
    }

    // Strip a single pair of surrounding ASCII brackets, e.g. `[!]` -> `!`.
    let unwrapped = match prefix.chars().next() {
        Some('[') => prefix.strip_prefix('[').and_then(|s| s.strip_suffix(']')),
        Some('(') => prefix.strip_prefix('(').and_then(|s| s.strip_suffix(')')),
        Some('{') => prefix.strip_prefix('{').and_then(|s| s.strip_suffix('}')),
        Some('<') => prefix.strip_prefix('<').and_then(|s| s.strip_suffix('>')),
        _ => None,
    };
    let prefix = unwrapped
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(prefix);

    // Condense a leading run of dots (`...`) into a single ellipsis.
    if prefix.starts_with("..") {
        return Some("\u{2026}".into());
    }

    // Take the first grapheme cluster so multi-codepoint emoji stay intact.
    let first_grapheme = prefix.graphemes(true).next()?;
    if first_grapheme.trim().is_empty() {
        return None;
    }

    Some(first_grapheme.to_string().into())
}

fn draft_display_label_for_thread_metadata(
    metadata: &ThreadMetadata,
    workspace: &ThreadEntryWorkspace,
    cx: &App,
) -> Option<(SharedString, DraftKind)> {
    let workspace = match workspace {
        ThreadEntryWorkspace::Open(workspace) => Some(workspace),
        ThreadEntryWorkspace::Closed { .. } => None,
    };

    if let Some(label) =
        agent_ui::draft_prompt_store::display_label_for_draft(workspace, metadata.thread_id, cx)
    {
        return Some((label, DraftKind::WithContent));
    }

    let placeholder = agent_ui::draft_prompt_store::empty_draft_placeholder_label(
        workspace,
        &metadata.agent_id,
        cx,
    );
    Some((placeholder, DraftKind::Empty))
}

fn thread_metadata_would_render_sidebar_row(
    metadata: &ThreadMetadata,
    workspace: &ThreadEntryWorkspace,
    cx: &App,
) -> bool {
    if !metadata.is_draft() {
        return true;
    }

    draft_display_label_for_thread_metadata(metadata, workspace, cx).is_some()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum DraftKind {
    WithContent,
    Empty,
}

#[derive(Clone)]
struct ThreadEntry {
    metadata: ThreadMetadata,
    icon: IconName,
    icon_from_external_svg: Option<SharedString>,
    status: AgentThreadStatus,
    workspace: ThreadEntryWorkspace,
    is_live: bool,
    is_background: bool,
    is_title_generating: bool,
    draft: Option<DraftKind>,
    highlight_positions: Vec<usize>,
    worktrees: Vec<ThreadItemWorktreeInfo>,
    diff_stats: DiffStats,
    changed_files: Vec<PathBuf>,
}

#[derive(Clone)]
struct TerminalEntry {
    metadata: TerminalThreadMetadata,
    detected_agent_kind: Option<TerminalAgentKind>,
    workspace: ThreadEntryWorkspace,
    source: TerminalEntrySource,
    runtime: Option<TerminalRuntimeInfo>,
    agent: Option<TerminalAgentSnapshot>,
    worktrees: Vec<ThreadItemWorktreeInfo>,
    needs_attention: bool,
    attention_priority: TerminalAttentionPriority,
    has_notification: bool,
    highlight_positions: Vec<usize>,
}

#[derive(Clone)]
enum TerminalEntrySource {
    AgentPanel,
    WorkspaceItem(Entity<TerminalView>),
    HostSession(TerminalSessionId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StoredTerminalSource {
    AgentPanel,
    HostSession(TerminalSessionId),
}

fn stored_terminal_source(
    app_name: &str,
    has_live_agent_terminal: bool,
    host_session_id: Option<TerminalSessionId>,
) -> Option<StoredTerminalSource> {
    if app_name == "Zed" {
        Some(StoredTerminalSource::AgentPanel)
    } else if let Some(session_id) = host_session_id {
        Some(StoredTerminalSource::HostSession(session_id))
    } else if has_live_agent_terminal {
        Some(StoredTerminalSource::AgentPanel)
    } else {
        None
    }
}

#[derive(Clone, Copy)]
enum TerminalAttentionAction {
    Acknowledge,
    SnoozeOneHour,
    Resume,
    Resolve,
}

fn terminal_run_review_state(
    agent: Option<&TerminalAgentSnapshot>,
    runtime: Option<&TerminalRuntimeInfo>,
) -> RunReviewState {
    match runtime.map(|runtime| runtime.state) {
        Some(TerminalRuntimeState::Detached) => return RunReviewState::Detached,
        Some(TerminalRuntimeState::Reconnecting) => return RunReviewState::Reconnecting,
        Some(TerminalRuntimeState::Missing) => return RunReviewState::Missing,
        Some(TerminalRuntimeState::Incompatible) => return RunReviewState::Incompatible,
        Some(TerminalRuntimeState::Exited)
            if !agent.is_some_and(|agent| {
                matches!(
                    agent.state,
                    TerminalAgentState::Completed | TerminalAgentState::Failed
                )
            }) =>
        {
            return RunReviewState::Exited;
        }
        Some(TerminalRuntimeState::Live | TerminalRuntimeState::Exited) | None => {}
    }

    agent.map_or_else(
        || {
            if runtime.is_some() {
                RunReviewState::Running
            } else {
                RunReviewState::Saved
            }
        },
        |agent| match agent.state {
            TerminalAgentState::Starting | TerminalAgentState::Running => RunReviewState::Running,
            TerminalAgentState::WaitingForPermission => RunReviewState::WaitingForPermission,
            TerminalAgentState::WaitingForInput => RunReviewState::WaitingForInput,
            TerminalAgentState::Idle => RunReviewState::Idle,
            TerminalAgentState::Completed => RunReviewState::Completed,
            TerminalAgentState::Failed => RunReviewState::Failed,
            TerminalAgentState::Disconnected => RunReviewState::Reconnecting,
            TerminalAgentState::Resumable => RunReviewState::Resumable,
            TerminalAgentState::Exited => RunReviewState::Exited,
        },
    )
}

impl ThreadEntry {
    /// Updates this thread entry with active thread information.
    ///
    /// The existing [`ThreadEntry`] was likely deserialized from the database
    /// but if we have a correspond thread already loaded we want to apply the
    /// live information.
    fn apply_active_info(&mut self, info: &ActiveThreadInfo) {
        self.metadata.title = Some(info.title.clone());
        self.status = info.status;
        self.icon = info.icon;
        self.icon_from_external_svg = info.icon_from_external_svg.clone();
        self.is_live = true;
        self.is_background = info.is_background;
        self.is_title_generating = info.is_title_generating;
        self.diff_stats = info.diff_stats;
        self.changed_files = info.changed_files.clone();
    }

    fn review_brief(&self, cx: &App) -> RunReviewBrief {
        let state = if self.draft.is_some() {
            RunReviewState::Draft
        } else {
            match self.status {
                AgentThreadStatus::Completed => RunReviewState::Completed,
                AgentThreadStatus::Running => RunReviewState::Running,
                AgentThreadStatus::WaitingForConfirmation => RunReviewState::WaitingForPermission,
                AgentThreadStatus::Error => RunReviewState::Failed,
            }
        };
        let mut observed_risks = Vec::new();
        if !self.is_live && self.draft.is_none() {
            observed_risks.push(
                agent_session_label(
                    APP_NAME,
                    "The owning thread is not currently loaded; live runtime evidence may be stale.",
                    "The owning Agent Session is not currently loaded; live runtime evidence may be stale.",
                )
                .to_owned(),
            );
        }
        let (workspace_evidence, evidence_truncated, evidence_stale, evidence_unresolved) = self
            .workspace
            .authoritative_workspace_evidence(None, cx)
            .unwrap_or_else(|| {
                (
                    self.metadata
                        .folder_paths()
                        .paths()
                        .iter()
                        .cloned()
                        .map(|path| ObservedWorkspaceEvidence {
                            kind: WorkspaceEvidenceKind::WorkspaceRoot,
                            path,
                        })
                        .collect(),
                    false,
                    false,
                    false,
                )
            });
        if evidence_truncated {
            observed_risks.push("Workspace evidence is truncated.".to_owned());
        }
        if evidence_stale {
            observed_risks.push("Workspace evidence includes stale observations.".to_owned());
        }
        if evidence_unresolved {
            observed_risks.push("Workspace evidence includes unresolved observations.".to_owned());
        }
        let (repository_evidence, repository_evidence_truncated) = self
            .workspace
            .authoritative_repository_evidence(None, cx)
            .unwrap_or_default();
        if repository_evidence_truncated {
            observed_risks.push(
                "Repository evidence is truncated to the first eight observed worktrees."
                    .to_owned(),
            );
        }
        RunReviewBrief {
            run_label: self.metadata.display_title().to_string(),
            actor: if self.metadata.agent_id.as_ref() == ZED_AGENT_ID.as_ref() {
                "Dez Agent".to_owned()
            } else {
                self.metadata.agent_id.as_ref().to_owned()
            },
            state,
            host: self
                .metadata
                .remote_connection
                .as_ref()
                .map(|_| {
                    "Remote host (connection details remain in the owning workspace)".to_owned()
                })
                .or_else(|| Some("Local host (no remote connection observed)".to_owned())),
            session: self.metadata.session_id.as_ref().map(ToString::to_string),
            workspace_evidence,
            repository_evidence,
            lines_added: u64::from(self.diff_stats.lines_added),
            lines_removed: u64::from(self.diff_stats.lines_removed),
            changed_files: self.changed_files.clone(),
            file_targets: Vec::new(),
            file_targets_truncated: false,
            activity: Vec::new(),
            commands: Vec::new(),
            checks: Vec::new(),
            observed_risks,
        }
    }
}

impl TerminalEntry {
    fn review_brief(&self, cx: &App) -> RunReviewBrief {
        let state = terminal_run_review_state(self.agent.as_ref(), self.runtime.as_ref());
        let commands = self
            .agent
            .iter()
            .filter(|agent| agent.capabilities.command_evidence)
            .flat_map(|agent| &agent.events)
            .filter(|event| event.kind == TerminalAgentEventKind::ToolFinished)
            .filter_map(|event| {
                event.command.as_ref().map(|command| ObservedRunCommand {
                    command: command.clone(),
                    exit_code: event.exit_code,
                    source_path: event.working_directory.clone(),
                })
            })
            .collect::<Vec<_>>();
        let checks = if self
            .agent
            .as_ref()
            .is_some_and(|agent| agent.capabilities.check_results)
        {
            commands
                .iter()
                .filter_map(ObservedRunCheck::from_command)
                .collect()
        } else {
            Vec::new()
        };
        let activity = self
            .agent
            .iter()
            .filter(|agent| agent.capabilities.activity_events)
            .flat_map(|agent| &agent.events)
            .map(|event| ObservedRunActivity {
                sequence: event.sequence,
                summary: event.summary.clone(),
                source_path: event.working_directory.clone(),
            })
            .collect();
        let file_targets = self
            .agent
            .iter()
            .filter(|agent| agent.capabilities.file_targets)
            .flat_map(|agent| &agent.events)
            .flat_map(|event| event.file_targets.iter().cloned())
            .collect::<Vec<_>>();
        let file_targets_truncated = self.agent.as_ref().is_some_and(|agent| {
            agent.capabilities.file_targets
                && agent
                    .events
                    .iter()
                    .any(|event| event.file_targets_truncated)
        });
        let mut observed_risks = Vec::new();
        if self.agent.is_none() {
            observed_risks.push(
                "This terminal adapter has not observed structured command, check, or file-change events."
                    .to_owned(),
            );
        } else if let Some(agent) = &self.agent {
            if agent.events_truncated {
                observed_risks.push(
                    "Earlier structured activity was evicted from the Host's bounded history; retained commands, checks, and file targets are partial."
                        .to_owned(),
                );
            }
            if !agent.capabilities.command_evidence {
                observed_risks
                    .push("The adapter does not advertise structured command evidence.".to_owned());
            }
            if !agent.capabilities.check_results {
                observed_risks
                    .push("The adapter does not advertise structured check results.".to_owned());
            } else {
                observed_risks.push(
                    "Only recognized validation commands with observed exit codes count as checks."
                        .to_owned(),
                );
            }
            if state == RunReviewState::WaitingForPermission
                && !agent.capabilities.permission_responses
            {
                observed_risks.push(
                    "The adapter reports a permission request but does not provide a scoped, auditable response contract; respond in the owning terminal."
                        .to_owned(),
                );
            }
            if state == RunReviewState::WaitingForInput && !agent.capabilities.input_responses {
                observed_risks.push(
                    "The adapter reports an input request but cannot submit bounded provider input; respond in the owning terminal."
                        .to_owned(),
                );
            }
            if !agent.capabilities.file_targets {
                observed_risks.push(
                    "The adapter does not provide structured file-target evidence.".to_owned(),
                );
            } else if file_targets.is_empty() {
                observed_risks.push(
                    "No structured file target was observed; this does not prove the Run touched no files."
                        .to_owned(),
                );
            }
        }
        match self.runtime.as_ref().map(|runtime| runtime.state) {
            Some(TerminalRuntimeState::Detached) => observed_risks.push(
                "The Host owns this detached session; the current surface is not receiving live output."
                    .to_owned(),
            ),
            Some(TerminalRuntimeState::Reconnecting) => observed_risks.push(
                "The Host connection is recovering; agent evidence may not include the latest activity."
                    .to_owned(),
            ),
            Some(TerminalRuntimeState::Missing) => observed_risks.push(
                "The Host cannot find this session; no replacement computation was started."
                    .to_owned(),
            ),
            Some(TerminalRuntimeState::Incompatible) => observed_risks.push(
                "The Host session uses an incompatible protocol; no replacement computation was started."
                    .to_owned(),
            ),
            Some(TerminalRuntimeState::Exited) => observed_risks.push(
                "The terminal process has exited; no newer terminal evidence will arrive."
                    .to_owned(),
            ),
            Some(TerminalRuntimeState::Live) | None => {}
        }
        if self.needs_attention {
            observed_risks.push(
                "The terminal has an active attention condition; opening it only acknowledges the presentation and does not resolve the condition."
                    .to_owned(),
            );
        }
        let terminal_session_id = self
            .metadata
            .session_ref
            .map(|session_ref| session_ref.session_id.to_string());
        let (mut workspace_evidence, evidence_truncated, evidence_stale, evidence_unresolved) =
            self.workspace
                .authoritative_workspace_evidence(terminal_session_id.as_deref(), cx)
                .unwrap_or_else(|| {
                    (
                        self.metadata
                            .worktree_paths
                            .folder_path_list()
                            .paths()
                            .iter()
                            .cloned()
                            .map(|path| ObservedWorkspaceEvidence {
                                kind: WorkspaceEvidenceKind::WorkspaceRoot,
                                path,
                            })
                            .collect::<Vec<_>>(),
                        false,
                        false,
                        false,
                    )
                });
        if evidence_truncated {
            observed_risks.push("Workspace evidence is truncated.".to_owned());
        }
        if evidence_stale {
            observed_risks
                .push("The owning terminal's working-directory evidence is stale.".to_owned());
        }
        if evidence_unresolved {
            observed_risks.push(
                "The owning terminal's working-directory evidence is unresolved until the saved Session reattaches."
                    .to_owned(),
            );
        }
        if let Some(path) = self.metadata.working_directory.clone() {
            if !workspace_evidence.iter().any(|evidence| {
                evidence.kind == WorkspaceEvidenceKind::TerminalWorkingDirectory
                    && evidence.path == path
            }) {
                workspace_evidence.push(ObservedWorkspaceEvidence {
                    kind: WorkspaceEvidenceKind::TerminalWorkingDirectory,
                    path,
                });
            }
        }
        let (repository_evidence, repository_evidence_truncated) = self
            .workspace
            .authoritative_repository_evidence(self.metadata.working_directory.as_deref(), cx)
            .unwrap_or_default();
        if repository_evidence.is_empty() {
            observed_risks.push(
                "No Git repository owning the terminal working directory was observed.".to_owned(),
            );
        }
        if repository_evidence_truncated {
            observed_risks.push("Repository evidence is truncated.".to_owned());
        }
        RunReviewBrief {
            run_label: self.metadata.display_title().to_string(),
            actor: self.agent.as_ref().map_or_else(
                || {
                    self.detected_agent_kind
                        .map(|kind| kind.display_name().to_owned())
                        .unwrap_or_else(|| "Terminal process".to_owned())
                },
                |agent| agent.actor.clone(),
            ),
            state,
            host: self
                .metadata
                .session_ref
                .map(|session_ref| session_ref.host_id.to_string())
                .or_else(|| {
                    self.metadata
                        .remote_connection
                        .as_ref()
                        .map(|_| "Remote host (identity retained by workspace)".to_owned())
                }),
            session: self
                .metadata
                .session_ref
                .map(|session_ref| session_ref.session_id.to_string()),
            workspace_evidence,
            repository_evidence,
            lines_added: 0,
            lines_removed: 0,
            changed_files: Vec::new(),
            file_targets,
            file_targets_truncated,
            activity,
            commands,
            checks,
            observed_risks,
        }
    }
}

#[derive(Clone)]
enum ListEntry {
    ProjectHeader {
        key: ProjectGroupKey,
        label: SharedString,
        highlight_positions: Vec<usize>,
        layout_label: Option<SharedString>,
        has_running_threads: bool,
        attention_thread_count: usize,
        has_notifications: bool,
        is_active: bool,
        has_threads: bool,
    },
    Thread(Arc<ThreadEntry>),
    Terminal(TerminalEntry),
}

#[derive(Clone)]
enum ActivatableEntry {
    Thread {
        metadata: ThreadMetadata,
        workspace: ThreadEntryWorkspace,
    },
    Terminal {
        metadata: TerminalThreadMetadata,
        workspace: ThreadEntryWorkspace,
        source: TerminalEntrySource,
    },
}

impl ActivatableEntry {
    fn from_list_entry(entry: &ListEntry) -> Option<Self> {
        match entry {
            ListEntry::Thread(thread) => Some(Self::Thread {
                metadata: thread.metadata.clone(),
                workspace: thread.workspace.clone(),
            }),
            ListEntry::Terminal(terminal) => Some(Self::Terminal {
                metadata: terminal.metadata.clone(),
                workspace: terminal.workspace.clone(),
                source: terminal.source.clone(),
            }),
            ListEntry::ProjectHeader { .. } => None,
        }
    }

    fn project_location(&self, cx: &App) -> (PathList, ProjectGroupKey) {
        match self {
            Self::Thread {
                workspace: ThreadEntryWorkspace::Open(workspace),
                ..
            }
            | Self::Terminal {
                workspace: ThreadEntryWorkspace::Open(workspace),
                ..
            } => (
                PathList::new(&workspace.read(cx).root_paths(cx)),
                workspace.read(cx).project_group_key(cx),
            ),
            Self::Thread {
                workspace:
                    ThreadEntryWorkspace::Closed {
                        folder_paths,
                        project_group_key,
                    },
                ..
            }
            | Self::Terminal {
                workspace:
                    ThreadEntryWorkspace::Closed {
                        folder_paths,
                        project_group_key,
                    },
                ..
            } => (folder_paths.clone(), project_group_key.clone()),
        }
    }
}

#[cfg(test)]
impl ListEntry {
    fn session_id(&self) -> Option<&acp::SessionId> {
        match self {
            ListEntry::Thread(thread_entry) => thread_entry.metadata.session_id.as_ref(),
            ListEntry::Terminal(_) | ListEntry::ProjectHeader { .. } => None,
        }
    }

    fn reachable_workspaces<'a>(
        &'a self,
        multi_workspace: &'a workspace::MultiWorkspace,
        cx: &'a App,
    ) -> Vec<Entity<Workspace>> {
        match self {
            ListEntry::Thread(thread) => match &thread.workspace {
                ThreadEntryWorkspace::Open(ws) => vec![ws.clone()],
                ThreadEntryWorkspace::Closed { .. } => Vec::new(),
            },
            ListEntry::Terminal(terminal) => match &terminal.workspace {
                ThreadEntryWorkspace::Open(workspace) => vec![workspace.clone()],
                ThreadEntryWorkspace::Closed { .. } => Vec::new(),
            },
            ListEntry::ProjectHeader { key, .. } => multi_workspace
                .workspaces_for_project_group(key, cx)
                .unwrap_or_default(),
        }
    }
}

impl From<ThreadEntry> for ListEntry {
    fn from(thread: ThreadEntry) -> Self {
        ListEntry::Thread(Arc::new(thread))
    }
}

impl From<TerminalEntry> for ListEntry {
    fn from(terminal: TerminalEntry) -> Self {
        ListEntry::Terminal(terminal)
    }
}

#[derive(Default)]
struct SidebarContents {
    entries: Vec<ListEntry>,
    notified_threads: HashSet<agent_ui::ThreadId>,
    notified_terminals: HashSet<TerminalId>,
    project_header_indices: Vec<usize>,
    has_open_projects: bool,
    has_attention: bool,
    session_count: usize,
    attention_count: usize,
}

/// Identity-and-layout key for a [`ListEntry`] used to preserve measured list items
/// across rebuilds. Equal shapes must render to the same height; add any new
/// height-affecting state here.
#[derive(Debug, PartialEq, Eq)]
enum EntryShape {
    ProjectHeader {
        key: ProjectGroupKey,
        // Toggles the "No sessions yet" empty-state row when not collapsed.
        has_threads: bool,
        // Determines whether the "No sessions yet" row is rendered (only shown when
        // `!is_collapsed && !has_threads`).
        is_collapsed: bool,
    },
    Thread(ThreadId),
    Terminal(TerminalId),
}

/// Stable identity for the keyboard-selected row. Rebuilds may reorder or
/// filter the rail, so retaining the previous numeric index can silently move
/// focus to a different session.
#[derive(Clone, Debug, PartialEq, Eq)]
enum SelectedEntryKey {
    ProjectHeader(ProjectGroupKey),
    Thread(ThreadId),
    Terminal(TerminalId),
}

impl SelectedEntryKey {
    fn from_entry(entry: &ListEntry) -> Self {
        match entry {
            ListEntry::ProjectHeader { key, .. } => Self::ProjectHeader(key.clone()),
            ListEntry::Thread(thread) => Self::Thread(thread.metadata.thread_id),
            ListEntry::Terminal(terminal) => Self::Terminal(terminal.metadata.terminal_id),
        }
    }

    fn matches(&self, entry: &ListEntry) -> bool {
        match (self, entry) {
            (Self::ProjectHeader(selected), ListEntry::ProjectHeader { key, .. }) => {
                selected == key
            }
            (Self::Thread(selected), ListEntry::Thread(thread)) => {
                *selected == thread.metadata.thread_id
            }
            (Self::Terminal(selected), ListEntry::Terminal(terminal)) => {
                *selected == terminal.metadata.terminal_id
            }
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "id", rename_all = "snake_case")]
enum ManualEntryOrderKey {
    Thread(String),
    Terminal(String),
}

impl ManualEntryOrderKey {
    fn from_entry(entry: &ListEntry) -> Option<Self> {
        match entry {
            ListEntry::Thread(thread) => {
                Some(Self::Thread(thread.metadata.thread_id.to_key_string()))
            }
            ListEntry::Terminal(terminal) => {
                Some(Self::Terminal(terminal.metadata.terminal_id.to_string()))
            }
            ListEntry::ProjectHeader { .. } => None,
        }
    }
}

impl SidebarContents {
    fn is_thread_notified(&self, thread_id: &agent_ui::ThreadId) -> bool {
        self.notified_threads.contains(thread_id)
    }

    fn is_terminal_notified(&self, terminal_id: TerminalId) -> bool {
        self.notified_terminals.contains(&terminal_id)
    }
}

fn thread_needs_attention(
    thread: &ThreadEntry,
    notified_threads: &HashSet<agent_ui::ThreadId>,
) -> bool {
    notified_threads.contains(&thread.metadata.thread_id)
        || agent_status_needs_attention(thread.status)
}

fn agent_status_needs_attention(status: AgentThreadStatus) -> bool {
    matches!(
        status,
        AgentThreadStatus::WaitingForConfirmation | AgentThreadStatus::Error
    )
}

#[cfg(test)]
mod attention_state_tests {
    use super::*;

    #[test]
    fn waiting_and_error_states_need_attention() {
        assert!(agent_status_needs_attention(
            AgentThreadStatus::WaitingForConfirmation
        ));
        assert!(agent_status_needs_attention(AgentThreadStatus::Error));
        assert!(!agent_status_needs_attention(AgentThreadStatus::Running));
        assert!(!agent_status_needs_attention(AgentThreadStatus::Completed));
    }
}

fn entry_needs_attention(
    entry: &ListEntry,
    notified_threads: &HashSet<agent_ui::ThreadId>,
    notified_terminals: &HashSet<TerminalId>,
) -> bool {
    match entry {
        ListEntry::ProjectHeader {
            attention_thread_count,
            has_notifications,
            ..
        } => *attention_thread_count > 0 || *has_notifications,
        ListEntry::Thread(thread) => thread_needs_attention(thread, notified_threads),
        ListEntry::Terminal(terminal) => {
            terminal.needs_attention || notified_terminals.contains(&terminal.metadata.terminal_id)
        }
    }
}

fn attention_entries(
    entries: Vec<ListEntry>,
    notified_threads: &HashSet<agent_ui::ThreadId>,
    notified_terminals: &HashSet<TerminalId>,
) -> (Vec<ListEntry>, Vec<usize>) {
    fn flush_group(
        filtered: &mut Vec<ListEntry>,
        header: &mut Option<ListEntry>,
        rows: &mut Vec<ListEntry>,
        notified_threads: &HashSet<agent_ui::ThreadId>,
        notified_terminals: &HashSet<TerminalId>,
    ) {
        let show_header = header.as_ref().is_some_and(|header| {
            entry_needs_attention(header, notified_threads, notified_terminals)
        }) || !rows.is_empty();
        if show_header && let Some(header) = header.take() {
            filtered.push(header);
            filtered.append(rows);
        } else {
            header.take();
            rows.clear();
        }
    }

    let mut filtered = Vec::new();
    let mut header = None;
    let mut rows = Vec::new();

    for entry in entries {
        if matches!(entry, ListEntry::ProjectHeader { .. }) {
            flush_group(
                &mut filtered,
                &mut header,
                &mut rows,
                notified_threads,
                notified_terminals,
            );
            header = Some(entry);
        } else if entry_needs_attention(&entry, notified_threads, notified_terminals) {
            rows.push(entry);
        }
    }
    flush_group(
        &mut filtered,
        &mut header,
        &mut rows,
        notified_threads,
        notified_terminals,
    );

    let project_header_indices = filtered
        .iter()
        .enumerate()
        .filter_map(|(index, entry)| {
            matches!(entry, ListEntry::ProjectHeader { .. }).then_some(index)
        })
        .collect();
    (filtered, project_header_indices)
}

// TODO: The mapping from workspace root paths to git repositories needs a
// unified approach across the codebase: this function, `AgentPanel::classify_worktrees`,
// thread persistence (which PathList is saved to the database), and thread
// querying (which PathList is used to read threads back). All of these need
// to agree on how repos are resolved for a given workspace, especially in
// multi-root and nested-repo configurations.
fn root_repository_snapshots(
    workspace: &Entity<Workspace>,
    cx: &App,
) -> impl Iterator<Item = project::git_store::RepositorySnapshot> {
    let path_list = workspace_path_list(workspace, cx);
    let project = workspace.read(cx).project().read(cx);
    project.repositories(cx).values().filter_map(move |repo| {
        let snapshot = repo.read(cx).snapshot();
        let is_root = path_list
            .paths()
            .iter()
            .any(|p| p.as_path() == snapshot.work_directory_abs_path.as_ref());
        is_root.then_some(snapshot)
    })
}

fn workspace_path_list(workspace: &Entity<Workspace>, cx: &App) -> PathList {
    PathList::new(&workspace.read(cx).root_paths(cx))
}

fn linked_worktree_path_lists_for_workspaces(
    workspaces: &[Entity<Workspace>],
    cx: &App,
) -> Vec<PathList> {
    let mut linked_worktree_paths = Vec::new();
    for workspace in workspaces {
        if workspace.read(cx).visible_worktrees(cx).count() != 1 {
            continue;
        }
        for snapshot in root_repository_snapshots(workspace, cx) {
            linked_worktree_paths.extend(
                snapshot.linked_worktrees().iter().map(|linked_worktree| {
                    PathList::new(std::slice::from_ref(&linked_worktree.path))
                }),
            );
        }
    }

    linked_worktree_paths.sort_by(|a, b| a.paths()[0].cmp(&b.paths()[0]));
    linked_worktree_paths
}

fn workspace_has_terminal_metadata_except(
    workspace: &Entity<Workspace>,
    except_terminal_id: Option<TerminalId>,
    cx: &App,
) -> bool {
    let Some(store) = TerminalThreadMetadataStore::try_global(cx) else {
        return false;
    };
    let path_list = workspace_path_list(workspace, cx);
    let remote_connection = workspace
        .read(cx)
        .project()
        .read(cx)
        .remote_connection_options(cx);
    store
        .read(cx)
        .entries_for_path(&path_list, remote_connection.as_ref())
        .any(|terminal| except_terminal_id != Some(terminal.terminal_id))
}

#[derive(Clone)]
struct WorkspaceMenuWorktreeLabel {
    icon: Option<IconName>,
    primary_name: SharedString,
    secondary_name: Option<SharedString>,
}

impl WorkspaceMenuWorktreeLabel {
    fn render(&self) -> impl IntoElement {
        h_flex()
            .min_w_0()
            .gap_0p5()
            .when_some(self.icon, |this, icon| {
                this.child(Icon::new(icon).size(IconSize::XSmall).color(Color::Muted))
            })
            .child(Label::new(self.primary_name.clone()).truncate())
            .when_some(self.secondary_name.clone(), |this, secondary_name| {
                this.child(Label::new("/").alpha(0.5))
                    .child(Label::new(secondary_name).truncate())
            })
    }
}

fn workspace_menu_worktree_accessible_name(labels: &[WorkspaceMenuWorktreeLabel]) -> String {
    labels
        .iter()
        .map(|label| match &label.secondary_name {
            Some(secondary_name) => {
                format!(
                    "{} / {}",
                    label.primary_name.as_ref(),
                    secondary_name.as_ref()
                )
            }
            None => label.primary_name.as_ref().to_owned(),
        })
        .join(" • ")
}

fn workspace_menu_worktree_labels(
    workspace: &Entity<Workspace>,
    cx: &App,
) -> Vec<WorkspaceMenuWorktreeLabel> {
    let root_paths = workspace.read(cx).root_paths(cx);
    let show_folder_name = root_paths.len() > 1;
    let project = workspace.read(cx).project().clone();
    let repository_snapshots: Vec<_> = project
        .read(cx)
        .repositories(cx)
        .values()
        .map(|repo| repo.read(cx).snapshot())
        .collect();

    root_paths
        .into_iter()
        .map(|root_path| {
            let root_path = root_path.as_ref();
            let folder_name = root_path
                .file_name()
                .map(|name| SharedString::from(name.to_string_lossy().to_string()))
                .unwrap_or_default();
            let repository_snapshot = repository_snapshots
                .iter()
                .find(|snapshot| snapshot.work_directory_abs_path.as_ref() == root_path);

            if let Some(snapshot) = repository_snapshot {
                let worktree_name = if snapshot.is_linked_worktree() {
                    snapshot
                        .main_worktree_abs_path()
                        .and_then(|main_worktree_path| {
                            project::linked_worktree_short_name(main_worktree_path, root_path)
                        })
                        .unwrap_or_else(|| folder_name.clone())
                } else {
                    "main".into()
                };

                if show_folder_name {
                    WorkspaceMenuWorktreeLabel {
                        icon: Some(IconName::GitWorktree),
                        primary_name: folder_name,
                        secondary_name: Some(worktree_name),
                    }
                } else {
                    WorkspaceMenuWorktreeLabel {
                        icon: Some(IconName::GitWorktree),
                        primary_name: worktree_name,
                        secondary_name: None,
                    }
                }
            } else {
                WorkspaceMenuWorktreeLabel {
                    icon: None,
                    primary_name: folder_name,
                    secondary_name: None,
                }
            }
        })
        .collect()
}

fn apply_worktree_label_mode(
    mut worktrees: Vec<ThreadItemWorktreeInfo>,
    mode: AgentThreadWorktreeLabel,
) -> Vec<ThreadItemWorktreeInfo> {
    match mode {
        AgentThreadWorktreeLabel::Both => {}
        AgentThreadWorktreeLabel::Worktree => {
            for wt in &mut worktrees {
                wt.branch_name = None;
            }
        }
        AgentThreadWorktreeLabel::Branch => {
            for wt in &mut worktrees {
                // Fall back to showing the worktree name when no branch is
                // known; an empty chip would be worse than a mismatched icon.
                if wt.branch_name.is_some() {
                    wt.worktree_name = None;
                }
            }
        }
    }
    worktrees
}

/// Shows a [`RemoteConnectionModal`] on the given workspace and establishes
/// an SSH connection. Suitable for passing to
/// [`MultiWorkspace::find_or_create_workspace`] as the `connect_remote`
/// argument.
fn connect_remote(
    modal_workspace: Entity<Workspace>,
    connection_options: RemoteConnectionOptions,
    window: &mut Window,
    cx: &mut Context<MultiWorkspace>,
) -> gpui::Task<anyhow::Result<Option<Entity<remote::RemoteClient>>>> {
    remote_connection::connect_with_modal(&modal_workspace, connection_options, window, cx)
}

// Per-project-group cache of the remote default branch, used to populate the
// "Create New Worktree" submenu without doing git I/O while the menu is open.
enum DefaultBranchCache {
    Pending,
    Resolved(Option<RemoteBranchName>),
}

// Mirrors the behavior of the worktree picker's "Create new worktree" entries.
fn create_worktree_in_workspace(
    workspace: &Entity<Workspace>,
    branch_target: NewWorktreeBranchTarget,
    window: &mut Window,
    cx: &mut App,
) {
    workspace.update(cx, |workspace, cx| {
        let focused_dock = workspace.focused_dock_position(window, cx);
        git_ui::worktree_service::handle_create_worktree(
            workspace,
            &CreateWorktree {
                worktree_name: None,
                branch_target,
            },
            window,
            focused_dock,
            cx,
        );
    });
}

/// The sidebar re-derives its entire entry list from scratch on every
/// change via `update_entries` → `rebuild_contents`. Avoid adding
/// incremental or inter-event coordination state — if something can
/// be computed from the current world state, compute it in the rebuild.
pub struct Sidebar {
    multi_workspace: WeakEntity<MultiWorkspace>,
    width: Pixels,
    focus_handle: FocusHandle,
    filter_editor: Entity<Editor>,
    thread_rename_editor: Entity<Editor>,
    list_state: ListState,
    contents: SidebarContents,
    /// A transient projection over the session list. Authoritative session and
    /// agent state remains in the existing stores and owning surfaces.
    attention_only: bool,
    /// The index of the list item that currently has the keyboard focus
    ///
    /// Note: This is NOT the same as the active item.
    selection: Option<usize>,
    /// Tracks which sidebar entry is currently active (highlighted).
    active_entry: Option<ActiveEntry>,
    hovered_thread_index: Option<usize>,
    renaming_thread_id: Option<ThreadId>,
    /// Threads in the database-backed regeneration path need their own loading
    /// state because they do not have a live `agent::Thread` to report it.
    regenerating_titles: HashSet<ThreadId>,
    /// start_renaming_thread must seed current title into the title editor
    /// so this prevents that BufferEdited event from being interpreted as user input.
    suppress_next_rename_edit: bool,

    /// Updated only in response to explicit user actions (clicking a
    /// thread, confirming in the thread switcher, etc.) — never from
    /// background data changes. Used to sort the thread switcher popup.
    thread_last_accessed: HashMap<ThreadId, DateTime<Utc>>,
    terminal_last_accessed: HashMap<TerminalId, DateTime<Utc>>,
    manual_entry_order: Vec<ManualEntryOrderKey>,
    standalone_terminal_created_at: HashMap<TerminalId, DateTime<Utc>>,
    thread_switcher: Option<Entity<ThreadSwitcher>>,
    _thread_switcher_subscriptions: Vec<gpui::Subscription>,
    pending_thread_activation: Option<agent_ui::ThreadId>,
    /// Structured Host sessions currently carrying an active attention
    /// condition. This is transient transition memory only; the Host snapshot
    /// remains authoritative.
    host_attention_sessions: HashSet<TerminalSessionId>,
    /// Persists live thread statuses across rebuilds so that Running→Completed
    /// transitions can be detected even when the group is collapsed (and
    /// thread entries are not present in the list).
    live_thread_statuses: HashMap<acp::SessionId, (AgentThreadStatus, ThreadId)>,
    /// Remembers whether each draft last rendered as empty or with content so
    /// that when a draft that was empty gains content again, we refresh
    /// its interaction time.
    draft_kinds: HashMap<ThreadId, DraftKind>,
    view: SidebarView,
    restoring_tasks: HashMap<agent_ui::ThreadId, Task<()>>,
    agent_options_menu_handle: PopoverMenuHandle<ContextMenu>,
    recent_projects_popover_handle: PopoverMenuHandle<SidebarRecentProjects>,
    sidebar_chrome: Entity<title_bar::SidebarChrome>,
    project_header_menu_handles: HashMap<usize, PopoverMenuHandle<ContextMenu>>,
    project_header_new_thread_menu_handles: HashMap<usize, PopoverMenuHandle<ContextMenu>>,
    project_header_menu_ix: Option<usize>,
    worktree_default_branches: HashMap<ProjectGroupKey, DefaultBranchCache>,
    _subscriptions: Vec<gpui::Subscription>,
    _draft_editor_observations: Vec<gpui::Subscription>,
    update_task: Option<Task<()>>,
    /// For the thread import banners, if there is just one we show "Import
    /// Threads" but if we are showing both the external agents and other
    /// channels import banners then we change the text to disambiguate the
    /// buttons. This field tracks whether we were using verbose labels so they
    /// can stay stable after dismissing one of the banners.
    import_banners_use_verbose_labels: Option<bool>,
    /// Display names of other release channels that have threads available to
    /// import.
    cross_channel_import_channels: Vec<SharedString>,
}

impl Sidebar {
    pub fn new(
        multi_workspace: Entity<MultiWorkspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        cx.on_focus_in(&focus_handle, window, Self::focus_in)
            .detach();

        AgentThreadWorktreeLabelFlag::watch(cx);

        let filter_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Search sessions…", window, cx);
            editor
        });
        let thread_rename_editor = cx.new(|cx| Editor::single_line(window, cx));
        let sidebar_chrome = cx.new(|cx| {
            let workspace = multi_workspace.read(cx).workspace().clone();
            title_bar::SidebarChrome::new(
                "sidebar-title-bar-controls",
                workspace,
                Some(multi_workspace.downgrade()),
                window,
                cx,
            )
        });

        cx.subscribe_in(
            &multi_workspace,
            window,
            |this, _multi_workspace, event: &MultiWorkspaceEvent, window, cx| match event {
                MultiWorkspaceEvent::ActiveWorkspaceChanged { .. } => {
                    let workspace = _multi_workspace.read(cx).workspace().clone();
                    this.sidebar_chrome = cx.new(|cx| {
                        title_bar::SidebarChrome::new(
                            "sidebar-title-bar-controls",
                            workspace,
                            Some(_multi_workspace.downgrade()),
                            window,
                            cx,
                        )
                    });
                    this.sync_active_entry_from_active_workspace(cx);
                    this.replace_archived_panel_thread(window, cx);
                    this.schedule_update_entries(false, cx);
                }
                MultiWorkspaceEvent::WorkspaceAdded(workspace) => {
                    this.subscribe_to_workspace(workspace, window, cx);
                    this.schedule_update_entries(false, cx);
                }
                MultiWorkspaceEvent::WorkspaceRemoved(_)
                | MultiWorkspaceEvent::ProjectGroupsChanged => {
                    this.schedule_update_entries(false, cx);
                }
            },
        )
        .detach();

        cx.subscribe(&filter_editor, |this: &mut Self, _, event, cx| {
            if let editor::EditorEvent::BufferEdited = event {
                let query = this.filter_editor.read(cx).text(cx);
                if !query.is_empty() {
                    this.selection.take();
                }
                this.schedule_update_entries(!query.is_empty(), cx);
            }
        })
        .detach();

        cx.subscribe_in(
            &thread_rename_editor,
            window,
            |this, title_editor, event, window, cx| {
                this.handle_thread_rename_editor_event(title_editor, event, window, cx);
            },
        )
        .detach();

        cx.observe(&ThreadMetadataStore::global(cx), |this, _store, cx| {
            this.schedule_update_entries(false, cx);
        })
        .detach();

        cx.observe(
            &TerminalThreadMetadataStore::global(cx),
            |this, _store, cx| {
                this.schedule_update_entries(false, cx);
            },
        )
        .detach();

        TerminalHostSnapshotRevision::init(cx);
        TerminalHostStartupStatus::init(cx);
        cx.observe_global_in::<TerminalHostSnapshotRevision>(window, |this, window, cx| {
            let current_attention_sessions = TerminalHostSnapshotStore::try_global(cx)
                .map(|store| {
                    store
                        .read(cx)
                        .snapshots()
                        .iter()
                        .filter(|snapshot| {
                            snapshot
                                .agent
                                .as_ref()
                                .is_some_and(|agent| agent.attention_required)
                        })
                        .map(|snapshot| snapshot.session_id)
                        .collect::<HashSet<_>>()
                })
                .unwrap_or_default();
            let has_new_attention = current_attention_sessions
                .iter()
                .any(|session_id| !this.host_attention_sessions.contains(session_id));
            this.host_attention_sessions = current_attention_sessions;
            if has_new_attention && CanvasAgentUiSettings::get_global(cx).announce_agent_attention {
                window.request_attention();
            }
            this.schedule_update_entries(false, cx);
        })
        .detach();
        cx.observe_global::<TerminalHostStartupStatus>(|this, cx| {
            this.schedule_update_entries(false, cx);
        })
        .detach();

        let app_session = multi_workspace
            .read(cx)
            .workspace()
            .read(cx)
            .app_state()
            .session
            .clone();
        cx.subscribe(
            &app_session,
            |_this, _session, _event: &AppSessionEvent, cx| {
                cx.notify();
            },
        )
        .detach();

        if APP_NAME == "Zed" {
            let channels_with_threads = channels_with_threads(cx);
            cx.spawn(async move |this, cx| {
                let channels = channels_with_threads.await;
                this.update(cx, |this, cx| {
                    this.cross_channel_import_channels = channels;
                    cx.notify();
                })
                .ok();
            })
            .detach();
        }

        let deferred_multi_workspace = multi_workspace.downgrade();
        cx.defer_in(window, move |this, window, cx| {
            if let Some(multi_workspace) = deferred_multi_workspace.upgrade() {
                let workspaces: Vec<_> = multi_workspace.read(cx).workspaces().cloned().collect();
                for workspace in &workspaces {
                    this.subscribe_to_workspace(workspace, window, cx);
                }
            }
            this.schedule_update_entries(false, cx);
        });

        Self {
            multi_workspace: multi_workspace.downgrade(),
            width: DEFAULT_WIDTH,
            focus_handle,
            filter_editor,
            thread_rename_editor,
            list_state: ListState::new(0, gpui::ListAlignment::Top, px(1000.)),
            contents: SidebarContents::default(),
            attention_only: false,
            selection: None,
            active_entry: None,
            hovered_thread_index: None,
            renaming_thread_id: None,
            regenerating_titles: HashSet::new(),
            suppress_next_rename_edit: false,

            thread_last_accessed: HashMap::new(),
            terminal_last_accessed: HashMap::new(),
            manual_entry_order: Vec::new(),
            standalone_terminal_created_at: HashMap::new(),
            thread_switcher: None,
            _thread_switcher_subscriptions: Vec::new(),
            pending_thread_activation: None,
            host_attention_sessions: HashSet::new(),
            live_thread_statuses: HashMap::new(),
            draft_kinds: HashMap::new(),
            view: SidebarView::default(),
            restoring_tasks: HashMap::new(),
            agent_options_menu_handle: PopoverMenuHandle::default(),
            recent_projects_popover_handle: PopoverMenuHandle::default(),
            sidebar_chrome,
            project_header_menu_handles: HashMap::new(),
            project_header_new_thread_menu_handles: HashMap::new(),
            project_header_menu_ix: None,
            worktree_default_branches: HashMap::new(),
            _subscriptions: Vec::new(),
            _draft_editor_observations: Vec::new(),
            update_task: None,
            import_banners_use_verbose_labels: None,
            cross_channel_import_channels: Vec::new(),
        }
    }

    fn serialize(&mut self, cx: &mut Context<Self>) {
        cx.emit(workspace::SidebarEvent::SerializeNeeded);
    }

    fn is_group_collapsed(&self, key: &ProjectGroupKey, cx: &App) -> bool {
        self.multi_workspace
            .upgrade()
            .and_then(|mw| {
                mw.read(cx)
                    .group_state_by_key(key)
                    .map(|state| !state.expanded)
            })
            .unwrap_or(false)
    }

    fn set_group_expanded(&self, key: &ProjectGroupKey, expanded: bool, cx: &mut Context<Self>) {
        if let Some(mw) = self.multi_workspace.upgrade() {
            mw.update(cx, |mw, cx| {
                if let Some(state) = mw.group_state_by_key_mut(key) {
                    state.expanded = expanded;
                }
                mw.serialize(cx);
            });
        }
    }

    fn is_active_workspace(&self, workspace: &Entity<Workspace>, cx: &App) -> bool {
        self.multi_workspace
            .upgrade()
            .map_or(false, |mw| mw.read(cx).workspace() == workspace)
    }

    fn subscribe_to_workspace(
        &mut self,
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let project = workspace.read(cx).project().clone();
        if project.read(cx).is_via_collab() {
            return;
        }

        cx.subscribe_in(
            &project,
            window,
            |this, project, event, _window, cx| match event {
                ProjectEvent::WorktreeAdded(_)
                | ProjectEvent::WorktreeRemoved(_)
                | ProjectEvent::WorktreeOrderChanged => {
                    this.schedule_update_entries(false, cx);
                }
                ProjectEvent::WorktreePathsChanged { old_worktree_paths } => {
                    this.move_entry_paths(project, old_worktree_paths, cx);
                    this.schedule_update_entries(false, cx);
                }
                _ => {}
            },
        )
        .detach();

        let git_store = workspace.read(cx).project().read(cx).git_store().clone();
        cx.subscribe_in(
            &git_store,
            window,
            |this, _, event: &project::git_store::GitStoreEvent, _window, cx| {
                if matches!(
                    event,
                    project::git_store::GitStoreEvent::RepositoryUpdated(
                        _,
                        project::git_store::RepositoryEvent::GitWorktreeListChanged
                            | project::git_store::RepositoryEvent::HeadChanged,
                        _,
                    )
                ) {
                    this.schedule_update_entries(false, cx);
                }
            },
        )
        .detach();

        cx.subscribe_in(
            workspace,
            window,
            move |this, workspace, event: &workspace::Event, window, cx| match event {
                workspace::Event::ActiveItemChanged
                | workspace::Event::ItemAdded { .. }
                | workspace::Event::ItemRemoved { .. } => {
                    this.sync_active_entry_from_active_workspace(cx);
                    this.schedule_update_entries(false, cx);
                }
                workspace::Event::PanelAdded(view) => {
                    if let Ok(agent_panel) = view.clone().downcast::<AgentPanel>() {
                        this.subscribe_to_agent_panel(workspace, &agent_panel, window, cx);
                        this.schedule_update_entries(false, cx);
                    }
                }
                _ => {}
            },
        )
        .detach();

        self.observe_docks(workspace, cx);

        if let Some(agent_panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
            self.subscribe_to_agent_panel(workspace, &agent_panel, window, cx);
        }
    }

    fn move_entry_paths(
        &mut self,
        project: &Entity<project::Project>,
        old_paths: &WorktreePaths,
        cx: &mut Context<Self>,
    ) {
        if project.read(cx).is_via_collab() {
            return;
        }

        let new_paths = project.read(cx).worktree_paths(cx);
        let old_folder_paths = old_paths.folder_path_list().clone();

        let added_pairs: Vec<_> = new_paths
            .ordered_pairs()
            .filter(|(main, folder)| {
                !old_paths
                    .ordered_pairs()
                    .any(|(old_main, old_folder)| old_main == *main && old_folder == *folder)
            })
            .map(|(m, f)| (m.clone(), f.clone()))
            .collect();

        let new_folder_paths = new_paths.folder_path_list();
        let removed_folder_paths: Vec<PathBuf> = old_folder_paths
            .paths()
            .iter()
            .filter(|p| !new_folder_paths.paths().contains(p))
            .cloned()
            .collect();

        if added_pairs.is_empty() && removed_folder_paths.is_empty() {
            return;
        }

        let remote_connection = project.read(cx).remote_connection_options(cx);
        let apply_path_changes = |paths: &mut WorktreePaths| {
            for (main_path, folder_path) in &added_pairs {
                paths.add_path(main_path, folder_path);
            }
            for path in &removed_folder_paths {
                paths.remove_folder_path(path);
            }
        };
        ThreadMetadataStore::global(cx).update(cx, |store, store_cx| {
            store.change_worktree_paths(
                &old_folder_paths,
                remote_connection.as_ref(),
                &apply_path_changes,
                store_cx,
            );
        });
        TerminalThreadMetadataStore::global(cx).update(cx, |store, store_cx| {
            store.change_worktree_paths(
                &old_folder_paths,
                remote_connection.as_ref(),
                &apply_path_changes,
                store_cx,
            );
        });
    }

    fn subscribe_to_agent_panel(
        &mut self,
        workspace: &Entity<Workspace>,
        agent_panel: &Entity<AgentPanel>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let workspace = workspace.downgrade();
        cx.subscribe_in(
            agent_panel,
            window,
            move |this, agent_panel, event: &AgentPanelEvent, window, cx| match event {
                AgentPanelEvent::ActiveViewChanged
                | AgentPanelEvent::ActiveViewFocused
                | AgentPanelEvent::EntryChanged => {
                    this.sync_active_entry_from_panel(agent_panel, cx);
                    this.schedule_update_entries(false, cx);
                }
                AgentPanelEvent::TerminalClosed { metadata } => {
                    if let Some(workspace) = workspace.upgrade() {
                        let workspace = ThreadEntryWorkspace::Open(workspace);
                        this.close_terminal(
                            metadata,
                            &workspace,
                            &TerminalEntrySource::AgentPanel,
                            window,
                            cx,
                        );
                    }
                }
                AgentPanelEvent::ThreadInteracted { thread_id } => {
                    this.record_thread_interacted(thread_id, cx);
                    this.schedule_update_entries(false, cx);
                }
            },
        )
        .detach();
    }

    fn sync_active_entry_from_active_workspace(&mut self, cx: &App) {
        let Some(active_workspace) = self.active_workspace(cx) else {
            return;
        };

        if let Some(item) = active_workspace
            .read(cx)
            .active_item_as::<AgentThreadItem>(cx)
        {
            let item = item.read(cx);
            let thread_id = item.thread_id(cx);
            self.active_entry = Some(ActiveEntry::Thread {
                thread_id,
                session_id: item.session_id(cx),
                workspace: active_workspace,
            });
            if self.pending_thread_activation == Some(thread_id) {
                self.pending_thread_activation = None;
            }
            return;
        }

        if let Some(item) = active_workspace.read(cx).active_item_as::<TerminalView>(cx) {
            let terminal_id = standalone_terminal_id(&active_workspace, &item, cx);
            self.active_entry = Some(ActiveEntry::Terminal {
                terminal_id,
                workspace: active_workspace,
            });
            return;
        }

        if let Some(panel) = active_workspace.read(cx).panel::<AgentPanel>(cx) {
            self.sync_active_entry_from_panel(&panel, cx);
        }
    }

    fn focused_thread_entry(&self, window: &Window, cx: &App) -> Option<ActiveEntry> {
        let active_workspace = self.active_workspace(cx)?;
        let active_pane = active_workspace.read(cx).active_pane().clone();
        let active_item = {
            let active_pane = active_pane.read(cx);
            if !active_pane.has_focus(window, cx) {
                return None;
            }
            active_pane.active_item()?.downcast::<AgentThreadItem>()?
        };

        let active_item = active_item.read(cx);
        Some(ActiveEntry::Thread {
            thread_id: active_item.thread_id(cx),
            session_id: active_item.session_id(cx),
            workspace: active_workspace,
        })
    }

    /// When switching workspaces, the active panel may still be showing
    /// a thread that was archived from a different workspace. In that
    /// case, create a fresh draft so the panel has valid content and
    /// `active_entry` can point at it.
    fn replace_archived_panel_thread(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(workspace) = self.active_workspace(cx) else {
            return;
        };
        let Some(panel) = workspace.read(cx).panel::<AgentPanel>(cx) else {
            return;
        };
        let Some(thread_id) = panel.read(cx).active_thread_id(cx) else {
            return;
        };
        let is_archived = ThreadMetadataStore::global(cx)
            .read(cx)
            .entry(thread_id)
            .is_some_and(|m| m.archived);
        if is_archived {
            self.create_new_thread(&workspace, window, cx);
        }
    }

    /// Syncs `active_entry` from the agent panel's current state.
    /// Called from `ActiveViewChanged` — the panel has settled into its
    /// new view, so we can safely read it without race conditions.
    ///
    /// Also resolves `pending_thread_activation` when the panel's
    /// active thread matches the pending activation.
    fn sync_active_entry_from_panel(&mut self, agent_panel: &Entity<AgentPanel>, cx: &App) -> bool {
        let Some(active_workspace) = self.active_workspace(cx) else {
            return false;
        };

        // Only sync when the event comes from the active workspace's panel.
        let is_active_panel = active_workspace
            .read(cx)
            .panel::<AgentPanel>(cx)
            .is_some_and(|p| p == *agent_panel);
        if !is_active_panel {
            return false;
        }

        let panel = agent_panel.read(cx);

        if let Some(pending_thread_id) = self.pending_thread_activation {
            let panel_thread_id = panel
                .active_conversation_view()
                .map(|cv| cv.read(cx).parent_id());

            if panel_thread_id == Some(pending_thread_id) {
                let session_id = panel
                    .active_agent_thread(cx)
                    .map(|thread| thread.read(cx).session_id().clone());
                self.active_entry = Some(ActiveEntry::Thread {
                    thread_id: pending_thread_id,
                    session_id,
                    workspace: active_workspace,
                });
                self.pending_thread_activation = None;
                return true;
            }
            // Pending activation not yet resolved — keep current active_entry.
            return false;
        }

        if let Some(terminal_id) = panel.active_terminal_id() {
            self.active_entry = Some(ActiveEntry::Terminal {
                terminal_id,
                workspace: active_workspace,
            });
        } else if let Some(thread_id) = panel.active_thread_id(cx) {
            let is_archived = ThreadMetadataStore::global(cx)
                .read(cx)
                .entry(thread_id)
                .is_some_and(|m| m.archived);
            if !is_archived {
                let session_id = panel
                    .active_agent_thread(cx)
                    .map(|thread| thread.read(cx).session_id().clone());
                self.active_entry = Some(ActiveEntry::Thread {
                    thread_id,
                    session_id,
                    workspace: active_workspace,
                });
            }
        }

        false
    }

    fn observe_docks(&mut self, workspace: &Entity<Workspace>, cx: &mut Context<Self>) {
        let docks: Vec<_> = workspace
            .read(cx)
            .all_docks()
            .into_iter()
            .cloned()
            .collect();
        let workspace = workspace.downgrade();
        for dock in docks {
            let workspace = workspace.clone();
            cx.observe(&dock, move |this, _dock, cx| {
                let Some(workspace) = workspace.upgrade() else {
                    return;
                };
                if !this.is_active_workspace(&workspace, cx) {
                    return;
                }

                cx.notify();
            })
            .detach();
        }
    }

    /// Opens a new workspace for a group that has no open workspaces.
    fn open_workspace_for_group(
        &mut self,
        project_group_key: &ProjectGroupKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };
        let path_list = project_group_key.path_list().clone();
        let host = project_group_key.host();
        let provisional_key = Some(project_group_key.clone());
        let active_workspace = multi_workspace.read(cx).workspace().clone();
        let modal_workspace = active_workspace.clone();

        let task = multi_workspace.update(cx, |this, cx| {
            this.find_or_create_workspace(
                path_list,
                host,
                provisional_key,
                |options, window, cx| connect_remote(active_workspace, options, window, cx),
                &[],
                None,
                OpenMode::Activate,
                window,
                cx,
            )
        });

        cx.spawn_in(window, async move |_this, cx| {
            let result = task.await;
            remote_connection::dismiss_connection_modal(&modal_workspace, cx);
            result?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn open_workspace_and_create_entry(
        &mut self,
        project_group_key: &ProjectGroupKey,
        target: NewEntryTarget,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        let path_list = project_group_key.path_list().clone();
        let host = project_group_key.host();
        let provisional_key = Some(project_group_key.clone());
        let active_workspace = multi_workspace.read(cx).workspace().clone();

        let task = multi_workspace.update(cx, |this, cx| {
            this.find_or_create_workspace(
                path_list,
                host,
                provisional_key,
                |options, window, cx| connect_remote(active_workspace, options, window, cx),
                &[],
                None,
                OpenMode::Activate,
                window,
                cx,
            )
        });

        cx.spawn_in(window, async move |this, cx| {
            let workspace = task.await?;
            this.update_in(cx, |this, window, cx| match target {
                NewEntryTarget::Terminal => this.create_new_terminal(&workspace, window, cx),
                NewEntryTarget::AgentThread => this.create_new_thread(&workspace, window, cx),
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    /// Rebuilds the sidebar contents from current workspace and thread state.
    ///
    /// Iterates [`MultiWorkspace::project_group_keys`] to determine project
    /// groups, then populates thread entries from the metadata store and
    /// merges live thread info from active agent panels.
    ///
    /// Aim for a single forward pass over workspaces and threads plus an
    /// O(T log T) sort. Avoid adding extra scans over the data.
    ///
    /// Properties:
    ///
    /// - Should always show every workspace in the multiworkspace
    ///     - If you have no threads, and two workspaces for the worktree and the main workspace, make sure at least one is shown
    /// - Should always show every thread, associated with each workspace in the multiworkspace
    /// - After every build_contents, our "active" state should exactly match the current workspace's, current agent panel's current thread.
    fn rebuild_contents(&mut self, cx: &App) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };
        let mw = multi_workspace.read(cx);
        let workspaces: Vec<_> = mw.workspaces().cloned().collect();
        let active_workspace = Some(mw.workspace().clone());
        let agent_ui_settings = CanvasAgentUiSettings::get_global(cx);
        let show_terminal_agents = agent_ui_settings.show_terminal_agents_in_session_rail;
        let detect_terminal_agents = agent_ui_settings.detect_terminal_agents;
        let notify_on_terminal_attention = agent_ui_settings.notify_on_attention;
        let session_rail_settings = SessionRailSettings::get_global(cx);
        let session_rail_sort_by = session_rail_settings.sort_by;
        let show_layout_metadata = session_rail_settings.show_layout_metadata;

        let agent_server_store = workspaces
            .first()
            .map(|ws| ws.read(cx).project().read(cx).agent_server_store().clone());

        let query = self.filter_editor.read(cx).text(cx);

        let previous = mem::take(&mut self.contents);
        let mut manual_entry_order: HashMap<ManualEntryOrderKey, usize> = self
            .manual_entry_order
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, key)| (key, index))
            .collect();
        let manual_entry_order_len = manual_entry_order.len();
        for (index, entry) in previous.entries.iter().enumerate() {
            if let Some(key) = ManualEntryOrderKey::from_entry(entry) {
                manual_entry_order
                    .entry(key)
                    .or_insert(manual_entry_order_len + index);
            }
        }

        let old_statuses = &self.live_thread_statuses;

        let mut entries = Vec::new();
        let mut notified_threads = previous.notified_threads;
        let mut notified_terminals: HashSet<TerminalId> = HashSet::new();
        let mut new_live_statuses: HashMap<acp::SessionId, (AgentThreadStatus, ThreadId)> =
            HashMap::new();
        let mut current_session_ids: HashSet<acp::SessionId> = HashSet::new();
        let mut current_thread_ids: HashSet<agent_ui::ThreadId> = HashSet::new();
        let mut current_terminal_ids: HashSet<TerminalId> = HashSet::new();
        let mut project_header_indices: Vec<usize> = Vec::new();
        let mut seen_thread_ids: HashSet<agent_ui::ThreadId> = HashSet::new();
        let mut seen_terminal_ids: HashSet<TerminalId> = HashSet::new();

        let has_open_projects = workspaces
            .iter()
            .any(|ws| !workspace_path_list(ws, cx).paths().is_empty());

        let resolve_agent_icon = |agent_id: &AgentId| -> (IconName, Option<SharedString>) {
            let agent = Agent::from(agent_id.clone());
            let icon = match agent {
                Agent::NativeAgent => agent::native_agent_icon(),
                Agent::Custom { .. } => IconName::Terminal,

                _ => agent::native_agent_icon(),
            };
            let icon_from_external_svg = agent_server_store
                .as_ref()
                .and_then(|store| store.read(cx).agent_icon(&agent_id));
            (icon, icon_from_external_svg)
        };

        let groups = mw.project_groups(cx);
        let mut live_notified_terminal_ids: HashSet<TerminalId> = HashSet::new();
        let mut live_terminal_runtime: HashMap<TerminalId, TerminalRuntimeInfo> = HashMap::new();
        if show_terminal_agents {
            for workspace in &workspaces {
                if let Some(agent_panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
                    for terminal in agent_panel.read(cx).terminals(cx) {
                        if notify_on_terminal_attention && terminal.has_notification {
                            live_notified_terminal_ids.insert(terminal.id);
                        }
                        live_terminal_runtime.insert(
                            terminal.id,
                            TerminalRuntimeInfo {
                                state: TerminalRuntimeState::Live,
                            },
                        );
                    }
                }
            }
        }
        let local_host_sessions = LocalTerminalHost::try_global(cx)
            .map(|host| host.read(cx).list())
            .unwrap_or_default();
        let helper_host_sessions = TerminalHostSnapshotStore::try_global(cx)
            .map(|store| store.read(cx).snapshots().to_vec())
            .unwrap_or_default();
        let helper_snapshot_by_session = helper_host_sessions
            .iter()
            .map(|snapshot| (snapshot.session_id, snapshot.clone()))
            .collect::<HashMap<_, _>>();
        let detached_local_sessions = local_host_sessions
            .into_iter()
            .chain(helper_host_sessions.iter().cloned())
            .filter(|snapshot| {
                matches!(
                    snapshot.state,
                    TerminalSessionState::Detached
                        | TerminalSessionState::Reconnecting
                        | TerminalSessionState::Exited { .. }
                )
            })
            .filter_map(|snapshot| {
                let workspace = workspace_for_local_terminal_session(
                    &snapshot,
                    &workspaces,
                    active_workspace.as_ref(),
                    cx,
                )?;
                Some((snapshot, workspace))
            })
            .collect::<Vec<_>>();

        let mut all_paths: Vec<PathBuf> = groups
            .iter()
            .flat_map(|group| group.key.path_list().paths().iter().cloned())
            .collect();
        all_paths.sort_unstable();
        all_paths.dedup();
        let path_details =
            util::disambiguate::compute_disambiguation_details(&all_paths, |path, detail| {
                project::path_suffix(path, detail)
            });
        let path_detail_map: HashMap<PathBuf, usize> =
            all_paths.into_iter().zip(path_details).collect();

        let mut branches_by_workspace: HashMap<EntityId, HashMap<PathBuf, SharedString>> =
            HashMap::new();
        let mut unambiguous_branch_by_path: HashMap<PathBuf, SharedString> = HashMap::new();
        let mut ambiguous_branch_paths = HashSet::new();
        for ws in &workspaces {
            let project = ws.read(cx).project().read(cx);
            let mut workspace_branches = HashMap::new();
            for repo in project.repositories(cx).values() {
                let snapshot = repo.read(cx).snapshot();
                if let Some(branch) = &snapshot.branch {
                    workspace_branches.insert(
                        snapshot.work_directory_abs_path.to_path_buf(),
                        SharedString::from(Arc::<str>::from(branch.name())),
                    );
                }
                for linked_wt in snapshot.linked_worktrees() {
                    if let Some(branch) = linked_wt.branch_name() {
                        workspace_branches.insert(
                            linked_wt.path.clone(),
                            SharedString::from(Arc::<str>::from(branch)),
                        );
                    }
                }
            }
            for (path, branch) in &workspace_branches {
                merge_unambiguous_branch(
                    &mut unambiguous_branch_by_path,
                    &mut ambiguous_branch_paths,
                    path.clone(),
                    branch.clone(),
                );
            }
            branches_by_workspace.insert(ws.entity_id(), workspace_branches);
        }

        for group in &groups {
            let group_key = &group.key;
            let group_workspaces = &group.workspaces;

            let workspace_by_path_list: HashMap<PathList, &Entity<Workspace>> = group_workspaces
                .iter()
                .map(|ws| (workspace_path_list(ws, cx), ws))
                .collect();
            let resolve_workspace = |folder_paths: &PathList| -> ThreadEntryWorkspace {
                workspace_by_path_list
                    .get(folder_paths)
                    .map(|ws| ThreadEntryWorkspace::Open((*ws).clone()))
                    .unwrap_or_else(|| ThreadEntryWorkspace::Closed {
                        folder_paths: folder_paths.clone(),
                        project_group_key: group_key.clone(),
                    })
            };
            let linked_worktree_path_lists =
                linked_worktree_path_lists_for_workspaces(group_workspaces, cx);
            let make_terminal_entry =
                |metadata: TerminalThreadMetadata, workspace: ThreadEntryWorkspace| {
                    let branch_by_path = match &workspace {
                        ThreadEntryWorkspace::Open(workspace) => branches_by_workspace
                            .get(&workspace.entity_id())
                            .unwrap_or(&unambiguous_branch_by_path),
                        ThreadEntryWorkspace::Closed { .. } => &unambiguous_branch_by_path,
                    };
                    let worktrees =
                        worktree_info_from_thread_paths(&metadata.worktree_paths, branch_by_path);
                    let host_snapshot = metadata.session_ref.and_then(|session_ref| {
                        helper_snapshot_by_session.get(&session_ref.session_id)
                    });
                    let source = match stored_terminal_source(
                        APP_NAME,
                        live_terminal_runtime.contains_key(&metadata.terminal_id),
                        host_snapshot.map(|snapshot| snapshot.session_id),
                    )? {
                        StoredTerminalSource::AgentPanel => TerminalEntrySource::AgentPanel,
                        StoredTerminalSource::HostSession(session_id) => {
                            TerminalEntrySource::HostSession(session_id)
                        }
                    };
                    let agent = host_snapshot.and_then(|snapshot| snapshot.agent.clone());
                    let now = Utc::now();
                    let adapter_attention =
                        agent.as_ref().is_some_and(|agent| agent.attention_required);
                    let live_notification =
                        live_notified_terminal_ids.contains(&metadata.terminal_id);
                    let needs_attention = metadata.attention.requires_action_at(now)
                        || live_notification
                        || adapter_attention;
                    let has_notification = metadata.attention.is_unread_at(now)
                        || live_notification
                        || adapter_attention;
                    let attention_priority = terminal_attention_priority(&metadata, agent.as_ref());
                    let detected_agent_kind = detect_terminal_agents
                        .then(|| {
                            terminal_agent_kind_from_snapshot(agent.as_ref())
                                .or_else(|| metadata.detected_agent_kind())
                        })
                        .flatten();
                    Some(TerminalEntry {
                        runtime: live_terminal_runtime
                            .get(&metadata.terminal_id)
                            .cloned()
                            .or_else(|| host_snapshot.and_then(terminal_runtime_from_snapshot)),
                        agent,
                        metadata,
                        detected_agent_kind,
                        workspace,
                        source,
                        worktrees,
                        needs_attention,
                        attention_priority,
                        has_notification,
                        highlight_positions: Vec::new(),
                    })
                };

            let mut terminals = Vec::new();
            if show_terminal_agents {
                let terminal_store = TerminalThreadMetadataStore::global(cx);
                let group_host = group_key.host();
                let mut push_terminal_metadata =
                    |metadata: TerminalThreadMetadata, workspace: ThreadEntryWorkspace| {
                        if seen_terminal_ids.contains(&metadata.terminal_id) {
                            return;
                        }
                        let Some(entry) = make_terminal_entry(metadata, workspace) else {
                            return;
                        };
                        seen_terminal_ids.insert(entry.metadata.terminal_id);
                        terminals.push(entry);
                    };
                for row in terminal_store
                    .read(cx)
                    .entries_for_main_worktree_path(group_key.path_list(), group_host.as_ref())
                    .cloned()
                {
                    let workspace = resolve_workspace(row.folder_paths());
                    push_terminal_metadata(row, workspace);
                }
                for row in terminal_store
                    .read(cx)
                    .entries_for_path(group_key.path_list(), group_host.as_ref())
                    .cloned()
                {
                    let workspace = resolve_workspace(row.folder_paths());
                    push_terminal_metadata(row, workspace);
                }
                for ws in group_workspaces {
                    let ws_paths = workspace_path_list(ws, cx);
                    if ws_paths.paths().is_empty() {
                        continue;
                    }
                    for row in terminal_store
                        .read(cx)
                        .entries_for_path(&ws_paths, group_host.as_ref())
                        .cloned()
                    {
                        push_terminal_metadata(row, ThreadEntryWorkspace::Open(ws.clone()));
                    }
                }
                for worktree_path_list in &linked_worktree_path_lists {
                    for row in terminal_store
                        .read(cx)
                        .entries_for_path(worktree_path_list, group_host.as_ref())
                        .cloned()
                    {
                        push_terminal_metadata(
                            row,
                            ThreadEntryWorkspace::Closed {
                                folder_paths: worktree_path_list.clone(),
                                project_group_key: group_key.clone(),
                            },
                        );
                    }
                }
                for ws in group_workspaces {
                    for terminal_view in ws.read(cx).items_of_type::<TerminalView>(cx) {
                        let terminal_id = standalone_terminal_id(ws, &terminal_view, cx);
                        let created_at = self
                            .standalone_terminal_created_at
                            .entry(terminal_id)
                            .or_insert_with(Utc::now);
                        let (metadata, detected_agent_kind, runtime) =
                            standalone_terminal_metadata(ws, &terminal_view, *created_at, cx);
                        if !seen_terminal_ids.insert(metadata.terminal_id) {
                            continue;
                        }

                        let worktrees = worktree_info_from_thread_paths(
                            &metadata.worktree_paths,
                            branches_by_workspace
                                .get(&ws.entity_id())
                                .unwrap_or(&unambiguous_branch_by_path),
                        );
                        let has_notification =
                            notify_on_terminal_attention && terminal_view.read(cx).has_bell();
                        let agent = metadata
                            .session_ref
                            .and_then(|session_ref| {
                                helper_snapshot_by_session.get(&session_ref.session_id)
                            })
                            .and_then(|snapshot| snapshot.agent.clone());
                        let has_notification = has_notification
                            || agent.as_ref().is_some_and(|agent| agent.attention_required);
                        let needs_attention =
                            metadata.attention.requires_action_at(Utc::now()) || has_notification;
                        let attention_priority =
                            terminal_attention_priority(&metadata, agent.as_ref());
                        terminals.push(TerminalEntry {
                            metadata,
                            detected_agent_kind: terminal_agent_kind_from_snapshot(agent.as_ref())
                                .or_else(|| {
                                    detect_terminal_agents
                                        .then_some(detected_agent_kind)
                                        .flatten()
                                }),
                            workspace: ThreadEntryWorkspace::Open(ws.clone()),
                            source: TerminalEntrySource::WorkspaceItem(terminal_view),
                            runtime: Some(runtime),
                            agent,
                            worktrees,
                            needs_attention,
                            attention_priority,
                            has_notification,
                            highlight_positions: Vec::new(),
                        });
                    }
                }
                for (snapshot, workspace) in &detached_local_sessions {
                    if !group_workspaces.contains(workspace) {
                        continue;
                    }
                    let terminal_id = TerminalId::from_stable_key(
                        "terminal-session",
                        &format!("{}:{}", snapshot.host_id, snapshot.session_id),
                    );
                    if !seen_terminal_ids.insert(terminal_id) {
                        continue;
                    }
                    let created_at = *self
                        .standalone_terminal_created_at
                        .entry(terminal_id)
                        .or_insert_with(Utc::now);
                    let project = workspace.read(cx).project().clone();
                    let project = project.read(cx);
                    let metadata = TerminalThreadMetadata {
                        terminal_id,
                        title: snapshot
                            .title
                            .clone()
                            .unwrap_or_else(|| "Terminal".to_string())
                            .into(),
                        custom_title: None,
                        created_at,
                        worktree_paths: project.worktree_paths(cx),
                        remote_connection: None,
                        working_directory: snapshot.working_directory.clone(),
                        attention: TerminalAttentionState::default(),
                        session_ref: Some(terminal::session_host::TerminalSessionRef {
                            host_id: snapshot.host_id,
                            session_id: snapshot.session_id,
                        }),
                    };
                    let agent = snapshot.agent.clone();
                    let detected_agent_kind = detect_terminal_agents
                        .then(|| {
                            terminal_agent_kind_from_snapshot(agent.as_ref())
                                .or_else(|| metadata.detected_agent_kind())
                        })
                        .flatten();
                    let worktrees = worktree_info_from_thread_paths(
                        &metadata.worktree_paths,
                        branches_by_workspace
                            .get(&workspace.entity_id())
                            .unwrap_or(&unambiguous_branch_by_path),
                    );
                    let attention_priority = terminal_attention_priority(&metadata, agent.as_ref());
                    terminals.push(TerminalEntry {
                        metadata,
                        detected_agent_kind,
                        workspace: ThreadEntryWorkspace::Open(workspace.clone()),
                        source: TerminalEntrySource::HostSession(snapshot.session_id),
                        runtime: terminal_runtime_from_snapshot(snapshot),
                        agent: agent.clone(),
                        worktrees,
                        needs_attention: agent
                            .as_ref()
                            .is_some_and(|agent| agent.attention_required),
                        attention_priority,
                        has_notification: agent
                            .as_ref()
                            .is_some_and(|agent| agent.attention_required),
                        highlight_positions: Vec::new(),
                    });
                }
            }
            current_terminal_ids.extend(
                terminals
                    .iter()
                    .map(|terminal| terminal.metadata.terminal_id),
            );
            notified_terminals.extend(terminals.iter().filter_map(|terminal| {
                terminal
                    .has_notification
                    .then_some(terminal.metadata.terminal_id)
            }));
            // Empty workspaces are valid terminal-first canvases. Keep hiding a
            // completely empty group, but do not discard live terminal rows just
            // because the user has not opened a project yet.
            if group_key.path_list().paths().is_empty() && terminals.is_empty() {
                continue;
            }

            let label = group_key.display_name(&path_detail_map);

            let is_collapsed = self.is_group_collapsed(group_key, cx);
            let should_load_threads = !is_collapsed || !query.is_empty();

            let is_active = active_workspace
                .as_ref()
                .is_some_and(|active| group_workspaces.contains(active));
            let layout_label = show_layout_metadata
                .then(|| {
                    let active_layout_label = active_workspace
                        .as_ref()
                        .filter(|active| group_workspaces.contains(active))
                        .into_iter()
                        .chain(group_workspaces.iter())
                        .find_map(|workspace| {
                            let recipe_id = workspace.read(cx).active_canvas_layout_recipe_id()?;
                            Some(canvas_layout_recipe_label(recipe_id)?.to_string())
                        });
                    let saved_layout_count = group_workspaces
                        .iter()
                        .map(|workspace| workspace.read(cx).saved_canvas_layout_count())
                        .sum::<usize>();

                    match (active_layout_label, saved_layout_count) {
                        (Some(layout), 0) => Some(SharedString::from(format!("Layout: {layout}"))),
                        (Some(layout), 1) => {
                            Some(SharedString::from(format!("Layout: {layout} · Saved: 1")))
                        }
                        (Some(layout), count) => Some(SharedString::from(format!(
                            "Layout: {layout} · Saved: {count}"
                        ))),
                        (None, 1) => Some(SharedString::from("Saved: 1")),
                        (None, count) if count > 1 => {
                            Some(SharedString::from(format!("Saved: {count}")))
                        }
                        (None, _) => None,
                    }
                })
                .flatten();

            // Collect live thread infos from all workspaces in this group.
            let live_infos = group_workspaces
                .iter()
                .flat_map(|ws| all_thread_infos_for_workspace(ws, cx));

            let mut threads: Vec<Arc<ThreadEntry>> = Vec::new();
            let mut has_running_threads = false;
            let mut attention_thread_count: usize = 0;
            let group_host = group_key.host();

            if should_load_threads {
                let thread_store = ThreadMetadataStore::global(cx);

                let make_thread_entry =
                    |row: ThreadMetadata, workspace: ThreadEntryWorkspace| -> Arc<ThreadEntry> {
                        let (icon, icon_from_external_svg) = resolve_agent_icon(&row.agent_id);
                        let branch_by_path = match &workspace {
                            ThreadEntryWorkspace::Open(workspace) => branches_by_workspace
                                .get(&workspace.entity_id())
                                .unwrap_or(&unambiguous_branch_by_path),
                            ThreadEntryWorkspace::Closed { .. } => &unambiguous_branch_by_path,
                        };
                        let worktrees =
                            worktree_info_from_thread_paths(&row.worktree_paths, branch_by_path);
                        let is_draft = row.is_draft();
                        Arc::new(ThreadEntry {
                            metadata: row,
                            icon,
                            icon_from_external_svg,
                            status: AgentThreadStatus::default(),
                            workspace,
                            is_live: false,
                            is_background: false,
                            is_title_generating: false,
                            draft: is_draft.then_some(DraftKind::Empty),
                            highlight_positions: Vec::new(),
                            worktrees,
                            diff_stats: DiffStats::default(),
                            changed_files: Vec::new(),
                        })
                    };

                // Main code path: one query per group via main_worktree_paths.
                // The main_worktree_paths column is set on all new threads and
                // points to the group's canonical paths regardless of which
                // linked worktree the thread was opened in.
                for row in thread_store
                    .read(cx)
                    .entries_for_main_worktree_path(group_key.path_list(), group_host.as_ref())
                    .cloned()
                {
                    if !seen_thread_ids.insert(row.thread_id) {
                        continue;
                    }
                    let workspace = resolve_workspace(row.folder_paths());
                    threads.push(make_thread_entry(row, workspace));
                }

                // Legacy threads did not have `main_worktree_paths` populated, so they
                // must be queried by their `folder_paths`.

                // Load any legacy threads for the main worktrees of this project group.
                for row in thread_store
                    .read(cx)
                    .entries_for_path(group_key.path_list(), group_host.as_ref())
                    .cloned()
                {
                    if !seen_thread_ids.insert(row.thread_id) {
                        continue;
                    }
                    let workspace = resolve_workspace(row.folder_paths());
                    threads.push(make_thread_entry(row, workspace));
                }

                // Also surface any thread whose `folder_paths` equals
                // one of this group's open workspaces' root paths.
                // The three lookups above can all miss when the
                // thread's stored `main_worktree_paths` disagree with
                // the group key (for example, a stale row whose main
                // paths equal its folder paths for a linked-worktree
                // workspace). The thread will be rewritten into the
                // correct shape the next time `handle_conversation_event`
                // fires, but until then the sidebar should still show
                // it under the group whose workspace it actually
                // belongs to.
                for ws in group_workspaces {
                    let ws_paths = workspace_path_list(ws, cx);
                    if ws_paths.paths().is_empty() {
                        continue;
                    }
                    for row in thread_store
                        .read(cx)
                        .entries_for_path(&ws_paths, group_host.as_ref())
                        .cloned()
                    {
                        if !seen_thread_ids.insert(row.thread_id) {
                            continue;
                        }
                        threads.push(make_thread_entry(
                            row,
                            ThreadEntryWorkspace::Open(ws.clone()),
                        ));
                    }
                }

                // Load any legacy threads for any single linked worktree of this project group.
                for worktree_path_list in &linked_worktree_path_lists {
                    for row in thread_store
                        .read(cx)
                        .entries_for_path(worktree_path_list, group_host.as_ref())
                        .cloned()
                    {
                        if !seen_thread_ids.insert(row.thread_id) {
                            continue;
                        }
                        threads.push(make_thread_entry(
                            row,
                            ThreadEntryWorkspace::Closed {
                                folder_paths: worktree_path_list.clone(),
                                project_group_key: group_key.clone(),
                            },
                        ));
                    }
                }

                for thread in &mut threads {
                    if thread.draft.is_none() {
                        continue;
                    }
                    if let Some((label, kind)) = draft_display_label_for_thread_metadata(
                        &thread.metadata,
                        &thread.workspace,
                        cx,
                    ) {
                        let thread = Arc::make_mut(thread);
                        thread.metadata.title = Some(label);
                        thread.draft = Some(kind);
                    }
                }
                threads.retain(|thread| thread.draft.is_none() || thread.metadata.title.is_some());

                // Keep empty drafts only while their thread is active; preserve
                // drafts with content because they hold user-typed state.
                let pending_activation = self.pending_thread_activation;
                let active_panel_thread_id = active_workspace
                    .as_ref()
                    .and_then(|ws| ws.read(cx).panel::<AgentPanel>(cx))
                    .and_then(|panel| panel.read(cx).active_thread_id(cx));
                threads.retain(|thread| {
                    if thread.draft != Some(DraftKind::Empty) {
                        return true;
                    }
                    if pending_activation.is_some() {
                        return false;
                    }
                    Some(thread.metadata.thread_id) == active_panel_thread_id
                });

                // Build a lookup from live_infos and compute running/waiting
                // counts in a single pass.
                let mut live_info_by_session: HashMap<acp::SessionId, ActiveThreadInfo> =
                    HashMap::new();
                for info in live_infos {
                    if info.status == AgentThreadStatus::Running {
                        has_running_threads = true;
                    }
                    if agent_status_needs_attention(info.status) {
                        attention_thread_count += 1;
                    }
                    live_info_by_session.insert(info.session_id.clone(), info);
                }

                // Merge live info into threads and update notification state
                // in a single pass.
                for thread in &mut threads {
                    if let Some(session_id) = thread.metadata.session_id.clone() {
                        if let Some(info) = live_info_by_session.get(&session_id) {
                            let status = info.status;
                            let thread_id = thread.metadata.thread_id;
                            Arc::make_mut(thread).apply_active_info(info);
                            new_live_statuses.insert(session_id, (status, thread_id));
                        }
                    }

                    let session_id = &thread.metadata.session_id;
                    let is_active_thread = self.active_entry.as_ref().is_some_and(|entry| {
                        entry.is_active_thread(&thread.metadata.thread_id)
                            && active_workspace
                                .as_ref()
                                .is_some_and(|active| active == entry.workspace())
                    });

                    if thread.status == AgentThreadStatus::Completed
                        && !is_active_thread
                        && session_id
                            .as_ref()
                            .and_then(|sid| old_statuses.get(sid))
                            .is_some_and(|(s, _)| *s == AgentThreadStatus::Running)
                    {
                        notified_threads.insert(thread.metadata.thread_id);
                    }

                    if is_active_thread && !thread.is_background {
                        notified_threads.remove(&thread.metadata.thread_id);
                    }
                }

                threads.sort_by(|a, b| {
                    let a_time = Self::thread_display_time(&a.metadata);
                    let b_time = Self::thread_display_time(&b.metadata);
                    b_time.cmp(&a_time)
                });
            } else {
                for info in live_infos {
                    if info.status == AgentThreadStatus::Running {
                        has_running_threads = true;
                    }
                    if agent_status_needs_attention(info.status) {
                        attention_thread_count += 1;
                    }
                    // Resolve the thread_id for this session so we can
                    // track its status and detect transitions even while
                    // the group is collapsed.
                    let thread_id = old_statuses
                        .get(&info.session_id)
                        .map(|(_, tid)| *tid)
                        .or_else(|| {
                            ThreadMetadataStore::global(cx)
                                .read(cx)
                                .entry_by_session(&info.session_id)
                                .map(|m| m.thread_id)
                        });

                    if let Some(thread_id) = thread_id {
                        let old_status = old_statuses.get(&info.session_id).map(|(s, _)| *s);
                        new_live_statuses.insert(info.session_id.clone(), (info.status, thread_id));
                        if info.status == AgentThreadStatus::Completed
                            && old_status == Some(AgentThreadStatus::Running)
                        {
                            notified_threads.insert(thread_id);
                        }
                    }
                }

                if is_active
                    && let Some(ActiveEntry::Thread { thread_id, .. }) = self.active_entry.as_ref()
                {
                    notified_threads.remove(thread_id);
                }
            }

            let has_visible_rows = !threads.is_empty() || !terminals.is_empty();
            let has_stored_thread_rows = !should_load_threads && !has_visible_rows && {
                let store = ThreadMetadataStore::global(cx).read(cx);
                store
                    .entries_for_main_worktree_path(group_key.path_list(), group_host.as_ref())
                    .any(|metadata| {
                        let workspace = resolve_workspace(metadata.folder_paths());
                        thread_metadata_would_render_sidebar_row(metadata, &workspace, cx)
                    })
                    || store
                        .entries_for_path(group_key.path_list(), group_host.as_ref())
                        .any(|metadata| {
                            let workspace = resolve_workspace(metadata.folder_paths());
                            thread_metadata_would_render_sidebar_row(metadata, &workspace, cx)
                        })
            };
            let has_threads = has_visible_rows || has_stored_thread_rows;

            if !query.is_empty() {
                let workspace_highlight_positions =
                    fuzzy_match_positions(&query, &label).unwrap_or_default();
                let workspace_matched = !workspace_highlight_positions.is_empty();

                let mut matched_threads: Vec<Arc<ThreadEntry>> = Vec::new();
                for mut thread in threads {
                    let mut worktree_matched = false;
                    {
                        let thread = Arc::make_mut(&mut thread);
                        let title = thread.metadata.display_title();
                        if let Some(positions) = fuzzy_match_positions(&query, title.as_ref()) {
                            thread.highlight_positions = positions;
                        }
                        for worktree in &mut thread.worktrees {
                            let Some(name) = worktree.worktree_name.as_ref() else {
                                continue;
                            };
                            if let Some(positions) = fuzzy_match_positions(&query, name) {
                                worktree.highlight_positions = positions;
                                worktree_matched = true;
                            }
                        }
                    }
                    if workspace_matched
                        || !thread.highlight_positions.is_empty()
                        || worktree_matched
                    {
                        matched_threads.push(thread);
                    }
                }

                let mut matched_terminals: Vec<TerminalEntry> = Vec::new();
                for mut terminal in terminals {
                    let mut terminal_matched = false;
                    let terminal_title = terminal.metadata.display_title();
                    if let Some(positions) = fuzzy_match_positions(&query, terminal_title.as_ref())
                    {
                        terminal.highlight_positions = positions;
                        terminal_matched = true;
                    }
                    let mut worktree_matched = false;
                    for worktree in &mut terminal.worktrees {
                        let Some(name) = worktree.worktree_name.as_ref() else {
                            continue;
                        };
                        if let Some(positions) = fuzzy_match_positions(&query, name) {
                            worktree.highlight_positions = positions;
                            worktree_matched = true;
                        }
                    }
                    if workspace_matched || terminal_matched || worktree_matched {
                        matched_terminals.push(terminal);
                    }
                }

                if matched_threads.is_empty() && matched_terminals.is_empty() && !workspace_matched
                {
                    continue;
                }

                // Check for notifications: threads that completed while not active.
                let has_thread_notifications = matched_threads
                    .iter()
                    .any(|t| notified_threads.contains(&t.metadata.thread_id));
                let has_terminal_notifications = matched_terminals
                    .iter()
                    .any(|terminal| terminal.needs_attention);

                project_header_indices.push(entries.len());
                entries.push(ListEntry::ProjectHeader {
                    key: group_key.clone(),
                    label,
                    highlight_positions: workspace_highlight_positions,
                    layout_label: layout_label.clone(),
                    has_running_threads,
                    attention_thread_count,
                    has_notifications: has_thread_notifications || has_terminal_notifications,
                    is_active,
                    has_threads,
                });

                Self::push_entries_by_session_rail_sort(
                    &mut entries,
                    matched_terminals,
                    matched_threads,
                    session_rail_sort_by,
                    &notified_threads,
                    &notified_terminals,
                    &manual_entry_order,
                    &mut current_session_ids,
                    &mut current_thread_ids,
                );
            } else {
                let has_terminal_notifications =
                    terminals.iter().any(|terminal| terminal.needs_attention);

                // When collapsed, threads aren't loaded into `threads`, so we
                // query the store for thread IDs to check notifications and
                // to prevent the retain below from purging them.
                let has_thread_notifications = if threads.is_empty() && !notified_threads.is_empty()
                {
                    let thread_store = ThreadMetadataStore::global(cx);
                    let store = thread_store.read(cx);
                    let group_thread_ids = store
                        .entries_for_main_worktree_path(group_key.path_list(), group_host.as_ref())
                        .chain(store.entries_for_path(group_key.path_list(), group_host.as_ref()))
                        .map(|m| m.thread_id)
                        .collect::<HashSet<_>>();
                    current_thread_ids.extend(group_thread_ids.iter());
                    group_thread_ids
                        .iter()
                        .any(|id| notified_threads.contains(id))
                } else {
                    threads
                        .iter()
                        .any(|t| notified_threads.contains(&t.metadata.thread_id))
                };

                project_header_indices.push(entries.len());
                entries.push(ListEntry::ProjectHeader {
                    key: group_key.clone(),
                    label,
                    highlight_positions: Vec::new(),
                    layout_label: layout_label.clone(),
                    has_running_threads,
                    attention_thread_count,
                    has_notifications: has_thread_notifications || has_terminal_notifications,
                    is_active,
                    has_threads,
                });

                if is_collapsed {
                    continue;
                }

                Self::push_entries_by_session_rail_sort(
                    &mut entries,
                    terminals,
                    threads,
                    session_rail_sort_by,
                    &notified_threads,
                    &notified_terminals,
                    &manual_entry_order,
                    &mut current_session_ids,
                    &mut current_thread_ids,
                );
            }
        }

        notified_threads.retain(|id| current_thread_ids.contains(id));

        self.thread_last_accessed
            .retain(|id, _| current_thread_ids.contains(id));
        self.terminal_last_accessed
            .retain(|id, _| current_terminal_ids.contains(id));
        self.standalone_terminal_created_at
            .retain(|id, _| current_terminal_ids.contains(id));

        self.live_thread_statuses = new_live_statuses;

        let session_count = entries
            .iter()
            .filter(|entry| matches!(entry, ListEntry::Thread(_) | ListEntry::Terminal(_)))
            .count();
        let attention_count = entries
            .iter()
            .filter(|entry| entry_needs_attention(entry, &notified_threads, &notified_terminals))
            .count();
        let has_attention = attention_count > 0;
        if self.attention_only {
            (entries, project_header_indices) =
                attention_entries(entries, &notified_threads, &notified_terminals);
        }

        self.contents = SidebarContents {
            entries,
            notified_threads,
            notified_terminals,
            project_header_indices,
            has_open_projects,
            has_attention,
            session_count,
            attention_count,
        };
    }

    fn schedule_update_entries(&mut self, select_first_after_update: bool, cx: &mut Context<Self>) {
        if self.update_task.is_some() && !select_first_after_update {
            return;
        }

        self.update_task = Some(cx.spawn(async move |this, cx| {
            this.update(cx, |this, cx| {
                this.update_task = None;
                this.update_entries(cx);
                if select_first_after_update {
                    this.select_first_entry();
                    cx.notify();
                }
            })
            .ok();
        }));
    }

    /// Rebuilds the sidebar's visible entries from already-cached state.
    fn update_entries(&mut self, cx: &mut Context<Self>) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };
        if !multi_workspace.read(cx).multi_workspace_enabled(cx) {
            return;
        }

        let had_notifications = self.has_notifications(cx);
        let previous_selection = self.selection.and_then(|index| {
            self.contents
                .entries
                .get(index)
                .map(|entry| (index, SelectedEntryKey::from_entry(entry)))
        });
        let previous_shapes: Vec<EntryShape> =
            self.entry_shapes(multi_workspace.read(cx)).collect();

        self.rebuild_contents(cx);
        self.restore_selection_after_rebuild(previous_selection);
        self.refresh_refilled_draft_times(cx);
        self.refresh_draft_editor_observations(cx);

        // Preserve measurements for unchanged entries so sticky headers do not flicker.
        self.apply_list_state_diff(&previous_shapes, multi_workspace.read(cx));

        self.prefetch_worktree_default_branches(cx);

        if had_notifications != self.has_notifications(cx) {
            multi_workspace.update(cx, |_, cx| {
                cx.notify();
            });
        }

        cx.notify();
    }

    fn restore_selection_after_rebuild(
        &mut self,
        previous_selection: Option<(usize, SelectedEntryKey)>,
    ) {
        let Some((previous_index, selected_key)) = previous_selection else {
            return;
        };

        if let Some(index) = self
            .contents
            .entries
            .iter()
            .position(|entry| selected_key.matches(entry))
        {
            self.selection = Some(index);
            return;
        }

        // The selected row was removed or filtered out. Keep keyboard
        // navigation near the same visual position, but prefer an actionable
        // session over a project header.
        self.selection = self
            .contents
            .entries
            .iter()
            .enumerate()
            .skip(previous_index.min(self.contents.entries.len()))
            .find_map(|(index, entry)| {
                matches!(entry, ListEntry::Thread(_) | ListEntry::Terminal(_)).then_some(index)
            })
            .or_else(|| {
                self.contents
                    .entries
                    .iter()
                    .enumerate()
                    .take(previous_index.min(self.contents.entries.len()))
                    .rev()
                    .find_map(|(index, entry)| {
                        matches!(entry, ListEntry::Thread(_) | ListEntry::Terminal(_))
                            .then_some(index)
                    })
            })
            .or_else(|| (!self.contents.entries.is_empty()).then_some(0));
    }

    /// Splices only the changed entry range, leaving unchanged item measurements intact.
    fn apply_list_state_diff(
        &self,
        previous_shapes: &[EntryShape],
        multi_workspace: &MultiWorkspace,
    ) {
        let mut new_iter = self.entry_shapes(multi_workspace);
        let mut prefix_len = 0;
        let leading_new = loop {
            match (previous_shapes.get(prefix_len), new_iter.next()) {
                (Some(prev), Some(next)) if *prev == next => prefix_len += 1,
                (None, None) => return,
                (_, leading) => break leading,
            }
        };

        let new_tail: Vec<EntryShape> = leading_new.into_iter().chain(new_iter).collect();
        let prev_tail = &previous_shapes[prefix_len..];
        let suffix_len = prev_tail
            .iter()
            .rev()
            .zip(new_tail.iter().rev())
            .take_while(|(prev, next)| prev == next)
            .count();

        let old_changed = prefix_len..previous_shapes.len() - suffix_len;
        let new_changed_count = new_tail.len() - suffix_len;
        self.list_state.splice(old_changed, new_changed_count);
    }

    fn entry_shapes<'a>(
        &'a self,
        multi_workspace: &'a MultiWorkspace,
    ) -> impl Iterator<Item = EntryShape> + 'a {
        self.contents.entries.iter().map(move |entry| match entry {
            ListEntry::ProjectHeader {
                key, has_threads, ..
            } => EntryShape::ProjectHeader {
                key: key.clone(),
                has_threads: *has_threads,
                is_collapsed: multi_workspace
                    .group_state_by_key(key)
                    .map(|state| !state.expanded)
                    .unwrap_or(false),
            },
            ListEntry::Thread(thread) => EntryShape::Thread(thread.metadata.thread_id),
            ListEntry::Terminal(terminal) => EntryShape::Terminal(terminal.metadata.terminal_id),
        })
    }

    /// Detects drafts that just went from empty back to having content and
    /// refreshes their interaction time to now, so a re-filled draft sorts to
    /// the top of the list instead of falling back to its original creation time.
    fn refresh_refilled_draft_times(&mut self, cx: &mut Context<Self>) {
        let mut new_kinds: HashMap<ThreadId, DraftKind> = HashMap::new();
        let mut refilled: Vec<ThreadId> = Vec::new();

        for entry in &self.contents.entries {
            let ListEntry::Thread(thread) = entry else {
                continue;
            };
            let Some(kind) = thread.draft else {
                continue;
            };
            let thread_id = thread.metadata.thread_id;

            if kind == DraftKind::WithContent
                && self.draft_kinds.get(&thread_id) == Some(&DraftKind::Empty)
            {
                refilled.push(thread_id);
            }
            new_kinds.insert(thread_id, kind);
        }
        self.draft_kinds = new_kinds;

        if refilled.is_empty() {
            return;
        }

        let now = Utc::now();

        ThreadMetadataStore::global(cx).update(cx, |store, store_cx| {
            for thread_id in refilled {
                store.update_interacted_at(&thread_id, now, store_cx);
            }
        });
    }

    /// Re-establishes subscriptions to each visible draft's message editor
    /// so we rebuild entries (and their displayed titles) as the user types.
    fn refresh_draft_editor_observations(&mut self, cx: &mut Context<Self>) {
        self._draft_editor_observations.clear();
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        let draft_conversation_views: Vec<Entity<agent_ui::ConversationView>> = multi_workspace
            .read(cx)
            .workspaces()
            .flat_map(|ws| {
                ws.read(cx)
                    .items_of_type::<AgentThreadItem>(cx)
                    .map(|item| item.read(cx).conversation_view())
            })
            .collect();

        for cv in draft_conversation_views {
            if let Some(thread_view) = cv.read(cx).active_thread() {
                let editor = thread_view.read(cx).message_editor.clone();
                self._draft_editor_observations.push(cx.subscribe(
                    &editor,
                    |this, _editor, event, cx| match event {
                        MessageEditorEvent::Edited => this.schedule_update_entries(false, cx),
                        _ => (),
                    },
                ));
            }
            // Also subscribe to the ConversationView itself so that editor
            // replacements during lifecycle transitions (Loading →
            // Connected) re-wire the editor observation above.
            self._draft_editor_observations.push(cx.subscribe(
                &cv,
                |this, _cv, _event: &StateChange, cx| {
                    this.schedule_update_entries(false, cx);
                },
            ));
        }
    }

    fn select_first_entry(&mut self) {
        self.selection = self
            .contents
            .entries
            .iter()
            .position(|entry| matches!(entry, ListEntry::Thread(_) | ListEntry::Terminal(_)))
            .or_else(|| {
                if self.contents.entries.is_empty() {
                    None
                } else {
                    Some(0)
                }
            });
    }

    fn render_list_entry(
        &mut self,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(entry) = self.contents.entries.get(ix) else {
            return div().into_any_element();
        };
        let is_focused = self.focus_handle.is_focused(window);
        // is_selected means the keyboard selector is here.
        let is_selected = is_focused && self.selection == Some(ix);

        let is_group_header_after_first =
            ix > 0 && matches!(entry, ListEntry::ProjectHeader { .. });

        let is_active = self
            .active_entry
            .as_ref()
            .is_some_and(|active| active.matches_entry(entry));

        let rendered = match entry {
            ListEntry::ProjectHeader {
                key,
                label,
                highlight_positions,
                layout_label,
                has_running_threads,
                attention_thread_count,
                has_notifications,
                is_active: is_active_group,
                has_threads,
            } => {
                self.project_header_menu_handles.entry(ix).or_default();
                self.project_header_new_thread_menu_handles
                    .entry(ix)
                    .or_default();

                self.render_project_header(
                    ix,
                    false,
                    key,
                    label,
                    highlight_positions,
                    layout_label.as_ref(),
                    *has_running_threads,
                    *attention_thread_count,
                    *has_notifications,
                    *is_active_group,
                    is_selected,
                    *has_threads,
                    // has_active_draft,
                    cx,
                )
            }
            ListEntry::Thread(thread) => {
                let is_active = self
                    .focused_thread_entry(window, cx)
                    .as_ref()
                    .is_some_and(|active| active.matches_entry(entry));
                self.render_thread(ix, thread, is_active, is_selected, cx)
            }
            ListEntry::Terminal(terminal) => {
                self.render_terminal(ix, terminal, is_active, is_selected, cx)
            }
        };

        if is_group_header_after_first {
            v_flex()
                .w_full()
                .border_t_1()
                .border_color(cx.theme().colors().border)
                .child(rendered)
                .into_any_element()
        } else {
            rendered
        }
    }

    fn render_remote_project_icon(
        &self,
        ix: usize,
        host: Option<&RemoteConnectionOptions>,
    ) -> Option<AnyElement> {
        let remote_icon_per_type = match host? {
            RemoteConnectionOptions::Wsl(_) => IconName::Linux,
            RemoteConnectionOptions::Docker(_) => IconName::Box,
            _ => IconName::Server,
        };

        Some(
            div()
                .id(format!("remote-project-icon-{}", ix))
                .child(
                    Icon::new(remote_icon_per_type)
                        .size(IconSize::XSmall)
                        .color(Color::Muted),
                )
                .tooltip(Tooltip::text("Remote Workspace"))
                .into_any_element(),
        )
    }

    fn render_project_header(
        &self,
        ix: usize,
        is_sticky: bool,
        key: &ProjectGroupKey,
        label: &SharedString,
        highlight_positions: &[usize],
        layout_label: Option<&SharedString>,
        has_running_threads: bool,
        attention_thread_count: usize,
        has_notifications: bool,
        is_active: bool,
        is_focused: bool,
        has_threads: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let host = key.host();

        let has_filter = self.has_filter_query(cx);
        let session_rail_settings = SessionRailSettings::get_global(cx);
        let design_system = DesignSystemSettings::get_global(cx);
        let labels_visible = session_rail_labels_visible(&design_system);
        let show_agent_attention =
            WorkspaceBarAttentionSettings::get_global(cx).show_agent_attention;
        let (header_height, header_padding_left, header_padding_right, header_gap, label_size) =
            match design_system.density {
                settings::CanvasDensity::Compact => {
                    (px(28.0), px(6.0), px(4.0), px(4.0), LabelSize::XSmall)
                }
                settings::CanvasDensity::Balanced => (
                    Tab::content_height(cx),
                    px(8.0),
                    px(6.0),
                    px(4.0),
                    LabelSize::Small,
                ),
                settings::CanvasDensity::Spacious => (
                    Tab::container_height(cx),
                    px(10.0),
                    px(8.0),
                    px(6.0),
                    LabelSize::Small,
                ),
            };

        let id_prefix = if is_sticky { "sticky-" } else { "" };
        let id = SharedString::from(format!("{id_prefix}project-header-{ix}"));
        let group_name = SharedString::from(format!("{id_prefix}header-group-{ix}"));
        let workspace_name = label.clone();
        let workspace_accessibility_label = workspace_header_accessibility_label(
            workspace_name.as_ref(),
            has_threads,
            has_running_threads,
            attention_thread_count,
        );

        let is_collapsed = self.is_group_collapsed(key, cx);
        let disclosure_icon = if is_collapsed {
            IconName::ChevronRight
        } else {
            IconName::ChevronDown
        };

        let key_for_toggle = key.clone();
        let key_for_focus = key.clone();
        let key_for_empty_terminal = key.clone();

        // The fade gradient renders as a visible patch on transparent windows,
        // so truncate the label instead.
        let opaque_window =
            cx.theme().window_background_appearance() == WindowBackgroundAppearance::Opaque;

        let label = if highlight_positions.is_empty() {
            Label::new(label.clone())
                .size(label_size)
                .when(!is_active, |this| this.color(Color::Muted))
                .when(!opaque_window, |this| this.truncate())
                .into_any_element()
        } else {
            HighlightedLabel::new(label.clone(), highlight_positions.to_vec())
                .size(label_size)
                .when(!is_active, |this| this.color(Color::Muted))
                .when(!opaque_window, |this| this.truncate())
                .into_any_element()
        };

        let color = cx.theme().colors();
        let base_bg = color.editor_background;

        let hover_base = match design_system.contrast {
            settings::CanvasContrast::Low => color
                .element_active
                .blend(color.element_background.opacity(0.12)),
            settings::CanvasContrast::Standard => color
                .element_active
                .blend(color.element_background.opacity(0.2)),
            settings::CanvasContrast::High => color
                .element_active
                .blend(color.border_focused.opacity(0.12)),
        };
        let hover_solid = base_bg.blend(hover_base);
        let active_background = match design_system.contrast {
            settings::CanvasContrast::Low => color.element_active.opacity(0.55),
            settings::CanvasContrast::Standard => color.element_active,
            settings::CanvasContrast::High => color
                .element_active
                .blend(color.border_focused.opacity(0.16)),
        };
        let focused_border_color = if design_system.is_low_contrast() {
            color.border_variant
        } else {
            color.border_focused
        };

        let group_name_for_gradient = group_name.clone();
        let gradient_overlay = move || {
            GradientFade::new(base_bg, hover_solid, hover_solid)
                .width(px(92.0))
                .right(px(-2.0))
                .gradient_stop(0.7)
                .when(!has_filter, |this| {
                    this.group_name(group_name_for_gradient.clone())
                })
        };

        let header = h_flex()
            .id(id)
            .when(!is_sticky, |this| {
                this.role(gpui::Role::ListItem)
                    .aria_label(workspace_accessibility_label)
                    .aria_selected(is_active)
                    .aria_expanded(!is_collapsed)
                    .when(is_focused, |this| this.aria_active_descendant())
            })
            .group(&group_name)
            .when(!has_filter, |this| this.cursor_pointer())
            .relative()
            .h(header_height)
            .w_full()
            .pl(header_padding_left)
            .pr(header_padding_right)
            .justify_between()
            .border_1()
            .map(|this| {
                if is_focused {
                    this.border_color(focused_border_color)
                } else {
                    this.border_color(gpui::transparent_black())
                }
            })
            .when(is_active, |this| this.bg(active_background))
            .when(
                design_system.radius == settings::CanvasRadius::Subtle,
                |this| this.rounded_sm(),
            )
            .when(
                design_system.radius == settings::CanvasRadius::Rounded,
                |this| this.rounded_lg(),
            )
            .when(!has_filter, |this| this.hover(|s| s.bg(hover_solid)))
            .child(
                h_flex()
                    .relative()
                    .min_w_0()
                    .w_full()
                    .gap(header_gap)
                    .when(!labels_visible, |this| {
                        this.child(
                            Icon::new(IconName::FolderOpen)
                                .size(
                                    if design_system.density == settings::CanvasDensity::Compact {
                                        IconSize::XSmall
                                    } else {
                                        IconSize::Small
                                    },
                                )
                                .color(Color::Muted),
                        )
                    })
                    .when(labels_visible, |this| this.child(label))
                    .when_some(
                        layout_label.filter(|_| labels_visible),
                        |this, layout_label| {
                            this.child(
                                Label::new(SharedString::from(layout_label.as_ref()))
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted)
                                    .truncate(),
                            )
                        },
                    )
                    .when_some(
                        self.render_remote_project_icon(ix, host.as_ref()),
                        |this, icon| this.child(icon),
                    )
                    .when(is_collapsed, |this| {
                        this.when(has_running_threads, |this| {
                            this.child(
                                Icon::new(IconName::LoadCircle)
                                    .size(IconSize::XSmall)
                                    .color(Color::Muted)
                                    .with_rotate_animation(2),
                            )
                        })
                        .when(show_agent_attention && attention_thread_count > 0, |this| {
                            let tooltip_text = if attention_thread_count == 1 {
                                "1 session needs attention".to_string()
                            } else {
                                format!("{attention_thread_count} sessions need attention")
                            };
                            this.child(
                                div()
                                    .id(format!("{id_prefix}waiting-indicator-{ix}"))
                                    .child(
                                        Icon::new(IconName::Warning)
                                            .size(IconSize::XSmall)
                                            .color(Color::Warning),
                                    )
                                    .tooltip(Tooltip::text(tooltip_text)),
                            )
                        })
                        .when(
                            show_agent_attention
                                && session_rail_settings.show_latest_attention_metadata
                                && has_notifications
                                && !has_running_threads
                                && attention_thread_count == 0,
                            |this| {
                                this.child(
                                    Icon::new(IconName::Circle)
                                        .size(IconSize::Small)
                                        .color(Color::Accent),
                                )
                            },
                        )
                    })
                    .when(!has_filter, |this| {
                        this.child(
                            div().child(
                                Icon::new(disclosure_icon)
                                    .size(IconSize::Small)
                                    .color(Color::Muted),
                            ),
                        )
                    }),
            )
            .children(opaque_window.then(|| gradient_overlay()))
            .child(
                h_flex()
                    .gap(px(1.0))
                    .pr(header_padding_right)
                    .children(opaque_window.then(|| gradient_overlay()))
                    .child(self.render_new_session_button(
                        ix,
                        id_prefix,
                        key,
                        &group_name,
                        &workspace_name,
                        is_active,
                        cx,
                    ))
                    .child(self.render_project_header_ellipsis_menu(
                        ix,
                        id_prefix,
                        key,
                        is_active,
                        has_threads,
                        &group_name,
                        &workspace_name,
                        cx,
                    ))
                    .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| {
                        cx.stop_propagation();
                    }),
            )
            .on_mouse_down(gpui::MouseButton::Right, {
                let menu_handle = self
                    .project_header_menu_handles
                    .get(&ix)
                    .cloned()
                    .unwrap_or_default();
                move |_, window, cx| {
                    cx.stop_propagation();
                    menu_handle.toggle(window, cx);
                }
            })
            .on_click(
                cx.listener(move |this, event: &gpui::ClickEvent, window, cx| {
                    if event.modifiers().secondary() {
                        this.activate_or_open_workspace_for_group(&key_for_focus, window, cx);
                    } else if !this.has_filter_query(cx) {
                        this.toggle_collapse(&key_for_toggle, window, cx);
                    }
                }),
            )
            .block_mouse_except_scroll();

        if labels_visible && !is_collapsed && !has_threads {
            v_flex()
                .w_full()
                .child(header)
                .child(
                    v_flex()
                        .px_2()
                        .pt_1()
                        .pb_2()
                        .gap_1()
                        .child(
                            h_flex()
                                .min_w_0()
                                .gap_1p5()
                                .child(Icon::new(IconName::Circle).size(IconSize::XSmall).color(
                                    Color::Custom(
                                        cx.theme().colors().icon_placeholder.opacity(0.2),
                                    ),
                                ))
                                .child(
                                    Label::new("Ready for a session")
                                        .size(LabelSize::XSmall)
                                        .color(Color::Placeholder)
                                        .truncate(),
                                ),
                        )
                        .child(
                            Button::new(
                                SharedString::from(format!(
                                    "{id_prefix}empty-project-new-terminal-{ix}"
                                )),
                                "New Terminal",
                            )
                            .full_width()
                            .size(ButtonSize::Medium)
                            .style(ButtonStyle::OutlinedCustom(cx.theme().colors().border))
                            .start_icon(Icon::new(IconName::Terminal).size(IconSize::XSmall))
                            .aria_label(SharedString::from(format!(
                                "New Terminal in {}",
                                workspace_name.as_ref()
                            )))
                            .tooltip(|_, cx| {
                                Tooltip::for_action(
                                    "New Terminal in This Workspace",
                                    &NewCenterTerminal::default(),
                                    cx,
                                )
                            })
                            .on_click(cx.listener(
                                move |this, _, window, cx| {
                                    this.set_group_expanded(&key_for_empty_terminal, true, cx);
                                    this.selection = None;
                                    if let Some(workspace) =
                                        this.workspace_for_group(&key_for_empty_terminal, cx)
                                    {
                                        this.create_new_terminal(&workspace, window, cx);
                                    } else {
                                        this.open_workspace_and_create_entry(
                                            &key_for_empty_terminal,
                                            NewEntryTarget::Terminal,
                                            window,
                                            cx,
                                        );
                                    }
                                },
                            )),
                        ),
                )
                .into_any_element()
        } else {
            header.into_any_element()
        }
    }

    fn render_new_session_button(
        &self,
        ix: usize,
        id_prefix: &str,
        key: &ProjectGroupKey,
        group_name: &SharedString,
        workspace_name: &SharedString,
        is_active: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let focus_handle = self.focus_handle.clone();

        let menu_handle = self
            .project_header_new_thread_menu_handles
            .get(&ix)
            .cloned()
            .unwrap_or_default();
        let is_menu_open = menu_handle.is_deployed();
        let new_terminal_label =
            SharedString::from(workspace_new_terminal_control_label(workspace_name));

        let button = IconButton::new(
            SharedString::from(format!("{id_prefix}workspace-new-session-{ix}")),
            IconName::Terminal,
        )
        .size(ButtonSize::Medium)
        .selected_style(ButtonStyle::Tinted(TintColor::Accent))
        .icon_size(IconSize::Small)
        .aria_label(new_terminal_label.clone())
        .when(
            !workspace_new_terminal_action_persistent(is_active, is_menu_open),
            |this| this.visible_on_hover(group_name),
        );

        let open_workspaces = self
            .multi_workspace
            .upgrade()
            .and_then(|mw| mw.read(cx).workspaces_for_project_group(key, cx))
            .unwrap_or_default();

        if open_workspaces.is_empty() {
            let key = key.clone();
            let new_terminal_label = new_terminal_label.clone();
            return button
                .tooltip(move |_, cx| {
                    Tooltip::for_action_in(
                        new_terminal_label.clone(),
                        &NewSessionInGroup,
                        &focus_handle,
                        cx,
                    )
                })
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.set_group_expanded(&key, true, cx);
                    this.selection = None;
                    if let Some(workspace) = this.workspace_for_group(&key, cx) {
                        this.create_new_terminal(&workspace, window, cx);
                    } else {
                        this.open_workspace_and_create_entry(
                            &key,
                            default_new_session_target(),
                            window,
                            cx,
                        );
                    }
                }))
                .into_any_element();
        }

        let this = cx.weak_entity();
        let key = key.clone();
        let new_terminal_tooltip = new_terminal_label.clone();

        PopoverMenu::new(SharedString::from(format!(
            "{id_prefix}workspace-new-session-menu-{ix}"
        )))
        .with_handle(menu_handle)
        .trigger_with_tooltip(button, move |_, cx| {
            Tooltip::for_action_in(
                new_terminal_tooltip.clone(),
                &NewSessionInGroup,
                &focus_handle,
                cx,
            )
        })
        .anchor(gpui::Anchor::TopLeft)
        .on_open(Rc::new({
            let this = this.clone();
            move |_window, cx| {
                this.update(cx, |_sidebar, cx| cx.notify()).ok();
            }
        }))
        .menu(move |window, cx| {
            let this = this.clone();
            let key = key.clone();
            let open_workspaces = open_workspaces.clone();
            let active_workspace = this
                .read_with(cx, |sidebar, cx| {
                    sidebar
                        .multi_workspace
                        .upgrade()
                        .map(|mw| mw.read(cx).workspace().clone())
                })
                .ok()
                .flatten();
            let workspace_labels: Vec<_> = open_workspaces
                .iter()
                .map(|workspace| workspace_menu_worktree_labels(workspace, cx))
                .collect();

            Some(ContextMenu::build(
                window,
                cx,
                move |mut menu, _window, cx| {
                    menu = menu.header("New Terminal In…");

                    for (workspace, labels) in open_workspaces
                        .iter()
                        .cloned()
                        .zip(workspace_labels.iter().cloned())
                    {
                        let is_active_workspace = active_workspace.as_ref() == Some(&workspace);
                        menu = menu.custom_entry(
                            move |_window, _cx| {
                                h_flex()
                                    .w_full()
                                    .gap_2()
                                    .justify_between()
                                    .child(h_flex().min_w_0().gap_1().children(
                                        labels.iter().enumerate().map(|(label_ix, label)| {
                                            h_flex()
                                                .gap_1()
                                                .when(label_ix > 0, |this| {
                                                    this.child(Label::new("•").alpha(0.25))
                                                })
                                                .child(label.render())
                                                .into_any_element()
                                        }),
                                    ))
                                    .when(is_active_workspace, |this| {
                                        this.child(
                                            Icon::new(IconName::Check)
                                                .size(IconSize::Small)
                                                .color(Color::Accent),
                                        )
                                    })
                                    .into_any_element()
                            },
                            {
                                let this = this.clone();
                                let key = key.clone();
                                let workspace = workspace.clone();
                                move |window, cx| {
                                    this.update(cx, |sidebar, cx| {
                                        sidebar.set_group_expanded(&key, true, cx);
                                        sidebar.selection = None;
                                        sidebar.create_new_terminal(&workspace, window, cx);
                                    })
                                    .ok();
                                }
                            },
                        );
                    }

                    let base_workspace = active_workspace
                        .as_ref()
                        .filter(|workspace| open_workspaces.contains(workspace))
                        .cloned()
                        .or_else(|| open_workspaces.first().cloned());

                    // Only offer worktree creation when the base project can
                    // actually create one; otherwise the submenu would expand to
                    // nothing. Mirrors the picker's `creation_blocked_reason`.
                    let creation_blocked = base_workspace.as_ref().is_none_or(|base_workspace| {
                        let project = base_workspace.read(cx).project().read(cx);
                        project.is_via_collab() || project.repositories(cx).is_empty()
                    });

                    if let Some(base_workspace) = base_workspace.filter(|_| !creation_blocked) {
                        menu = menu.separator().submenu("Create New Worktree…", {
                            let this = this.clone();
                            move |mut submenu, _window, submenu_cx| {
                                let project = base_workspace.read(submenu_cx).project().clone();
                                let project_ref = project.read(submenu_cx);
                                let has_multiple_repositories =
                                    project_ref.repositories(submenu_cx).len() > 1;
                                let current_branch =
                                    project_ref.active_repository(submenu_cx).and_then(|repo| {
                                        repo.read(submenu_cx)
                                            .branch
                                            .as_ref()
                                            .map(|branch| branch.name().to_string())
                                    });
                                let default_branch = this
                                    .read_with(submenu_cx, |sidebar, _| {
                                        match sidebar.worktree_default_branches.get(&key) {
                                            Some(DefaultBranchCache::Resolved(branch)) => {
                                                branch.clone()
                                            }
                                            _ => None,
                                        }
                                    })
                                    .ok()
                                    .flatten();

                                let targets = worktree_create_targets(
                                    has_multiple_repositories,
                                    default_branch,
                                    current_branch.as_deref(),
                                );
                                for target in targets {
                                    let label = format!(
                                        "Based on {}",
                                        target.branch_label(
                                            has_multiple_repositories,
                                            current_branch.as_deref(),
                                        )
                                    );
                                    let branch_target = target.branch_target();
                                    let workspace = base_workspace.clone();
                                    submenu = submenu.entry(label, None, move |window, cx| {
                                        create_worktree_in_workspace(
                                            &workspace,
                                            branch_target.clone(),
                                            window,
                                            cx,
                                        );
                                    });
                                }

                                submenu
                            }
                        });
                    }

                    menu
                },
            ))
        })
        .anchor(gpui::Anchor::TopRight)
        .offset(gpui::Point {
            x: px(0.),
            y: px(1.),
        })
        .into_any_element()
    }

    // Warms `worktree_default_branches` for every project group with at least one
    // open workspace. The git query runs off the menu path so the submenu can read
    // the result synchronously when it opens. Worktrees of a repository share the
    // same default branch, so any workspace in the group yields the same answer.
    fn prefetch_worktree_default_branches(&mut self, cx: &mut Context<Self>) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };
        let keys: Vec<ProjectGroupKey> = self
            .contents
            .entries
            .iter()
            .filter_map(|entry| match entry {
                ListEntry::ProjectHeader { key, .. } => Some(key.clone()),
                _ => None,
            })
            .collect();
        for key in keys {
            if self.worktree_default_branches.contains_key(&key) {
                continue;
            }
            let Some(base) = multi_workspace
                .read(cx)
                .workspaces_for_project_group(&key, cx)
                .and_then(|workspaces| workspaces.first().cloned())
            else {
                continue;
            };
            self.prefetch_worktree_default_branch(&key, &base, cx);
        }
    }

    fn prefetch_worktree_default_branch(
        &mut self,
        key: &ProjectGroupKey,
        workspace: &Entity<Workspace>,
        cx: &mut Context<Self>,
    ) {
        // Presence of the key means the group is already pending or resolved. The
        // no-repository case is deliberately not inserted so it retries on a
        // later rebuild once the repository has finished loading.
        if self.worktree_default_branches.contains_key(key) {
            return;
        }
        let Some(repository) = workspace.read(cx).project().read(cx).active_repository(cx) else {
            return;
        };
        let request = repository.update(cx, |repository, _| repository.default_branch(true));
        self.worktree_default_branches
            .insert(key.clone(), DefaultBranchCache::Pending);
        let key = key.clone();
        cx.spawn(async move |this, cx| {
            let default_branch = request.await.ok().and_then(Result::ok).flatten();
            let parsed = default_branch.as_deref().and_then(RemoteBranchName::parse);
            this.update(cx, |sidebar, cx| {
                sidebar
                    .worktree_default_branches
                    .insert(key, DefaultBranchCache::Resolved(parsed));
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn render_project_header_ellipsis_menu(
        &self,
        ix: usize,
        id_prefix: &str,
        project_group_key: &ProjectGroupKey,
        is_active: bool,
        has_threads: bool,
        group_name: &SharedString,
        workspace_name: &SharedString,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let multi_workspace = self.multi_workspace.clone();
        let project_group_key = project_group_key.clone();

        let show_multi_project_entries = multi_workspace
            .read_with(cx, |mw, _| {
                project_group_key.host().is_none() && mw.project_group_keys().len() >= 2
            })
            .unwrap_or(false);

        let this = cx.weak_entity();

        let trigger_id = SharedString::from(format!("{id_prefix}-ellipsis-menu-{ix}"));
        let menu_handle = self
            .project_header_menu_handles
            .get(&ix)
            .cloned()
            .unwrap_or_default();
        let is_menu_open = menu_handle.is_deployed();
        let workspace_options_label =
            SharedString::from(workspace_options_control_label(workspace_name));

        PopoverMenu::new(format!("{id_prefix}project-header-menu-{ix}"))
            .with_handle(menu_handle)
            .trigger(
                IconButton::new(trigger_id, IconName::Ellipsis)
                    .size(ButtonSize::Medium)
                    .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                    .icon_size(IconSize::Small)
                    .aria_label(workspace_options_label.clone())
                    .tooltip(Tooltip::text(workspace_options_label))
                    .when(
                        !workspace_options_action_persistent(is_active, is_menu_open),
                        |el| el.visible_on_hover(group_name),
                    ),
            )
            .on_open(Rc::new({
                let this = this.clone();
                move |_window, cx| {
                    this.update(cx, |sidebar, cx| {
                        sidebar.project_header_menu_ix = Some(ix);
                        cx.notify();
                    })
                    .ok();
                }
            }))
            .menu(move |window, cx| {
                let multi_workspace = multi_workspace.clone();
                let project_group_key = project_group_key.clone();
                let this_for_menu = this.clone();

                let open_workspaces = multi_workspace
                    .read_with(cx, |multi_workspace, cx| {
                        multi_workspace
                            .workspaces_for_project_group(&project_group_key, cx)
                            .unwrap_or_default()
                    })
                    .unwrap_or_default();

                // Compute reorder state at menu-open time so it reflects the
                // most recent group ordering.
                let (group_index, total_groups) = multi_workspace
                    .read_with(cx, |mw, _| {
                        let keys = mw.project_group_keys();
                        let index = keys.iter().position(|k| k == &project_group_key);
                        (index, keys.len())
                    })
                    .unwrap_or((None, 0));
                let show_reorder_entries = total_groups >= 2;
                let can_move_up = group_index.is_some_and(|i| i > 0);
                let can_move_down = group_index.is_some_and(|i| i + 1 < total_groups);

                let active_workspace = multi_workspace
                    .read_with(cx, |multi_workspace, _cx| {
                        multi_workspace.workspace().clone()
                    })
                    .ok();
                let workspace_labels: Vec<_> = open_workspaces
                    .iter()
                    .map(|workspace| workspace_menu_worktree_labels(workspace, cx))
                    .collect();
                let workspace_is_active: Vec<_> = open_workspaces
                    .iter()
                    .map(|workspace| active_workspace.as_ref() == Some(workspace))
                    .collect();
                let closable_workspaces: Vec<_> = open_workspaces
                    .iter()
                    .cloned()
                    .zip(workspace_labels.iter())
                    .zip(workspace_is_active.iter().copied())
                    .filter_map(|((workspace, labels), is_active_workspace)| {
                        (!is_active_workspace)
                            .then(|| (workspace, workspace_menu_worktree_accessible_name(labels)))
                    })
                    .collect();

                let menu =
                    ContextMenu::build_persistent(window, cx, move |menu, _window, menu_cx| {
                        let menu = menu.end_slot_action(Box::new(menu::SecondaryConfirm));
                        let weak_menu = menu_cx.weak_entity();

                        let new_agent_key = project_group_key.clone();
                        let new_agent_sidebar = this_for_menu.clone();
                        let new_agent_menu = weak_menu.clone();
                        let menu = menu
                            .entry(
                                agent_session_label(
                                    APP_NAME,
                                    "New Agent Thread",
                                    "New Agent Session",
                                ),
                                Some(Box::new(NewThreadInGroup)),
                                move |window, cx| {
                                    new_agent_sidebar
                                        .update(cx, |sidebar, cx| {
                                            sidebar.set_group_expanded(&new_agent_key, true, cx);
                                            sidebar.selection = None;
                                            if let Some(workspace) =
                                                sidebar.workspace_for_group(&new_agent_key, cx)
                                            {
                                                sidebar.create_new_thread(&workspace, window, cx);
                                            } else {
                                                sidebar.open_workspace_and_create_entry(
                                                    &new_agent_key,
                                                    NewEntryTarget::AgentThread,
                                                    window,
                                                    cx,
                                                );
                                            }
                                        })
                                        .ok();
                                    new_agent_menu
                                        .update(cx, |_, cx| cx.emit(DismissEvent))
                                        .ok();
                                },
                            )
                            .separator();

                        let menu = menu.when(is_active, |menu| {
                            menu.submenu("Canvas Layout", |menu, _window, _cx| {
                                menu.action("Full", Box::new(title_bar::ApplyCanvasFullLayout))
                                    .action(
                                        "Agent Control",
                                        Box::new(title_bar::ApplyCanvasAgentControlLayout),
                                    )
                                    .action(
                                        "Focus Editor",
                                        Box::new(title_bar::ApplyCanvasEditorFocusLayout),
                                    )
                                    .action(
                                        "Code, Run, Observe",
                                        Box::new(title_bar::ApplyCanvasCodeRunObserveLayout),
                                    )
                                    .action("Review", Box::new(title_bar::ApplyCanvasReviewLayout))
                                    .action("Debug", Box::new(title_bar::ApplyCanvasDebugLayout))
                                    .separator()
                                    .action("Cycle Layout", Box::new(title_bar::CycleCanvasLayout))
                                    .action(
                                        "Save Layout As…",
                                        Box::new(workspace::SaveCurrentCanvasLayoutAs),
                                    )
                                    .action(
                                        "Manage Saved Layouts…",
                                        Box::new(workspace::ManageSavedCanvasLayouts),
                                    )
                                    .action(
                                        "Restore Previous Layout",
                                        Box::new(title_bar::RestorePreviousCanvasLayout),
                                    )
                            })
                            .separator()
                        });

                        let menu = menu.when(show_multi_project_entries, |this| {
                            this.entry(
                                "Open Workspace in New Window",
                                Some(Box::new(workspace::MoveProjectToNewWindow)),
                                {
                                    let project_group_key = project_group_key.clone();
                                    let multi_workspace = multi_workspace.clone();
                                    move |window, cx| {
                                        multi_workspace
                                            .update(cx, |multi_workspace, cx| {
                                                multi_workspace
                                                    .open_project_group_in_new_window(
                                                        &project_group_key,
                                                        window,
                                                        cx,
                                                    )
                                                    .detach_and_log_err(cx);
                                            })
                                            .ok();
                                    }
                                },
                            )
                        });

                        let menu = menu
                            .custom_entry(
                                {
                                    move |_window, cx| {
                                        let action = h_flex()
                                            .opacity(0.6)
                                            .children(render_modifiers(
                                                &Modifiers::secondary_key(),
                                                PlatformStyle::platform(),
                                                None,
                                                Some(TextSize::Default.rems(cx).into()),
                                                false,
                                            ))
                                            .child(Label::new("-click").color(Color::Muted));

                                        let label = if has_threads {
                                            "Focus Last Workspace"
                                        } else {
                                            "Focus Workspace"
                                        };

                                        h_flex()
                                            .w_full()
                                            .justify_between()
                                            .gap_4()
                                            .child(
                                                Label::new(label)
                                                    .when(is_active, |s| s.color(Color::Disabled)),
                                            )
                                            .child(action)
                                            .into_any_element()
                                    }
                                },
                                {
                                    let project_group_key = project_group_key.clone();
                                    let this = this_for_menu.clone();
                                    move |window, cx| {
                                        if is_active {
                                            return;
                                        }
                                        this.update(cx, |sidebar, cx| {
                                            if let Some(workspace) =
                                                sidebar.workspace_for_group(&project_group_key, cx)
                                            {
                                                sidebar.activate_workspace(&workspace, window, cx);
                                            } else {
                                                sidebar.open_workspace_for_group(
                                                    &project_group_key,
                                                    window,
                                                    cx,
                                                );
                                            }
                                            sidebar.selection = None;
                                            sidebar.active_entry = None;
                                        })
                                        .ok();
                                    }
                                },
                            )
                            .selectable(!is_active);

                        let menu = if open_workspaces.is_empty() {
                            menu
                        } else {
                            let mut menu = menu.separator().header("Open Worktrees");

                            for (
                                workspace_index,
                                ((workspace, workspace_label), is_active_workspace),
                            ) in open_workspaces
                                .iter()
                                .cloned()
                                .zip(workspace_labels.iter().cloned())
                                .zip(workspace_is_active.iter().copied())
                                .enumerate()
                            {
                                let activate_multi_workspace = multi_workspace.clone();
                                let close_multi_workspace = multi_workspace.clone();
                                let activate_weak_menu = weak_menu.clone();
                                let close_weak_menu = weak_menu.clone();
                                let activate_workspace = workspace.clone();
                                let close_workspace = workspace.clone();

                                menu = menu.custom_entry(
                                    move |_window, _cx| {
                                        let close_multi_workspace = close_multi_workspace.clone();
                                        let close_weak_menu = close_weak_menu.clone();
                                        let close_workspace = close_workspace.clone();
                                        let row_group_name = SharedString::from(format!(
                                            "workspace-menu-row-{workspace_index}"
                                        ));

                                        h_flex()
                                            .group(&row_group_name)
                                            .w_full()
                                            .gap_2()
                                            .justify_between()
                                            .child(h_flex().min_w_0().gap_1().children(
                                                workspace_label.iter().enumerate().map(
                                                    |(label_ix, label)| {
                                                        h_flex()
                                                            .gap_1()
                                                            .when(label_ix > 0, |this| {
                                                                this.child(
                                                                    Label::new("•").alpha(0.25),
                                                                )
                                                            })
                                                            .child(label.render())
                                                            .into_any_element()
                                                    },
                                                ),
                                            ))
                                            .when(is_active_workspace, |this| {
                                                this.pr_1().child(
                                                    Icon::new(IconName::Check)
                                                        .size(IconSize::Small)
                                                        .color(Color::Accent),
                                                )
                                            })
                                            .when(!is_active_workspace, |this| {
                                                let close_multi_workspace =
                                                    close_multi_workspace.clone();
                                                let close_weak_menu = close_weak_menu.clone();
                                                let close_workspace = close_workspace.clone();

                                                this.child(
                                                    IconButton::new(
                                                        ("close-workspace", workspace_index),
                                                        IconName::Close,
                                                    )
                                                    .size(ButtonSize::Medium)
                                                    .icon_size(IconSize::Small)
                                                    .aria_label("Close Worktree from Window")
                                                    .visible_on_hover(&row_group_name)
                                                    .tooltip(Tooltip::text(
                                                        "Close Worktree from Window",
                                                    ))
                                                    .on_click(move |_, window, cx| {
                                                        cx.stop_propagation();
                                                        window.prevent_default();
                                                        close_multi_workspace
                                                            .update(cx, |multi_workspace, cx| {
                                                                multi_workspace
                                                                    .close_workspace(
                                                                        &close_workspace,
                                                                        window,
                                                                        cx,
                                                                    )
                                                                    .detach_and_log_err(cx);
                                                            })
                                                            .ok();
                                                        close_weak_menu
                                                            .update(cx, |_, cx| {
                                                                cx.emit(DismissEvent)
                                                            })
                                                            .ok();
                                                    }),
                                                )
                                            })
                                            .into_any_element()
                                    },
                                    move |window, cx| {
                                        activate_multi_workspace
                                            .update(cx, |multi_workspace, cx| {
                                                multi_workspace.activate(
                                                    activate_workspace.clone(),
                                                    None,
                                                    window,
                                                    cx,
                                                );
                                            })
                                            .ok();
                                        activate_weak_menu
                                            .update(cx, |_, cx| cx.emit(DismissEvent))
                                            .ok();
                                    },
                                );
                            }

                            menu
                        };

                        let menu = menu.when(!closable_workspaces.is_empty(), |this| {
                            let closable_workspaces = closable_workspaces.clone();
                            let multi_workspace = multi_workspace.clone();
                            this.separator().submenu(
                                "Close Worktree from Window…",
                                move |mut submenu, _window, _cx| {
                                    for (workspace, label) in closable_workspaces.iter().cloned() {
                                        let multi_workspace = multi_workspace.clone();
                                        submenu = submenu.entry(label, None, move |window, cx| {
                                            multi_workspace
                                                .update(cx, |multi_workspace, cx| {
                                                    multi_workspace
                                                        .close_workspace(&workspace, window, cx)
                                                        .detach_and_log_err(cx);
                                                })
                                                .ok();
                                        });
                                    }
                                    submenu
                                },
                            )
                        });

                        let menu = menu.when(show_reorder_entries, |this| {
                            let move_up_multi_workspace = multi_workspace.clone();
                            let move_up_key = project_group_key.clone();
                            let move_up_weak_menu = weak_menu.clone();
                            let move_down_multi_workspace = multi_workspace.clone();
                            let move_down_key = project_group_key.clone();
                            let move_down_weak_menu = weak_menu.clone();

                            this.separator()
                                .item(
                                    ContextMenuEntry::new("Move Up")
                                        .disabled(!can_move_up)
                                        .handler(move |_window, cx| {
                                            move_up_multi_workspace
                                                .update(cx, |mw, cx| {
                                                    mw.move_project_group_up(&move_up_key, cx);
                                                })
                                                .ok();
                                            move_up_weak_menu
                                                .update(cx, |_, cx| cx.emit(DismissEvent))
                                                .ok();
                                        }),
                                )
                                .item(
                                    ContextMenuEntry::new("Move Down")
                                        .disabled(!can_move_down)
                                        .handler(move |_window, cx| {
                                            move_down_multi_workspace
                                                .update(cx, |mw, cx| {
                                                    mw.move_project_group_down(&move_down_key, cx);
                                                })
                                                .ok();
                                            move_down_weak_menu
                                                .update(cx, |_, cx| cx.emit(DismissEvent))
                                                .ok();
                                        }),
                                )
                        });

                        let project_group_key = project_group_key.clone();
                        let remove_multi_workspace = multi_workspace.clone();
                        menu.separator().entry(
                            "Remove Workspace from Window",
                            None,
                            move |window, cx| {
                                remove_multi_workspace
                                    .update(cx, |multi_workspace, cx| {
                                        multi_workspace
                                            .remove_project_group(&project_group_key, window, cx)
                                            .detach_and_log_err(cx);
                                    })
                                    .ok();
                                weak_menu.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
                            },
                        )
                    });

                let this = this.clone();

                window
                    .subscribe(&menu, cx, move |_, _: &gpui::DismissEvent, _window, cx| {
                        this.update(cx, |sidebar, cx| {
                            sidebar.project_header_menu_ix = None;
                            cx.notify();
                        })
                        .ok();
                    })
                    .detach();

                Some(menu)
            })
            .anchor(gpui::Anchor::TopRight)
            .offset(gpui::Point {
                x: px(0.),
                y: px(1.),
            })
            .into_any_element()
    }

    fn render_sticky_header(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let scroll_top = self.list_state.logical_scroll_top();

        let &header_idx = self
            .contents
            .project_header_indices
            .iter()
            .rev()
            .find(|&&idx| idx <= scroll_top.item_ix)?;

        let needs_sticky = header_idx < scroll_top.item_ix
            || (header_idx == scroll_top.item_ix && scroll_top.offset_in_item > px(0.));

        if !needs_sticky {
            return None;
        }

        let ListEntry::ProjectHeader {
            key,
            label,
            highlight_positions,
            layout_label,
            has_running_threads,
            attention_thread_count,
            has_notifications,
            is_active,
            has_threads,
        } = self.contents.entries.get(header_idx)?
        else {
            return None;
        };

        let is_focused = self.focus_handle.is_focused(window);
        let is_selected = is_focused && self.selection == Some(header_idx);

        let header_element = self.render_project_header(
            header_idx,
            true,
            key,
            &label,
            &highlight_positions,
            layout_label.as_ref(),
            *has_running_threads,
            *attention_thread_count,
            *has_notifications,
            *is_active,
            is_selected,
            *has_threads,
            cx,
        );

        let top_offset = self
            .contents
            .project_header_indices
            .iter()
            .find(|&&idx| idx > header_idx)
            .and_then(|&next_idx| {
                let bounds = self.list_state.bounds_for_item(next_idx)?;
                let viewport = self.list_state.viewport_bounds();
                let y_in_viewport = bounds.origin.y - viewport.origin.y;
                let header_height = bounds.size.height;
                (y_in_viewport < header_height).then_some(y_in_viewport - header_height)
            })
            .unwrap_or(px(0.));

        let color = cx.theme().colors();
        let background = color.editor_background;

        let element = v_flex()
            .absolute()
            .top(top_offset)
            .left_0()
            .w_full()
            .bg(background)
            .border_b_1()
            .border_color(color.border.opacity(0.5))
            .child(header_element)
            .shadow_sm()
            .into_any_element();

        Some(element)
    }

    fn toggle_collapse(
        &mut self,
        project_group_key: &ProjectGroupKey,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let is_collapsed = self.is_group_collapsed(project_group_key, cx);
        self.set_group_expanded(project_group_key, is_collapsed, cx);
        self.update_entries(cx);
    }

    fn dispatch_context(&self, window: &Window, cx: &Context<Self>) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("Sidebar");
        dispatch_context.add("menu");

        let is_renaming_thread = self
            .thread_rename_editor
            .focus_handle(cx)
            .is_focused(window);

        let is_searching = self.filter_editor.focus_handle(cx).is_focused(window)
            || matches!(
                &self.view,
                SidebarView::Archive(archive)
                    if archive.read(cx).is_filter_editor_focused(window, cx)
            );

        let identifier = if is_searching {
            "searching"
        } else if is_renaming_thread {
            "editing"
        } else {
            "not_searching"
        };

        dispatch_context.add(identifier);
        dispatch_context
    }

    fn focus_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.focus_handle.is_focused(window) {
            return;
        }

        cx.notify();
    }

    fn cancel(&mut self, _: &Cancel, window: &mut Window, cx: &mut Context<Self>) {
        if self.renaming_thread_id.is_some() {
            self.finish_thread_rename(window, cx);
            return;
        }

        if self.filter_editor.read(cx).is_focused(window) {
            if self.reset_filter_editor_text(window, cx) {
                self.selection = None;
                self.update_entries(cx);
                return;
            }

            if self.selection.is_none() {
                self.select_first_entry();
            }
            if self.selection.is_some() {
                self.focus_handle.focus(window, cx);
                cx.notify();
            }
            return;
        }

        if self.reset_filter_editor_text(window, cx) {
            self.update_entries(cx);
        } else {
            self.selection = None;
            self.filter_editor.focus_handle(cx).focus(window, cx);
            cx.notify();
        }
    }

    fn focus_sidebar_filter(
        &mut self,
        _: &FocusSidebarFilter,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selection = None;
        if let SidebarView::Archive(archive) = &self.view {
            archive.update(cx, |view, cx| {
                view.clear_selection();
                view.focus_filter_editor(window, cx);
            });
        } else {
            self.filter_editor.focus_handle(cx).focus(window, cx);
        }

        cx.notify();
    }

    fn reset_filter_editor_text(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        self.filter_editor.update(cx, |editor, cx| {
            if editor.buffer().read(cx).len(cx).0 > 0 {
                editor.set_text("", window, cx);
                true
            } else {
                false
            }
        })
    }

    fn has_filter_query(&self, cx: &App) -> bool {
        !self.filter_editor.read(cx).text(cx).is_empty()
    }

    fn start_renaming_thread(
        &mut self,
        ix: usize,
        thread_id: ThreadId,
        title: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.renaming_thread_id.is_some() && self.renaming_thread_id != Some(thread_id) {
            self.finish_thread_rename(window, cx);
        }

        self.selection = Some(ix);
        self.renaming_thread_id = Some(thread_id);
        self.suppress_next_rename_edit = true;
        self.list_state.scroll_to_reveal_item(ix);
        self.thread_rename_editor.update(cx, |editor, cx| {
            editor.set_text(title, window, cx);
            editor.select_all(&editor::actions::SelectAll, window, cx);
            editor.focus_handle(cx).focus(window, cx);
        });
        cx.notify();
    }

    fn handle_thread_rename_editor_event(
        &mut self,
        title_editor: &Entity<Editor>,
        event: &editor::EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            editor::EditorEvent::BufferEdited => {
                if self.suppress_next_rename_edit {
                    self.suppress_next_rename_edit = false;
                    return;
                }
                if !title_editor.read(cx).is_focused(window) {
                    return;
                }
                let new_title = title_editor.read(cx).text(cx);
                if new_title.is_empty() {
                    return;
                }
                let Some(thread_id) = self.renaming_thread_id else {
                    return;
                };
                self.apply_thread_rename(thread_id, SharedString::from(new_title), window, cx);
            }
            editor::EditorEvent::Blurred => {
                self.finish_thread_rename(window, cx);
            }
            _ => {}
        }
    }

    fn apply_thread_rename(
        &mut self,
        thread_id: ThreadId,
        title: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut found = false;
        if let Some(multi_workspace) = self.multi_workspace.upgrade() {
            let workspaces: Vec<_> = multi_workspace.read(cx).workspaces().cloned().collect();
            for workspace in workspaces {
                let agent_thread_item = workspace
                    .read(cx)
                    .items_of_type::<AgentThreadItem>(cx)
                    .find(|item| item.read(cx).thread_id(cx) == thread_id);
                if let Some(agent_thread_item) = agent_thread_item
                    && let Some(thread_view) = agent_thread_item
                        .read(cx)
                        .conversation_view()
                        .read(cx)
                        .root_thread_view()
                {
                    thread_view.update(cx, |thread_view, cx| {
                        thread_view.rename(title.clone(), window, cx);
                    });
                    found = true;
                }

                if let Some(agent_panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
                    if let Some(view) = agent_panel
                        .read(cx)
                        .conversation_view_for_id(&thread_id, cx)
                        && let Some(thread_view) = view.read(cx).root_thread_view()
                    {
                        thread_view.update(cx, |thread_view, cx| {
                            thread_view.rename(title.clone(), window, cx);
                        });
                        found = true;
                    }
                }
            }
        }

        if !found {
            ThreadMetadataStore::global(cx).update(cx, |store, cx| {
                store.set_title_override(thread_id, title, cx);
            });
        }
    }

    fn finish_thread_rename(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if self.renaming_thread_id.take().is_none() {
            return false;
        }
        self.focus_handle.focus(window, cx);
        self.update_entries(cx);
        true
    }

    fn editor_move_down(&mut self, _: &MoveDown, window: &mut Window, cx: &mut Context<Self>) {
        self.select_next(&SelectNext, window, cx);
        if self.selection.is_some() {
            self.focus_handle.focus(window, cx);
        }
    }

    fn editor_move_up(&mut self, _: &MoveUp, window: &mut Window, cx: &mut Context<Self>) {
        self.select_previous(&SelectPrevious, window, cx);
        if self.selection.is_some() {
            self.focus_handle.focus(window, cx);
        }
    }

    fn editor_confirm(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.selection.is_none() {
            self.select_next(&SelectNext, window, cx);
        }
        if self.selection.is_some() {
            self.focus_handle.focus(window, cx);
        }
    }

    fn move_selected_entry_up(
        &mut self,
        _: &MoveSelectedEntryUp,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_selected_entry_in_manual_order(true, cx);
    }

    fn move_selected_entry_down(
        &mut self,
        _: &MoveSelectedEntryDown,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_selected_entry_in_manual_order(false, cx);
    }

    fn move_selected_entry_in_manual_order(&mut self, up: bool, cx: &mut Context<Self>) {
        let Some(selected_index) = self.selection else {
            return;
        };
        let Some(selected_key) = self
            .contents
            .entries
            .get(selected_index)
            .and_then(ManualEntryOrderKey::from_entry)
        else {
            return;
        };
        let Some(target_index) = self.manual_order_target_index(selected_index, up) else {
            return;
        };
        let Some(target_key) = self
            .contents
            .entries
            .get(target_index)
            .and_then(ManualEntryOrderKey::from_entry)
        else {
            return;
        };

        self.ensure_manual_entry_order_contains_visible_entries();
        let Some(selected_order_index) = self
            .manual_entry_order
            .iter()
            .position(|key| key == &selected_key)
        else {
            return;
        };
        let Some(target_order_index) = self
            .manual_entry_order
            .iter()
            .position(|key| key == &target_key)
        else {
            return;
        };

        self.manual_entry_order
            .swap(selected_order_index, target_order_index);
        self.selection = Some(target_index);
        self.update_entries(cx);
        self.serialize(cx);
    }

    fn manual_order_target_index(&self, selected_index: usize, up: bool) -> Option<usize> {
        if up {
            for index in (0..selected_index).rev() {
                match self.contents.entries.get(index) {
                    Some(ListEntry::Thread(_) | ListEntry::Terminal(_)) => return Some(index),
                    Some(ListEntry::ProjectHeader { .. }) | None => return None,
                }
            }
            None
        } else {
            for index in selected_index + 1..self.contents.entries.len() {
                match self.contents.entries.get(index) {
                    Some(ListEntry::Thread(_) | ListEntry::Terminal(_)) => return Some(index),
                    Some(ListEntry::ProjectHeader { .. }) | None => return None,
                }
            }
            None
        }
    }

    fn ensure_manual_entry_order_contains_visible_entries(&mut self) {
        let mut seen: HashSet<ManualEntryOrderKey> =
            self.manual_entry_order.iter().cloned().collect();
        for entry in &self.contents.entries {
            if let Some(key) = ManualEntryOrderKey::from_entry(entry)
                && seen.insert(key.clone())
            {
                self.manual_entry_order.push(key);
            }
        }
    }

    fn select_next(&mut self, _: &SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        let next = match self.selection {
            Some(ix) if ix + 1 < self.contents.entries.len() => ix + 1,
            Some(_) if !self.contents.entries.is_empty() => 0,
            None if !self.contents.entries.is_empty() => 0,
            _ => return,
        };
        self.selection = Some(next);
        self.list_state.scroll_to_reveal_item(next);
        cx.notify();
    }

    fn select_previous(&mut self, _: &SelectPrevious, window: &mut Window, cx: &mut Context<Self>) {
        match self.selection {
            Some(0) => {
                self.selection = None;
                self.focus_handle.focus(window, cx);
                cx.notify();
            }
            Some(ix) => {
                self.selection = Some(ix - 1);
                self.list_state.scroll_to_reveal_item(ix - 1);
                cx.notify();
            }
            None if !self.contents.entries.is_empty() => {
                let last = self.contents.entries.len() - 1;
                self.selection = Some(last);
                self.list_state.scroll_to_reveal_item(last);
                cx.notify();
            }
            None => {}
        }
    }

    fn select_first(&mut self, _: &SelectFirst, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.contents.entries.is_empty() {
            self.selection = Some(0);
            self.list_state.scroll_to_reveal_item(0);
            cx.notify();
        }
    }

    fn select_last(&mut self, _: &SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(last) = self.contents.entries.len().checked_sub(1) {
            self.selection = Some(last);
            self.list_state.scroll_to_reveal_item(last);
            cx.notify();
        }
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if self.finish_thread_rename(window, cx) {
            return;
        }

        let Some(ix) = self.selection else { return };
        let Some(entry) = self.contents.entries.get(ix) else {
            return;
        };

        match entry {
            ListEntry::ProjectHeader { key, .. } => {
                let key = key.clone();
                self.toggle_collapse(&key, window, cx);
            }
            ListEntry::Thread(thread) => {
                let metadata = thread.metadata.clone();
                match &thread.workspace {
                    ThreadEntryWorkspace::Open(workspace) => {
                        let workspace = workspace.clone();
                        self.activate_thread(metadata, &workspace, false, window, cx);
                    }
                    ThreadEntryWorkspace::Closed {
                        folder_paths,
                        project_group_key,
                    } => {
                        let folder_paths = folder_paths.clone();
                        let project_group_key = project_group_key.clone();
                        self.open_workspace_and_activate_thread(
                            metadata,
                            folder_paths,
                            &project_group_key,
                            window,
                            cx,
                        );
                    }
                }
            }
            ListEntry::Terminal(terminal) => {
                let metadata = terminal.metadata.clone();
                let workspace = terminal.workspace.clone();
                let source = terminal.source.clone();
                self.activate_terminal_entry(metadata, workspace, source, false, window, cx);
            }
        }
    }

    fn find_workspace_across_windows(
        &self,
        cx: &App,
        predicate: impl Fn(&Entity<Workspace>, &App) -> bool,
    ) -> Option<(WindowHandle<MultiWorkspace>, Entity<Workspace>)> {
        cx.windows()
            .into_iter()
            .filter_map(|window| window.downcast::<MultiWorkspace>())
            .find_map(|window| {
                let workspace = window.read(cx).ok().and_then(|multi_workspace| {
                    multi_workspace
                        .workspaces()
                        .find(|workspace| predicate(workspace, cx))
                        .cloned()
                })?;
                Some((window, workspace))
            })
    }

    fn find_workspace_in_current_window(
        &self,
        cx: &App,
        predicate: impl Fn(&Entity<Workspace>, &App) -> bool,
    ) -> Option<Entity<Workspace>> {
        self.multi_workspace.upgrade().and_then(|multi_workspace| {
            multi_workspace
                .read(cx)
                .workspaces()
                .find(|workspace| predicate(workspace, cx))
                .cloned()
        })
    }

    fn load_agent_thread_in_workspace(
        workspace: &Entity<Workspace>,
        metadata: &ThreadMetadata,
        focus: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        open_agent_thread_in_workspace(workspace, metadata, focus, window, cx);
    }

    fn open_closed_native_thread_as_markdown(
        session_id: &acp::SessionId,
        title: Option<SharedString>,
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let thread_store = ThreadStore::global(cx);
        let load_task =
            thread_store.update(cx, |store, cx| store.load_thread(session_id.clone(), cx));

        let thread_title = title
            .map(|t| t.to_string())
            .unwrap_or_else(|| default_agent_session_title(APP_NAME).to_string());

        let workspace = workspace.clone();

        window
            .spawn(cx, async move |cx| {
                let db_thread = load_task.await?;
                let Some(db_thread) = db_thread else {
                    anyhow::bail!("Thread not found in database");
                };

                let markdown = db_thread.to_markdown();

                cx.update(|window, cx| {
                    agent_ui::open_markdown_in_workspace(
                        thread_title,
                        markdown,
                        workspace,
                        window,
                        cx,
                    )
                })?
                .await
            })
            .detach_and_log_err(cx);
    }

    fn open_run_review_brief(
        brief: RunReviewBrief,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let title = format!(
            "Review: {}",
            util::truncate_and_trailoff(&brief.run_label, 48)
        );
        agent_ui::open_markdown_beside_in_workspace(
            title,
            brief.to_markdown(),
            workspace,
            window,
            cx,
        )
        .detach_and_log_err(cx);
    }

    fn open_terminal_run_review(
        sidebar: WeakEntity<Self>,
        metadata: TerminalThreadMetadata,
        owner_workspace: ThreadEntryWorkspace,
        source: TerminalEntrySource,
        brief: RunReviewBrief,
        review_workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        sidebar
            .update(cx, |sidebar, cx| {
                sidebar.activate_terminal_entry(
                    metadata,
                    owner_workspace,
                    source,
                    false,
                    window,
                    cx,
                );
            })
            .ok();
        Self::open_run_review_brief(brief, review_workspace, window, cx);
    }

    fn open_thread_run_review(
        sidebar: WeakEntity<Self>,
        metadata: ThreadMetadata,
        owner_workspace: ThreadEntryWorkspace,
        brief: RunReviewBrief,
        review_workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        sidebar
            .update(cx, |sidebar, cx| match &owner_workspace {
                ThreadEntryWorkspace::Open(workspace) => {
                    sidebar.activate_thread(metadata.clone(), workspace, false, window, cx);
                }
                ThreadEntryWorkspace::Closed {
                    folder_paths,
                    project_group_key,
                } => {
                    sidebar.open_workspace_and_activate_thread(
                        metadata.clone(),
                        folder_paths.clone(),
                        project_group_key,
                        window,
                        cx,
                    );
                }
            })
            .ok();
        Self::open_run_review_brief(brief, review_workspace, window, cx);
    }

    fn show_thread_title_toast(workspace: Entity<Workspace>, message: &'static str, cx: &mut App) {
        workspace.update(cx, |workspace, cx| {
            let toast = StatusToast::new(message, cx, |this, _cx| {
                this.icon(
                    Icon::new(IconName::Warning)
                        .size(IconSize::Small)
                        .color(Color::Warning),
                )
                .dismiss_button(true)
            });
            workspace.toggle_status_toast(toast, cx);
        });
    }

    fn show_no_thread_summary_model_toast(workspace: Entity<Workspace>, cx: &mut App) {
        Self::show_thread_title_toast(
            workspace,
            agent_session_label(
                APP_NAME,
                "No model is configured for summarizing thread titles.",
                "No model is configured for summarizing Agent Session titles.",
            ),
            cx,
        );
    }

    fn regenerate_thread_title(
        &mut self,
        session_id: &acp::SessionId,
        thread_id: ThreadId,
        folder_paths: PathList,
        thread_workspace: Option<Entity<Workspace>>,
        cx: &mut Context<Self>,
    ) {
        if let Some(panel) = thread_workspace
            .as_ref()
            .and_then(|w| w.read(cx).panel::<AgentPanel>(cx))
        {
            match panel.update(cx, |panel, cx| panel.regenerate_thread_title(thread_id, cx)) {
                ThreadTitleRegenerationResult::Started
                | ThreadTitleRegenerationResult::AlreadyGenerating => return,
                ThreadTitleRegenerationResult::NoModel => {
                    if let Some(workspace) = self.active_workspace(cx) {
                        Self::show_no_thread_summary_model_toast(workspace, cx);
                    }
                    return;
                }
                ThreadTitleRegenerationResult::NotOpen => {}
            }
        }

        let Some(configured_model) =
            LanguageModelRegistry::read_global(cx).thread_summary_model(cx)
        else {
            if let Some(workspace) = self.active_workspace(cx) {
                Self::show_no_thread_summary_model_toast(workspace, cx);
            }
            return;
        };

        if !self.regenerating_titles.insert(thread_id) {
            return;
        }

        let model = configured_model.model;
        let temperature = AgentSettings::temperature_for_model(&model, cx);

        let thread_store = ThreadStore::global(cx);
        let load_task =
            thread_store.update(cx, |store, cx| store.load_thread(session_id.clone(), cx));
        let session_id = session_id.clone();

        cx.notify();

        cx.spawn(async move |this, cx| {
            let result: anyhow::Result<SharedString> = async {
                let Some(db_thread) = load_task.await? else {
                    anyhow::bail!("Thread not found in database");
                };

                let request = agent::build_thread_title_request(&db_thread.messages, temperature);
                let title =
                    SharedString::from(agent::stream_thread_title(model, request, cx).await?);

                let Some(mut db_thread) = thread_store
                    .update(cx, |store, cx| store.load_thread(session_id.clone(), cx))
                    .await?
                else {
                    anyhow::bail!("Thread not found in database");
                };
                db_thread.title = title.clone();

                thread_store
                    .update(cx, |store, cx| {
                        store.save_thread(session_id, db_thread, folder_paths, cx)
                    })
                    .await?;

                anyhow::Ok(title)
            }
            .await;

            this.update(cx, |this, cx| {
                this.regenerating_titles.remove(&thread_id);
                match &result {
                    Ok(title) => {
                        ThreadMetadataStore::global(cx).update(cx, |store, cx| {
                            store.set_generated_title(thread_id, title.clone(), cx);
                        });
                    }
                    Err(_) => {
                        if let Some(workspace) = this.active_workspace(cx) {
                            Self::show_thread_title_toast(
                                workspace,
                                agent_session_label(
                                    APP_NAME,
                                    "Failed to regenerate thread title.",
                                    "Failed to regenerate the Agent Session title.",
                                ),
                                cx,
                            );
                        }
                    }
                }
                cx.notify();
            })
            .ok();

            result.map(|_| ())
        })
        .detach_and_log_err(cx);
    }

    fn is_thread_active_in_workspace(
        &self,
        thread_id: &ThreadId,
        workspace: &Entity<Workspace>,
        cx: &App,
    ) -> bool {
        self.active_workspace(cx).as_ref() == Some(workspace)
            && self.active_entry.as_ref().is_some_and(|entry| {
                entry.is_active_thread(thread_id) && entry.workspace() == workspace
            })
    }

    fn activate_thread_locally(
        &mut self,
        metadata: &ThreadMetadata,
        workspace: &Entity<Workspace>,
        retain: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        if self.is_thread_active_in_workspace(&metadata.thread_id, workspace, cx) {
            Self::load_agent_thread_in_workspace(workspace, metadata, true, window, cx);
            return;
        }

        // Set active_entry eagerly so the sidebar highlight updates
        // immediately, rather than waiting for a deferred item activation
        // event which can race with ActiveWorkspaceChanged clearing it.
        self.active_entry = Some(ActiveEntry::Thread {
            thread_id: metadata.thread_id,
            session_id: metadata.session_id.clone(),
            workspace: workspace.clone(),
        });
        self.record_thread_access(&metadata.thread_id);
        self.pending_thread_activation = Some(metadata.thread_id);

        multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.activate(workspace.clone(), None, window, cx);
            if retain {
                multi_workspace.retain_active_workspace(cx);
            }
        });

        Self::load_agent_thread_in_workspace(workspace, metadata, true, window, cx);

        self.update_entries(cx);
    }

    fn activate_thread_in_other_window(
        &self,
        metadata: ThreadMetadata,
        workspace: Entity<Workspace>,
        target_window: WindowHandle<MultiWorkspace>,
        cx: &mut Context<Self>,
    ) {
        let target_session_id = metadata.session_id.clone();
        let metadata_thread_id = metadata.thread_id;
        let workspace_for_entry = workspace.clone();

        let activated = target_window
            .update(cx, |multi_workspace, window, cx| {
                window.activate_window();
                multi_workspace.activate(workspace.clone(), None, window, cx);
                Self::load_agent_thread_in_workspace(&workspace, &metadata, true, window, cx);
            })
            .log_err()
            .is_some();

        if activated {
            if let Some(target_sidebar) = target_window
                .read(cx)
                .ok()
                .and_then(|multi_workspace| {
                    multi_workspace.sidebar().map(|sidebar| sidebar.to_any())
                })
                .and_then(|sidebar| sidebar.downcast::<Self>().ok())
            {
                target_sidebar.update(cx, |sidebar, cx| {
                    sidebar.pending_thread_activation = Some(metadata_thread_id);
                    sidebar.active_entry = Some(ActiveEntry::Thread {
                        thread_id: metadata_thread_id,
                        session_id: target_session_id.clone(),
                        workspace: workspace_for_entry.clone(),
                    });
                    sidebar.record_thread_access(&metadata_thread_id);
                    sidebar.update_entries(cx);
                });
            }
        }
    }

    fn activate_thread(
        &mut self,
        metadata: ThreadMetadata,
        workspace: &Entity<Workspace>,
        retain: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self
            .find_workspace_in_current_window(cx, |candidate, _| candidate == workspace)
            .is_some()
        {
            self.activate_thread_locally(&metadata, &workspace, retain, window, cx);
            return;
        }

        let Some((target_window, workspace)) =
            self.find_workspace_across_windows(cx, |candidate, _| candidate == workspace)
        else {
            return;
        };

        self.activate_thread_in_other_window(metadata, workspace, target_window, cx);
    }

    fn open_workspace_and_activate_thread(
        &mut self,
        metadata: ThreadMetadata,
        folder_paths: PathList,
        project_group_key: &ProjectGroupKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        let pending_thread_id = metadata.thread_id;
        // Mark the pending thread activation so rebuild_contents
        // preserves the Thread active_entry during loading and
        // reconciliation cannot synthesize an empty fallback draft.
        self.pending_thread_activation = Some(pending_thread_id);

        let host = project_group_key.host();
        let provisional_key = Some(project_group_key.clone());
        let active_workspace = multi_workspace.read(cx).workspace().clone();
        let modal_workspace = active_workspace.clone();

        let open_task = multi_workspace.update(cx, |this, cx| {
            this.find_or_create_workspace(
                folder_paths,
                host,
                provisional_key,
                |options, window, cx| connect_remote(active_workspace, options, window, cx),
                &[],
                None,
                OpenMode::Activate,
                window,
                cx,
            )
        });

        cx.spawn_in(window, async move |this, cx| {
            let result = open_task.await;
            // Dismiss the modal as soon as the open attempt completes so
            // failures or cancellations do not leave a stale connection modal behind.
            remote_connection::dismiss_connection_modal(&modal_workspace, cx);

            if result.is_err() {
                this.update(cx, |this, _cx| {
                    if this.pending_thread_activation == Some(pending_thread_id) {
                        this.pending_thread_activation = None;
                    }
                })
                .ok();
            }

            let workspace = result?;
            this.update_in(cx, |this, window, cx| {
                this.activate_thread(metadata, &workspace, false, window, cx);
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn find_current_workspace_for_path_list(
        &self,
        path_list: &PathList,
        remote_connection: Option<&RemoteConnectionOptions>,
        cx: &App,
    ) -> Option<Entity<Workspace>> {
        self.find_workspace_in_current_window(cx, |workspace, cx| {
            workspace_path_list(workspace, cx).paths() == path_list.paths()
                && same_remote_connection_identity(
                    workspace
                        .read(cx)
                        .project()
                        .read(cx)
                        .remote_connection_options(cx)
                        .as_ref(),
                    remote_connection,
                )
        })
    }

    fn find_open_workspace_for_path_list(
        &self,
        path_list: &PathList,
        remote_connection: Option<&RemoteConnectionOptions>,
        cx: &App,
    ) -> Option<(WindowHandle<MultiWorkspace>, Entity<Workspace>)> {
        self.find_workspace_across_windows(cx, |workspace, cx| {
            workspace_path_list(workspace, cx).paths() == path_list.paths()
                && same_remote_connection_identity(
                    workspace
                        .read(cx)
                        .project()
                        .read(cx)
                        .remote_connection_options(cx)
                        .as_ref(),
                    remote_connection,
                )
        })
    }

    fn open_thread_from_archive(
        &mut self,
        metadata: ThreadMetadata,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let thread_id = metadata.thread_id;
        let weak_archive_view = match &self.view {
            SidebarView::Archive(view) => Some(view.downgrade()),
            _ => None,
        };

        if metadata.folder_paths().paths().is_empty() {
            ThreadMetadataStore::global(cx).update(cx, |store, cx| store.unarchive(thread_id, cx));

            let active_workspace = self
                .multi_workspace
                .upgrade()
                .map(|w| w.read(cx).workspace().clone());

            if let Some(workspace) = active_workspace {
                self.activate_thread_locally(&metadata, &workspace, false, window, cx);
            } else {
                let path_list = metadata.folder_paths().clone();
                if let Some((target_window, workspace)) = self.find_open_workspace_for_path_list(
                    &path_list,
                    metadata.remote_connection.as_ref(),
                    cx,
                ) {
                    self.activate_thread_in_other_window(metadata, workspace, target_window, cx);
                } else {
                    let key = ProjectGroupKey::from_worktree_paths(
                        &metadata.worktree_paths,
                        metadata.remote_connection.clone(),
                    );
                    self.open_workspace_and_activate_thread(metadata, path_list, &key, window, cx);
                }
            }
            self.show_thread_list(window, cx);
            return;
        }

        let store = ThreadMetadataStore::global(cx);
        let task = if metadata.archived {
            store
                .read(cx)
                .get_archived_worktrees_for_thread(thread_id, cx)
        } else {
            Task::ready(Ok(Vec::new()))
        };
        let path_list = metadata.folder_paths().clone();

        let restore_task = cx.spawn_in(window, async move |this, cx| {
            let result: anyhow::Result<()> = async {
                let archived_worktrees = task.await?;

                if archived_worktrees.is_empty() {
                    this.update_in(cx, |this, window, cx| {
                        this.restoring_tasks.remove(&thread_id);
                        if metadata.archived {
                            ThreadMetadataStore::global(cx)
                                .update(cx, |store, cx| store.unarchive(thread_id, cx));
                        }

                        if let Some(workspace) = this.find_current_workspace_for_path_list(
                            &path_list,
                            metadata.remote_connection.as_ref(),
                            cx,
                        ) {
                            this.activate_thread_locally(&metadata, &workspace, false, window, cx);
                        } else if let Some((target_window, workspace)) = this
                            .find_open_workspace_for_path_list(
                                &path_list,
                                metadata.remote_connection.as_ref(),
                                cx,
                            )
                        {
                            this.activate_thread_in_other_window(
                                metadata,
                                workspace,
                                target_window,
                                cx,
                            );
                        } else {
                            let key = ProjectGroupKey::from_worktree_paths(
                                &metadata.worktree_paths,
                                metadata.remote_connection.clone(),
                            );
                            this.open_workspace_and_activate_thread(
                                metadata, path_list, &key, window, cx,
                            );
                        }
                        this.show_thread_list(window, cx);
                    })?;
                    return anyhow::Ok(());
                }

                let mut path_replacements: Vec<(PathBuf, PathBuf)> = Vec::new();
                for row in &archived_worktrees {
                    match thread_worktree_archive::restore_worktree_via_git(
                        row,
                        metadata.remote_connection.as_ref(),
                        &mut *cx,
                    )
                    .await
                    {
                        Ok(restored_path) => {
                            thread_worktree_archive::cleanup_archived_worktree_record(
                                row,
                                metadata.remote_connection.as_ref(),
                                &mut *cx,
                            )
                            .await;
                            path_replacements.push((row.worktree_path.clone(), restored_path));
                        }
                        Err(error) => {
                            log::error!("Failed to restore worktree: {error:#}");
                            this.update_in(cx, |this, _window, cx| {
                                this.restoring_tasks.remove(&thread_id);
                                if let Some(weak_archive_view) = &weak_archive_view {
                                    weak_archive_view
                                        .update(cx, |view, cx| {
                                            view.clear_restoring(&thread_id, cx);
                                        })
                                        .ok();
                                }

                                if let Some(multi_workspace) = this.multi_workspace.upgrade() {
                                    let workspace = multi_workspace.read(cx).workspace().clone();
                                    workspace.update(cx, |workspace, cx| {
                                        struct RestoreWorktreeErrorToast;
                                        workspace.show_toast(
                                            Toast::new(
                                                NotificationId::unique::<RestoreWorktreeErrorToast>(
                                                ),
                                                format!("Failed to restore worktree: {error:#}"),
                                            )
                                            .autohide(),
                                            cx,
                                        );
                                    });
                                }
                            })
                            .ok();
                            return anyhow::Ok(());
                        }
                    }
                }

                if !path_replacements.is_empty() {
                    cx.update(|_window, cx| {
                        store.update(cx, |store, cx| {
                            store.update_restored_worktree_paths(thread_id, &path_replacements, cx);
                        });
                    })?;

                    let updated_metadata =
                        cx.update(|_window, cx| store.read(cx).entry(thread_id).cloned())?;

                    if let Some(updated_metadata) = updated_metadata {
                        let new_paths = updated_metadata.folder_paths().clone();
                        let key = ProjectGroupKey::from_worktree_paths(
                            &updated_metadata.worktree_paths,
                            updated_metadata.remote_connection.clone(),
                        );

                        cx.update(|_window, cx| {
                            store.update(cx, |store, cx| {
                                store.unarchive(updated_metadata.thread_id, cx);
                            });
                        })?;

                        this.update_in(cx, |this, window, cx| {
                            this.restoring_tasks.remove(&thread_id);
                            this.open_workspace_and_activate_thread(
                                updated_metadata,
                                new_paths,
                                &key,
                                window,
                                cx,
                            );
                            this.show_thread_list(window, cx);
                        })?;
                    }
                }

                anyhow::Ok(())
            }
            .await;
            if let Err(error) = result {
                log::error!("{error:#}");
            }
        });
        self.restoring_tasks.insert(thread_id, restore_task);
    }

    fn expand_selected_entry(
        &mut self,
        _: &SelectChild,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(ix) = self.selection else { return };

        match self.contents.entries.get(ix) {
            Some(ListEntry::ProjectHeader { key, .. }) => {
                let key = key.clone();
                if self.is_group_collapsed(&key, cx) {
                    self.set_group_expanded(&key, true, cx);
                    self.update_entries(cx);
                } else if ix + 1 < self.contents.entries.len() {
                    self.selection = Some(ix + 1);
                    self.list_state.scroll_to_reveal_item(ix + 1);
                    cx.notify();
                }
            }
            _ => {}
        }
    }

    fn collapse_selected_entry(
        &mut self,
        _: &SelectParent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(ix) = self.selection else { return };

        match self.contents.entries.get(ix) {
            Some(ListEntry::ProjectHeader { key, .. }) => {
                let key = key.clone();
                if !self.is_group_collapsed(&key, cx) {
                    self.set_group_expanded(&key, false, cx);
                    self.update_entries(cx);
                }
            }
            Some(ListEntry::Thread(_) | ListEntry::Terminal(_)) => {
                for i in (0..ix).rev() {
                    if let Some(ListEntry::ProjectHeader { key, .. }) = self.contents.entries.get(i)
                    {
                        let key = key.clone();
                        self.selection = Some(i);
                        self.set_group_expanded(&key, false, cx);
                        self.update_entries(cx);
                        break;
                    }
                }
            }
            None => {}
        }
    }

    fn toggle_selected_fold(
        &mut self,
        _: &editor::actions::ToggleFold,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(ix) = self.selection else { return };

        // Find the group header for the current selection.
        let header_ix = match self.contents.entries.get(ix) {
            Some(ListEntry::ProjectHeader { .. }) => Some(ix),
            Some(ListEntry::Thread(_) | ListEntry::Terminal(_)) => (0..ix).rev().find(|&i| {
                matches!(
                    self.contents.entries.get(i),
                    Some(ListEntry::ProjectHeader { .. })
                )
            }),
            None => None,
        };

        if let Some(header_ix) = header_ix {
            if let Some(ListEntry::ProjectHeader { key, .. }) = self.contents.entries.get(header_ix)
            {
                let key = key.clone();
                if self.is_group_collapsed(&key, cx) {
                    self.set_group_expanded(&key, true, cx);
                } else {
                    self.selection = Some(header_ix);
                    self.set_group_expanded(&key, false, cx);
                }
                self.update_entries(cx);
            }
        }
    }

    fn fold_all(
        &mut self,
        _: &editor::actions::FoldAll,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(mw) = self.multi_workspace.upgrade() {
            mw.update(cx, |mw, _cx| {
                mw.set_all_groups_expanded(false);
            });
        }
        self.update_entries(cx);
    }

    fn unfold_all(
        &mut self,
        _: &editor::actions::UnfoldAll,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(mw) = self.multi_workspace.upgrade() {
            mw.update(cx, |mw, _cx| {
                mw.set_all_groups_expanded(true);
            });
        }
        self.update_entries(cx);
    }

    fn stop_thread(&mut self, thread_id: &agent_ui::ThreadId, cx: &mut Context<Self>) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        let workspaces: Vec<_> = multi_workspace.read(cx).workspaces().cloned().collect();
        for workspace in workspaces {
            let item = workspace
                .read(cx)
                .items_of_type::<AgentThreadItem>(cx)
                .find(|item| item.read(cx).thread_id(cx) == *thread_id);
            if let Some(item) = item {
                item.update(cx, |item, cx| item.cancel_thread(cx));
                return;
            }
        }
    }

    fn apply_terminal_attention_action(
        terminal_id: TerminalId,
        metadata: TerminalThreadMetadata,
        action: TerminalAttentionAction,
        panel: Option<Entity<AgentPanel>>,
        terminal_view: Option<Entity<TerminalView>>,
        session_ref: Option<terminal::session_host::TerminalSessionRef>,
        cx: &mut App,
    ) {
        if !matches!(action, TerminalAttentionAction::Resume) {
            if let Some(panel) = panel {
                panel.update(cx, |panel, cx| {
                    panel.clear_terminal_attention_presentation(terminal_id, cx)
                });
            }
            if let Some(terminal_view) = terminal_view {
                terminal_view.update(cx, |terminal_view, cx| terminal_view.clear_bell(cx));
            }
            if let Some(session_ref) = session_ref
                && let Some(connection) = TerminalHostConnection::try_global(cx)
                && connection.host_id() == session_ref.host_id
            {
                connection.acknowledge_agent_attention(session_ref.session_id);
            }
        }

        if let Some(store) = TerminalThreadMetadataStore::try_global(cx) {
            store.update(cx, |store, cx| {
                if store.entry(terminal_id).is_none() {
                    store.save(metadata, cx);
                }
                match action {
                    TerminalAttentionAction::Acknowledge => {
                        store.acknowledge_attention(terminal_id, cx)
                    }
                    TerminalAttentionAction::SnoozeOneHour => {
                        store.snooze_attention(terminal_id, chrono::Duration::hours(1), cx)
                    }
                    TerminalAttentionAction::Resume => store.resume_attention(terminal_id, cx),
                    TerminalAttentionAction::Resolve => store.resolve_attention(terminal_id, cx),
                }
            });
        }
    }

    /// Find the neighbor thread in the sidebar (by display position).
    /// Look below first, then above, for the nearest thread that isn't
    /// the one being archived. We capture both the neighbor's metadata
    /// (for activation) and its workspace paths (for the workspace
    /// removal fallback).
    fn neighboring_activatable_entry(&self, current_position: usize) -> Option<ActivatableEntry> {
        let after = self
            .contents
            .entries
            .get(current_position.checked_add(1)?..)?;
        let before = self.contents.entries.get(..current_position)?;
        after
            .iter()
            .chain(before.iter().rev())
            .find_map(ActivatableEntry::from_list_entry)
    }

    fn activate_entry(
        &mut self,
        entry: &ActivatableEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        match entry {
            ActivatableEntry::Thread { metadata, .. } => {
                let Some(workspace) = self.multi_workspace.upgrade().and_then(|multi_workspace| {
                    multi_workspace
                        .read(cx)
                        .workspace_for_paths(metadata.folder_paths(), None, cx)
                }) else {
                    return false;
                };

                self.active_entry = Some(ActiveEntry::Thread {
                    thread_id: metadata.thread_id,
                    session_id: metadata.session_id.clone(),
                    workspace: workspace.clone(),
                });
                self.activate_workspace(&workspace, window, cx);
                Self::load_agent_thread_in_workspace(&workspace, metadata, true, window, cx);
                true
            }
            ActivatableEntry::Terminal {
                metadata,
                workspace,
                source,
            } => {
                self.activate_terminal_entry(
                    metadata.clone(),
                    workspace.clone(),
                    source.clone(),
                    false,
                    window,
                    cx,
                );
                true
            }
        }
    }

    fn activate_terminal_entry(
        &mut self,
        metadata: TerminalThreadMetadata,
        workspace: ThreadEntryWorkspace,
        source: TerminalEntrySource,
        retain: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(session_ref) = metadata.session_ref
            && let Some(connection) = TerminalHostConnection::try_global(cx)
            && connection.host_id() == session_ref.host_id
        {
            connection.acknowledge_agent_attention(session_ref.session_id);
        }
        match (workspace, source) {
            (
                ThreadEntryWorkspace::Open(workspace),
                TerminalEntrySource::WorkspaceItem(terminal_view),
            ) => self.activate_workspace_terminal_item(
                &workspace,
                &terminal_view,
                metadata,
                retain,
                true,
                window,
                cx,
            ),
            (ThreadEntryWorkspace::Open(workspace), TerminalEntrySource::AgentPanel) => {
                self.activate_terminal_in_workspace(&workspace, metadata, retain, window, cx);
            }
            (
                ThreadEntryWorkspace::Open(workspace),
                TerminalEntrySource::HostSession(session_id),
            ) => self.attach_host_terminal_session(
                &workspace, metadata, session_id, retain, true, window, cx,
            ),
            (
                ThreadEntryWorkspace::Closed {
                    folder_paths,
                    project_group_key,
                },
                _,
            ) => {
                self.open_workspace_and_activate_terminal(
                    metadata,
                    folder_paths,
                    &project_group_key,
                    window,
                    cx,
                );
            }
        }
    }

    fn attach_host_terminal_session(
        &mut self,
        workspace: &Entity<Workspace>,
        metadata: TerminalThreadMetadata,
        session_id: TerminalSessionId,
        retain: bool,
        focus_item: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };
        let local_terminal = LocalTerminalHost::try_global(cx).and_then(|host| {
            host.update(cx, |host, _cx| host.attach(session_id, None))
                .terminal
        });
        if let Some(terminal) = local_terminal {
            multi_workspace.update(cx, |multi_workspace, cx| {
                multi_workspace.activate(workspace.clone(), None, window, cx);
                if retain {
                    multi_workspace.retain_active_workspace(cx);
                }
            });
            let terminal_view = workspace.update(cx, |workspace, cx| {
                let pane = workspace.active_pane().clone();
                terminal_view::attach_terminal_to_workspace(
                    workspace, pane, terminal, focus_item, window, cx,
                )
            });
            self.active_entry = Some(ActiveEntry::Terminal {
                terminal_id: metadata.terminal_id,
                workspace: workspace.clone(),
            });
            self.record_terminal_access(metadata.terminal_id);
            if focus_item {
                terminal_view.focus_handle(cx).focus(window, cx);
            }
            self.update_entries(cx);
            return;
        }

        let Some(session_ref) = metadata.session_ref else {
            log::warn!("terminal session {session_id} has no host identity");
            return;
        };
        let workspace = workspace.clone();
        let project = workspace.read(cx).project().clone();
        cx.spawn_in(window, async move |this, cx| {
            let connection =
                terminal_view::wait_for_hosted_terminal_connection(session_ref.host_id, cx).await;
            let (terminal, session_unavailable) = match connection {
                Some(connection) => match terminal_view::restore_hosted_terminal(
                    &project,
                    connection,
                    session_ref,
                    metadata.working_directory.clone(),
                    cx,
                )
                .await
                {
                    Ok(Some(terminal)) => (terminal, false),
                    Ok(None) => {
                        log::warn!("terminal host no longer owns session {session_id}");
                        let terminal = cx.update(|window, cx| {
                            terminal_view::session_unavailable_terminal(
                                &project,
                                "The terminal host no longer owns this saved session.",
                                window,
                                cx,
                            )
                        })?;
                        (terminal, true)
                    }
                    Err(error) => {
                        log::warn!(
                            "failed to restore hosted terminal session {session_id}: {error:#}"
                        );
                        let terminal = cx.update(|window, cx| {
                            terminal_view::session_unavailable_terminal(
                                &project,
                                "The terminal host could not confirm this saved session.",
                                window,
                                cx,
                            )
                        })?;
                        (terminal, true)
                    }
                },
                None => {
                    log::warn!("terminal host is unavailable for session {session_id}");
                    let terminal = cx.update(|window, cx| {
                        terminal_view::session_unavailable_terminal(
                            &project,
                            "The terminal host is unavailable for this saved session.",
                            window,
                            cx,
                        )
                    })?;
                    (terminal, true)
                }
            };
            this.update_in(cx, |this, window, cx| {
                multi_workspace.update(cx, |multi_workspace, cx| {
                    multi_workspace.activate(workspace.clone(), None, window, cx);
                    if retain {
                        multi_workspace.retain_active_workspace(cx);
                    }
                });
                let terminal_view = workspace.update(cx, |workspace, cx| {
                    let pane = workspace.active_pane().clone();
                    terminal_view::attach_terminal_to_workspace(
                        workspace, pane, terminal, focus_item, window, cx,
                    )
                });
                if session_unavailable {
                    terminal_view.update(cx, |terminal_view, cx| {
                        terminal_view.set_session_unavailable(true, cx);
                    });
                }
                this.active_entry = Some(ActiveEntry::Terminal {
                    terminal_id: metadata.terminal_id,
                    workspace: workspace.clone(),
                });
                this.record_terminal_access(metadata.terminal_id);
                if focus_item {
                    terminal_view.focus_handle(cx).focus(window, cx);
                }
                this.update_entries(cx);
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn activate_workspace_terminal_item(
        &mut self,
        workspace: &Entity<Workspace>,
        terminal_view: &Entity<TerminalView>,
        metadata: TerminalThreadMetadata,
        retain: bool,
        focus_item: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        let terminal_id = metadata.terminal_id;
        self.record_terminal_access(terminal_id);
        self.active_entry = Some(ActiveEntry::Terminal {
            terminal_id,
            workspace: workspace.clone(),
        });

        multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.activate(workspace.clone(), None, window, cx);
            if retain {
                multi_workspace.retain_active_workspace(cx);
            }
        });

        workspace.update(cx, |workspace, cx| {
            workspace.activate_item(terminal_view, true, focus_item, window, cx);
        });
        terminal_view.update(cx, |terminal_view, cx| terminal_view.clear_bell(cx));

        self.update_entries(cx);
    }

    fn load_agent_terminal_in_workspace(
        workspace: &Entity<Workspace>,
        metadata: &TerminalThreadMetadata,
        focus: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        let restore_terminal = |agent_panel: Entity<AgentPanel>,
                                metadata: &TerminalThreadMetadata,
                                focus: bool,
                                workspace: Option<&Workspace>,
                                window: &mut Window,
                                cx: &mut App| {
            agent_panel.update(cx, |panel, cx| {
                panel.restore_terminal(
                    metadata.clone(),
                    focus,
                    AgentThreadSource::Sidebar,
                    workspace,
                    window,
                    cx,
                );
            });
        };

        let mut existing_panel = None;
        workspace.update(cx, |workspace, cx| {
            if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                existing_panel = Some(panel);
            }
        });

        if let Some(agent_panel) = existing_panel {
            restore_terminal(agent_panel, metadata, focus, None, window, cx);
            workspace.update(cx, |workspace, cx| {
                if focus {
                    workspace.focus_panel::<AgentPanel>(window, cx);
                } else {
                    workspace.reveal_panel::<AgentPanel>(window, cx);
                }
            });
            return;
        }

        let workspace = workspace.downgrade();
        let metadata = metadata.clone();
        let mut async_window_cx = window.to_async(cx);
        cx.spawn(async move |_cx| {
            let panel = AgentPanel::load(workspace.clone(), async_window_cx.clone()).await?;

            workspace.update_in(&mut async_window_cx, |workspace, window, cx| {
                let panel = workspace.panel::<AgentPanel>(cx).unwrap_or_else(|| {
                    workspace.add_panel(panel.clone(), window, cx);
                    panel.clone()
                });
                restore_terminal(panel, &metadata, focus, Some(workspace), window, cx);
                if focus {
                    workspace.focus_panel::<AgentPanel>(window, cx);
                } else {
                    workspace.reveal_panel::<AgentPanel>(window, cx);
                }
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn activate_terminal_in_workspace(
        &mut self,
        workspace: &Entity<Workspace>,
        metadata: TerminalThreadMetadata,
        retain: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        let terminal_id = metadata.terminal_id;
        self.record_terminal_access(terminal_id);
        self.active_entry = Some(ActiveEntry::Terminal {
            terminal_id,
            workspace: workspace.clone(),
        });

        multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.activate(workspace.clone(), None, window, cx);
            if retain {
                multi_workspace.retain_active_workspace(cx);
            }
        });

        Self::load_agent_terminal_in_workspace(workspace, &metadata, true, window, cx);

        self.update_entries(cx);
    }

    fn open_workspace_and_activate_terminal(
        &mut self,
        metadata: TerminalThreadMetadata,
        folder_paths: PathList,
        project_group_key: &ProjectGroupKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        let host = project_group_key.host();
        let provisional_key = Some(project_group_key.clone());
        let active_workspace = multi_workspace.read(cx).workspace().clone();
        let modal_workspace = active_workspace.clone();

        let open_task = multi_workspace.update(cx, |this, cx| {
            this.find_or_create_workspace(
                folder_paths,
                host,
                provisional_key,
                |options, window, cx| connect_remote(active_workspace, options, window, cx),
                &[],
                None,
                OpenMode::Activate,
                window,
                cx,
            )
        });

        cx.spawn_in(window, async move |this, cx| {
            let result = open_task.await;
            remote_connection::dismiss_connection_modal(&modal_workspace, cx);
            let workspace = result?;
            this.update_in(cx, |this, window, cx| {
                this.activate_terminal_in_workspace(&workspace, metadata, false, window, cx);
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn should_load_closed_workspace_for_archive(
        &self,
        folder_paths: &PathList,
        project_group_key: &ProjectGroupKey,
        remote_connection: Option<&RemoteConnectionOptions>,
        except_thread_id: Option<ThreadId>,
        except_terminal_id: Option<TerminalId>,
        cx: &App,
    ) -> bool {
        if folder_paths.is_empty() || folder_paths == project_group_key.path_list() {
            return false;
        }

        let archive_workspaces = self.archive_workspaces(cx);
        let thread_store = ThreadMetadataStore::global(cx);
        let thread_store = thread_store.read(cx);
        if folder_paths.ordered_paths().any(|path| {
            Self::path_is_referenced_by_unarchived_threads_for_archive(
                &thread_store,
                except_thread_id,
                path,
                remote_connection,
                &archive_workspaces,
                cx,
            )
        }) {
            return false;
        }

        TerminalThreadMetadataStore::try_global(cx).is_none_or(|terminal_store| {
            let terminal_store = terminal_store.read(cx);
            !folder_paths.ordered_paths().any(|path| {
                terminal_store.path_is_referenced_by_terminal(
                    except_terminal_id,
                    path,
                    remote_connection,
                )
            })
        })
    }

    fn path_is_referenced_by_unarchived_threads_for_archive(
        thread_store: &ThreadMetadataStore,
        except_thread_id: Option<ThreadId>,
        path: &Path,
        remote_connection: Option<&RemoteConnectionOptions>,
        archive_workspaces: &[Entity<Workspace>],
        cx: &App,
    ) -> bool {
        thread_store.path_is_referenced_by_unarchived_threads_matching(
            except_thread_id,
            path,
            remote_connection,
            |thread| Self::thread_blocks_worktree_archive(thread, archive_workspaces, cx),
        )
    }

    fn archive_workspaces(&self, cx: &App) -> Vec<Entity<Workspace>> {
        let multi_workspace = self.multi_workspace.upgrade();
        thread_worktree_archive::workspaces_for_archive(multi_workspace.as_ref(), cx)
    }

    fn count_threads_blocking_worktree_archive(
        &self,
        path_list: &PathList,
        remote_connection: Option<&RemoteConnectionOptions>,
        except_thread_id: Option<ThreadId>,
        cx: &App,
    ) -> usize {
        let archive_workspaces = self.archive_workspaces(cx);
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entries_for_path(path_list, remote_connection)
            .filter(|thread| Some(thread.thread_id) != except_thread_id)
            .filter(|thread| Self::thread_blocks_worktree_archive(thread, &archive_workspaces, cx))
            .count()
    }

    fn roots_to_archive_for_paths(
        &self,
        folder_paths: &PathList,
        remote_connection: Option<&RemoteConnectionOptions>,
        except_thread_id: Option<ThreadId>,
        except_terminal_id: Option<TerminalId>,
        cx: &App,
    ) -> Vec<thread_worktree_archive::RootPlan> {
        let workspaces = self.archive_workspaces(cx);
        folder_paths
            .ordered_paths()
            .filter_map(|path| {
                thread_worktree_archive::build_root_plan(path, remote_connection, &workspaces, cx)
            })
            .filter(|plan| {
                let store = ThreadMetadataStore::global(cx);
                let store = store.read(cx);
                !Self::path_is_referenced_by_unarchived_threads_for_archive(
                    &store,
                    except_thread_id,
                    plan.root_path.as_path(),
                    remote_connection,
                    &workspaces,
                    cx,
                )
            })
            .filter(|root| {
                TerminalThreadMetadataStore::try_global(cx).is_none_or(|terminal_store| {
                    !terminal_store.read(cx).path_is_referenced_by_terminal(
                        except_terminal_id,
                        root.root_path.as_path(),
                        remote_connection,
                    )
                })
            })
            .collect()
    }

    fn linked_worktree_workspace_to_remove(
        &self,
        folder_paths: &PathList,
        remote_connection: Option<&RemoteConnectionOptions>,
        except_thread_id: Option<ThreadId>,
        except_terminal_id: Option<TerminalId>,
        roots_to_archive: &[thread_worktree_archive::RootPlan],
        cx: &App,
    ) -> Option<Entity<Workspace>> {
        if folder_paths.is_empty() {
            return None;
        }

        let remaining = self.count_threads_blocking_worktree_archive(
            folder_paths,
            remote_connection,
            except_thread_id,
            cx,
        );

        if remaining > 0 {
            return None;
        }

        let multi_workspace = self.multi_workspace.upgrade()?;
        let workspace =
            multi_workspace
                .read(cx)
                .workspace_for_paths(folder_paths, remote_connection, cx)?;

        if workspace_has_terminal_metadata_except(&workspace, except_terminal_id, cx) {
            return None;
        }

        if !roots_to_archive.is_empty() {
            let archive_paths: HashSet<&Path> = roots_to_archive
                .iter()
                .map(|root| root.root_path.as_path())
                .collect();
            let project = workspace.read(cx).project().clone();
            let visible_worktree_paths = project
                .read(cx)
                .visible_worktrees(cx)
                .map(|worktree| worktree.read(cx).abs_path())
                .collect::<Vec<_>>();
            return (!visible_worktree_paths.is_empty()
                && visible_worktree_paths
                    .iter()
                    .all(|path| archive_paths.contains(path.as_ref())))
            .then_some(workspace);
        }

        let group_key = workspace.read(cx).project_group_key(cx);
        (group_key.path_list() != folder_paths).then_some(workspace)
    }

    fn delete_empty_drafts_for_archive_roots(
        &self,
        roots: &[thread_worktree_archive::RootPlan],
        cx: &mut Context<Self>,
    ) {
        self.delete_empty_drafts_for_archive_targets(
            roots
                .iter()
                .map(|root| (root.root_path.as_path(), root.remote_connection.as_ref())),
            cx,
        );
    }

    fn delete_empty_drafts_for_archive_paths(
        &self,
        paths: &PathList,
        remote_connection: Option<&RemoteConnectionOptions>,
        cx: &mut Context<Self>,
    ) {
        self.delete_empty_drafts_for_archive_targets(
            paths
                .ordered_paths()
                .map(|path| (path.as_path(), remote_connection)),
            cx,
        );
    }

    fn delete_empty_drafts_for_archive_targets<'a>(
        &self,
        targets: impl IntoIterator<Item = (&'a Path, Option<&'a RemoteConnectionOptions>)>,
        cx: &mut Context<Self>,
    ) {
        let targets = targets.into_iter().collect::<Vec<_>>();
        if targets.is_empty() {
            return;
        }

        let archive_workspaces = self.archive_workspaces(cx);
        let draft_thread_ids = ThreadMetadataStore::global(cx)
            .read(cx)
            .unarchived_draft_ids_matching(|thread| {
                targets.iter().any(|(path, remote_connection)| {
                    thread.matches_remote_connection(*remote_connection)
                        && thread.references_folder_path(path)
                }) && !Self::thread_blocks_worktree_archive(thread, &archive_workspaces, cx)
            });
        if draft_thread_ids.is_empty() {
            return;
        }

        ThreadMetadataStore::global(cx).update(cx, |store, cx| {
            store.delete_all(draft_thread_ids, cx);
        });
    }

    fn thread_blocks_worktree_archive(
        thread: &ThreadMetadata,
        archive_workspaces: &[Entity<Workspace>],
        cx: &App,
    ) -> bool {
        if !thread.is_draft() {
            return true;
        }

        agent_ui::draft_prompt_store::draft_has_user_content(
            thread.thread_id,
            archive_workspaces,
            cx,
        )
    }

    async fn wait_for_archive_workspace_metadata(
        workspace: &Entity<Workspace>,
        cx: &mut gpui::AsyncApp,
    ) {
        let scans_complete =
            workspace.read_with(cx, |workspace, cx| workspace.worktree_scans_complete(cx));
        scans_complete.await;

        let project = workspace.read_with(cx, |workspace, _| workspace.project().clone());
        let barriers = project.update(cx, |project, cx| {
            let repositories = project
                .repositories(cx)
                .values()
                .cloned()
                .collect::<Vec<_>>();
            repositories
                .into_iter()
                .map(|repository| repository.update(cx, |repository, _| repository.barrier()))
                .collect::<Vec<_>>()
        });
        for barrier in barriers {
            let result: anyhow::Result<()> = barrier.await.map_err(|_| {
                anyhow::anyhow!("git repository barrier canceled while archiving worktree")
            });
            result.log_err();
        }
    }

    fn open_workspace_for_archive(
        &mut self,
        folder_paths: PathList,
        project_group_key: ProjectGroupKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<(Task<anyhow::Result<Entity<Workspace>>>, Entity<Workspace>)> {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return None;
        };

        let host = project_group_key.host();
        let active_workspace = multi_workspace.read(cx).workspace().clone();
        let modal_workspace = active_workspace.clone();

        let open_task = multi_workspace.update(cx, |this, cx| {
            this.find_or_create_workspace(
                folder_paths,
                host,
                Some(project_group_key),
                |options, window, cx| connect_remote(active_workspace, options, window, cx),
                &[],
                None,
                OpenMode::Add,
                window,
                cx,
            )
        });

        Some((open_task, modal_workspace))
    }

    fn open_workspace_and_archive_thread(
        &mut self,
        session_id: acp::SessionId,
        folder_paths: PathList,
        project_group_key: ProjectGroupKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((open_task, modal_workspace)) =
            self.open_workspace_for_archive(folder_paths, project_group_key, window, cx)
        else {
            return;
        };

        cx.spawn_in(window, async move |this, cx| {
            let result = open_task.await;
            remote_connection::dismiss_connection_modal(&modal_workspace, cx);
            let workspace = result?;
            Self::wait_for_archive_workspace_metadata(&workspace, cx).await;

            this.update_in(cx, |this, window, cx| {
                this.update_entries(cx);
                this.archive_thread(&session_id, window, cx);
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn open_workspace_and_close_terminal(
        &mut self,
        metadata: TerminalThreadMetadata,
        folder_paths: PathList,
        project_group_key: ProjectGroupKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((open_task, modal_workspace)) =
            self.open_workspace_for_archive(folder_paths, project_group_key, window, cx)
        else {
            return;
        };

        cx.spawn_in(window, async move |this, cx| {
            let result = open_task.await;
            remote_connection::dismiss_connection_modal(&modal_workspace, cx);
            let workspace = result?;
            Self::wait_for_archive_workspace_metadata(&workspace, cx).await;

            this.update_in(cx, |this, window, cx| {
                let workspace = ThreadEntryWorkspace::Open(workspace);
                this.close_terminal(
                    &metadata,
                    &workspace,
                    &TerminalEntrySource::AgentPanel,
                    window,
                    cx,
                );
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn close_terminal(
        &mut self,
        metadata: &TerminalThreadMetadata,
        workspace: &ThreadEntryWorkspace,
        source: &TerminalEntrySource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let TerminalEntrySource::HostSession(session_id) = source {
            if let Some(host) = LocalTerminalHost::try_global(cx) {
                host.update(cx, |host, cx| {
                    host.terminate(*session_id, cx);
                });
            }
            if let Some(session_ref) = metadata.session_ref
                && let Some(connection) = TerminalHostConnection::try_global(cx)
                    .filter(|connection| connection.host_id() == session_ref.host_id)
            {
                let session_id = *session_id;
                cx.spawn(async move |this, cx| {
                    let result: anyhow::Result<()> = async {
                        let response = connection
                            .command(TerminalSessionCommand::Terminate { session_id })
                            .await?;
                        match response {
                            terminal::session_host::transport::TerminalHostResponse::Error {
                                message,
                            }
                            | terminal::session_host::transport::TerminalHostResponse::Unsupported {
                                message,
                            } => anyhow::bail!("failed to terminate hosted terminal: {message}"),
                            terminal::session_host::transport::TerminalHostResponse::Sessions {
                                ..
                            }
                            | terminal::session_host::transport::TerminalHostResponse::Attachment {
                                ..
                            }
                            | terminal::session_host::transport::TerminalHostResponse::Heartbeat {
                                ..
                            }
                            | terminal::session_host::transport::TerminalHostResponse::Events {
                                ..
                            } => {
                                anyhow::bail!("terminal host returned an invalid terminate response")
                            }
                            terminal::session_host::transport::TerminalHostResponse::Snapshot {
                                ..
                            } => {}
                        }
                        Ok(())
                    }
                    .await;

                    if let Err(error) = &result {
                        this.update(cx, |this, cx| {
                            if let Some(workspace) = this.active_workspace(cx) {
                                workspace.update(cx, |workspace, cx| {
                                    struct TerminateTerminalErrorToast;
                                    workspace.show_toast(
                                        Toast::new(
                                            NotificationId::unique::<TerminateTerminalErrorToast>(),
                                            format!(
                                                "Durable session was not terminated: {error:#}"
                                            ),
                                        )
                                        .autohide(),
                                        cx,
                                    );
                                });
                            }
                        })
                        .ok();
                    }

                    result
                })
                .detach_and_log_err(cx);
            }
            self.standalone_terminal_created_at
                .remove(&metadata.terminal_id);
            self.update_entries(cx);
            return;
        }
        if let (ThreadEntryWorkspace::Open(workspace), TerminalEntrySource::WorkspaceItem(item)) =
            (workspace, source)
        {
            self.close_workspace_terminal_item(metadata, workspace, item, window, cx);
            return;
        }

        if let ThreadEntryWorkspace::Closed {
            folder_paths,
            project_group_key,
        } = workspace
            && self.should_load_closed_workspace_for_archive(
                folder_paths,
                project_group_key,
                metadata.remote_connection.as_ref(),
                None,
                Some(metadata.terminal_id),
                cx,
            )
        {
            self.open_workspace_and_close_terminal(
                metadata.clone(),
                folder_paths.clone(),
                project_group_key.clone(),
                window,
                cx,
            );
            return;
        }

        let terminal_id = metadata.terminal_id;
        let is_active = self
            .active_entry
            .as_ref()
            .is_some_and(|entry| entry.is_active_terminal(terminal_id));
        let neighbor = self
            .contents
            .entries
            .iter()
            .position(|entry| {
                matches!(
                    entry,
                    ListEntry::Terminal(terminal)
                        if terminal.metadata.terminal_id == terminal_id
                )
            })
            .and_then(|position| self.neighboring_activatable_entry(position));

        let terminal_folder_paths = metadata.folder_paths().clone();
        let roots_to_archive = self.roots_to_archive_for_paths(
            metadata.folder_paths(),
            metadata.remote_connection.as_ref(),
            None,
            Some(terminal_id),
            cx,
        );

        let workspace_to_remove = self.linked_worktree_workspace_to_remove(
            &terminal_folder_paths,
            metadata.remote_connection.as_ref(),
            None,
            Some(terminal_id),
            &roots_to_archive,
            cx,
        );

        let mut workspaces_to_remove: Vec<Entity<Workspace>> =
            workspace_to_remove.into_iter().collect();
        let close_item_tasks = self.close_items_for_archived_worktrees(
            &roots_to_archive,
            &mut workspaces_to_remove,
            window,
            cx,
        );

        if !workspaces_to_remove.is_empty() {
            let multi_workspace = self.multi_workspace.upgrade().unwrap();
            let terminal_workspace_removed = matches!(
                workspace,
                ThreadEntryWorkspace::Open(workspace) if workspaces_to_remove.contains(workspace)
            );
            let (fallback_paths, project_group_key) = neighbor
                .as_ref()
                .map(|neighbor| neighbor.project_location(cx))
                .unwrap_or_else(|| {
                    workspaces_to_remove
                        .first()
                        .map(|workspace| {
                            let key = workspace.read(cx).project_group_key(cx);
                            (key.path_list().clone(), key)
                        })
                        .unwrap_or_default()
                });

            let excluded = workspaces_to_remove.clone();
            let remove_task = multi_workspace.update(cx, |multi_workspace, cx| {
                multi_workspace.remove(
                    workspaces_to_remove,
                    move |this, window, cx| {
                        let active_workspace = this.workspace().clone();
                        this.find_or_create_workspace(
                            fallback_paths,
                            project_group_key.host(),
                            Some(project_group_key),
                            |options, window, cx| {
                                connect_remote(active_workspace, options, window, cx)
                            },
                            &excluded,
                            None,
                            OpenMode::Activate,
                            window,
                            cx,
                        )
                    },
                    window,
                    cx,
                )
            });

            let metadata = metadata.clone();
            let workspace = workspace.clone();
            cx.spawn_in(window, async move |this, cx| {
                if !remove_task.await? {
                    return anyhow::Ok(());
                }

                for task in close_item_tasks {
                    let result: anyhow::Result<()> = task.await;
                    result.log_err();
                }

                this.update_in(cx, |this, window, cx| {
                    if terminal_workspace_removed {
                        this.delete_empty_drafts_for_archive_paths(
                            metadata.folder_paths(),
                            metadata.remote_connection.as_ref(),
                            cx,
                        );
                    }
                    // If the terminal's workspace has already been removed,
                    // don't synthesize a fallback draft in the detached
                    // AgentPanel.
                    this.close_terminal_entry(
                        &metadata,
                        &workspace,
                        is_active,
                        neighbor.as_ref(),
                        !terminal_workspace_removed,
                        roots_to_archive,
                        window,
                        cx,
                    );
                })?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        } else if !close_item_tasks.is_empty() {
            let metadata = metadata.clone();
            let workspace = workspace.clone();
            cx.spawn_in(window, async move |this, cx| {
                for task in close_item_tasks {
                    let result: anyhow::Result<()> = task.await;
                    result.log_err();
                }

                this.update_in(cx, |this, window, cx| {
                    this.close_terminal_entry(
                        &metadata,
                        &workspace,
                        is_active,
                        neighbor.as_ref(),
                        true,
                        roots_to_archive,
                        window,
                        cx,
                    );
                })?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        } else {
            self.close_terminal_entry(
                metadata,
                workspace,
                is_active,
                neighbor.as_ref(),
                true,
                roots_to_archive,
                window,
                cx,
            );
        }
    }

    fn close_terminal_with_confirmation(
        &mut self,
        metadata: TerminalThreadMetadata,
        workspace: ThreadEntryWorkspace,
        source: TerminalEntrySource,
        requires_termination_confirmation: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !requires_termination_confirmation {
            self.close_terminal(&metadata, &workspace, &source, window, cx);
            return;
        }

        let title = metadata.display_title();
        let (heading, detail) = terminal_termination_confirmation_copy(title.as_ref());
        let prompt = window.prompt(
            PromptLevel::Critical,
            heading,
            Some(&detail),
            &["Terminate", "Cancel"],
            cx,
        );

        cx.spawn_in(window, async move |this, cx| -> anyhow::Result<()> {
            if prompt.await.log_err() != Some(0) {
                return Ok(());
            }
            this.update_in(cx, |this, window, cx| {
                this.close_terminal(&metadata, &workspace, &source, window, cx);
            })?;
            Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn close_workspace_terminal_item(
        &mut self,
        metadata: &TerminalThreadMetadata,
        workspace: &Entity<Workspace>,
        terminal_view: &Entity<TerminalView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let terminal_id = metadata.terminal_id;
        let is_active = self
            .active_entry
            .as_ref()
            .is_some_and(|entry| entry.is_active_terminal(terminal_id));
        let neighbor = self
            .contents
            .entries
            .iter()
            .position(|entry| {
                matches!(
                    entry,
                    ListEntry::Terminal(terminal)
                        if terminal.metadata.terminal_id == terminal_id
                )
            })
            .and_then(|position| self.neighboring_activatable_entry(position));

        let item_id = terminal_view.entity_id();
        workspace.update(cx, |workspace, cx| {
            if let Some(pane) = workspace.pane_for(terminal_view) {
                pane.update(cx, |pane, cx| {
                    pane.close_item_by_id(item_id, SaveIntent::Close, window, cx)
                        .detach_and_log_err(cx);
                });
            }
        });

        self.standalone_terminal_created_at.remove(&terminal_id);

        if is_active {
            self.active_entry = None;
            if neighbor
                .as_ref()
                .is_some_and(|neighbor| self.activate_entry(neighbor, window, cx))
            {
                return;
            }
            self.sync_active_entry_from_active_workspace(cx);
        }

        self.update_entries(cx);
    }

    fn close_terminal_entry(
        &mut self,
        metadata: &TerminalThreadMetadata,
        workspace: &ThreadEntryWorkspace,
        is_active: bool,
        neighbor: Option<&ActivatableEntry>,
        activate_panel_draft: bool,
        roots_to_archive: Vec<thread_worktree_archive::RootPlan>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let terminal_id = metadata.terminal_id;

        // Closing from the sidebar must not steal focus, since the row's
        // workspace may not be the active workspace.
        if let ThreadEntryWorkspace::Open(workspace) = workspace {
            workspace.update(cx, |workspace, cx| {
                if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                    panel.update(cx, |panel, cx| {
                        if activate_panel_draft {
                            panel.close_terminal(terminal_id, window, cx);
                        } else {
                            panel.close_terminal_without_activating_draft(terminal_id, window, cx);
                        }
                    });
                }
            });
        }
        if let Some(store) = TerminalThreadMetadataStore::try_global(cx) {
            store.update(cx, |store, cx| {
                store.delete(terminal_id, cx);
            });
        }

        self.start_detached_archive_worktree_task(roots_to_archive, cx);

        if is_active {
            self.active_entry = None;
            if neighbor
                .as_ref()
                .is_some_and(|neighbor| self.activate_entry(neighbor, window, cx))
            {
                return;
            }
            self.sync_active_entry_from_active_workspace(cx);
        }
        self.update_entries(cx);
    }

    fn close_items_for_archived_worktrees(
        &self,
        roots_to_archive: &[thread_worktree_archive::RootPlan],
        workspaces_to_remove: &mut Vec<Entity<Workspace>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<Task<anyhow::Result<()>>> {
        if roots_to_archive.is_empty() {
            return Vec::new();
        }

        let archive_paths: HashSet<&Path> = roots_to_archive
            .iter()
            .map(|root| root.root_path.as_path())
            .collect();

        let mut mixed_workspaces: Vec<(Entity<Workspace>, Vec<WorktreeId>)> = Vec::new();

        if let Some(multi_workspace) = self.multi_workspace.upgrade() {
            let all_workspaces: Vec<_> = multi_workspace.read(cx).workspaces().cloned().collect();

            for workspace in all_workspaces {
                if workspaces_to_remove.contains(&workspace) {
                    continue;
                }

                let project = workspace.read(cx).project().read(cx);
                let visible_worktrees: Vec<_> = project
                    .visible_worktrees(cx)
                    .map(|worktree| (worktree.read(cx).id(), worktree.read(cx).abs_path()))
                    .collect();

                let archived_worktree_ids: Vec<WorktreeId> = visible_worktrees
                    .iter()
                    .filter(|(_, path)| archive_paths.contains(path.as_ref()))
                    .map(|(id, _)| *id)
                    .collect();

                if archived_worktree_ids.is_empty() {
                    continue;
                }

                if visible_worktrees.len() == archived_worktree_ids.len() {
                    workspaces_to_remove.push(workspace);
                } else {
                    mixed_workspaces.push((workspace, archived_worktree_ids));
                }
            }
        }

        let mut close_item_tasks = Vec::new();
        for (workspace, archived_worktree_ids) in &mixed_workspaces {
            let panes: Vec<_> = workspace.read(cx).panes().to_vec();
            for pane in panes {
                let items_to_close: Vec<EntityId> = pane
                    .read(cx)
                    .items()
                    .filter(|item| {
                        item.project_path(cx)
                            .is_some_and(|pp| archived_worktree_ids.contains(&pp.worktree_id))
                    })
                    .map(|item| item.item_id())
                    .collect();

                if !items_to_close.is_empty() {
                    let task = pane.update(cx, |pane, cx| {
                        pane.close_items(window, cx, SaveIntent::Close, &|item_id| {
                            items_to_close.contains(&item_id)
                        })
                    });
                    close_item_tasks.push(task);
                }
            }
        }

        close_item_tasks
    }

    fn close_agent_thread_tabs(
        &self,
        thread_id: ThreadId,
        workspaces_to_remove: &[Entity<Workspace>],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<Task<anyhow::Result<()>>> {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return Vec::new();
        };

        let workspaces: Vec<_> = multi_workspace
            .read(cx)
            .workspaces()
            .filter(|workspace| !workspaces_to_remove.contains(workspace))
            .cloned()
            .collect();

        let mut close_item_tasks = Vec::new();
        for workspace in workspaces {
            let panes = workspace.read(cx).panes().to_vec();
            for pane in panes {
                let items_to_close: Vec<EntityId> = pane
                    .read(cx)
                    .items()
                    .filter_map(|item| {
                        let item = item.downcast::<AgentThreadItem>()?;
                        (item.read(cx).thread_id(cx) == thread_id).then_some(item.entity_id())
                    })
                    .collect();

                if !items_to_close.is_empty() {
                    let task = pane.update(cx, |pane, cx| {
                        pane.close_items(window, cx, SaveIntent::Skip, &|item_id| {
                            items_to_close.contains(&item_id)
                        })
                    });
                    close_item_tasks.push(task);
                }
            }
        }

        close_item_tasks
    }

    fn archive_thread(
        &mut self,
        session_id: &acp::SessionId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let store = ThreadMetadataStore::global(cx);
        let metadata = store.read(cx).entry_by_session(session_id).cloned();
        let metadata_thread_id = metadata.as_ref().map(|metadata| metadata.thread_id);
        let thread_entry = self.contents.entries.iter().find_map(|entry| match entry {
            ListEntry::Thread(thread) => metadata_thread_id
                .map_or_else(
                    || thread.metadata.session_id.as_ref() == Some(session_id),
                    |thread_id| thread.metadata.thread_id == thread_id,
                )
                .then(|| thread.clone()),
            _ => None,
        });
        let thread_id = metadata_thread_id.or_else(|| {
            thread_entry
                .as_ref()
                .map(|thread| thread.metadata.thread_id)
        });
        let active_workspace = thread_id.and_then(|thread_id| {
            self.active_entry.as_ref().and_then(|entry| {
                if entry.is_active_thread(&thread_id) {
                    Some(entry.workspace().clone())
                } else {
                    None
                }
            })
        });
        let thread_folder_paths = metadata
            .as_ref()
            .map(|metadata| metadata.folder_paths().clone())
            .or_else(|| {
                thread_entry
                    .as_ref()
                    .map(|thread| thread.metadata.folder_paths().clone())
            })
            .or_else(|| {
                active_workspace
                    .as_ref()
                    .map(|workspace| PathList::new(&workspace.read(cx).root_paths(cx)))
            });
        let thread_entry_workspace = thread_entry.map(|thread| thread.workspace.clone());

        if let (
            Some(metadata),
            Some(ThreadEntryWorkspace::Closed {
                folder_paths,
                project_group_key,
            }),
        ) = (metadata.as_ref(), thread_entry_workspace)
            && self.should_load_closed_workspace_for_archive(
                &folder_paths,
                &project_group_key,
                metadata.remote_connection.as_ref(),
                Some(metadata.thread_id),
                None,
                cx,
            )
        {
            self.open_workspace_and_archive_thread(
                session_id.clone(),
                folder_paths,
                project_group_key,
                window,
                cx,
            );
            return;
        }

        // Compute which linked worktree roots should be archived from disk if
        // this thread is archived. This must happen before we remove any
        // workspace from the MultiWorkspace, because `build_root_plan` needs
        // the currently open workspaces in order to find the affected projects
        // and repository handles for each linked worktree.
        let roots_to_archive = metadata
            .as_ref()
            .map(|metadata| {
                self.roots_to_archive_for_paths(
                    metadata.folder_paths(),
                    metadata.remote_connection.as_ref(),
                    thread_id,
                    None,
                    cx,
                )
            })
            .unwrap_or_default();

        let current_pos = self.contents.entries.iter().position(|entry| match entry {
            ListEntry::Thread(thread) => thread_id.map_or_else(
                || thread.metadata.session_id.as_ref() == Some(session_id),
                |tid| thread.metadata.thread_id == tid,
            ),
            _ => false,
        });
        let neighbor =
            current_pos.and_then(|position| self.neighboring_activatable_entry(position));

        // Check if archiving this thread would leave its worktree workspace
        // with no threads, requiring workspace removal.
        let workspace_to_remove = thread_folder_paths.as_ref().and_then(|folder_paths| {
            let thread_remote_connection =
                metadata.as_ref().and_then(|m| m.remote_connection.as_ref());
            self.linked_worktree_workspace_to_remove(
                folder_paths,
                thread_remote_connection,
                thread_id,
                None,
                &roots_to_archive,
                cx,
            )
        });

        // Also find workspaces for root plans that aren't covered by
        // workspace_to_remove. For workspaces that exclusively contain
        // worktrees being archived, remove the whole workspace. For
        // "mixed" workspaces (containing both archived and non-archived
        // worktrees), close only the editor items referencing the
        // archived worktrees so their Entity<Worktree> handles are
        // dropped without destroying the user's workspace layout.
        let mut workspaces_to_remove: Vec<Entity<Workspace>> =
            workspace_to_remove.into_iter().collect();
        let mut close_item_tasks = self.close_items_for_archived_worktrees(
            &roots_to_archive,
            &mut workspaces_to_remove,
            window,
            cx,
        );
        if let Some(thread_id) = thread_id {
            close_item_tasks.extend(self.close_agent_thread_tabs(
                thread_id,
                &workspaces_to_remove,
                window,
                cx,
            ));
        }

        if !workspaces_to_remove.is_empty() {
            let multi_workspace = self.multi_workspace.upgrade().unwrap();
            let session_id = session_id.clone();

            let (fallback_paths, project_group_key) = neighbor
                .as_ref()
                .map(|neighbor| neighbor.project_location(cx))
                .unwrap_or_else(|| {
                    workspaces_to_remove
                        .first()
                        .map(|workspace| {
                            let key = workspace.read(cx).project_group_key(cx);
                            (key.path_list().clone(), key)
                        })
                        .unwrap_or_default()
                });

            let excluded = workspaces_to_remove.clone();
            let remove_task = multi_workspace.update(cx, |mw, cx| {
                mw.remove(
                    workspaces_to_remove,
                    move |this, window, cx| {
                        let active_workspace = this.workspace().clone();
                        this.find_or_create_workspace(
                            fallback_paths,
                            project_group_key.host(),
                            Some(project_group_key),
                            |options, window, cx| {
                                connect_remote(active_workspace, options, window, cx)
                            },
                            &excluded,
                            None,
                            OpenMode::Activate,
                            window,
                            cx,
                        )
                    },
                    window,
                    cx,
                )
            });

            let thread_folder_paths = thread_folder_paths.clone();
            let thread_remote_connection = metadata
                .as_ref()
                .and_then(|metadata| metadata.remote_connection.clone());
            cx.spawn_in(window, async move |this, cx| {
                if !remove_task.await? {
                    return anyhow::Ok(());
                }

                for task in close_item_tasks {
                    let result: anyhow::Result<()> = task.await;
                    result.log_err();
                }

                this.update_in(cx, |this, window, cx| {
                    if let Some(thread_folder_paths) = thread_folder_paths.as_ref() {
                        this.delete_empty_drafts_for_archive_paths(
                            thread_folder_paths,
                            thread_remote_connection.as_ref(),
                            cx,
                        );
                    }
                    let in_flight = thread_id.and_then(|tid| {
                        this.start_archive_worktree_task(tid, roots_to_archive, cx)
                    });
                    this.archive_and_activate(
                        &session_id,
                        thread_id,
                        neighbor.as_ref(),
                        thread_folder_paths.as_ref(),
                        thread_remote_connection.as_ref(),
                        in_flight,
                        window,
                        cx,
                    );
                })?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        } else if !close_item_tasks.is_empty() {
            let session_id = session_id.clone();
            let thread_folder_paths = thread_folder_paths.clone();
            let thread_remote_connection = metadata
                .as_ref()
                .and_then(|metadata| metadata.remote_connection.clone());
            cx.spawn_in(window, async move |this, cx| {
                for task in close_item_tasks {
                    let result: anyhow::Result<()> = task.await;
                    result.log_err();
                }

                this.update_in(cx, |this, window, cx| {
                    let in_flight = thread_id.and_then(|tid| {
                        this.start_archive_worktree_task(tid, roots_to_archive, cx)
                    });
                    this.archive_and_activate(
                        &session_id,
                        thread_id,
                        neighbor.as_ref(),
                        thread_folder_paths.as_ref(),
                        thread_remote_connection.as_ref(),
                        in_flight,
                        window,
                        cx,
                    );
                })?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        } else {
            let in_flight = thread_id
                .and_then(|tid| self.start_archive_worktree_task(tid, roots_to_archive, cx));
            self.archive_and_activate(
                session_id,
                thread_id,
                neighbor.as_ref(),
                thread_folder_paths.as_ref(),
                metadata
                    .as_ref()
                    .and_then(|metadata| metadata.remote_connection.as_ref()),
                in_flight,
                window,
                cx,
            );
        }
    }

    /// Archive a thread and activate the nearest neighbor or a draft.
    ///
    /// IMPORTANT: when activating a neighbor or creating a fallback draft,
    /// this method also activates the target workspace in the MultiWorkspace.
    /// This is critical because `rebuild_contents` derives the active
    /// workspace from `mw.workspace()`. If the linked worktree workspace is
    /// still active after archiving its last thread, `rebuild_contents` sees
    /// the threadless linked worktree as active and emits a spurious
    /// "+ New Thread" entry with the worktree chip — keeping the worktree
    /// alive and preventing disk cleanup.
    ///
    /// When `in_flight_archive` is present, it is the background task that
    /// persists the linked worktree's git state and deletes it from disk.
    /// We attach it to the metadata store at the same time we mark the thread
    /// archived so failures can automatically unarchive the thread and user-
    /// initiated unarchive can cancel the task.
    fn archive_and_activate(
        &mut self,
        _session_id: &acp::SessionId,
        thread_id: Option<agent_ui::ThreadId>,
        neighbor: Option<&ActivatableEntry>,
        thread_folder_paths: Option<&PathList>,
        thread_remote_connection: Option<&RemoteConnectionOptions>,
        in_flight_archive: Option<(Task<()>, async_channel::Sender<()>)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(thread_id) = thread_id {
            ThreadMetadataStore::global(cx).update(cx, |store, cx| {
                store.archive(thread_id, in_flight_archive, cx);
            });
        }

        let is_active = self
            .active_entry
            .as_ref()
            .is_some_and(|entry| thread_id.is_some_and(|tid| entry.is_active_thread(&tid)));

        if is_active {
            self.active_entry = None;
        }

        if !is_active {
            // The user is looking at a different thread/draft. Clear the
            // archived thread from its workspace's panel so that switching
            // to that workspace later doesn't show a stale thread.
            if let Some(folder_paths) = thread_folder_paths {
                if let Some(workspace) = self.multi_workspace.upgrade().and_then(|mw| {
                    mw.read(cx)
                        .workspace_for_paths(folder_paths, thread_remote_connection, cx)
                }) {
                    if let Some(panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
                        let panel_shows_archived = panel
                            .read(cx)
                            .active_conversation_view()
                            .map(|cv| cv.read(cx).parent_id())
                            .is_some_and(|live_thread_id| {
                                thread_id.is_some_and(|id| id == live_thread_id)
                            });
                        if panel_shows_archived {
                            panel.update(cx, |panel, cx| {
                                panel.clear_base_view(window, cx);
                            });
                        }
                    }
                }
            }
            return;
        }

        if neighbor.is_some_and(|neighbor| self.activate_entry(neighbor, window, cx)) {
            return;
        }

        // No neighbor or its workspace isn't open — just clear the
        // panel so the group is left empty.
        if let Some(folder_paths) = thread_folder_paths {
            let workspace = self.multi_workspace.upgrade().and_then(|mw| {
                mw.read(cx)
                    .workspace_for_paths(folder_paths, thread_remote_connection, cx)
            });
            if let Some(workspace) = workspace {
                if let Some(panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
                    panel.update(cx, |panel, cx| {
                        panel.clear_base_view(window, cx);
                    });
                }
            }
        }
    }

    fn start_archive_worktree_task(
        &self,
        thread_id: ThreadId,
        roots: Vec<thread_worktree_archive::RootPlan>,
        cx: &mut Context<Self>,
    ) -> Option<(Task<()>, async_channel::Sender<()>)> {
        if roots.is_empty() {
            return None;
        }

        self.delete_empty_drafts_for_archive_roots(&roots, cx);

        let (cancel_tx, cancel_rx) = async_channel::bounded::<()>(1);
        let task = cx.spawn(async move |_this, cx| {
            match Self::archive_worktree_roots(roots, cancel_rx, cx).await {
                Ok(ArchiveWorktreeOutcome::Success) => {
                    cx.update(|cx| {
                        ThreadMetadataStore::global(cx).update(cx, |store, _cx| {
                            store.cleanup_completed_archive(thread_id);
                        });
                    });
                }
                Ok(ArchiveWorktreeOutcome::Cancelled) => {}
                Err(error) => {
                    log::error!("Failed to archive worktree: {error:#}");
                    cx.update(|cx| {
                        ThreadMetadataStore::global(cx).update(cx, |store, cx| {
                            store.unarchive(thread_id, cx);
                        });
                    });
                }
            }
        });

        Some((task, cancel_tx))
    }

    fn start_detached_archive_worktree_task(
        &self,
        roots: Vec<thread_worktree_archive::RootPlan>,
        cx: &mut Context<Self>,
    ) {
        if roots.is_empty() {
            return;
        }

        self.delete_empty_drafts_for_archive_roots(&roots, cx);

        let (cancel_tx, cancel_rx) = async_channel::bounded::<()>(1);
        cx.spawn(async move |_this, cx| {
            let outcome = Self::archive_worktree_roots(roots, cancel_rx, cx).await;
            drop(cancel_tx);
            match outcome {
                Ok(ArchiveWorktreeOutcome::Success | ArchiveWorktreeOutcome::Cancelled) => {}
                Err(error) => {
                    log::error!("Failed to archive worktree after closing sidebar item: {error:#}");
                }
            }
        })
        .detach();
    }

    async fn archive_worktree_roots(
        roots: Vec<thread_worktree_archive::RootPlan>,
        cancel_rx: async_channel::Receiver<()>,
        cx: &mut gpui::AsyncApp,
    ) -> anyhow::Result<ArchiveWorktreeOutcome> {
        let mut completed_persists: Vec<(i64, thread_worktree_archive::RootPlan)> = Vec::new();

        for root in &roots {
            if cancel_rx.is_closed() {
                for &(id, ref completed_root) in completed_persists.iter().rev() {
                    thread_worktree_archive::rollback_persist(id, completed_root, cx).await;
                }
                return Ok(ArchiveWorktreeOutcome::Cancelled);
            }

            match thread_worktree_archive::persist_worktree_state(root, cx).await {
                Ok(id) => {
                    completed_persists.push((id, root.clone()));
                }
                Err(error) => {
                    for &(id, ref completed_root) in completed_persists.iter().rev() {
                        thread_worktree_archive::rollback_persist(id, completed_root, cx).await;
                    }
                    return Err(error);
                }
            }

            if cancel_rx.is_closed() {
                for &(id, ref completed_root) in completed_persists.iter().rev() {
                    thread_worktree_archive::rollback_persist(id, completed_root, cx).await;
                }
                return Ok(ArchiveWorktreeOutcome::Cancelled);
            }

            if let Err(error) = thread_worktree_archive::remove_root(root.clone(), cx).await {
                if let Some(&(id, ref completed_root)) = completed_persists.last() {
                    if completed_root.root_path == root.root_path {
                        thread_worktree_archive::rollback_persist(id, completed_root, cx).await;
                        completed_persists.pop();
                    }
                }
                for &(id, ref completed_root) in completed_persists.iter().rev() {
                    thread_worktree_archive::rollback_persist(id, completed_root, cx).await;
                }
                return Err(error);
            }
        }

        Ok(ArchiveWorktreeOutcome::Success)
    }

    fn activate_workspace(
        &self,
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(multi_workspace) = self.multi_workspace.upgrade() {
            multi_workspace.update(cx, |mw, cx| {
                mw.activate(workspace.clone(), None, window, cx);
            });
        }
    }

    fn archive_selected_thread(
        &mut self,
        _: &ArchiveSelectedThread,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(ix) = self.selection else {
            return;
        };
        match self.contents.entries.get(ix) {
            Some(ListEntry::Thread(thread)) => {
                match thread.status {
                    AgentThreadStatus::Running | AgentThreadStatus::WaitingForConfirmation => {
                        return;
                    }
                    AgentThreadStatus::Completed | AgentThreadStatus::Error => {}
                }
                if thread.draft.is_some() {
                    let workspace = thread.workspace.clone();
                    let draft_id = thread.metadata.thread_id;
                    self.remove_draft_with_confirmation(draft_id, workspace, window, cx);
                } else if let Some(session_id) = thread.metadata.session_id.clone() {
                    self.archive_thread(&session_id, window, cx);
                }
            }
            Some(ListEntry::Terminal(terminal)) => {
                let metadata = terminal.metadata.clone();
                let workspace = terminal.workspace.clone();
                let source = terminal.source.clone();
                let is_host_session =
                    matches!(&terminal.source, TerminalEntrySource::HostSession(_));
                let (_, requires_termination_confirmation) = terminal_row_close_presentation(
                    is_host_session,
                    terminal.runtime.as_ref().map(|runtime| runtime.state),
                );
                self.close_terminal_with_confirmation(
                    metadata,
                    workspace,
                    source,
                    requires_termination_confirmation,
                    window,
                    cx,
                );
            }
            _ => {}
        }
    }

    fn open_selected_review_brief(
        &mut self,
        _: &OpenSelectedReviewBrief,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let entry = self
            .selection
            .and_then(|ix| self.contents.entries.get(ix).cloned())
            .or_else(|| {
                self.contents.entries.iter().find_map(|entry| {
                    let is_active = match (entry, self.active_entry.as_ref()) {
                        (
                            ListEntry::Thread(thread),
                            Some(ActiveEntry::Thread { thread_id, .. }),
                        ) => thread.metadata.thread_id == *thread_id,
                        (
                            ListEntry::Terminal(terminal),
                            Some(ActiveEntry::Terminal { terminal_id, .. }),
                        ) => terminal.metadata.terminal_id == *terminal_id,
                        (ListEntry::ProjectHeader { .. }, _)
                        | (ListEntry::Thread(_), _)
                        | (ListEntry::Terminal(_), _) => false,
                    };
                    is_active.then(|| entry.clone())
                })
            });
        let Some(entry) = entry else {
            return;
        };
        match entry {
            ListEntry::Thread(thread) if thread.draft.is_none() => {
                let brief = thread.review_brief(cx);
                let owner_workspace = thread.workspace.clone();
                let review_workspace = match &owner_workspace {
                    ThreadEntryWorkspace::Open(workspace) => Some(workspace.clone()),
                    ThreadEntryWorkspace::Closed { .. } => self.active_workspace(cx),
                };
                match &owner_workspace {
                    ThreadEntryWorkspace::Open(workspace) => {
                        self.activate_thread(thread.metadata.clone(), workspace, false, window, cx)
                    }
                    ThreadEntryWorkspace::Closed {
                        folder_paths,
                        project_group_key,
                    } => self.open_workspace_and_activate_thread(
                        thread.metadata.clone(),
                        folder_paths.clone(),
                        project_group_key,
                        window,
                        cx,
                    ),
                }
                if let Some(workspace) = review_workspace {
                    Self::open_run_review_brief(brief, workspace, window, cx);
                }
            }
            ListEntry::Terminal(terminal) => {
                let brief = terminal.review_brief(cx);
                let owner_workspace = terminal.workspace.clone();
                let review_workspace = match &owner_workspace {
                    ThreadEntryWorkspace::Open(workspace) => Some(workspace.clone()),
                    ThreadEntryWorkspace::Closed { .. } => self.active_workspace(cx),
                };
                self.activate_terminal_entry(
                    terminal.metadata.clone(),
                    owner_workspace,
                    terminal.source.clone(),
                    false,
                    window,
                    cx,
                );
                if let Some(workspace) = review_workspace {
                    Self::open_run_review_brief(brief, workspace, window, cx);
                }
            }
            ListEntry::Thread(_) | ListEntry::ProjectHeader { .. } => {}
        }
    }

    fn rename_selected_thread(
        &mut self,
        _: &RenameSelectedThread,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(ix) = self.selection else {
            return;
        };
        let Some(ListEntry::Thread(thread)) = self.contents.entries.get(ix) else {
            return;
        };
        let thread_id = thread.metadata.thread_id;
        let title = thread.metadata.display_title();
        self.start_renaming_thread(ix, thread_id, title, window, cx);
    }

    fn record_thread_access(&mut self, id: &ThreadId) {
        self.thread_last_accessed.insert(*id, Utc::now());
    }

    fn record_terminal_access(&mut self, id: TerminalId) {
        self.terminal_last_accessed.insert(id, Utc::now());
    }

    fn record_thread_interacted(&mut self, thread_id: &agent_ui::ThreadId, cx: &mut App) {
        let store = ThreadMetadataStore::global(cx);
        store.update(cx, |store, cx| {
            store.update_interacted_at(thread_id, Utc::now(), cx);
        })
    }

    fn thread_display_time(metadata: &ThreadMetadata) -> DateTime<Utc> {
        metadata.interacted_at.unwrap_or(metadata.updated_at)
    }

    fn thread_creation_time(metadata: &ThreadMetadata) -> DateTime<Utc> {
        metadata.created_at.unwrap_or(metadata.updated_at)
    }

    fn push_entries_by_session_rail_sort(
        entries: &mut Vec<ListEntry>,
        terminals: Vec<TerminalEntry>,
        threads: Vec<Arc<ThreadEntry>>,
        sort_by: settings::SessionRailSorting,
        notified_threads: &HashSet<agent_ui::ThreadId>,
        notified_terminals: &HashSet<TerminalId>,
        manual_entry_order: &HashMap<ManualEntryOrderKey, usize>,
        current_session_ids: &mut HashSet<acp::SessionId>,
        current_thread_ids: &mut HashSet<agent_ui::ThreadId>,
    ) {
        fn display_time(entry: &ListEntry) -> DateTime<Utc> {
            match entry {
                ListEntry::Thread(thread) if thread.draft == Some(DraftKind::Empty) => {
                    DateTime::<Utc>::MAX_UTC
                }
                ListEntry::Thread(thread) => Sidebar::thread_display_time(&thread.metadata),
                ListEntry::Terminal(terminal) => terminal.metadata.created_at,
                ListEntry::ProjectHeader { .. } => unreachable!(),
            }
        }

        fn creation_time(entry: &ListEntry) -> DateTime<Utc> {
            match entry {
                ListEntry::Thread(thread) if thread.draft == Some(DraftKind::Empty) => {
                    DateTime::<Utc>::MAX_UTC
                }
                ListEntry::Thread(thread) => Sidebar::thread_creation_time(&thread.metadata),
                ListEntry::Terminal(terminal) => terminal.metadata.created_at,
                ListEntry::ProjectHeader { .. } => unreachable!(),
            }
        }

        fn has_attention(
            entry: &ListEntry,
            notified_threads: &HashSet<agent_ui::ThreadId>,
            notified_terminals: &HashSet<TerminalId>,
        ) -> bool {
            match entry {
                ListEntry::Thread(thread) => {
                    notified_threads.contains(&thread.metadata.thread_id)
                        || agent_status_needs_attention(thread.status)
                }
                ListEntry::Terminal(terminal) => {
                    terminal.needs_attention
                        || notified_terminals.contains(&terminal.metadata.terminal_id)
                }
                ListEntry::ProjectHeader { .. } => unreachable!(),
            }
        }

        fn agent_state_rank(entry: &ListEntry) -> u8 {
            match entry {
                ListEntry::Thread(thread) => match thread.status {
                    AgentThreadStatus::WaitingForConfirmation => 0,
                    AgentThreadStatus::Running => 1,
                    AgentThreadStatus::Error => 2,
                    AgentThreadStatus::Completed => 3,
                },
                ListEntry::Terminal(terminal)
                    if terminal.needs_attention
                        && terminal.attention_priority == TerminalAttentionPriority::Urgent =>
                {
                    0
                }
                ListEntry::Terminal(terminal) if terminal.needs_attention => 1,
                ListEntry::Terminal(terminal) if terminal.runtime.is_some() => 2,
                ListEntry::Terminal(_) => 3,
                ListEntry::ProjectHeader { .. } => unreachable!(),
            }
        }

        fn project_sort_key(entry: &ListEntry) -> String {
            let worktrees = match entry {
                ListEntry::Thread(thread) => &thread.worktrees,
                ListEntry::Terminal(terminal) => &terminal.worktrees,
                ListEntry::ProjectHeader { .. } => unreachable!(),
            };

            worktrees
                .iter()
                .find_map(|worktree| worktree.worktree_name.as_ref())
                .or_else(|| worktrees.first().map(|worktree| &worktree.full_path))
                .map(|label| label.to_string().to_lowercase())
                .unwrap_or_default()
        }

        let row_entries = terminals
            .into_iter()
            .map(ListEntry::Terminal)
            .chain(threads.into_iter().map(ListEntry::Thread))
            .sorted_by(|left, right| match sort_by {
                settings::SessionRailSorting::Attention => {
                    has_attention(right, notified_threads, notified_terminals)
                        .cmp(&has_attention(left, notified_threads, notified_terminals))
                        .then_with(|| display_time(right).cmp(&display_time(left)))
                }
                settings::SessionRailSorting::AgentState => agent_state_rank(left)
                    .cmp(&agent_state_rank(right))
                    .then_with(|| display_time(right).cmp(&display_time(left))),
                settings::SessionRailSorting::CreationTime => {
                    creation_time(right).cmp(&creation_time(left))
                }
                settings::SessionRailSorting::Project => project_sort_key(left)
                    .cmp(&project_sort_key(right))
                    .then_with(|| display_time(right).cmp(&display_time(left))),
                settings::SessionRailSorting::Manual => {
                    let left_order = ManualEntryOrderKey::from_entry(left)
                        .and_then(|key| manual_entry_order.get(&key).copied());
                    let right_order = ManualEntryOrderKey::from_entry(right)
                        .and_then(|key| manual_entry_order.get(&key).copied());

                    match (left_order, right_order) {
                        (Some(left_order), Some(right_order)) => left_order.cmp(&right_order),
                        (Some(_), None) => Ordering::Less,
                        (None, Some(_)) => Ordering::Greater,
                        (None, None) => display_time(right).cmp(&display_time(left)),
                    }
                }
                settings::SessionRailSorting::RecentActivity => {
                    display_time(right).cmp(&display_time(left))
                }
            });

        for entry in row_entries {
            if let ListEntry::Thread(thread) = &entry {
                if let Some(session_id) = &thread.metadata.session_id {
                    current_session_ids.insert(session_id.clone());
                }
                current_thread_ids.insert(thread.metadata.thread_id);
            }
            entries.push(entry);
        }
    }

    /// The sort order used by the ctrl-tab switcher
    fn switcher_entry_cmp(
        &self,
        left: &ThreadSwitcherEntry,
        right: &ThreadSwitcherEntry,
    ) -> Ordering {
        let sort_time = |entry: &ThreadSwitcherEntry| match entry {
            ThreadSwitcherEntry::Thread(entry) => self
                .thread_last_accessed
                .get(&entry.metadata.thread_id)
                .copied()
                .or(entry.metadata.interacted_at)
                .unwrap_or(entry.metadata.updated_at),
            ThreadSwitcherEntry::Terminal(entry) => self
                .terminal_last_accessed
                .get(&entry.metadata.terminal_id)
                .copied()
                .unwrap_or(entry.metadata.created_at),
        };

        // .reverse() = most recent first
        sort_time(left).cmp(&sort_time(right)).reverse()
    }

    fn mru_entries_for_switcher(&self, cx: &App) -> Vec<ThreadSwitcherEntry> {
        let mut current_header_label: Option<SharedString> = None;
        let mut current_header_key: Option<ProjectGroupKey> = None;
        let mut entries: Vec<ThreadSwitcherEntry> = self
            .contents
            .entries
            .iter()
            .filter_map(|entry| match entry {
                ListEntry::ProjectHeader { label, key, .. } => {
                    current_header_label = Some(label.clone());
                    current_header_key = Some(key.clone());
                    None
                }
                ListEntry::Thread(thread) => {
                    if thread.draft == Some(DraftKind::Empty) {
                        return None;
                    }
                    let workspace = match &thread.workspace {
                        ThreadEntryWorkspace::Open(workspace) => Some(workspace.clone()),
                        ThreadEntryWorkspace::Closed { .. } => {
                            current_header_key.as_ref().and_then(|key| {
                                self.multi_workspace.upgrade().and_then(|mw| {
                                    mw.read(cx).workspace_for_paths(
                                        key.path_list(),
                                        key.host().as_ref(),
                                        cx,
                                    )
                                })
                            })
                        }
                    }?;
                    let notified = self.contents.is_thread_notified(&thread.metadata.thread_id);
                    let timestamp: SharedString =
                        format_history_entry_timestamp(Self::thread_display_time(&thread.metadata))
                            .into();
                    Some(ThreadSwitcherEntry::Thread(ThreadSwitcherThreadEntry {
                        title: thread.metadata.display_title(),
                        icon: thread.icon,
                        icon_from_external_svg: thread.icon_from_external_svg.clone(),
                        status: thread.status,
                        metadata: thread.metadata.clone(),
                        workspace,
                        project_name: current_header_label.clone(),
                        worktrees: thread
                            .worktrees
                            .iter()
                            .cloned()
                            .map(|mut wt| {
                                wt.highlight_positions = Vec::new();
                                wt
                            })
                            .collect(),
                        diff_stats: thread.diff_stats,
                        is_draft: thread.draft.is_some(),
                        is_title_generating: thread.is_title_generating,
                        notified,
                        timestamp,
                    }))
                }
                ListEntry::Terminal(terminal) => {
                    let timestamp: SharedString =
                        format_history_entry_timestamp(terminal.metadata.created_at).into();
                    Some(ThreadSwitcherEntry::Terminal(ThreadSwitcherTerminalEntry {
                        metadata: terminal.metadata.clone(),
                        workspace: terminal.workspace.clone(),
                        source: terminal.source.clone(),
                        detected_agent_kind: terminal.detected_agent_kind,
                        project_name: current_header_label.clone(),
                        worktrees: terminal
                            .worktrees
                            .iter()
                            .cloned()
                            .map(|mut wt| {
                                wt.highlight_positions = Vec::new();
                                wt
                            })
                            .collect(),
                        notified: self
                            .contents
                            .is_terminal_notified(terminal.metadata.terminal_id),
                        timestamp,
                    }))
                }
            })
            .collect();

        entries.sort_by(|a, b| self.switcher_entry_cmp(a, b));

        entries
    }

    fn switcher_selection_for_active_entry(
        &self,
        entries: &[ThreadSwitcherEntry],
    ) -> Option<ThreadSwitcherSelection> {
        let active_entry = self.active_entry.as_ref()?;
        entries
            .iter()
            .find_map(|switcher_entry| match (active_entry, switcher_entry) {
                (
                    ActiveEntry::Thread {
                        thread_id,
                        session_id,
                        ..
                    },
                    ThreadSwitcherEntry::Thread(entry),
                ) if entry.metadata.thread_id == *thread_id
                    || session_id
                        .as_ref()
                        .zip(entry.metadata.session_id.as_ref())
                        .is_some_and(|(active, candidate)| active == candidate) =>
                {
                    Some(switcher_entry.selection())
                }
                (
                    ActiveEntry::Terminal { terminal_id, .. },
                    ThreadSwitcherEntry::Terminal(entry),
                ) if entry.metadata.terminal_id == *terminal_id => Some(switcher_entry.selection()),
                _ => None,
            })
    }

    fn dismiss_thread_switcher(&mut self, cx: &mut Context<Self>) {
        self.thread_switcher = None;
        self._thread_switcher_subscriptions.clear();
        if let Some(mw) = self.multi_workspace.upgrade() {
            mw.update(cx, |mw, cx| {
                mw.set_sidebar_overlay(None, cx);
            });
        }
    }

    fn on_toggle_thread_switcher(
        &mut self,
        action: &ToggleThreadSwitcher,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.toggle_thread_switcher_impl(action.select_last, window, cx);
    }

    fn preview_switcher_selection(
        &mut self,
        selection: &ThreadSwitcherSelection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match selection {
            ThreadSwitcherSelection::Thread {
                metadata,
                workspace,
            } => {
                if let Some(multi_workspace) = self.multi_workspace.upgrade() {
                    multi_workspace.update(cx, |multi_workspace, cx| {
                        multi_workspace.activate(workspace.clone(), None, window, cx);
                    });
                }
                self.active_entry = Some(ActiveEntry::Thread {
                    thread_id: metadata.thread_id,
                    session_id: metadata.session_id.clone(),
                    workspace: workspace.clone(),
                });
                self.update_entries(cx);
                Self::load_agent_thread_in_workspace(workspace, metadata, false, window, cx);
            }
            ThreadSwitcherSelection::Terminal {
                metadata,
                workspace,
                source,
            } => {
                if let ThreadEntryWorkspace::Open(workspace) = workspace {
                    match source {
                        TerminalEntrySource::WorkspaceItem(terminal_view) => {
                            self.activate_workspace_terminal_item(
                                workspace,
                                terminal_view,
                                metadata.clone(),
                                false,
                                false,
                                window,
                                cx,
                            );
                        }
                        TerminalEntrySource::AgentPanel => {
                            if let Some(multi_workspace) = self.multi_workspace.upgrade() {
                                multi_workspace.update(cx, |multi_workspace, cx| {
                                    multi_workspace.activate(workspace.clone(), None, window, cx);
                                });
                            }
                            self.active_entry = Some(ActiveEntry::Terminal {
                                terminal_id: metadata.terminal_id,
                                workspace: workspace.clone(),
                            });
                            self.update_entries(cx);
                            Self::load_agent_terminal_in_workspace(
                                workspace, metadata, false, window, cx,
                            );
                        }
                        TerminalEntrySource::HostSession(session_id) => {
                            self.attach_host_terminal_session(
                                workspace,
                                metadata.clone(),
                                *session_id,
                                false,
                                false,
                                window,
                                cx,
                            );
                        }
                    }
                }
            }
        }
    }

    fn confirm_switcher_selection(
        &mut self,
        selection: &ThreadSwitcherSelection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match selection {
            ThreadSwitcherSelection::Thread {
                metadata,
                workspace,
            } => {
                if let Some(multi_workspace) = self.multi_workspace.upgrade() {
                    multi_workspace.update(cx, |multi_workspace, cx| {
                        multi_workspace.activate(workspace.clone(), None, window, cx);
                        multi_workspace.retain_active_workspace(cx);
                    });
                }
                self.record_thread_access(&metadata.thread_id);
                self.active_entry = Some(ActiveEntry::Thread {
                    thread_id: metadata.thread_id,
                    session_id: metadata.session_id.clone(),
                    workspace: workspace.clone(),
                });
                self.update_entries(cx);
                self.dismiss_thread_switcher(cx);
                Self::load_agent_thread_in_workspace(workspace, metadata, true, window, cx);
            }
            ThreadSwitcherSelection::Terminal {
                metadata,
                workspace,
                source,
            } => {
                self.dismiss_thread_switcher(cx);
                self.activate_terminal_entry(
                    metadata.clone(),
                    workspace.clone(),
                    source.clone(),
                    true,
                    window,
                    cx,
                );
            }
        }
    }

    fn toggle_thread_switcher_impl(
        &mut self,
        select_last: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(thread_switcher) = &self.thread_switcher {
            thread_switcher.update(cx, |switcher, cx| {
                if select_last {
                    switcher.select_last(cx);
                } else {
                    switcher.cycle_selection(cx);
                }
            });
            return;
        }

        let entries = self.mru_entries_for_switcher(cx);
        if entries.len() < 2 {
            return;
        }

        let weak_multi_workspace = self.multi_workspace.clone();

        // Preserve the exact source as well as identity. A center terminal,
        // Host Session, and retained compatibility terminal can share the same
        // row shape but require different restoration paths when a preview is
        // cancelled.
        let original_selection = self.switcher_selection_for_active_entry(&entries);
        let original_workspace = self
            .active_entry
            .as_ref()
            .map(|entry| entry.workspace().clone())
            .or_else(|| {
                self.multi_workspace
                    .upgrade()
                    .map(|mw| mw.read(cx).workspace().clone())
            });

        let thread_switcher = cx.new(|cx| ThreadSwitcher::new(entries, select_last, window, cx));

        let mut subscriptions = Vec::new();

        subscriptions.push(cx.subscribe_in(&thread_switcher, window, {
            let thread_switcher = thread_switcher.clone();
            move |this, _emitter, event: &ThreadSwitcherEvent, window, cx| match event {
                ThreadSwitcherEvent::Preview(selection) => {
                    this.preview_switcher_selection(selection, window, cx);
                    let focus = thread_switcher.focus_handle(cx);
                    window.focus(&focus, cx);
                }
                ThreadSwitcherEvent::Confirmed(selection) => {
                    this.confirm_switcher_selection(selection, window, cx);
                }
                ThreadSwitcherEvent::Dismissed => {
                    if let Some(selection) = &original_selection {
                        this.preview_switcher_selection(selection, window, cx);
                    } else if let Some(mw) = weak_multi_workspace.upgrade() {
                        if let Some(original_ws) = &original_workspace {
                            mw.update(cx, |mw, cx| {
                                mw.activate(original_ws.clone(), None, window, cx);
                            });
                        }
                    }
                    this.dismiss_thread_switcher(cx);
                }
            }
        }));

        subscriptions.push(cx.subscribe_in(
            &thread_switcher,
            window,
            |this, _emitter, _event: &gpui::DismissEvent, _window, cx| {
                this.dismiss_thread_switcher(cx);
            },
        ));

        let focus = thread_switcher.focus_handle(cx);
        let overlay_view = gpui::AnyView::from(thread_switcher.clone());

        // Replay the initial preview that was emitted during construction
        // before subscriptions were wired up.
        let initial_preview = thread_switcher
            .read(cx)
            .selected_entry()
            .map(ThreadSwitcherEntry::selection);

        self.thread_switcher = Some(thread_switcher);
        self._thread_switcher_subscriptions = subscriptions;
        if let Some(mw) = self.multi_workspace.upgrade() {
            mw.update(cx, |mw, cx| {
                mw.set_sidebar_overlay(Some(overlay_view), cx);
            });
        }

        if let Some(selection) = initial_preview {
            self.preview_switcher_selection(&selection, window, cx);
        }

        window.focus(&focus, cx);
    }

    fn render_thread(
        &self,
        ix: usize,
        thread: &ThreadEntry,
        is_active: bool,
        is_focused: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let has_notification = self.contents.is_thread_notified(&thread.metadata.thread_id);

        let title: SharedString = thread.metadata.display_title();
        let metadata = thread.metadata.clone();
        let thread_workspace = thread.workspace.clone();
        let sidebar = cx.weak_entity();
        let hover_review_workspace = match &thread.workspace {
            ThreadEntryWorkspace::Open(workspace) => Some(workspace.clone()),
            ThreadEntryWorkspace::Closed { .. } => self.active_workspace(cx),
        };
        let hover_review_brief = thread.review_brief(cx);

        let is_hovered = self.hovered_thread_index == Some(ix);
        let is_selected = is_active;
        let is_draft = thread.draft.is_some();
        let is_empty_draft = thread.draft == Some(DraftKind::Empty);
        let is_running = matches!(
            thread.status,
            AgentThreadStatus::Running | AgentThreadStatus::WaitingForConfirmation
        );
        let has_changes = thread.diff_stats.lines_added > 0 || thread.diff_stats.lines_removed > 0;
        let is_renaming = self.renaming_thread_id == Some(thread.metadata.thread_id);

        let thread_id_for_actions = thread.metadata.thread_id;
        let session_id_for_delete = thread.metadata.session_id.clone();
        let focus_handle = self.focus_handle.clone();
        let title_editor = self.thread_rename_editor.clone();
        let rename_label = agent_session_label(APP_NAME, "Rename Thread", "Rename Agent Session");
        let rename_title_label =
            agent_session_label(APP_NAME, "Rename Title", "Rename Agent Session");
        let archive_label =
            agent_session_label(APP_NAME, "Archive Thread", "Archive Agent Session");
        let regenerate_title_label = agent_session_label(
            APP_NAME,
            "Regenerate Thread Title",
            "Regenerate Agent Session Title",
        );
        let open_as_markdown_label = agent_session_label(
            APP_NAME,
            "Open Thread as Markdown",
            "Open Agent Session as Markdown",
        );

        let id = SharedString::from(format!("thread-entry-{}", ix));

        let color = cx.theme().colors();
        let sidebar_bg = color.editor_background;
        let session_rail_settings = SessionRailSettings::get_global(cx);
        let rail_width = session_rail_settings.width(self.width);
        let compact_row = session_rail_row_is_compact(rail_width);
        let supplemental_metadata_visible = session_rail_supplemental_metadata_visible(rail_width);
        let design_system = DesignSystemSettings::get_global(cx);
        let labels_visible = session_rail_labels_visible(&design_system);
        let show_agent_attention =
            WorkspaceBarAttentionSettings::get_global(cx).show_agent_attention;

        let timestamp: SharedString =
            if is_empty_draft || !session_rail_recency_visible(rail_width, has_changes) {
                SharedString::default()
            } else if !session_rail_settings.show_latest_attention_metadata {
                SharedString::default()
            } else {
                format_history_entry_timestamp(Self::thread_display_time(&thread.metadata)).into()
            };

        let is_remote = thread.workspace.is_remote(cx);

        let worktrees =
            if session_rail_settings.show_worktree_metadata && supplemental_metadata_visible {
                apply_worktree_label_mode(
                    thread.worktrees.clone(),
                    cx.flag_value::<AgentThreadWorktreeLabelFlag>(),
                )
            } else {
                Vec::new()
            };

        let (icon, icon_svg) = if is_draft {
            (IconName::Circle, None)
        } else {
            (thread.icon, thread.icon_from_external_svg.clone())
        };
        let actor_label: SharedString =
            if thread.metadata.agent_id.as_ref() == ZED_AGENT_ID.as_ref() {
                "Dez Agent".into()
            } else {
                thread.metadata.agent_id.as_ref().to_owned().into()
            };
        let state_label = if is_draft {
            "Draft"
        } else {
            match thread.status {
                AgentThreadStatus::Running => "Running",
                AgentThreadStatus::WaitingForConfirmation => "Waiting for permission",
                AgentThreadStatus::Error => "Error",
                AgentThreadStatus::Completed => "Completed",
            }
        };

        let title_generating = thread.is_title_generating
            || self
                .regenerating_titles
                .contains(&thread.metadata.thread_id);

        let thread_item =
            canvas_thread_item_style(ThreadItem::new(id, title.clone()), &design_system)
                .base_bg(sidebar_bg)
                .icon(icon)
                .when(is_draft, |this| {
                    this.icon_color(Color::Custom(cx.theme().colors().icon_muted.opacity(0.2)))
                })
                .status(
                    session_rail_settings
                        .show_agent_state_metadata
                        .then_some(thread.status)
                        .unwrap_or_default(),
                )
                .when(session_rail_settings.show_agent_state_metadata, |this| {
                    this.actor_label(actor_label).state_label(state_label)
                })
                .actor_label_visible(supplemental_metadata_visible)
                .is_remote(is_remote)
                .when_some(icon_svg, |this, svg| {
                    this.custom_icon_from_external_svg(svg)
                })
                .worktrees(worktrees)
                .timestamp(timestamp)
                .highlight_positions(thread.highlight_positions.to_vec())
                .title_generating(title_generating)
                .labels_visible(labels_visible)
                .notified(
                    show_agent_attention
                        && session_rail_settings.show_latest_attention_metadata
                        && has_notification,
                )
                .when(thread.diff_stats.lines_added > 0, |this| {
                    this.added(thread.diff_stats.lines_added as usize)
                })
                .when(thread.diff_stats.lines_removed > 0, |this| {
                    this.removed(thread.diff_stats.lines_removed as usize)
                })
                .selected(is_selected)
                .focused(is_focused)
                .hovered(is_hovered)
                .on_hover(cx.listener(move |this, is_hovered: &bool, _window, cx| {
                    if *is_hovered {
                        this.hovered_thread_index = Some(ix);
                    } else if this.hovered_thread_index == Some(ix) {
                        this.hovered_thread_index = None;
                    }
                    cx.notify();
                }))
                .when(is_renaming, |this| {
                    this.is_truncated(false).title_slot(
                        div()
                            .h_full()
                            .min_w_0()
                            .flex_1()
                            .capture_action(cx.listener(
                                |this, _: &editor::actions::Newline, window, cx| {
                                    this.finish_thread_rename(window, cx);
                                },
                            ))
                            .on_action(cx.listener(|this, _: &Confirm, window, cx| {
                                this.finish_thread_rename(window, cx);
                            }))
                            .on_action(cx.listener(
                                |this, _: &editor::actions::Cancel, window, cx| {
                                    this.finish_thread_rename(window, cx);
                                },
                            ))
                            .child(title_editor),
                    )
                })
                .when(is_hovered && !is_renaming, |this| {
                    let rename_button = IconButton::new(("rename-thread", ix), IconName::Pencil)
                        .size(ButtonSize::Medium)
                        .icon_size(IconSize::Small)
                        .aria_label(rename_label)
                        .tooltip({
                            let focus_handle = focus_handle.clone();
                            move |_window, cx| {
                                Tooltip::for_action_in(
                                    rename_label,
                                    &RenameSelectedThread,
                                    &focus_handle,
                                    cx,
                                )
                            }
                        })
                        .on_click({
                            let title = title.clone();
                            cx.listener(move |this, _, window, cx| {
                                this.start_renaming_thread(
                                    ix,
                                    thread_id_for_actions,
                                    title.clone(),
                                    window,
                                    cx,
                                );
                            })
                        });

                    let contextual_action: Option<AnyElement> = if is_running {
                        Some(
                            IconButton::new("stop-thread", IconName::Stop)
                                .size(ButtonSize::Medium)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Error)
                                .style(ButtonStyle::Tinted(TintColor::Error))
                                .aria_label("Stop Generation")
                                .tooltip(Tooltip::text("Stop Generation"))
                                .on_click(cx.listener(move |this, _, _window, cx| {
                                    this.stop_thread(&thread_id_for_actions, cx);
                                }))
                                .into_any_element(),
                        )
                    } else {
                        match thread.draft {
                            Some(DraftKind::Empty) => None,
                            Some(DraftKind::WithContent) => Some(
                                IconButton::new("discard_thread", IconName::Close)
                                    .size(ButtonSize::Medium)
                                    .icon_size(IconSize::Small)
                                    .aria_label("Discard Draft")
                                    .tooltip(Tooltip::text("Discard Draft"))
                                    .on_click({
                                        let thread_workspace = thread_workspace.clone();
                                        cx.listener(move |this, _, window, cx| {
                                            this.remove_draft_with_confirmation(
                                                thread_id_for_actions,
                                                thread_workspace.clone(),
                                                window,
                                                cx,
                                            );
                                        })
                                    })
                                    .into_any_element(),
                            ),
                            None => Some(
                                IconButton::new("archive-thread", IconName::Archive)
                                    .size(ButtonSize::Medium)
                                    .icon_size(IconSize::Small)
                                    .aria_label(archive_label)
                                    .tooltip({
                                        let focus_handle = focus_handle.clone();
                                        move |_window, cx| {
                                            Tooltip::for_action_in(
                                                archive_label,
                                                &ArchiveSelectedThread,
                                                &focus_handle,
                                                cx,
                                            )
                                        }
                                    })
                                    .on_click({
                                        let session_id = session_id_for_delete.clone();
                                        cx.listener(move |this, _, window, cx| {
                                            if let Some(ref session_id) = session_id {
                                                this.archive_thread(session_id, window, cx);
                                            }
                                        })
                                    })
                                    .into_any_element(),
                            ),
                        }
                    };

                    this.action_slot(
                        h_flex()
                            .gap_0p5()
                            .when(!compact_row, |this| this.child(rename_button))
                            .when(!compact_row && has_changes, |this| {
                                let metadata = thread.metadata.clone();
                                let owner_workspace = thread.workspace.clone();
                                this.child(
                                    IconButton::new(("review-thread-changes", ix), IconName::Diff)
                                        .size(ButtonSize::Medium)
                                        .icon_size(IconSize::Small)
                                        .aria_label("Review Changes")
                                        .tooltip(Tooltip::text("Review Changes"))
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            if let ThreadEntryWorkspace::Open(workspace) =
                                                &owner_workspace
                                            {
                                                this.activate_thread(
                                                    metadata.clone(),
                                                    workspace,
                                                    true,
                                                    window,
                                                    cx,
                                                );
                                                window.dispatch_action(
                                                    OpenAgentDiff.boxed_clone(),
                                                    cx,
                                                );
                                            }
                                        })),
                                )
                            })
                            .when_some(
                                (!is_draft)
                                    .then_some(hover_review_workspace.clone())
                                    .flatten(),
                                |this, review_workspace| {
                                    this.child(
                                        IconButton::new(
                                            ("review-thread-run", ix),
                                            IconName::ListTodo,
                                        )
                                        .size(ButtonSize::Medium)
                                        .icon_size(IconSize::Small)
                                        .aria_label("Open Review Brief")
                                        .aria_keyshortcuts("Shift+V")
                                        .tooltip({
                                            let focus_handle = focus_handle.clone();
                                            move |_window, cx| {
                                                Tooltip::for_action_in(
                                                    "Open Review Brief",
                                                    &OpenSelectedReviewBrief,
                                                    &focus_handle,
                                                    cx,
                                                )
                                            }
                                        })
                                        .on_click({
                                            let review_brief = hover_review_brief.clone();
                                            let sidebar = sidebar.clone();
                                            let metadata = thread.metadata.clone();
                                            let owner_workspace = thread.workspace.clone();
                                            move |_, window, cx| {
                                                Self::open_thread_run_review(
                                                    sidebar.clone(),
                                                    metadata.clone(),
                                                    owner_workspace.clone(),
                                                    review_brief.clone(),
                                                    review_workspace.clone(),
                                                    window,
                                                    cx,
                                                );
                                            }
                                        }),
                                    )
                                },
                            )
                            .when_some(contextual_action, |this, action| this.child(action)),
                    )
                })
                .on_click({
                    let thread_workspace = thread_workspace.clone();
                    cx.listener(move |this, _, window, cx| {
                        this.selection = None;
                        match &thread_workspace {
                            ThreadEntryWorkspace::Open(workspace) => {
                                this.activate_thread(
                                    metadata.clone(),
                                    workspace,
                                    false,
                                    window,
                                    cx,
                                );
                            }
                            ThreadEntryWorkspace::Closed {
                                folder_paths,
                                project_group_key,
                            } => {
                                this.open_workspace_and_activate_thread(
                                    metadata.clone(),
                                    folder_paths.clone(),
                                    project_group_key,
                                    window,
                                    cx,
                                );
                            }
                        }
                    })
                });

        if is_draft || thread.metadata.session_id.is_none() {
            return thread_item.into_any_element();
        }

        let Some(session_id) = thread.metadata.session_id.clone() else {
            return thread_item.into_any_element();
        };

        let context_menu_id = SharedString::from(format!("thread-context-menu-{}", ix));
        let active_workspace = self.active_workspace(cx);
        let thread_workspace = match &thread_workspace {
            ThreadEntryWorkspace::Open(workspace) => Some(workspace.clone()),
            ThreadEntryWorkspace::Closed { .. } => None,
        };
        let review_workspace = thread_workspace
            .clone()
            .or_else(|| active_workspace.clone());
        let review_brief = thread.review_brief(cx);
        let review_owner_workspace = thread.workspace.clone();
        let review_metadata = thread.metadata.clone();
        let has_changes = thread.diff_stats.lines_added > 0 || thread.diff_stats.lines_removed > 0;

        let is_zed_thread = thread.metadata.agent_id.as_ref() == ZED_AGENT_ID.as_ref();
        let can_open_as_markdown = thread.is_live || is_zed_thread;
        let folder_paths = thread.metadata.folder_paths().clone();

        right_click_menu(context_menu_id)
            .trigger(move |_, _, _| thread_item)
            .menu({
                let thread_id = thread.metadata.thread_id;
                let markdown_title = Some(thread.metadata.display_title());
                let rename_title = title;
                let review_brief = review_brief.clone();
                let review_owner_workspace = review_owner_workspace.clone();
                let review_metadata = review_metadata.clone();
                move |_window, cx| {
                    let session_id = session_id.clone();
                    let sidebar = sidebar.clone();
                    let active_workspace = active_workspace.clone();
                    let thread_workspace = thread_workspace.clone();
                    let markdown_title = markdown_title.clone();
                    let rename_title = rename_title.clone();
                    let folder_paths = folder_paths.clone();
                    let review_workspace = review_workspace.clone();
                    let review_brief = review_brief.clone();
                    let review_owner_workspace = review_owner_workspace.clone();
                    let review_metadata = review_metadata.clone();
                    ContextMenu::build(_window, cx, move |mut menu, _window, _cx| {
                        menu = menu.entry(rename_title_label, None, {
                            let sidebar = sidebar.clone();
                            let rename_title = rename_title.clone();
                            move |window, cx| {
                                sidebar
                                    .update(cx, |sidebar, cx| {
                                        sidebar.start_renaming_thread(
                                            ix,
                                            thread_id,
                                            rename_title.clone(),
                                            window,
                                            cx,
                                        );
                                    })
                                    .ok();
                            }
                        });

                        if is_zed_thread {
                            menu = menu.entry(regenerate_title_label, None, {
                                let session_id = session_id.clone();
                                let sidebar = sidebar.clone();
                                let thread_workspace = thread_workspace.clone();
                                let folder_paths = folder_paths.clone();
                                move |_window, cx| {
                                    sidebar
                                        .update(cx, |sidebar, cx| {
                                            sidebar.regenerate_thread_title(
                                                &session_id,
                                                thread_id,
                                                folder_paths.clone(),
                                                thread_workspace.clone(),
                                                cx,
                                            );
                                        })
                                        .ok();
                                }
                            });
                        }

                        if let Some(review_workspace) = review_workspace.clone() {
                            menu = menu.entry("Open Review Brief", None, {
                                let review_brief = review_brief.clone();
                                let sidebar = sidebar.clone();
                                let review_owner_workspace = review_owner_workspace.clone();
                                let review_metadata = review_metadata.clone();
                                move |window, cx| {
                                    Self::open_thread_run_review(
                                        sidebar.clone(),
                                        review_metadata.clone(),
                                        review_owner_workspace.clone(),
                                        review_brief.clone(),
                                        review_workspace.clone(),
                                        window,
                                        cx,
                                    );
                                }
                            });
                        }

                        if has_changes && let Some(owner_workspace) = thread_workspace.clone() {
                            menu = menu.entry("Review Changes", None, {
                                let sidebar = sidebar.clone();
                                let metadata = review_metadata.clone();
                                move |window, cx| {
                                    sidebar
                                        .update(cx, |sidebar, cx| {
                                            sidebar.activate_thread(
                                                metadata.clone(),
                                                &owner_workspace,
                                                true,
                                                window,
                                                cx,
                                            );
                                        })
                                        .ok();
                                    window.dispatch_action(OpenAgentDiff.boxed_clone(), cx);
                                }
                            });
                        }

                        if can_open_as_markdown {
                            menu = menu.entry(open_as_markdown_label, None, {
                                let session_id = session_id.clone();
                                let markdown_title = markdown_title.clone();
                                let thread_workspace = thread_workspace.clone();
                                move |window, cx| {
                                    if let Some(thread_workspace) = thread_workspace.as_ref()
                                        && let Some(panel) =
                                            thread_workspace.read(cx).panel::<AgentPanel>(cx)
                                    {
                                        let opened = panel.update(cx, |panel, cx| {
                                            panel.open_thread_as_markdown(
                                                thread_id,
                                                thread_workspace.clone(),
                                                window,
                                                cx,
                                            )
                                        });
                                        if opened {
                                            return;
                                        }
                                    }

                                    if is_zed_thread
                                        && let Some(active_workspace) = &active_workspace
                                    {
                                        Self::open_closed_native_thread_as_markdown(
                                            &session_id,
                                            markdown_title.clone(),
                                            active_workspace,
                                            window,
                                            cx,
                                        );
                                    }
                                }
                            });
                        }

                        menu.separator().entry(archive_label, None, {
                            let session_id = session_id.clone();
                            move |window, cx| {
                                sidebar
                                    .update(cx, |sidebar, cx| {
                                        sidebar.archive_thread(&session_id, window, cx);
                                    })
                                    .ok();
                            }
                        })
                    })
                }
            })
            .into_any_element()
    }

    fn render_terminal(
        &self,
        ix: usize,
        terminal: &TerminalEntry,
        is_active: bool,
        is_focused: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let id = ElementId::from(format!("terminal-{}", terminal.metadata.terminal_id));
        let latest_agent_event_at = terminal
            .agent
            .as_ref()
            .and_then(|agent| agent.events.last())
            .filter(|event| event.observed_at_unix_ms > 0)
            .and_then(|event| {
                let seconds = i64::try_from(event.observed_at_unix_ms / 1000).ok()?;
                let nanoseconds = u32::try_from(event.observed_at_unix_ms % 1000)
                    .ok()?
                    .saturating_mul(1_000_000);
                Utc.timestamp_opt(seconds, nanoseconds).single()
            });
        let timestamp = format_history_entry_timestamp(
            latest_agent_event_at.unwrap_or(terminal.metadata.created_at),
        );
        let is_hovered = self.hovered_thread_index == Some(ix);
        let color = cx.theme().colors();
        let sidebar_bg = color.editor_background;
        let metadata = terminal.metadata.clone();
        let workspace = terminal.workspace.clone();
        let source = terminal.source.clone();
        let sidebar = cx.weak_entity();
        let review_action_metadata = metadata.clone();
        let review_action_workspace = workspace.clone();
        let review_action_source = source.clone();
        let close_action_metadata = metadata.clone();
        let close_action_workspace = workspace.clone();
        let close_action_source = source.clone();
        let review_brief = terminal.review_brief(cx);
        let evidence_label = observed_run_evidence_label(
            &review_brief.checks,
            review_brief.commands.len(),
            terminal
                .agent
                .as_ref()
                .is_some_and(|agent| agent.events_truncated),
        );
        let review_workspace = match &terminal.workspace {
            ThreadEntryWorkspace::Open(workspace) => Some(workspace.clone()),
            ThreadEntryWorkspace::Closed { .. } => self.active_workspace(cx),
        };
        let focus_handle = self.focus_handle.clone();
        let session_rail_settings = SessionRailSettings::get_global(cx);
        let rail_width = session_rail_settings.width(self.width);
        let supplemental_metadata_visible = session_rail_supplemental_metadata_visible(rail_width);
        let has_evidence = evidence_label.is_some();
        let worktrees =
            if session_rail_settings.show_worktree_metadata && supplemental_metadata_visible {
                apply_worktree_label_mode(
                    terminal.worktrees.clone(),
                    cx.flag_value::<AgentThreadWorktreeLabelFlag>(),
                )
            } else {
                Vec::new()
            };
        let is_remote = terminal.workspace.is_remote(cx);

        let display_title = terminal.metadata.display_title();
        let agent_ui_settings = CanvasAgentUiSettings::get_global(cx);
        let design_system = DesignSystemSettings::get_global(cx);
        let labels_visible = session_rail_labels_visible(&design_system);
        let show_agent_attention =
            WorkspaceBarAttentionSettings::get_global(cx).show_agent_attention;
        let terminal_agent_kind = terminal.detected_agent_kind;
        let is_host_session = matches!(&terminal.source, TerminalEntrySource::HostSession(_));
        let (close_label, requires_termination_confirmation) = terminal_row_close_presentation(
            is_host_session,
            terminal.runtime.as_ref().map(|runtime| runtime.state),
        );
        let close_icon = if requires_termination_confirmation {
            IconName::Stop
        } else {
            IconName::Close
        };
        let close_icon_color = if requires_termination_confirmation {
            Color::Error
        } else {
            Color::Muted
        };
        let show_detection_confidence = agent_ui_settings.show_detection_confidence;
        let needs_attention = terminal.needs_attention;
        let has_notification = terminal.has_notification;
        let can_copy_codex_hook = terminal_agent_kind == Some(TerminalAgentKind::Codex)
            && terminal.agent.is_none()
            && terminal.metadata.session_ref.is_some_and(|session_ref| {
                TerminalHostConnection::try_global(cx)
                    .is_some_and(|connection| connection.host_id() == session_ref.host_id)
            });
        let context_review_workspace = review_workspace.clone();
        let context_review_brief = review_brief.clone();
        let context_working_directory = terminal.metadata.working_directory.clone();
        let context_session_ref = terminal.metadata.session_ref;
        let context_terminal_id = terminal.metadata.terminal_id;
        let context_attention_metadata = terminal.metadata.clone();
        let attention_is_active =
            terminal.metadata.attention.condition == TerminalAttentionCondition::Active;
        let attention_is_unread =
            terminal.metadata.attention.presentation == TerminalAttentionPresentation::Unread;
        let attention_is_muted = terminal.metadata.attention.is_muted_at(Utc::now());
        let context_agent_panel = match (&terminal.workspace, &terminal.source) {
            (ThreadEntryWorkspace::Open(workspace), TerminalEntrySource::AgentPanel) => {
                workspace.read(cx).panel::<AgentPanel>(cx)
            }
            _ => None,
        };
        let context_terminal_view = match &terminal.source {
            TerminalEntrySource::WorkspaceItem(terminal_view) => Some(terminal_view.clone()),
            TerminalEntrySource::AgentPanel | TerminalEntrySource::HostSession(_) => None,
        };
        let host_label =
            terminal_row_owner_label(terminal.metadata.session_ref.is_some(), is_remote);
        let (icon_char, title, highlight_positions) =
            match split_leading_icon_char(&display_title, &terminal.highlight_positions) {
                Some((icon_char, title, positions)) => (Some(icon_char), title, positions),
                None => (None, display_title, terminal.highlight_positions.clone()),
            };

        let terminal_item = canvas_thread_item_style(ThreadItem::new(id, title), &design_system)
            .base_bg(sidebar_bg)
            .icon(
                terminal_agent_kind
                    .map(terminal_agent_icon)
                    .unwrap_or(IconName::Terminal),
            )
            .when_some(icon_char, |this, icon_char| this.icon_char(icon_char))
            .is_remote(is_remote)
            .when(session_rail_settings.show_agent_state_metadata, |this| {
                this.when_some(terminal_agent_kind, |this, agent_kind| {
                    this.actor_label(agent_kind.display_name()).state_label(
                        terminal_agent_state_label(
                            terminal.agent.as_ref(),
                            terminal.runtime.as_ref(),
                            needs_attention,
                            attention_is_muted,
                            can_copy_codex_hook,
                            show_detection_confidence,
                        ),
                    )
                })
                .when(terminal_agent_kind.is_none(), |this| {
                    this.state_label(terminal_agent_state_label(
                        terminal.agent.as_ref(),
                        terminal.runtime.as_ref(),
                        needs_attention,
                        attention_is_muted,
                        false,
                        false,
                    ))
                })
                .host_label(host_label)
                .host_label_visible(supplemental_metadata_visible)
            })
            .actor_label_visible(supplemental_metadata_visible)
            .worktrees(worktrees)
            .when_some(evidence_label, |this, (label, status)| {
                this.evidence(label, status)
            })
            .timestamp(
                (session_rail_settings.show_latest_attention_metadata
                    && session_rail_recency_visible(rail_width, has_evidence))
                .then_some(timestamp)
                .unwrap_or_default(),
            )
            .labels_visible(labels_visible)
            .notified(
                show_agent_attention
                    && session_rail_settings.show_latest_attention_metadata
                    && has_notification,
            )
            .highlight_positions(highlight_positions)
            .selected(is_active)
            .focused(is_focused)
            .hovered(is_hovered)
            .on_hover(cx.listener(move |this, is_hovered: &bool, _window, cx| {
                if *is_hovered {
                    this.hovered_thread_index = Some(ix);
                } else if this.hovered_thread_index == Some(ix) {
                    this.hovered_thread_index = None;
                }
                cx.notify();
            }))
            .when(is_hovered, |this| {
                this.action_slot(
                    h_flex()
                        .gap_0p5()
                        .when(can_copy_codex_hook, |this| {
                            this.child(
                                IconButton::new("copy-codex-hook-setup", IconName::Copy)
                                    .size(ButtonSize::Medium)
                                    .icon_size(IconSize::Small)
                                    .icon_color(Color::Muted)
                                    .aria_label("Copy Codex Hook Setup")
                                    .tooltip(Tooltip::text("Copy Codex Hook Setup"))
                                    .on_click(|_, _window, cx| {
                                        cx.write_to_clipboard(ClipboardItem::new_string(
                                            CODEX_HOOK_SETUP.to_owned(),
                                        ));
                                    }),
                            )
                        })
                        .when_some(review_workspace, |this, review_workspace| {
                            this.child(
                                IconButton::new("review-terminal-run", IconName::ListTodo)
                                    .size(ButtonSize::Medium)
                                    .icon_size(IconSize::Small)
                                    .icon_color(Color::Muted)
                                    .aria_label("Open Review Brief")
                                    .aria_keyshortcuts("Shift+V")
                                    .tooltip({
                                        let focus_handle = focus_handle.clone();
                                        move |_window, cx| {
                                            Tooltip::for_action_in(
                                                "Open Review Brief",
                                                &OpenSelectedReviewBrief,
                                                &focus_handle,
                                                cx,
                                            )
                                        }
                                    })
                                    .on_click({
                                        let review_brief = review_brief.clone();
                                        let sidebar = sidebar.clone();
                                        let metadata = review_action_metadata.clone();
                                        let workspace = review_action_workspace.clone();
                                        let source = review_action_source.clone();
                                        move |_, window, cx| {
                                            Self::open_terminal_run_review(
                                                sidebar.clone(),
                                                metadata.clone(),
                                                workspace.clone(),
                                                source.clone(),
                                                review_brief.clone(),
                                                review_workspace.clone(),
                                                window,
                                                cx,
                                            );
                                        }
                                    }),
                            )
                        })
                        .child(
                            IconButton::new("close-terminal", close_icon)
                                .size(ButtonSize::Medium)
                                .icon_size(IconSize::Small)
                                .icon_color(close_icon_color)
                                .aria_label(close_label)
                                .tooltip({
                                    let focus_handle = focus_handle.clone();
                                    move |_window, cx| {
                                        Tooltip::for_action_in(
                                            close_label,
                                            &ArchiveSelectedThread,
                                            &focus_handle,
                                            cx,
                                        )
                                    }
                                })
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.close_terminal_with_confirmation(
                                        close_action_metadata.clone(),
                                        close_action_workspace.clone(),
                                        close_action_source.clone(),
                                        requires_termination_confirmation,
                                        window,
                                        cx,
                                    );
                                })),
                        ),
                )
            })
            .on_click(cx.listener({
                let metadata = terminal.metadata.clone();
                let workspace = terminal.workspace.clone();
                let source = terminal.source.clone();
                move |this, _, window, cx| {
                    this.activate_terminal_entry(
                        metadata.clone(),
                        workspace.clone(),
                        source.clone(),
                        false,
                        window,
                        cx,
                    );
                }
            }));

        let context_menu_id = SharedString::from(format!("terminal-context-menu-{ix}"));
        right_click_menu(context_menu_id)
            .trigger(move |_, _, _| terminal_item)
            .menu(move |window, cx| {
                let review_workspace = context_review_workspace.clone();
                let review_brief = context_review_brief.clone();
                let working_directory = context_working_directory.clone();
                let agent_panel = context_agent_panel.clone();
                let terminal_view = context_terminal_view.clone();
                let attention_metadata = context_attention_metadata.clone();
                let sidebar = sidebar.clone();
                let close_metadata = metadata.clone();
                let close_workspace = workspace.clone();
                let close_source = source.clone();
                ContextMenu::build(window, cx, move |mut menu, _window, _cx| {
                    if let Some(review_workspace) = review_workspace.clone() {
                        menu = menu.entry("Open Review Brief", None, {
                            let review_brief = review_brief.clone();
                            let sidebar = sidebar.clone();
                            let metadata = close_metadata.clone();
                            let workspace = close_workspace.clone();
                            let source = close_source.clone();
                            move |window, cx| {
                                Self::open_terminal_run_review(
                                    sidebar.clone(),
                                    metadata.clone(),
                                    workspace.clone(),
                                    source.clone(),
                                    review_brief.clone(),
                                    review_workspace.clone(),
                                    window,
                                    cx,
                                );
                            }
                        });
                    }
                    if let Some(working_directory) = working_directory.clone() {
                        menu = menu.entry("Copy Working Directory", None, move |_window, cx| {
                            cx.write_to_clipboard(ClipboardItem::new_string(
                                working_directory.to_string_lossy().into_owned(),
                            ));
                        });
                    }
                    if let Some(session_ref) = context_session_ref {
                        menu = menu.entry("Copy Session Reference", None, move |_window, cx| {
                            cx.write_to_clipboard(ClipboardItem::new_string(format!(
                                "{}:{}",
                                session_ref.host_id, session_ref.session_id
                            )));
                        });
                    }
                    if can_copy_codex_hook {
                        menu = menu.entry("Copy Codex Hook Setup", None, |_window, cx| {
                            cx.write_to_clipboard(ClipboardItem::new_string(
                                CODEX_HOOK_SETUP.to_owned(),
                            ));
                        });
                    }
                    if attention_is_active {
                        menu = menu.separator();
                        if attention_is_unread {
                            menu = menu.entry("Acknowledge Attention", None, {
                                let agent_panel = agent_panel.clone();
                                let terminal_view = terminal_view.clone();
                                let attention_metadata = attention_metadata.clone();
                                move |_window, cx| {
                                    Self::apply_terminal_attention_action(
                                        context_terminal_id,
                                        attention_metadata.clone(),
                                        TerminalAttentionAction::Acknowledge,
                                        agent_panel.clone(),
                                        terminal_view.clone(),
                                        context_session_ref,
                                        cx,
                                    );
                                }
                            });
                        }
                        if attention_is_muted {
                            menu = menu.entry("Resume Attention", None, {
                                let attention_metadata = attention_metadata.clone();
                                move |_window, cx| {
                                    Self::apply_terminal_attention_action(
                                        context_terminal_id,
                                        attention_metadata.clone(),
                                        TerminalAttentionAction::Resume,
                                        None,
                                        None,
                                        context_session_ref,
                                        cx,
                                    );
                                }
                            });
                        } else {
                            menu = menu.entry("Snooze Attention for 1 Hour", None, {
                                let agent_panel = agent_panel.clone();
                                let terminal_view = terminal_view.clone();
                                let attention_metadata = attention_metadata.clone();
                                move |_window, cx| {
                                    Self::apply_terminal_attention_action(
                                        context_terminal_id,
                                        attention_metadata.clone(),
                                        TerminalAttentionAction::SnoozeOneHour,
                                        agent_panel.clone(),
                                        terminal_view.clone(),
                                        context_session_ref,
                                        cx,
                                    );
                                }
                            });
                        }
                        menu = menu.entry("Mark Attention Resolved", None, {
                            let agent_panel = agent_panel.clone();
                            let terminal_view = terminal_view.clone();
                            let attention_metadata = attention_metadata.clone();
                            move |_window, cx| {
                                Self::apply_terminal_attention_action(
                                    context_terminal_id,
                                    attention_metadata.clone(),
                                    TerminalAttentionAction::Resolve,
                                    agent_panel.clone(),
                                    terminal_view.clone(),
                                    context_session_ref,
                                    cx,
                                );
                            }
                        });
                    }
                    menu = menu.separator().entry(close_label, None, {
                        move |window, cx| {
                            sidebar
                                .update(cx, |sidebar, cx| {
                                    sidebar.close_terminal_with_confirmation(
                                        close_metadata.clone(),
                                        close_workspace.clone(),
                                        close_source.clone(),
                                        requires_termination_confirmation,
                                        window,
                                        cx,
                                    );
                                })
                                .ok();
                        }
                    });
                    menu
                })
            })
            .into_any_element()
    }

    fn render_recent_projects_button(
        &self,
        labels_visible: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let multi_workspace = self.multi_workspace.upgrade();

        let workspace = multi_workspace
            .as_ref()
            .map(|mw| mw.read(cx).workspace().downgrade());

        let focus_handle = workspace
            .as_ref()
            .and_then(|ws| ws.upgrade())
            .map(|w| w.read(cx).focus_handle(cx))
            .unwrap_or_else(|| cx.focus_handle());

        let window_project_groups: Vec<ProjectGroupKey> = multi_workspace
            .as_ref()
            .map(|mw| mw.read(cx).project_group_keys())
            .unwrap_or_default();

        let popover_handle = self.recent_projects_popover_handle.clone();

        PopoverMenu::new("sidebar-recent-projects-menu")
            .with_handle(popover_handle)
            .menu(move |window, cx| {
                workspace.as_ref().map(|ws| {
                    SidebarRecentProjects::popover(
                        ws.clone(),
                        window_project_groups.clone(),
                        focus_handle.clone(),
                        window,
                        cx,
                    )
                })
            })
            .trigger_with_tooltip(
                Button::new(
                    "open-recent-workspaces",
                    if labels_visible { "Workspaces" } else { "" },
                )
                .size(ButtonSize::Medium)
                .label_size(LabelSize::Small)
                .start_icon(Icon::new(IconName::FolderOpen).size(IconSize::Small))
                .aria_label("Open Recent Workspaces")
                .selected_style(ButtonStyle::Tinted(TintColor::Accent)),
                |_window, cx| {
                    Tooltip::for_action("Open Recent Workspaces", &OpenRecent::default(), cx)
                },
            )
            .offset(gpui::Point {
                x: px(-2.0),
                y: px(-2.0),
            })
            .anchor(gpui::Anchor::BottomRight)
    }

    fn new_thread_in_group(
        &mut self,
        _: &NewThreadInGroup,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(key) = self.selected_group_key() {
            self.set_group_expanded(&key, true, cx);
            self.selection = None;
            if let Some(workspace) = self.workspace_for_group(&key, cx) {
                self.create_new_thread(&workspace, window, cx);
            } else {
                self.open_workspace_and_create_entry(&key, NewEntryTarget::AgentThread, window, cx);
            }
        } else if let Some(workspace) = self.active_workspace(cx) {
            self.create_new_thread(&workspace, window, cx);
        }
    }

    fn new_session_in_group(
        &mut self,
        _: &NewSessionInGroup,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.stop_propagation();

        if let Some(key) = self.selected_group_key() {
            self.set_group_expanded(&key, true, cx);
            self.selection = None;
            if let Some(workspace) = self.workspace_for_group(&key, cx) {
                self.create_new_terminal(&workspace, window, cx);
            } else {
                self.open_workspace_and_create_entry(
                    &key,
                    default_new_session_target(),
                    window,
                    cx,
                );
            }
        } else if let Some(workspace) = self.active_workspace(cx) {
            self.create_new_terminal(&workspace, window, cx);
        }
    }

    fn new_terminal_thread(
        &mut self,
        _: &NewTerminalThread,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.stop_propagation();

        if let Some(key) = self.selected_group_key() {
            self.set_group_expanded(&key, true, cx);
            self.selection = None;
            if let Some(workspace) = self.workspace_for_group(&key, cx) {
                self.create_new_terminal(&workspace, window, cx);
            } else {
                self.open_workspace_and_create_entry(&key, NewEntryTarget::Terminal, window, cx);
            }
        } else if let Some(workspace) = self.active_workspace(cx) {
            self.create_new_terminal(&workspace, window, cx);
        }
    }

    /// Closed linked-worktree drafts need an open workspace so archive root
    /// planning can inspect repositories before deleting the worktree.
    fn open_workspace_and_remove_draft(
        &mut self,
        draft_id: ThreadId,
        folder_paths: PathList,
        project_group_key: ProjectGroupKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((open_task, modal_workspace)) =
            self.open_workspace_for_archive(folder_paths, project_group_key, window, cx)
        else {
            return;
        };

        cx.spawn_in(window, async move |this, cx| {
            let result = open_task.await;
            remote_connection::dismiss_connection_modal(&modal_workspace, cx);
            let workspace = result?;
            Self::wait_for_archive_workspace_metadata(&workspace, cx).await;

            this.update_in(cx, |this, window, cx| {
                let workspace = ThreadEntryWorkspace::Open(workspace);
                this.remove_draft(draft_id, &workspace, window, cx);
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn remove_draft(
        &mut self,
        draft_id: ThreadId,
        workspace: &ThreadEntryWorkspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let metadata = ThreadMetadataStore::global(cx)
            .read(cx)
            .entry(draft_id)
            .cloned();

        if let ThreadEntryWorkspace::Closed {
            folder_paths,
            project_group_key,
        } = workspace
            && self.should_load_closed_workspace_for_archive(
                folder_paths,
                project_group_key,
                metadata
                    .as_ref()
                    .and_then(|metadata| metadata.remote_connection.as_ref()),
                Some(draft_id),
                None,
                cx,
            )
        {
            self.open_workspace_and_remove_draft(
                draft_id,
                folder_paths.clone(),
                project_group_key.clone(),
                window,
                cx,
            );
            return;
        }

        let draft_folder_paths = metadata
            .as_ref()
            .map(|metadata| metadata.folder_paths().clone())
            .or_else(|| match workspace {
                ThreadEntryWorkspace::Open(workspace) => {
                    Some(PathList::new(&workspace.read(cx).root_paths(cx)))
                }
                ThreadEntryWorkspace::Closed { folder_paths, .. } => Some(folder_paths.clone()),
            });
        let draft_remote_connection = metadata
            .as_ref()
            .and_then(|metadata| metadata.remote_connection.clone());
        let roots_to_archive = metadata
            .as_ref()
            .map(|metadata| {
                self.roots_to_archive_for_paths(
                    metadata.folder_paths(),
                    metadata.remote_connection.as_ref(),
                    Some(draft_id),
                    None,
                    cx,
                )
            })
            .unwrap_or_default();

        let was_active = self
            .active_entry
            .as_ref()
            .is_some_and(|entry| entry.is_active_thread(&draft_id));
        let neighbor = self
            .contents
            .entries
            .iter()
            .position(|entry| {
                matches!(
                    entry,
                    ListEntry::Thread(thread) if thread.metadata.thread_id == draft_id
                )
            })
            .and_then(|position| self.neighboring_activatable_entry(position));

        let workspace_to_remove = draft_folder_paths.as_ref().and_then(|folder_paths| {
            self.linked_worktree_workspace_to_remove(
                folder_paths,
                draft_remote_connection.as_ref(),
                Some(draft_id),
                None,
                &roots_to_archive,
                cx,
            )
        });
        let mut workspaces_to_remove: Vec<Entity<Workspace>> =
            workspace_to_remove.into_iter().collect();
        let close_item_tasks = self.close_items_for_archived_worktrees(
            &roots_to_archive,
            &mut workspaces_to_remove,
            window,
            cx,
        );

        if !workspaces_to_remove.is_empty() {
            let Some(multi_workspace) = self.multi_workspace.upgrade() else {
                return;
            };
            let draft_workspace_removed = matches!(
                workspace,
                ThreadEntryWorkspace::Open(workspace) if workspaces_to_remove.contains(workspace)
            );
            let (fallback_paths, project_group_key) = neighbor
                .as_ref()
                .map(|neighbor| neighbor.project_location(cx))
                .unwrap_or_else(|| {
                    workspaces_to_remove
                        .first()
                        .map(|workspace| {
                            let key = workspace.read(cx).project_group_key(cx);
                            (key.path_list().clone(), key)
                        })
                        .unwrap_or_default()
                });

            let excluded = workspaces_to_remove.clone();
            let remove_task = multi_workspace.update(cx, |multi_workspace, cx| {
                multi_workspace.remove(
                    workspaces_to_remove,
                    move |this, window, cx| {
                        let active_workspace = this.workspace().clone();
                        this.find_or_create_workspace(
                            fallback_paths,
                            project_group_key.host(),
                            Some(project_group_key),
                            |options, window, cx| {
                                connect_remote(active_workspace, options, window, cx)
                            },
                            &excluded,
                            None,
                            OpenMode::Activate,
                            window,
                            cx,
                        )
                    },
                    window,
                    cx,
                )
            });

            let workspace = workspace.clone();
            cx.spawn_in(window, async move |this, cx| {
                if !remove_task.await? {
                    return anyhow::Ok(());
                }

                for task in close_item_tasks {
                    let result: anyhow::Result<()> = task.await;
                    result.log_err();
                }

                this.update_in(cx, |this, window, cx| {
                    if draft_workspace_removed {
                        if let Some(draft_folder_paths) = draft_folder_paths.as_ref() {
                            this.delete_empty_drafts_for_archive_paths(
                                draft_folder_paths,
                                draft_remote_connection.as_ref(),
                                cx,
                            );
                        }
                    }
                    this.remove_draft_entry(
                        draft_id,
                        &workspace,
                        was_active,
                        neighbor.as_ref(),
                        !draft_workspace_removed,
                        roots_to_archive,
                        window,
                        cx,
                    );
                })?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        } else if !close_item_tasks.is_empty() {
            let workspace = workspace.clone();
            cx.spawn_in(window, async move |this, cx| {
                for task in close_item_tasks {
                    let result: anyhow::Result<()> = task.await;
                    result.log_err();
                }

                this.update_in(cx, |this, window, cx| {
                    this.remove_draft_entry(
                        draft_id,
                        &workspace,
                        was_active,
                        neighbor.as_ref(),
                        true,
                        roots_to_archive,
                        window,
                        cx,
                    );
                })?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        } else {
            self.remove_draft_entry(
                draft_id,
                workspace,
                was_active,
                neighbor.as_ref(),
                true,
                roots_to_archive,
                window,
                cx,
            );
        }
    }

    fn remove_draft_with_confirmation(
        &mut self,
        draft_id: ThreadId,
        workspace: ThreadEntryWorkspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !draft_discard_requires_confirmation(APP_NAME) {
            self.remove_draft(draft_id, &workspace, window, cx);
            return;
        }

        let title = ThreadMetadataStore::global(cx)
            .read(cx)
            .entry(draft_id)
            .map(ThreadMetadata::display_title)
            .unwrap_or_else(|| "Untitled Agent Session".into());
        let message = format!("Discard draft Agent Session “{title}”?");
        let confirmation = window.prompt(
            PromptLevel::Warning,
            &message,
            Some("Any unsent prompt text in this draft will be permanently removed."),
            &["Discard Draft", "Cancel"],
            cx,
        );

        cx.spawn_in(window, async move |this, cx| {
            if !matches!(confirmation.await, Ok(0)) {
                return anyhow::Ok(());
            }
            this.update_in(cx, |this, window, cx| {
                this.remove_draft(draft_id, &workspace, window, cx);
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn remove_draft_entry(
        &mut self,
        draft_id: ThreadId,
        workspace: &ThreadEntryWorkspace,
        was_active: bool,
        neighbor: Option<&ActivatableEntry>,
        activate_panel_draft: bool,
        roots_to_archive: Vec<thread_worktree_archive::RootPlan>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Fallback to a neighbor thread when the discarded
        // draft was the active entry.
        let activate_panel_draft = activate_panel_draft && !(was_active && neighbor.is_some());

        let removed_from_panel = if let ThreadEntryWorkspace::Open(workspace) = workspace {
            workspace.update(cx, |workspace, cx| {
                if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                    panel.update(cx, |panel, cx| {
                        if activate_panel_draft {
                            panel.remove_thread(draft_id, window, cx);
                        } else {
                            panel.remove_thread_without_activating_draft(draft_id, window, cx);
                        }
                    });
                    true
                } else {
                    false
                }
            })
        } else {
            false
        };

        if !removed_from_panel {
            ThreadMetadataStore::global(cx).update(cx, |store, cx| {
                store.delete(draft_id, cx);
            });
        }

        self.start_detached_archive_worktree_task(roots_to_archive, cx);

        if was_active {
            self.active_entry = None;
            if !activate_panel_draft {
                if neighbor
                    .as_ref()
                    .is_some_and(|neighbor| self.activate_entry(neighbor, window, cx))
                {
                    return;
                }
                self.sync_active_entry_from_active_workspace(cx);
            }
        }

        self.update_entries(cx);
    }

    fn create_new_thread(
        &mut self,
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if workspace_path_list(workspace, cx).paths().is_empty() {
            return;
        }

        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.activate(workspace.clone(), None, window, cx);
        });

        let draft = create_agent_thread_in_workspace(workspace, true, window, cx);

        if let Some(draft) = draft {
            let draft_id = draft.read(cx).thread_id(cx);
            self.active_entry = Some(ActiveEntry::Thread {
                thread_id: draft_id,
                session_id: None,
                workspace: workspace.clone(),
            });
        }
    }

    fn create_new_terminal(
        &mut self,
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if workspace_path_list(workspace, cx).paths().is_empty() {
            return;
        }

        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.activate(workspace.clone(), None, window, cx);
        });

        let workspace_focus = workspace.read(cx).focus_handle(cx);
        workspace_focus.dispatch_action(&NewCenterTerminal::default(), window, cx);
    }

    fn selected_group_key(&self) -> Option<ProjectGroupKey> {
        let ix = self.selection?;
        match self.contents.entries.get(ix) {
            Some(ListEntry::ProjectHeader { key, .. }) => Some(key.clone()),
            Some(ListEntry::Thread(_) | ListEntry::Terminal(_)) => {
                (0..ix)
                    .rev()
                    .find_map(|i| match self.contents.entries.get(i) {
                        Some(ListEntry::ProjectHeader { key, .. }) => Some(key.clone()),
                        _ => None,
                    })
            }
            _ => None,
        }
    }

    fn workspace_for_group(&self, key: &ProjectGroupKey, cx: &App) -> Option<Entity<Workspace>> {
        let mw = self.multi_workspace.upgrade()?;
        let mw = mw.read(cx);
        let active = mw.workspace().clone();
        let active_key = active.read(cx).project_group_key(cx);
        if active_key == *key {
            Some(active)
        } else {
            mw.workspace_for_paths(key.path_list(), key.host().as_ref(), cx)
        }
    }

    pub(crate) fn activate_or_open_workspace_for_group(
        &mut self,
        key: &ProjectGroupKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let workspace = self
            .multi_workspace
            .upgrade()
            .and_then(|mw| mw.read(cx).last_active_workspace_for_group(key, cx))
            .or_else(|| self.workspace_for_group(key, cx));
        if let Some(workspace) = workspace {
            if self.is_active_workspace(&workspace, cx) {
                return;
            }
            self.activate_workspace(&workspace, window, cx);
        } else {
            self.open_workspace_for_group(key, window, cx);
        }
        self.selection = None;
        self.active_entry = None;
    }

    fn active_project_group_key(&self, cx: &App) -> Option<ProjectGroupKey> {
        let multi_workspace = self.multi_workspace.upgrade()?;
        let multi_workspace = multi_workspace.read(cx);
        Some(multi_workspace.project_group_key_for_workspace(multi_workspace.workspace(), cx))
    }

    fn active_project_header_position(&self, cx: &App) -> Option<usize> {
        let active_key = self.active_project_group_key(cx)?;
        self.contents
            .project_header_indices
            .iter()
            .position(|&entry_ix| {
                matches!(
                    &self.contents.entries[entry_ix],
                    ListEntry::ProjectHeader { key, .. } if *key == active_key
                )
            })
    }

    fn cycle_project_impl(&mut self, forward: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        let header_count = self.contents.project_header_indices.len();
        if header_count == 0 {
            return;
        }

        let current_pos = self.active_project_header_position(cx);

        let next_pos = match current_pos {
            Some(pos) => {
                if forward {
                    (pos + 1) % header_count
                } else {
                    (pos + header_count - 1) % header_count
                }
            }
            None => 0,
        };

        let header_entry_ix = self.contents.project_header_indices[next_pos];
        let Some(ListEntry::ProjectHeader { key, .. }) = self.contents.entries.get(header_entry_ix)
        else {
            return;
        };
        let key = key.clone();

        // Uncollapse the target group so that threads become visible.
        self.set_group_expanded(&key, true, cx);

        if let Some(workspace) = self.multi_workspace.upgrade().and_then(|mw| {
            mw.read(cx)
                .workspace_for_paths(key.path_list(), key.host().as_ref(), cx)
        }) {
            multi_workspace.update(cx, |multi_workspace, cx| {
                multi_workspace.activate(workspace, None, window, cx);
                multi_workspace.retain_active_workspace(cx);
            });
        } else {
            self.open_workspace_for_group(&key, window, cx);
        }
    }

    fn on_next_project(&mut self, _: &NextProject, window: &mut Window, cx: &mut Context<Self>) {
        self.cycle_project_impl(true, window, cx);
    }

    fn on_previous_project(
        &mut self,
        _: &PreviousProject,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.cycle_project_impl(false, window, cx);
    }

    fn cycle_thread_impl(&mut self, forward: bool, window: &mut Window, cx: &mut Context<Self>) {
        let thread_indices: Vec<usize> = self
            .contents
            .entries
            .iter()
            .enumerate()
            .filter_map(|(ix, entry)| match entry {
                ListEntry::Thread(_) | ListEntry::Terminal(_) => Some(ix),
                _ => None,
            })
            .collect();

        if thread_indices.is_empty() {
            return;
        }

        let current_thread_pos = self.active_entry.as_ref().and_then(|active| {
            thread_indices
                .iter()
                .position(|&ix| active.matches_entry(&self.contents.entries[ix]))
        });

        let next_pos = match current_thread_pos {
            Some(pos) => {
                let count = thread_indices.len();
                if forward {
                    (pos + 1) % count
                } else {
                    (pos + count - 1) % count
                }
            }
            None => 0,
        };

        let entry_ix = thread_indices[next_pos];
        match &self.contents.entries[entry_ix] {
            ListEntry::Thread(thread) => {
                let metadata = thread.metadata.clone();
                match &thread.workspace {
                    ThreadEntryWorkspace::Open(workspace) => {
                        let workspace = workspace.clone();
                        self.activate_thread(metadata, &workspace, true, window, cx);
                    }
                    ThreadEntryWorkspace::Closed {
                        folder_paths,
                        project_group_key,
                    } => {
                        let folder_paths = folder_paths.clone();
                        let project_group_key = project_group_key.clone();
                        self.open_workspace_and_activate_thread(
                            metadata,
                            folder_paths,
                            &project_group_key,
                            window,
                            cx,
                        );
                    }
                }
            }
            ListEntry::Terminal(terminal) => {
                let metadata = terminal.metadata.clone();
                let workspace = terminal.workspace.clone();
                let source = terminal.source.clone();
                self.activate_terminal_entry(metadata, workspace, source, true, window, cx);
            }
            ListEntry::ProjectHeader { .. } => {}
        }
    }

    fn on_next_thread(&mut self, _: &NextThread, window: &mut Window, cx: &mut Context<Self>) {
        self.cycle_thread_impl(true, window, cx);
    }

    fn on_previous_thread(
        &mut self,
        _: &PreviousThread,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.cycle_thread_impl(false, window, cx);
    }

    fn render_no_results(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let has_query = self.has_filter_query(cx);
        let (icon, title, description) = if has_query {
            (
                IconName::ListX,
                "No matching sessions",
                "Try another term or clear the search to return to your current work.",
            )
        } else {
            (
                IconName::Terminal,
                "No sessions yet",
                "Start a terminal in this workspace. It will appear here with its real state.",
            )
        };

        v_flex()
            .id("sidebar-no-results")
            .role(gpui::Role::Status)
            .aria_label(format!("{title}. {description}"))
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .px_3()
            .py_6()
            .child(
                v_flex()
                    .w_full()
                    .gap_3()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                div()
                                    .flex_none()
                                    .size_8()
                                    .rounded_md()
                                    .border_1()
                                    .border_color(cx.theme().colors().border_variant)
                                    .bg(cx.theme().colors().panel_background)
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        Icon::new(icon).size(IconSize::Small).color(Color::Muted),
                                    ),
                            )
                            .child(Label::new(title).size(LabelSize::Small)),
                    )
                    .child(
                        Label::new(description)
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    )
                    .when(has_query, |this| {
                        this.child(
                            Button::new("no-results-clear-search", "Clear Search")
                                .full_width()
                                .style(ButtonStyle::OutlinedCustom(cx.theme().colors().border))
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.reset_filter_editor_text(window, cx);
                                    this.update_entries(cx);
                                    this.select_first_entry();
                                })),
                        )
                    })
                    .when(!has_query, |this| {
                        this.child(
                            Button::new("no-results-new-terminal", "New Terminal")
                                .full_width()
                                .style(ButtonStyle::Filled)
                                .label_size(LabelSize::Small)
                                .start_icon(Icon::new(IconName::Terminal).size(IconSize::XSmall))
                                .tooltip(|_, cx| {
                                    Tooltip::for_action(
                                        "New Terminal",
                                        &NewCenterTerminal::default(),
                                        cx,
                                    )
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(
                                        NewCenterTerminal::default().boxed_clone(),
                                        cx,
                                    );
                                }),
                        )
                    }),
            )
    }

    fn render_attention_empty_state(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("sidebar-attention-empty")
            .role(gpui::Role::Status)
            .aria_label("You're caught up. No sessions need your attention.")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .px_3()
            .py_6()
            .gap_3()
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        div()
                            .flex_none()
                            .size_8()
                            .rounded_md()
                            .border_1()
                            .border_color(cx.theme().colors().border_variant)
                            .bg(cx.theme().colors().panel_background)
                            .items_center()
                            .justify_center()
                            .child(
                                Icon::new(IconName::Check)
                                    .size(IconSize::Small)
                                    .color(Color::Success),
                            ),
                    )
                    .child(Label::new("You're caught up").size(LabelSize::Small)),
            )
            .child(
                Label::new("No sessions need your attention. Ongoing work remains under All.")
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
            )
            .child(
                Button::new("show-all-sessions", "Show All Sessions")
                    .full_width()
                    .style(ButtonStyle::OutlinedCustom(cx.theme().colors().border))
                    .label_size(LabelSize::Small)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.set_attention_filter(false, window, cx);
                    })),
            )
    }

    fn render_session_overview(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let has_attention = self.contents.has_attention;
        let is_searching = self.has_filter_query(cx);
        let status_label = session_overview_status_label(
            self.contents.session_count,
            self.contents.attention_count,
            self.contents.project_header_indices.len(),
            is_searching,
        );
        let all_scope_label = format!("All {}", self.contents.session_count);
        let attention_scope_label = format!("Attention {}", self.contents.attention_count);
        let all_scope_aria_label = all_sessions_accessibility_label(self.contents.session_count);
        let attention_scope_aria_label =
            attention_sessions_accessibility_label(self.contents.attention_count);
        let all_scope_focus = self.focus_handle.clone();
        let attention_scope_focus = self.focus_handle.clone();
        let status_color = if has_attention && !is_searching {
            Color::Warning
        } else {
            Color::Muted
        };
        let status_icon = if is_searching {
            IconName::MagnifyingGlass
        } else if has_attention {
            IconName::Warning
        } else if self.contents.session_count == 0 {
            IconName::Terminal
        } else {
            IconName::Check
        };

        v_flex()
            .flex_none()
            .px_2()
            .py_1p5()
            .gap_1p5()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .gap_2()
                    .child(
                        v_flex()
                            .min_w_0()
                            .gap_0p5()
                            .child(Label::new("Session Rail").size(LabelSize::Small))
                            .child(
                                h_flex()
                                    .id("session-rail-status")
                                    .min_w_0()
                                    .gap_1()
                                    .role(gpui::Role::Status)
                                    .aria_label(status_label.clone())
                                    .child(
                                        Icon::new(status_icon)
                                            .size(IconSize::XSmall)
                                            .color(status_color),
                                    )
                                    .child(
                                        Label::new(status_label)
                                            .size(LabelSize::XSmall)
                                            .color(status_color)
                                            .truncate(),
                                    ),
                            ),
                    )
                    .when(
                        session_overview_create_action_visible(self.contents.session_count),
                        |this| {
                            this.child(
                                Button::new("new-session", "New Terminal")
                                    .size(ButtonSize::Medium)
                                    .style(ButtonStyle::Filled)
                                    .start_icon(Icon::new(IconName::Plus).size(IconSize::XSmall))
                                    .aria_label("New Terminal")
                                    .tooltip(|_, cx| {
                                        Tooltip::for_action(
                                            "New Terminal",
                                            &NewCenterTerminal::default(),
                                            cx,
                                        )
                                    })
                                    .on_click(|_, window, cx| {
                                        window.dispatch_action(
                                            NewCenterTerminal::default().boxed_clone(),
                                            cx,
                                        );
                                    }),
                            )
                        },
                    ),
            )
            .when(
                session_scope_controls_visible(self.contents.session_count),
                |this| {
                    this.child(
                        h_flex()
                            .id("session-scope")
                            .w_full()
                            .gap_1()
                            .role(gpui::Role::Group)
                            .aria_label("Session scope")
                            .child(
                                div().min_w_0().flex_1().child(
                                    Button::new("all-session-scope", all_scope_label)
                                        .full_width()
                                        .size(ButtonSize::Medium)
                                        .style(ButtonStyle::Subtle)
                                        .toggle_state(!self.attention_only)
                                        .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                                        .aria_label(all_scope_aria_label)
                                        .aria_keyshortcuts("Shift+A")
                                        .aria_description("Show every session in the rail")
                                        .tooltip(move |_window, cx| {
                                            Tooltip::for_action_in(
                                                "Show All Sessions",
                                                &ToggleAttentionFilter,
                                                &all_scope_focus,
                                                cx,
                                            )
                                        })
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.set_attention_filter(false, window, cx);
                                        })),
                                ),
                            )
                            .child(
                                div().min_w_0().flex_1().child(
                                    Button::new("attention-session-scope", attention_scope_label)
                                        .full_width()
                                        .size(ButtonSize::Medium)
                                        .style(ButtonStyle::Subtle)
                                        .toggle_state(self.attention_only)
                                        .selected_style(ButtonStyle::Tinted(TintColor::Warning))
                                        .aria_label(attention_scope_aria_label)
                                        .aria_keyshortcuts("Shift+A")
                                        .aria_description("Show only sessions that need attention")
                                        .tooltip(move |_window, cx| {
                                            Tooltip::for_action_in(
                                                "Show Attention Sessions",
                                                &ToggleAttentionFilter,
                                                &attention_scope_focus,
                                                cx,
                                            )
                                        })
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.set_attention_filter(true, window, cx);
                                        })),
                                ),
                            ),
                    )
                },
            )
    }

    fn render_filter_input(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .min_w_0()
            .flex_1()
            .capture_action(
                cx.listener(|this, _: &editor::actions::Newline, window, cx| {
                    this.editor_confirm(window, cx);
                }),
            )
            .child(self.filter_editor.clone())
    }

    fn render_session_search(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let has_query = self.has_filter_query(cx);

        h_flex()
            .id("session-search")
            .role(gpui::Role::Search)
            .aria_label("Search sessions")
            .flex_none()
            .h(Tab::content_height(cx))
            .px_1p5()
            .gap_1()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                Icon::new(IconName::MagnifyingGlass)
                    .size(IconSize::Small)
                    .color(Color::Muted),
            )
            .child(self.render_filter_input(cx))
            .when(has_query, |this| {
                this.child(
                    IconButton::new("clear-session-search", IconName::Close)
                        .size(ButtonSize::Medium)
                        .icon_size(IconSize::Small)
                        .aria_label("Clear Session Search")
                        .tooltip(Tooltip::text("Clear Session Search"))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.reset_filter_editor_text(window, cx);
                            this.update_entries(cx);
                        })),
                )
            })
    }

    fn render_empty_state(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let (description, new_terminal_label, open_workspace_label) = session_start_state_copy();

        v_flex()
            .id("sidebar-start-state")
            .role(gpui::Role::Group)
            .aria_label("Start working in Dez")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .px_2()
            .py_2()
            .child(
                v_flex()
                    .w_full()
                    .gap_2()
                    .child(
                        Label::new(description)
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(
                        Button::new("start-terminal", new_terminal_label)
                            .full_width()
                            .style(ButtonStyle::Filled)
                            .start_icon(Icon::new(IconName::Terminal).size(IconSize::Small))
                            .aria_label("New Terminal in Main Work Area")
                            .tooltip(|_, cx| {
                                Tooltip::for_action(
                                    "New Terminal in Main Work Area",
                                    &NewCenterTerminal::default(),
                                    cx,
                                )
                            })
                            .on_click(|_, window, cx| {
                                window.dispatch_action(
                                    NewCenterTerminal::default().boxed_clone(),
                                    cx,
                                );
                            }),
                    )
                    .child(
                        Button::new("start-open", open_workspace_label)
                            .full_width()
                            .style(ButtonStyle::OutlinedCustom(cx.theme().colors().border))
                            .start_icon(Icon::new(IconName::FolderOpen).size(IconSize::XSmall))
                            .aria_label(open_workspace_label)
                            .tooltip(|_, cx| {
                                Tooltip::for_action(
                                    open_workspace_label,
                                    &Open {
                                        create_new_window: Some(false),
                                    },
                                    cx,
                                )
                            })
                            .on_click(|_, window, cx| {
                                window.dispatch_action(
                                    Open {
                                        create_new_window: Some(false),
                                    }
                                    .boxed_clone(),
                                    cx,
                                );
                            }),
                    ),
            )
    }

    fn render_terminal_host_status(&self, cx: &App) -> Option<AnyElement> {
        let callout = match TerminalHostStartupStatus::state(cx) {
            TerminalHostStartupState::Disabled | TerminalHostStartupState::Connected { .. } => {
                return None;
            }
            TerminalHostStartupState::Connecting => Callout::new()
                .severity(Severity::Info)
                .icon(IconName::Info)
                .title("Connecting durable terminals")
                .description(
                    "New local terminals will wait instead of starting a disposable fallback. No process has started yet.",
                ),
            TerminalHostStartupState::Reconnecting { message } => {
                let details = message.clone();
                Callout::new()
                    .severity(Severity::Warning)
                    .icon(IconName::Warning)
                    .title("Reconnecting durable terminals")
                    .description(format!(
                        "Existing processes are left untouched and no replacement work will start. Wait for reconnection; if it persists, restart Dez. {}",
                        util::truncate_and_trailoff(&message, 180)
                    ))
                    .actions_slot(
                        Button::new("copy-host-reconnect-details", "Copy Details")
                            .size(ButtonSize::Medium)
                            .style(ButtonStyle::Outlined)
                            .aria_label("Copy Durable Host Reconnection Details")
                            .on_click(move |_, _window, cx| {
                                cx.write_to_clipboard(ClipboardItem::new_string(details.clone()));
                            }),
                    )
            }
            TerminalHostStartupState::Failed { message } => {
                let details = message.clone();
                Callout::new()
                    .severity(Severity::Error)
                    .icon(IconName::Warning)
                    .title("Durable terminal host unavailable")
                    .description(format!(
                        "No fallback terminal was started, so no replacement computation exists. Check the helper error, then restart Dez; to return to ordinary GUI-owned shells on the next launch, omit DEZ_EXPERIMENTAL_TERMINAL_HOST=1. {}",
                        util::truncate_and_trailoff(&message, 180)
                    ))
                    .actions_slot(
                        Button::new("copy-host-failure-details", "Copy Details")
                            .size(ButtonSize::Medium)
                            .style(ButtonStyle::Outlined)
                            .aria_label("Copy Durable Host Failure Details")
                            .on_click(move |_, _window, cx| {
                                cx.write_to_clipboard(ClipboardItem::new_string(details.clone()));
                            }),
                    )
            }
        };
        Some(callout.into_any_element())
    }

    fn app_session(&self, cx: &App) -> Option<Entity<AppSession>> {
        let multi_workspace = self.multi_workspace.upgrade()?;
        let workspace = multi_workspace.read(cx).workspace().clone();
        let app_session = workspace.read(cx).app_state().session.clone();
        Some(app_session)
    }

    fn unresolved_workspace_ids(&self, cx: &App) -> Vec<i64> {
        self.app_session(cx)
            .map(|app_session| {
                app_session
                    .read(cx)
                    .durable_workspace_memberships()
                    .filter_map(|membership| {
                        (membership.resolution == DurableWorkspaceResolution::RestoreFailed)
                            .then_some(membership.workspace_id)
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn render_workspace_restore_status(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let unresolved_workspace_ids = self.unresolved_workspace_ids(cx);
        if unresolved_workspace_ids.is_empty() {
            return None;
        }
        let count = unresolved_workspace_ids.len();
        let title = if count == 1 {
            "1 workspace needs recovery".to_string()
        } else {
            format!("{count} workspaces need recovery")
        };
        let description = if count == 1 {
            "Dez kept the failed Session reference. Reopen it from Recent Workspaces, or dismiss only this unresolved reference."
        } else {
            "Dez kept the failed Session references. Reopen them from Recent Workspaces, or dismiss only these unresolved references."
        };

        Some(
            Callout::new()
                .severity(Severity::Warning)
                .icon(IconName::Warning)
                .title(title)
                .description(description)
                .actions_slot(
                    h_flex()
                        .gap_1()
                        .child(
                            Button::new("recover-unresolved-workspace", "Open Recent")
                                .size(ButtonSize::Medium)
                                .style(ButtonStyle::Filled)
                                .aria_label("Open Recent Workspaces for Recovery")
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.recent_projects_popover_handle.toggle(window, cx);
                                })),
                        )
                        .child(
                            Button::new("dismiss-unresolved-workspace", "Dismiss")
                                .size(ButtonSize::Medium)
                                .style(ButtonStyle::Outlined)
                                .aria_label("Dismiss Unresolved Workspace References")
                                .tooltip(Tooltip::text(
                                    "Remove only the unresolved Session references; recent workspace data remains available",
                                ))
                                .on_click(cx.listener(move |this, _, _window, cx| {
                                    if let Some(app_session) = this.app_session(cx) {
                                        app_session.update(cx, |app_session, cx| {
                                            for workspace_id in &unresolved_workspace_ids {
                                                app_session.remove_durable_workspace(
                                                    *workspace_id,
                                                    cx,
                                                );
                                            }
                                        });
                                    }
                                    cx.notify();
                                })),
                        ),
                )
                .into_any_element(),
        )
    }

    fn render_sidebar_header(&self, window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sidebar_side = self.side(cx);
        let sidebar_state = SidebarRenderState {
            open: true,
            side: sidebar_side,
        };
        let sidebar_on_left = sidebar_side == SidebarSide::Left;
        let sidebar_on_right = sidebar_side == SidebarSide::Right;
        let not_fullscreen = !window.is_fullscreen();
        let traffic_lights = cfg!(target_os = "macos") && not_fullscreen && sidebar_on_left;
        let left_window_controls = !cfg!(target_os = "macos") && not_fullscreen && sidebar_on_left;
        let right_window_controls =
            !cfg!(target_os = "macos") && not_fullscreen && sidebar_on_right;
        let traffic_light_buttons = if traffic_lights {
            self.multi_workspace.upgrade().and_then(|multi_workspace| {
                render_sidebar_header_controls_with_state(multi_workspace, sidebar_state, cx)
            })
        } else {
            None
        };
        let left_header_buttons = if !traffic_lights && !sidebar_on_right {
            self.multi_workspace.upgrade().and_then(|multi_workspace| {
                render_sidebar_header_controls_with_state(multi_workspace, sidebar_state, cx)
            })
        } else {
            None
        };
        let right_header_buttons = if !traffic_lights && sidebar_on_right {
            self.multi_workspace.upgrade().and_then(|multi_workspace| {
                render_sidebar_header_controls_with_state(multi_workspace, sidebar_state, cx)
            })
        } else {
            None
        };

        h_flex()
            .relative()
            .flex_none()
            .h(Tab::container_height(cx))
            .bg(cx.theme().colors().tab_bar_background)
            .when(left_window_controls, |this| {
                this.children(Self::render_left_window_controls(window, cx))
            })
            .when(traffic_lights, |this| {
                this.child(ui::utils::traffic_light_spacer_with_child(
                    cx,
                    false,
                    traffic_light_buttons,
                ))
            })
            .map(|this| {
                if !traffic_lights && !left_window_controls {
                    this.pl_1p5()
                } else {
                    this
                }
            })
            .when(!right_window_controls, |this| this.pr_1p5())
            .gap_1()
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .border_b_1()
                    .border_color(cx.theme().colors().border),
            )
            .when_some(left_header_buttons, |this, buttons| this.child(buttons))
            .child(div().flex_1())
            .when_some(right_header_buttons, |this, buttons| this.child(buttons))
            .when(right_window_controls, |this| {
                this.children(Self::render_right_window_controls(window, cx))
            })
    }

    fn render_left_window_controls(window: &Window, cx: &mut App) -> Option<AnyElement> {
        platform_title_bar::render_left_window_controls(
            title_bar::sidebar_button_layout(cx).or_else(|| cx.button_layout()),
            Box::new(CloseWindow),
            window,
        )
    }

    fn render_right_window_controls(window: &Window, cx: &mut App) -> Option<AnyElement> {
        platform_title_bar::render_right_window_controls(
            title_bar::sidebar_button_layout(cx).or_else(|| cx.button_layout()),
            Box::new(CloseWindow),
            window,
        )
    }

    fn active_agent_conversation_view(&self, cx: &App) -> Option<Entity<ConversationView>> {
        self.active_workspace(cx)?
            .read(cx)
            .active_item_as::<AgentThreadItem>(cx)
            .map(|item| item.read(cx).conversation_view())
    }

    fn active_project_agents_md_exists(&self, cx: &App) -> bool {
        let Some(workspace) = self.active_workspace(cx) else {
            return false;
        };
        let project = workspace.read(cx).project().clone();
        let Ok(rel_path) = util::rel_path::RelPath::from_unix_str("AGENTS.md") else {
            return false;
        };
        project
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .and_then(|worktree| {
                let worktree = worktree.read(cx);
                worktree
                    .entry_for_path(rel_path)
                    .is_some_and(|entry| entry.is_file())
                    .then_some(())
            })
            .is_some()
    }

    fn render_agent_options_menu(
        &self,
        labels_visible: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let on_right = self.side(cx) == SidebarSide::Right;
        let active_conversation_view = self.active_agent_conversation_view(cx);
        let can_regenerate_thread_title =
            active_conversation_view
                .as_ref()
                .is_some_and(|conversation_view| {
                    let conversation_view = conversation_view.read(cx);
                    conversation_view.has_user_submitted_prompt(cx)
                        && conversation_view
                            .as_native_thread(cx)
                            .is_some_and(|thread| !thread.read(cx).is_generating_title())
                });
        let has_auth_methods = active_conversation_view
            .as_ref()
            .is_some_and(|conversation_view| conversation_view.read(cx).has_auth_methods());
        let supports_logout = active_conversation_view
            .as_ref()
            .is_some_and(|conversation_view| conversation_view.read(cx).supports_logout());
        let global_agents_md_loaded = UserAgentsMd::global(cx)
            .and_then(|md| md.content())
            .is_some();
        let project_agents_md_exists = self.active_project_agents_md_exists(cx);
        let focus_handle = self.focus_handle.clone();
        let sidebar = cx.weak_entity();

        PopoverMenu::new("agent-sidebar-options-menu")
            .trigger_with_tooltip(
                Button::new(
                    "agent-sidebar-options-menu",
                    if labels_visible { "Agent Tools" } else { "" },
                )
                .size(ButtonSize::Medium)
                .label_size(LabelSize::Small)
                .start_icon(Icon::new(IconName::Settings).size(IconSize::Small))
                .aria_label("Agent Tools and Settings"),
                Tooltip::text("Agent Tools and Settings"),
            )
            .anchor(if on_right {
                gpui::Anchor::BottomRight
            } else {
                gpui::Anchor::BottomLeft
            })
            .attach(if on_right {
                gpui::Anchor::TopRight
            } else {
                gpui::Anchor::TopLeft
            })
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-4.0),
            })
            .with_handle(self.agent_options_menu_handle.clone())
            .menu(move |window, cx| {
                let active_conversation_view = active_conversation_view.clone();
                let sidebar = sidebar.clone();
                let focus_handle = focus_handle.clone();
                Some(ContextMenu::build(
                    window,
                    cx,
                    move |mut menu, _window, _| {
                        menu = menu.context(focus_handle.clone());

                        if can_regenerate_thread_title {
                            menu = menu.header(agent_session_label(
                                APP_NAME,
                                "Current Thread",
                                "Current Agent Session",
                            ));
                            if let Some(conversation_view) = active_conversation_view.clone() {
                                menu = menu
                                    .entry(
                                        agent_session_label(
                                            APP_NAME,
                                            "Regenerate Thread Title",
                                            "Regenerate Agent Session Title",
                                        ),
                                        None,
                                        {
                                            let sidebar = sidebar.clone();
                                            move |_window, cx| {
                                                let result = conversation_view.update(
                                                    cx,
                                                    |conversation_view, cx| {
                                                        conversation_view
                                                            .regenerate_thread_title(cx)
                                                    },
                                                );
                                                if matches!(
                                                    result,
                                                    ThreadTitleRegenerationResult::NoModel
                                                ) {
                                                    sidebar
                                                        .update(cx, |sidebar, cx| {
                                                            if let Some(workspace) =
                                                                sidebar.active_workspace(cx)
                                                            {
                                                                Self::show_no_thread_summary_model_toast(
                                                                    workspace, cx,
                                                                );
                                                            }
                                                        })
                                                        .ok();
                                                }
                                            }
                                        },
                                    )
                                    .separator();
                            }
                        }

                        menu = menu
                            .header("MCP Servers")
                            .action(
                                "Add MCP Server…",
                                Box::new(zed_actions::OpenSettingsAt {
                                    path: "context_servers".to_string(),
                                    target: None,
                                }),
                            )
                            .action(
                                "Browse MCP Extensions…",
                                Box::new(zed_actions::Extensions {
                                    category_filter: Some(
                                        zed_actions::ExtensionCategoryFilter::ContextServers,
                                    ),
                                    id: None,
                                }),
                            )
                            .separator()
                            .header("Agent Context")
                            .action("Manage Skills", Box::new(ManageSkills));

                        if global_agents_md_loaded || project_agents_md_exists {
                            if global_agents_md_loaded {
                                menu = menu
                                    .action("Open Global Rules", Box::new(OpenGlobalAgentsMdRules));
                            }
                            if project_agents_md_exists {
                                menu = menu.action(
                                    "Open Workspace Rules",
                                    Box::new(OpenProjectAgentsMdRules),
                                );
                            }
                            menu = menu.separator();
                        }

                        menu = menu
                            .action("Agent Profiles", Box::new(ManageProfiles::default()))
                            .action(
                                agent_session_label(
                                    APP_NAME,
                                    "Open Settings",
                                    "Agent Settings",
                                ),
                                Box::new(OpenSettings),
                            )
                            .separator()
                            .action(
                                if APP_NAME == "Zed" {
                                    "Toggle Sidebar"
                                } else {
                                    "Hide Session Rail"
                                },
                                Box::new(ToggleSidebar),
                            );

                        if has_auth_methods || supports_logout {
                            menu = menu.separator();
                        }
                        if has_auth_methods {
                            if let Some(conversation_view) = active_conversation_view.clone() {
                                menu = menu.entry("Reauthenticate", None, move |window, cx| {
                                    conversation_view.update(cx, |conversation_view, cx| {
                                        conversation_view.reauthenticate(window, cx)
                                    });
                                });
                            }
                        }
                        if supports_logout {
                            if let Some(conversation_view) = active_conversation_view.clone() {
                                menu = menu.entry("Log Out", None, move |window, cx| {
                                    conversation_view.update(cx, |conversation_view, cx| {
                                        conversation_view.logout(window, cx)
                                    });
                                });
                            }
                        }

                        menu
                    },
                ))
            })
    }

    fn render_sidebar_bottom_bar(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let is_archive = matches!(self.view, SidebarView::Archive(..));
        let history_label = if is_archive {
            "Hide Agent History"
        } else {
            "Show Agent History"
        };
        let on_right = self.side(cx) == SidebarSide::Right;
        let rail_width = SessionRailSettings::get_global(cx).width(self.width);
        let labels_visible = !session_rail_row_is_compact(rail_width);

        v_flex()
            .p_1()
            .gap_1()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .child(self.sidebar_chrome.clone())
            .child(
                h_flex()
                    .id("session-rail-utilities")
                    .role(gpui::Role::Group)
                    .aria_label("Session Rail utilities")
                    .w_full()
                    .gap_1()
                    .when(on_right, |this| this.flex_row_reverse())
                    .child(self.render_agent_options_menu(labels_visible, cx))
                    .child(
                        Button::new("history", if labels_visible { "History" } else { "" })
                            .size(ButtonSize::Medium)
                            .label_size(LabelSize::Small)
                            .start_icon(Icon::new(IconName::Clock).size(IconSize::Small))
                            .toggle_state(is_archive)
                            .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                            .aria_label(history_label)
                            .tooltip(move |_, cx| {
                                Tooltip::for_action(history_label, &ToggleThreadHistory, cx)
                            })
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.toggle_archive(&ToggleThreadHistory, window, cx);
                            })),
                    )
                    .child(div().flex_1())
                    .child(self.render_recent_projects_button(labels_visible, cx)),
            )
    }

    fn toggle_attention_filter(
        &mut self,
        _: &ToggleAttentionFilter,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_attention_filter(!self.attention_only, window, cx);
    }

    fn set_attention_filter(
        &mut self,
        attention_only: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(self.view, SidebarView::Archive(_)) {
            self.show_thread_list(window, cx);
        }
        if self.attention_only != attention_only {
            self.attention_only = attention_only;
            self.update_entries(cx);
            if self.selection.is_none() {
                self.select_first_entry();
            }
            if let Some(index) = self.selection {
                self.list_state.scroll_to_reveal_item(index);
            }
        }
        self.focus_handle.focus(window, cx);
    }

    fn toggle_agent_options_menu(
        &mut self,
        _: &ToggleOptionsMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.stop_propagation();
        window.focus(&self.focus_handle, cx);
        self.agent_options_menu_handle.toggle(window, cx);
    }

    fn active_workspace(&self, cx: &App) -> Option<Entity<Workspace>> {
        self.multi_workspace
            .upgrade()
            .map(|w| w.read(cx).workspace().clone())
    }

    fn show_thread_import_modal(
        &mut self,
        source: &'static str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        telemetry::event!(
            "Agent Threads Import Clicked",
            source = source,
            side = match self.side(cx) {
                SidebarSide::Left => "left",
                SidebarSide::Right => "right",
            }
        );

        let Some(active_workspace) = self.active_workspace(cx) else {
            return;
        };

        let Some(agent_registry_store) = AgentRegistryStore::try_global(cx) else {
            return;
        };

        let agent_server_store = active_workspace
            .read(cx)
            .project()
            .read(cx)
            .agent_server_store()
            .clone();

        let workspace_handle = active_workspace.downgrade();
        let multi_workspace = self.multi_workspace.clone();

        active_workspace.update(cx, |workspace, cx| {
            workspace.toggle_modal(window, cx, |window, cx| {
                ThreadImportModal::new(
                    agent_server_store,
                    agent_registry_store,
                    workspace_handle.clone(),
                    multi_workspace.clone(),
                    window,
                    cx,
                )
            });
        });
    }

    fn should_render_acp_import_onboarding(&self, cx: &App) -> bool {
        let has_external_agents = self
            .active_workspace(cx)
            .map(|ws| {
                ws.read(cx)
                    .project()
                    .read(cx)
                    .agent_server_store()
                    .read(cx)
                    .has_external_agents()
            })
            .unwrap_or(false);

        has_external_agents && !AcpThreadImportOnboarding::dismissed(cx)
    }

    fn render_acp_import_onboarding(
        &mut self,
        verbose_labels: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let on_import = cx.listener(|this, _, window, cx| {
            this.show_archive(window, cx);
            this.show_thread_import_modal("external_agent_onboarding", window, cx);
        });
        render_import_onboarding_banner(
            "acp",
            "Agent history found",
            "Import conversations from Claude Agent, Codex, and other ACP clients as normal workspace surfaces.",
            if verbose_labels {
                "Import Agent History"
            } else {
                "Import History"
            },
            |_, _window, cx| AcpThreadImportOnboarding::dismiss(cx),
            on_import,
            cx,
        )
    }

    fn should_render_cross_channel_import_onboarding(&self, cx: &App) -> bool {
        APP_NAME == "Zed"
            && !CrossChannelImportOnboarding::dismissed(cx)
            && !self.cross_channel_import_channels.is_empty()
    }

    fn render_cross_channel_import_onboarding(
        &mut self,
        verbose_labels: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let channel_names = self
            .cross_channel_import_channels
            .iter()
            .map(SharedString::as_str)
            .join(" and ");

        let description = format!(
            "Import threads from {} to continue where you left off.",
            channel_names
        );

        let on_import = cx.listener(|this, _, _window, cx| {
            telemetry::event!(
                "Agent Threads Import Clicked",
                source = "cross_channel_onboarding",
                side = match this.side(cx) {
                    SidebarSide::Left => "left",
                    SidebarSide::Right => "right",
                }
            );
            CrossChannelImportOnboarding::dismiss(cx);
            if let Some(workspace) = this.active_workspace(cx) {
                workspace.update(cx, |workspace, cx| {
                    import_threads_from_other_channels(workspace, cx);
                });
            }
        });
        render_import_onboarding_banner(
            "channel",
            "Threads found from other channels",
            description,
            if verbose_labels {
                "Import Threads from Other Channels"
            } else {
                "Import Threads"
            },
            |_, _window, cx| CrossChannelImportOnboarding::dismiss(cx),
            on_import,
            cx,
        )
    }

    fn toggle_archive(
        &mut self,
        _: &ToggleThreadHistory,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match &self.view {
            SidebarView::ThreadList => {
                self.show_archive(window, cx);
            }
            SidebarView::Archive(_) => self.show_thread_list(window, cx),
        }
    }

    fn show_archive(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let side = match self.side(cx) {
            SidebarSide::Left => "left",
            SidebarSide::Right => "right",
        };
        telemetry::event!("Thread History Viewed", side = side);

        let Some(active_workspace) = self
            .multi_workspace
            .upgrade()
            .map(|w| w.read(cx).workspace().clone())
        else {
            return;
        };
        let project = active_workspace.read(cx).project().clone();
        let agent_server_store = project.read(cx).agent_server_store().downgrade();
        let agent_connection_store = connection_store_for_project(&project, cx).downgrade();

        let archive_view = cx.new(|cx| {
            ThreadsArchiveView::new(
                active_workspace.downgrade(),
                agent_connection_store.clone(),
                agent_server_store.clone(),
                window,
                cx,
            )
        });

        let subscription = cx.subscribe_in(
            &archive_view,
            window,
            |this, _, event: &ThreadsArchiveViewEvent, window, cx| match event {
                ThreadsArchiveViewEvent::Close => {
                    this.show_thread_list(window, cx);
                }
                ThreadsArchiveViewEvent::Activate { thread } => {
                    this.open_thread_from_archive(thread.clone(), window, cx);
                }
                ThreadsArchiveViewEvent::CancelRestore { thread_id } => {
                    this.restoring_tasks.remove(thread_id);
                }
                ThreadsArchiveViewEvent::Import => {
                    this.show_thread_import_modal("thread_history", window, cx);
                }
                ThreadsArchiveViewEvent::NewThread => {
                    this.show_thread_list(window, cx);
                    if let Some(workspace) = this.active_workspace(cx) {
                        this.create_new_thread(&workspace, window, cx);
                    }
                }
            },
        );

        self._subscriptions.push(subscription);
        self.view = SidebarView::Archive(archive_view.clone());
        self.serialize(cx);
        cx.notify();
    }

    fn show_thread_list(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.view = SidebarView::ThreadList;
        self._subscriptions.clear();
        self.focus_handle.focus(window, cx);
        self.serialize(cx);
        cx.notify();
    }
}

fn render_import_onboarding_banner(
    id: impl Into<SharedString>,
    title: impl Into<SharedString>,
    description: impl Into<SharedString>,
    button_label: impl Into<SharedString>,
    on_dismiss: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_import: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    cx: &App,
) -> impl IntoElement {
    let id: SharedString = id.into();
    let title: SharedString = title.into();
    let dismiss_label: SharedString = format!("Dismiss {title}").into();
    let bg = cx.theme().colors().text_accent;

    v_flex()
        .min_w_0()
        .w_full()
        .p_2()
        .border_t_1()
        .border_color(cx.theme().colors().border)
        .bg(linear_gradient(
            360.,
            linear_color_stop(bg.opacity(0.06), 1.),
            linear_color_stop(bg.opacity(0.), 0.),
        ))
        .child(
            h_flex()
                .min_w_0()
                .w_full()
                .gap_1()
                .justify_between()
                .flex_wrap()
                .child(Label::new(title).size(LabelSize::Small))
                .child(
                    IconButton::new(
                        SharedString::from(format!("close-{id}-onboarding")),
                        IconName::Close,
                    )
                    .size(ButtonSize::Medium)
                    .icon_size(IconSize::Small)
                    .aria_label(dismiss_label.clone())
                    .tooltip(Tooltip::text(dismiss_label))
                    .on_click(on_dismiss),
                ),
        )
        .child(
            Label::new(description)
                .size(LabelSize::Small)
                .color(Color::Muted)
                .mb_2(),
        )
        .child(
            Button::new(SharedString::from(format!("import-{id}")), button_label)
                .full_width()
                .style(ButtonStyle::OutlinedCustom(cx.theme().colors().border))
                .label_size(LabelSize::Small)
                .start_icon(
                    Icon::new(IconName::Download)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .on_click(on_import),
        )
}

impl WorkspaceSidebar for Sidebar {
    fn width(&self, cx: &App) -> Pixels {
        SessionRailSettings::get_global(cx).width(self.width)
    }

    fn set_width(&mut self, width: Option<Pixels>, cx: &mut Context<Self>) {
        let width = width.unwrap_or(DEFAULT_WIDTH).clamp(MIN_WIDTH, MAX_WIDTH);
        if self.width == width {
            return;
        }
        self.width = width;
        self.serialize(cx);
        cx.notify();
    }

    fn has_notifications(&self, cx: &App) -> bool {
        WorkspaceBarAttentionSettings::get_global(cx).show_agent_attention
            && self.contents.has_attention
    }

    fn is_threads_list_view_active(&self) -> bool {
        matches!(self.view, SidebarView::ThreadList)
    }

    fn side(&self, cx: &App) -> SidebarSide {
        SidebarSettings::get_global(cx).side()
    }

    fn prepare_for_focus(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.selection = None;
        cx.notify();
    }

    fn toggle_thread_switcher(
        &mut self,
        select_last: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.toggle_thread_switcher_impl(select_last, window, cx);
    }

    fn cycle_project(&mut self, forward: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.cycle_project_impl(forward, window, cx);
    }

    fn cycle_thread(&mut self, forward: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.cycle_thread_impl(forward, window, cx);
    }

    fn toggle_options_menu(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        window.focus(&self.focus_handle, cx);
        self.agent_options_menu_handle.toggle(window, cx);
    }

    fn simulate_update_available(&mut self, cx: &mut Context<Self>) {
        self.sidebar_chrome.update(cx, |sidebar_chrome, cx| {
            sidebar_chrome.toggle_update_simulation(cx);
        });
    }

    #[cfg(not(target_os = "macos"))]
    fn open_application_menu(
        &mut self,
        menu_name: String,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.sidebar_chrome.update(cx, |sidebar_chrome, cx| {
            sidebar_chrome.open_application_menu(menu_name, cx);
        });
    }

    #[cfg(not(target_os = "macos"))]
    fn activate_application_menu(
        &mut self,
        right: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.sidebar_chrome.update(cx, |sidebar_chrome, cx| {
            sidebar_chrome.activate_application_menu(right, window, cx);
        });
    }

    fn serialized_state(&self, _cx: &App) -> Option<String> {
        let serialized = SerializedSidebar {
            width: Some(f32::from(self.width)),
            active_view: match self.view {
                SidebarView::ThreadList => SerializedSidebarView::ThreadList,
                SidebarView::Archive(_) => SerializedSidebarView::History,
            },
            manual_entry_order: self.manual_entry_order.clone(),
        };
        serde_json::to_string(&serialized).ok()
    }

    fn restore_serialized_state(
        &mut self,
        state: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(serialized) = serde_json::from_str::<SerializedSidebar>(state).log_err() {
            if let Some(width) = serialized.width {
                self.width = px(width).clamp(MIN_WIDTH, MAX_WIDTH);
            }
            self.manual_entry_order = serialized.manual_entry_order;
            if serialized.active_view == SerializedSidebarView::History {
                cx.defer_in(window, |this, window, cx| {
                    this.show_archive(window, cx);
                });
            }
        }
        cx.notify();
    }
}

impl gpui::EventEmitter<workspace::SidebarEvent> for Sidebar {}

impl Focusable for Sidebar {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let session_rail_settings = SessionRailSettings::get_global(cx);
        if session_rail_settings.is_hidden() {
            return div()
                .id("workspace-sidebar-hidden")
                .size_0()
                .into_any_element();
        }
        let rail_width = session_rail_settings.width(self.width);

        let ui_font = theme_settings::setup_ui_font(window, cx);
        let sticky_header = self.render_sticky_header(window, cx);

        let color = cx.theme().colors();
        let bg = color.editor_background;

        let no_search_results = self.contents.entries.is_empty();
        let has_query = self.has_filter_query(cx);
        let show_session_search = session_search_visible(self.contents.session_count, has_query);
        let show_start_state = !self.contents.has_open_projects
            && self.contents.session_count == 0
            && !has_query
            && !self.attention_only;

        v_flex()
            .id("workspace-sidebar")
            .role(gpui::Role::Complementary)
            .aria_label("Session Rail")
            .key_context(self.dispatch_context(window, cx))
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::editor_move_down))
            .on_action(cx.listener(Self::editor_move_up))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::expand_selected_entry))
            .on_action(cx.listener(Self::collapse_selected_entry))
            .on_action(cx.listener(Self::toggle_selected_fold))
            .on_action(cx.listener(Self::fold_all))
            .on_action(cx.listener(Self::unfold_all))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::archive_selected_thread))
            .on_action(cx.listener(Self::rename_selected_thread))
            .on_action(cx.listener(Self::move_selected_entry_up))
            .on_action(cx.listener(Self::move_selected_entry_down))
            .on_action(cx.listener(Self::new_thread_in_group))
            .on_action(cx.listener(Self::new_session_in_group))
            .on_action(cx.listener(Self::new_terminal_thread))
            .on_action(cx.listener(Self::toggle_archive))
            .on_action(cx.listener(Self::toggle_attention_filter))
            .on_action(cx.listener(Self::open_selected_review_brief))
            .on_action(cx.listener(Self::focus_sidebar_filter))
            .on_action(cx.listener(Self::on_toggle_thread_switcher))
            .on_action(cx.listener(Self::on_next_project))
            .on_action(cx.listener(Self::on_previous_project))
            .on_action(cx.listener(Self::on_next_thread))
            .on_action(cx.listener(Self::on_previous_thread))
            .on_action(cx.listener(Self::toggle_agent_options_menu))
            .on_action(cx.listener(|this, _: &OpenRecent, window, cx| {
                this.recent_projects_popover_handle.toggle(window, cx);
            }))
            .font(ui_font)
            .map(|el| {
                let on_left = self.side(cx) == SidebarSide::Left;
                match window.window_decorations() {
                    Decorations::Server => el.h_full().w(rail_width),
                    // With client-side decorations the sidebar owns the window
                    // corners on its side, so round them like the title bar and
                    // status bar do. The sidebar is stretched 1px outwards over
                    // the window border on untiled edges (with compensating
                    // padding) so its rounded background lines up exactly with
                    // the window shape, avoiding a transparent gap in the
                    // rounded corners.
                    Decorations::Client { tiling, .. } => el
                        .absolute()
                        .w(rail_width)
                        .top(if tiling.top { px(0.) } else { px(-1.) })
                        .bottom(if tiling.bottom { px(0.) } else { px(-1.) })
                        .when(!tiling.top, |el| el.pt_px())
                        .when(!tiling.bottom, |el| el.pb_px())
                        .map(|el| {
                            if on_left {
                                el.left(if tiling.left { px(0.) } else { px(-1.) })
                                    .when(!tiling.left, |el| el.pl(px(1.)))
                            } else {
                                el.right(if tiling.right { px(0.) } else { px(-1.) })
                                    .when(!tiling.right, |el| el.pr(px(1.)))
                            }
                        })
                        .when(on_left && !(tiling.top || tiling.left), |el| {
                            el.rounded_tl(CLIENT_SIDE_DECORATION_ROUNDING)
                        })
                        .when(on_left && !(tiling.bottom || tiling.left), |el| {
                            el.rounded_bl(CLIENT_SIDE_DECORATION_ROUNDING)
                        })
                        .when(!on_left && !(tiling.top || tiling.right), |el| {
                            el.rounded_tr(CLIENT_SIDE_DECORATION_ROUNDING)
                        })
                        .when(!on_left && !(tiling.bottom || tiling.right), |el| {
                            el.rounded_br(CLIENT_SIDE_DECORATION_ROUNDING)
                        }),
                }
            })
            .bg(bg)
            .overflow_hidden()
            .rounded_lg()
            .border_1()
            .border_color(color.border)
            .child(self.render_sidebar_header(window, cx))
            .map(|this| match &self.view {
                SidebarView::ThreadList => this
                    .when(session_overview_visible(show_start_state), |this| {
                        this.child(self.render_session_overview(cx))
                    })
                    .map(|this| {
                        if show_start_state {
                            this.when_some(
                                self.render_workspace_restore_status(cx),
                                |this, status| this.child(status),
                            )
                            .when_some(self.render_terminal_host_status(cx), |this, status| {
                                this.child(status)
                            })
                            .child(self.render_empty_state(cx))
                        } else {
                            this.child(
                                v_flex()
                                    .flex_1()
                                    .overflow_hidden()
                                    .when_some(
                                        self.render_workspace_restore_status(cx),
                                        |this, status| this.child(status),
                                    )
                                    .when_some(
                                        self.render_terminal_host_status(cx),
                                        |this, status| this.child(status),
                                    )
                                    .when(show_session_search, |this| {
                                        this.child(self.render_session_search(cx))
                                    })
                                    .child(
                                        v_flex()
                                            .id("workspace-session-list")
                                            .role(gpui::Role::List)
                                            .aria_label("Workspace Sessions")
                                            .relative()
                                            .flex_1()
                                            .overflow_hidden()
                                            .map(|this| {
                                                if no_search_results {
                                                    if has_query {
                                                        this.child(self.render_no_results(cx))
                                                    } else if self.attention_only {
                                                        this.child(
                                                            self.render_attention_empty_state(cx),
                                                        )
                                                    } else {
                                                        this.child(self.render_no_results(cx))
                                                    }
                                                } else {
                                                    this.child(
                                                        list(
                                                            self.list_state.clone(),
                                                            cx.processor(Self::render_list_entry),
                                                        )
                                                        .flex_1()
                                                        .size_full(),
                                                    )
                                                    .when_some(sticky_header, |this, header| {
                                                        this.child(header)
                                                    })
                                                    .custom_scrollbars(
                                                        Scrollbars::new(ScrollAxes::Vertical)
                                                            .tracked_scroll_handle(
                                                                &self.list_state,
                                                            ),
                                                        window,
                                                        cx,
                                                    )
                                                }
                                            }),
                                    ),
                            )
                        }
                    }),
                SidebarView::Archive(archive_view) => this.child(archive_view.clone()),
            })
            .map(|this| {
                let show_acp = self.should_render_acp_import_onboarding(cx);
                let show_cross_channel = self.should_render_cross_channel_import_onboarding(cx);

                let verbose = *self
                    .import_banners_use_verbose_labels
                    .get_or_insert(show_acp && show_cross_channel);

                this.when(show_acp, |this| {
                    this.child(self.render_acp_import_onboarding(verbose, cx))
                })
                .when(show_cross_channel, |this| {
                    this.child(self.render_cross_channel_import_onboarding(verbose, cx))
                })
            })
            .child(self.render_sidebar_bottom_bar(cx))
            .into_any_element()
    }
}

fn all_thread_infos_for_workspace(
    workspace: &Entity<Workspace>,
    cx: &App,
) -> impl Iterator<Item = ActiveThreadInfo> {
    workspace
        .read(cx)
        .items_of_type::<AgentThreadItem>(cx)
        .filter_map(|item| item.read(cx).active_thread_info(cx))
        .map(|info| ActiveThreadInfo {
            session_id: info.session_id,
            title: info.title,
            status: info.status,
            icon: info.icon,
            icon_from_external_svg: info.icon_from_external_svg,
            is_background: false,
            is_title_generating: info.is_title_generating,
            diff_stats: info.diff_stats,
            changed_files: info.changed_files,
        })
}

pub fn dump_workspace_info(
    workspace: &mut Workspace,
    _: &DumpWorkspaceInfo,
    window: &mut gpui::Window,
    cx: &mut gpui::Context<Workspace>,
) {
    use std::fmt::Write;

    let mut output = String::new();
    let this_entity = cx.entity();

    let multi_workspace = workspace.multi_workspace().and_then(|weak| weak.upgrade());
    let workspaces: Vec<gpui::Entity<Workspace>> = match &multi_workspace {
        Some(mw) => mw.read(cx).workspaces().cloned().collect(),
        None => vec![this_entity.clone()],
    };
    let active_workspace = multi_workspace
        .as_ref()
        .map(|mw| mw.read(cx).workspace().clone());

    writeln!(output, "MultiWorkspace: {} workspace(s)", workspaces.len()).ok();

    if let Some(mw) = &multi_workspace {
        let keys: Vec<_> = mw.read(cx).project_group_keys();
        writeln!(output, "Project group keys ({}):", keys.len()).ok();
        for key in keys {
            writeln!(output, "  - {key:?}").ok();
        }
    }

    writeln!(output).ok();

    for (index, ws) in workspaces.iter().enumerate() {
        let is_active = active_workspace.as_ref() == Some(ws);
        writeln!(
            output,
            "--- Workspace {index}{} ---",
            if is_active { " (active)" } else { "" }
        )
        .ok();

        // project_group_key_for_workspace internally reads the workspace,
        // so we can only call it for workspaces other than this_entity
        // (which is already being updated).
        if let Some(mw) = &multi_workspace {
            if *ws == this_entity {
                let workspace_key = workspace.project_group_key(cx);
                writeln!(output, "ProjectGroupKey: {workspace_key:?}").ok();
            } else {
                let effective_key = mw.read(cx).project_group_key_for_workspace(ws, cx);
                let workspace_key = ws.read(cx).project_group_key(cx);
                if effective_key != workspace_key {
                    writeln!(
                        output,
                        "ProjectGroupKey (multi_workspace): {effective_key:?}"
                    )
                    .ok();
                    writeln!(
                        output,
                        "ProjectGroupKey (workspace, DISAGREES): {workspace_key:?}"
                    )
                    .ok();
                } else {
                    writeln!(output, "ProjectGroupKey: {effective_key:?}").ok();
                }
            }
        } else {
            let workspace_key = workspace.project_group_key(cx);
            writeln!(output, "ProjectGroupKey: {workspace_key:?}").ok();
        }

        // The action handler is already inside an update on `this_entity`,
        // so we must avoid a nested read/update on that same entity.
        if *ws == this_entity {
            dump_single_workspace(workspace, &mut output, cx);
        } else {
            ws.read_with(cx, |ws, cx| {
                dump_single_workspace(ws, &mut output, cx);
            });
        }
    }

    let project = workspace.project().clone();
    cx.spawn_in(window, async move |_this, cx| {
        let buffer = project
            .update(cx, |project, cx| project.create_buffer(None, false, cx))
            .await?;

        buffer.update(cx, |buffer, cx| {
            buffer.set_text(output, cx);
        });

        let buffer = cx.new(|cx| {
            editor::MultiBuffer::singleton(buffer, cx).with_title("Workspace Info".into())
        });

        _this.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(
                Box::new(cx.new(|cx| {
                    let mut editor =
                        editor::Editor::for_multibuffer(buffer, Some(project.clone()), window, cx);
                    editor.set_read_only(true);
                    editor.set_should_serialize(false, cx);
                    editor.set_breadcrumb_header("Workspace Info".into());
                    editor
                })),
                None,
                true,
                window,
                cx,
            );
        })
    })
    .detach_and_log_err(cx);
}

fn dump_single_workspace(workspace: &Workspace, output: &mut String, cx: &gpui::App) {
    use std::fmt::Write;

    let workspace_db_id = workspace.database_id();
    match workspace_db_id {
        Some(id) => writeln!(output, "Workspace DB ID: {id:?}").ok(),
        None => writeln!(output, "Workspace DB ID: (none)").ok(),
    };

    let project = workspace.project().read(cx);

    let repos: Vec<_> = project
        .repositories(cx)
        .values()
        .map(|repo| repo.read(cx).snapshot())
        .collect();

    writeln!(output, "Worktrees:").ok();
    for worktree in project.worktrees(cx) {
        let worktree = worktree.read(cx);
        let abs_path = worktree.abs_path();
        let visible = worktree.is_visible();

        let repo_info = repos
            .iter()
            .find(|snapshot| abs_path.starts_with(&*snapshot.work_directory_abs_path));

        let is_linked = repo_info.map(|s| s.is_linked_worktree()).unwrap_or(false);
        let main_worktree_path = repo_info.and_then(|s| s.main_worktree_abs_path());
        let branch = repo_info.and_then(|s| s.branch.as_ref().map(|b| b.ref_name.clone()));

        write!(output, "  - {}", abs_path.display()).ok();
        if !visible {
            write!(output, " (hidden)").ok();
        }
        if let Some(branch) = &branch {
            write!(output, " [branch: {branch}]").ok();
        }
        if is_linked {
            if let Some(main_worktree_path) = main_worktree_path {
                write!(
                    output,
                    " [linked worktree -> {}]",
                    main_worktree_path.display()
                )
                .ok();
            } else {
                write!(output, " [linked worktree]").ok();
            }
        }
        writeln!(output).ok();
    }

    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
        let panel = panel.read(cx);

        let panel_workspace_id = panel.workspace_id();
        if panel_workspace_id != workspace_db_id {
            writeln!(
                output,
                "  \u{26a0} workspace ID mismatch! panel has {panel_workspace_id:?}, workspace has {workspace_db_id:?}"
            )
            .ok();
        }

        if let Some(thread) = panel.active_agent_thread(cx) {
            let thread = thread.read(cx);
            let title = thread.title().unwrap_or_else(|| "(untitled)".into());
            let session_id = thread.session_id();
            let status = match thread.status() {
                ThreadStatus::Idle => "idle",
                ThreadStatus::Generating => "generating",
            };
            let entry_count = thread.entries().len();
            write!(output, "Active thread: {title} (session: {session_id})").ok();
            write!(output, " [{status}, {entry_count} entries").ok();
            if panel
                .active_conversation_view()
                .is_some_and(|conversation_view| {
                    conversation_view
                        .read(cx)
                        .root_thread_has_pending_tool_call(cx)
                })
            {
                write!(output, ", awaiting confirmation").ok();
            }
            writeln!(output, "]").ok();
        } else {
            writeln!(output, "Active thread: (none)").ok();
        }

        let background_threads = panel.retained_threads();
        if !background_threads.is_empty() {
            writeln!(
                output,
                "Background threads ({}): ",
                background_threads.len()
            )
            .ok();
            for (session_id, conversation_view) in background_threads {
                if let Some(thread_view) = conversation_view.read(cx).root_thread_view() {
                    let thread = thread_view.read(cx).thread.read(cx);
                    let title = thread.title().unwrap_or_else(|| "(untitled)".into());
                    let status = match thread.status() {
                        ThreadStatus::Idle => "idle",
                        ThreadStatus::Generating => "generating",
                    };
                    let entry_count = thread.entries().len();
                    write!(output, "  - {title} (thread: {session_id:?})").ok();
                    write!(output, " [{status}, {entry_count} entries").ok();
                    if conversation_view
                        .read(cx)
                        .root_thread_has_pending_tool_call(cx)
                    {
                        write!(output, ", awaiting confirmation").ok();
                    }
                    writeln!(output, "]").ok();
                } else {
                    writeln!(output, "  - (not connected) (thread: {session_id:?})").ok();
                }
            }
        }
    } else {
        writeln!(output, "Agent panel: not loaded").ok();
    }

    writeln!(output).ok();
}
