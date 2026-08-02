use crate::{
    BrowseRunningSessions, NewFile, Open, OpenFolder, OpenMode, PathList, RecentWorkspace,
    RevealFiles, RevealGitChanges, SerializedWorkspaceLocation, Workspace, WorkspaceId,
    WorkspaceSettings,
    item::{Item, ItemEvent},
    persistence::WorkspaceDb,
};
use agent_settings::{
    AgentSettings, configured_terminal_launcher_icon, configured_terminal_launcher_label,
};
use git::Clone as GitClone;
use gpui::WeakEntity;
use gpui::{
    Action, App, Context, Entity, EventEmitter, FocusHandle, Focusable, FontWeight,
    InteractiveElement, ParentElement, Pixels, Render, Styled, Task, TaskExt, Window, actions,
    container_query, px,
};
use menu::{SelectNext, SelectPrevious};
use paths::APP_NAME;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{DefaultOpenBehavior, Settings};
use ui::{
    ButtonLike, Callout, Divider, DividerColor, KeyBinding, Severity, Tooltip, prelude::*,
    theme_is_transparent,
};
use util::{ResultExt, paths::PathExt};
use zed_actions::{
    Extensions, OpenKeymap, OpenOnboarding, OpenRecent, OpenSettings, command_palette,
};

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, JsonSchema, Action)]
#[action(namespace = welcome)]
#[serde(transparent)]
pub struct OpenRecentProject {
    pub index: usize,
}

actions!(
    zed,
    [
        /// Opens Home.
        ShowWelcome
    ]
);

#[derive(IntoElement)]
struct SectionHeader {
    title: SharedString,
}

impl SectionHeader {
    fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
        }
    }
}

impl RenderOnce for SectionHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let title: SharedString = if APP_NAME == "Zed" {
            self.title.to_ascii_uppercase().into()
        } else {
            self.title
        };
        h_flex()
            .w_full()
            .min_w_0()
            .px_1()
            .mb_1()
            .gap_2()
            .child(
                div().flex_none().child(
                    Label::new(title)
                        .when(APP_NAME == "Zed", |label| label.buffer_font(cx))
                        .color(Color::Muted)
                        .size(LabelSize::XSmall),
                ),
            )
            .when(APP_NAME == "Zed", |this| {
                this.child(Divider::horizontal().color(DividerColor::BorderVariant))
            })
    }
}

#[derive(IntoElement)]
struct SectionButton {
    label: SharedString,
    icon: IconName,
    action: Box<dyn Action>,
    tab_index: usize,
    focus_handle: FocusHandle,
    primary: bool,
    meta: Option<SharedString>,
    show_meta: bool,
}

impl SectionButton {
    fn new(
        label: impl Into<SharedString>,
        icon: IconName,
        action: &dyn Action,
        tab_index: usize,
        focus_handle: FocusHandle,
        primary: bool,
    ) -> Self {
        Self {
            label: label.into(),
            icon,
            action: action.boxed_clone(),
            tab_index,
            focus_handle,
            primary,
            meta: None,
            show_meta: true,
        }
    }

    fn meta(mut self, meta: impl Into<SharedString>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    fn show_meta(mut self, show_meta: bool) -> Self {
        self.show_meta = show_meta;
        self
    }
}

impl RenderOnce for SectionButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let id = format!("home-action-{}-{}", self.label, self.tab_index);
        let action_ref: &dyn Action = &*self.action;
        let meta = self.meta.clone();
        let show_meta = self.show_meta;
        let aria_description = match (self.primary, meta.as_deref()) {
            (true, Some(meta)) => Some(SharedString::from(format!(
                "Recommended first step. {meta}"
            ))),
            (true, None) => Some(SharedString::from("Recommended first step")),
            (false, Some(meta)) => Some(SharedString::from(meta.to_owned())),
            (false, None) => None,
        };
        let icon_color = if self.primary {
            Color::Accent
        } else {
            Color::Muted
        };

        ButtonLike::new(id)
            .tab_index(self.tab_index as isize)
            .aria_label(self.label.clone())
            .when(APP_NAME != "Zed", |this| this.style(ButtonStyle::Subtle))
            .when(self.primary, |this| this.style(ButtonStyle::Filled))
            .when_some(aria_description, |this, description| {
                this.aria_description(description)
            })
            .when_some(meta.clone(), |this, meta| this.tooltip(Tooltip::text(meta)))
            .full_width()
            .size(ButtonSize::Medium)
            .child(
                h_flex()
                    .w_full()
                    .min_w_0()
                    .justify_between()
                    .child(
                        h_flex()
                            .min_w_0()
                            .flex_1()
                            .gap_2()
                            .child(Icon::new(self.icon).color(icon_color).size(IconSize::Small))
                            .child(
                                Label::new(self.label)
                                    .truncate()
                                    .when(self.primary, |label| label.weight(FontWeight::MEDIUM)),
                            ),
                    )
                    .child(
                        h_flex()
                            .flex_none()
                            .gap_2()
                            .when(show_meta, |this| {
                                this.when_some(meta, |this, meta| {
                                    this.child(
                                        div().max_w(rems_from_px(220.)).overflow_hidden().child(
                                            Label::new(meta)
                                                .truncate()
                                                .size(LabelSize::XSmall)
                                                .color(Color::Muted),
                                        ),
                                    )
                                })
                            })
                            .child(
                                KeyBinding::for_action_in(action_ref, &self.focus_handle, cx)
                                    .size(rems_from_px(12.)),
                            ),
                    ),
            )
            .on_click(move |_, window, cx| {
                self.focus_handle.dispatch_action(&*self.action, window, cx)
            })
    }
}

enum SectionVisibility {
    Always,
    LocalWorkspace,
}

impl SectionVisibility {
    fn is_visible(&self, local_workspace: bool) -> bool {
        match self {
            SectionVisibility::Always => true,
            SectionVisibility::LocalWorkspace => local_workspace,
        }
    }
}

struct SectionEntry {
    icon: IconName,
    title: &'static str,
    meta: Option<&'static str>,
    action: &'static dyn Action,
    visibility_guard: SectionVisibility,
}

impl SectionEntry {
    fn render(
        &self,
        button_index: usize,
        focus: &FocusHandle,
        primary: bool,
        show_meta: bool,
        meta_override: Option<SharedString>,
        icon_override: Option<IconName>,
        local_workspace: bool,
    ) -> Option<impl IntoElement> {
        self.visibility_guard.is_visible(local_workspace).then(|| {
            let button = SectionButton::new(
                self.title,
                icon_override.unwrap_or(self.icon),
                self.action,
                button_index,
                focus.clone(),
                primary,
            )
            .show_meta(show_meta);
            match meta_override.or_else(|| self.meta.map(SharedString::new_static)) {
                Some(meta) => button.meta(meta),
                None => button,
            }
        })
    }
}

const NEW_CENTER_TERMINAL: crate::NewCenterTerminal = crate::NewCenterTerminal {
    local: false,
    startup_command: None,
    working_directory: None,
};
const OPEN_AGENT_TERMINAL: zed_actions::terminal::OpenAgentTerminal =
    zed_actions::terminal::OpenAgentTerminal;
const OPEN_TMUX_TERMINAL: zed_actions::terminal::OpenTmuxTerminal =
    zed_actions::terminal::OpenTmuxTerminal;
const OPEN_CODEX_TERMINAL: zed_actions::terminal::OpenCodexTerminal =
    zed_actions::terminal::OpenCodexTerminal;
const OPEN_CLAUDE_CODE_TERMINAL: zed_actions::terminal::OpenClaudeCodeTerminal =
    zed_actions::terminal::OpenClaudeCodeTerminal;
const OPEN_OPEN_CODE_TERMINAL: zed_actions::terminal::OpenOpenCodeTerminal =
    zed_actions::terminal::OpenOpenCodeTerminal;
const OPEN_WORKSPACE_IN_CMUX: zed_actions::dez::OpenWorkspaceInCmux =
    zed_actions::dez::OpenWorkspaceInCmux;
const BROWSE_RUNNING_SESSIONS: BrowseRunningSessions = BrowseRunningSessions;
const OPEN_WORKSPACE: OpenFolder = OpenFolder {
    create_new_window: Some(false),
};
const REVEAL_FILES: RevealFiles = RevealFiles;
const REVEAL_GIT_CHANGES: RevealGitChanges = RevealGitChanges;

fn welcome_summary(app_name: &str, has_workspace: bool) -> &'static str {
    if app_name == "Zed" {
        "Write. Delegate. Watch. Verify."
    } else if has_workspace {
        "Start a terminal or agent in this Workspace, then inspect the code and verify the diff."
    } else {
        "Open a Workspace, run an agent, and review changes in one place."
    }
}

fn welcome_title(app_name: &str, _has_workspace: bool) -> &'static str {
    if app_name == "Zed" {
        "Terminal-native development"
    } else {
        "Continue your work"
    }
}

fn welcome_surface_label(app_name: &str) -> &'static str {
    if app_name == "Zed" { "Welcome" } else { "Home" }
}

fn welcome_identity_label(app_name: &str) -> Option<&'static str> {
    (app_name != "Zed").then_some("Dez")
}

fn welcome_forces_tab_bar(app_name: &str) -> bool {
    app_name != "Zed"
}

fn welcome_tab_icon(app_name: &str) -> Option<IconName> {
    (app_name != "Zed").then_some(IconName::Compass)
}

fn welcome_emphasizes_first_action(app_name: &str) -> bool {
    app_name != "Zed"
}

fn welcome_terminal_action_meta(
    app_name: &str,
    has_workspace: bool,
    configured_command: Option<&str>,
) -> Option<String> {
    (app_name != "Zed" && has_workspace)
        .then(|| configured_terminal_launcher_label(configured_command))
}

fn welcome_terminal_action_icon(
    app_name: &str,
    has_workspace: bool,
    configured_command: Option<&str>,
) -> Option<IconName> {
    (app_name != "Zed" && has_workspace)
        .then(|| configured_terminal_launcher_icon(configured_command))
}

const ZED_CONTENT: (Section, Section) = (
    Section {
        title: "Start Working",
        entries: &[
            SectionEntry {
                icon: IconName::Terminal,
                title: "New Terminal",
                meta: None,
                action: &NEW_CENTER_TERMINAL,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::FolderOpen,
                title: "Open Workspace",
                meta: None,
                action: &Open::DEFAULT,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::CloudDownload,
                title: "Clone Repository",
                meta: None,
                action: &GitClone,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::Plus,
                title: "New File",
                meta: None,
                action: &NewFile,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::ListCollapse,
                title: "Open Command Palette",
                meta: None,
                action: &command_palette::Toggle,
                visibility_guard: SectionVisibility::Always,
            },
        ],
    },
    Section {
        title: "Personalize",
        entries: &[
            SectionEntry {
                icon: IconName::Settings,
                title: "Open Settings",
                meta: None,
                action: &OpenSettings,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::Keyboard,
                title: "Keyboard Shortcuts",
                meta: None,
                action: &OpenKeymap,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::Blocks,
                title: "Explore Extensions",
                meta: None,
                action: &Extensions {
                    category_filter: None,
                    id: None,
                },
                visibility_guard: SectionVisibility::Always,
            },
        ],
    },
);

const DEZ_CONTENT: (Section, Section) = (
    Section {
        title: "Start a Workspace",
        entries: &[
            SectionEntry {
                icon: IconName::FolderOpen,
                title: "Open Workspace",
                meta: Some("Local folder"),
                action: &OPEN_WORKSPACE,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::CloudDownload,
                title: "Clone Repository",
                meta: Some("From Git"),
                action: &GitClone,
                visibility_guard: SectionVisibility::Always,
            },
        ],
    },
    Section {
        title: "",
        entries: &[],
    },
);

const DEZ_WORKSPACE_CONTENT: (Section, Section) = (
    Section {
        title: "Start with a tool",
        entries: &[
            SectionEntry {
                icon: IconName::Terminal,
                title: "Open Terminal",
                meta: Some("Default terminal"),
                action: &OPEN_AGENT_TERMINAL,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::AiOpenAi,
                title: "Codex",
                meta: Some("Agent CLI"),
                action: &OPEN_CODEX_TERMINAL,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::AiClaude,
                title: "Claude Code",
                meta: Some("Agent CLI"),
                action: &OPEN_CLAUDE_CODE_TERMINAL,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::AiOpenCode,
                title: "OpenCode",
                meta: Some("Agent CLI"),
                action: &OPEN_OPEN_CODE_TERMINAL,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::SplitAlt,
                title: "Workspace tmux",
                meta: Some("Native terminal session"),
                action: &OPEN_TMUX_TERMINAL,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::ArrowUpRight,
                title: "Open Workspace in cmux",
                meta: Some("External handoff"),
                action: &OPEN_WORKSPACE_IN_CMUX,
                visibility_guard: SectionVisibility::LocalWorkspace,
            },
        ],
    },
    Section {
        title: "Inspect and resume",
        entries: &[
            SectionEntry {
                icon: IconName::ListTree,
                title: "Browse Running Sessions…",
                meta: Some("tmux, Herdr, and cmux"),
                action: &BROWSE_RUNNING_SESSIONS,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::FolderOpen,
                title: "Open Files",
                meta: Some("Inspect code"),
                action: &REVEAL_FILES,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::Diff,
                title: "Review Changes",
                meta: Some("Verify the diff"),
                action: &REVEAL_GIT_CHANGES,
                visibility_guard: SectionVisibility::Always,
            },
        ],
    },
);

#[derive(Clone, Copy)]
struct Section {
    title: &'static str,
    entries: &'static [SectionEntry],
}

impl Section {
    fn visible_entry_count(self, local_workspace: bool) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.visibility_guard.is_visible(local_workspace))
            .count()
    }

    fn render(
        self,
        index_offset: usize,
        focus: &FocusHandle,
        emphasize_first: bool,
        show_meta: bool,
        first_entry_meta_override: Option<SharedString>,
        first_entry_icon_override: Option<IconName>,
        local_workspace: bool,
    ) -> impl IntoElement {
        v_flex()
            .w_full()
            .min_w_0()
            .gap_0p5()
            .child(SectionHeader::new(self.title))
            .children(
                self.entries
                    .iter()
                    .enumerate()
                    .filter_map(|(index, entry)| {
                        entry.render(
                            index_offset + index,
                            focus,
                            emphasize_first && index == 0,
                            show_meta,
                            (index == 0)
                                .then(|| first_entry_meta_override.clone())
                                .flatten(),
                            (index == 0).then_some(first_entry_icon_override).flatten(),
                            local_workspace,
                        )
                    }),
            )
    }
}

pub struct WelcomePage {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    fallback_to_recent_projects: bool,
    recent_workspaces: Option<Vec<RecentWorkspace>>,
    recent_workspaces_load_failed: bool,
    recent_workspaces_load_generation: u64,
}

const DEZ_WELCOME_COMPACT_BREAKPOINT: Pixels = px(760.);
const DEZ_WELCOME_SPLIT_BREAKPOINT: Pixels = px(980.);
const DEZ_RECENT_WORKSPACES_LOAD_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WelcomeRecentState {
    Hidden,
    Loading,
    Unavailable,
    Empty,
    Ready,
}

fn welcome_recent_state(
    app_name: &str,
    fallback_to_recent_projects: bool,
    recent_workspaces_loaded: bool,
    recent_workspaces_load_failed: bool,
    recent_workspace_count: usize,
) -> WelcomeRecentState {
    if !fallback_to_recent_projects {
        WelcomeRecentState::Hidden
    } else if recent_workspace_count > 0 {
        WelcomeRecentState::Ready
    } else if app_name == "Zed" {
        WelcomeRecentState::Hidden
    } else if recent_workspaces_load_failed {
        WelcomeRecentState::Unavailable
    } else if recent_workspaces_loaded {
        WelcomeRecentState::Empty
    } else {
        WelcomeRecentState::Loading
    }
}

fn welcome_loads_recent_workspaces(app_name: &str, requested: bool) -> bool {
    app_name != "Zed" || requested
}

fn dez_welcome_uses_compact_spacing(app_name: &str, viewport_width: Pixels) -> bool {
    app_name != "Zed" && viewport_width < DEZ_WELCOME_COMPACT_BREAKPOINT
}

fn dez_welcome_uses_split_layout(
    app_name: &str,
    viewport_width: Pixels,
    has_secondary_content: bool,
) -> bool {
    app_name != "Zed" && has_secondary_content && viewport_width >= DEZ_WELCOME_SPLIT_BREAKPOINT
}

impl WelcomePage {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        fallback_to_recent_projects: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let fallback_to_recent_projects =
            welcome_loads_recent_workspaces(APP_NAME, fallback_to_recent_projects);
        let focus_handle = cx.focus_handle();
        cx.on_focus(&focus_handle, window, |_, _, cx| cx.notify())
            .detach();
        if let Some(workspace) = workspace.upgrade() {
            cx.observe(&workspace, |_, _, cx| cx.notify()).detach();
        }

        let mut welcome_page = WelcomePage {
            workspace,
            focus_handle,
            fallback_to_recent_projects,
            recent_workspaces: None,
            recent_workspaces_load_failed: false,
            recent_workspaces_load_generation: 0,
        };
        if fallback_to_recent_projects {
            welcome_page.refresh_recent_workspaces(window, cx);
        }
        welcome_page
    }

    fn select_next(&mut self, _: &SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_next(cx);
        cx.notify();
    }

    fn select_previous(&mut self, _: &SelectPrevious, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_prev(cx);
        cx.notify();
    }

    fn refresh_recent_workspaces(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.recent_workspaces = None;
        self.recent_workspaces_load_failed = false;
        self.recent_workspaces_load_generation =
            self.recent_workspaces_load_generation.wrapping_add(1);
        let load_generation = self.recent_workspaces_load_generation;
        cx.notify();

        let db = WorkspaceDb::global(cx);
        cx.spawn_in(window, async move |this: WeakEntity<Self>, cx| {
            let recent_workspaces = futures::FutureExt::fuse(
                cx.background_spawn(async move { db.persisted_recent_project_workspaces() }),
            );
            let timeout = futures::FutureExt::fuse(
                cx.background_executor()
                    .timer(DEZ_RECENT_WORKSPACES_LOAD_TIMEOUT),
            );
            futures::pin_mut!(recent_workspaces, timeout);
            let result = futures::select_biased! {
                result = recent_workspaces => result,
                _ = timeout => Err(anyhow::anyhow!(
                    "recent Workspace history did not load within {} seconds",
                    DEZ_RECENT_WORKSPACES_LOAD_TIMEOUT.as_secs()
                )),
            };
            this.update(cx, |this, cx| {
                if this.recent_workspaces_load_generation != load_generation {
                    return;
                }

                match result {
                    Ok(workspaces) => {
                        this.recent_workspaces = Some(workspaces);
                    }
                    Err(error) => {
                        log::error!("failed to load recent Workspaces on Home: {error:#}");
                        this.recent_workspaces_load_failed = true;
                    }
                }
                cx.notify();
            })
            .log_err();
        })
        .detach();
    }

    fn open_recent_project(
        &mut self,
        action: &OpenRecentProject,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(recent_workspaces) = &self.recent_workspaces {
            if let Some(workspace) = recent_workspaces.get(action.index) {
                match &workspace.location {
                    SerializedWorkspaceLocation::Local => {
                        let paths = workspace.paths.paths().to_vec();
                        let open_mode =
                            match WorkspaceSettings::get_global(cx).default_open_behavior {
                                DefaultOpenBehavior::ExistingWindow => OpenMode::Activate,
                                DefaultOpenBehavior::NewWindow => OpenMode::NewWindow,
                            };
                        self.workspace
                            .update(cx, |workspace, cx| {
                                workspace
                                    .open_workspace_for_paths(open_mode, paths, window, cx)
                                    .detach_and_log_err(cx);
                            })
                            .log_err();
                    }
                    SerializedWorkspaceLocation::Remote(_) => {
                        window.dispatch_action(
                            open_remote_recent_workspace_action(workspace.workspace_id)
                                .boxed_clone(),
                            cx,
                        );
                    }
                }
            }
        }
    }

    fn render_recent_project_section(
        &self,
        recent_projects: Vec<impl IntoElement>,
    ) -> impl IntoElement {
        v_flex()
            .w_full()
            .child(SectionHeader::new("Recent Workspaces"))
            .children(recent_projects)
    }

    fn render_recent_workspace_status(
        title: &'static str,
        description: &'static str,
        icon: IconName,
    ) -> impl IntoElement {
        v_flex()
            .w_full()
            .child(SectionHeader::new("Recent Workspaces"))
            .child(
                h_flex()
                    .id("recent-workspaces-status")
                    .w_full()
                    .min_w_0()
                    .items_start()
                    .gap_2()
                    .px_1()
                    .py_2()
                    .role(gpui::Role::Status)
                    .aria_label(format!("{title}. {description}"))
                    .child(
                        div()
                            .flex_none()
                            .pt_0p5()
                            .child(Icon::new(icon).size(IconSize::Small).color(Color::Muted)),
                    )
                    .child(
                        v_flex()
                            .min_w_0()
                            .gap_0p5()
                            .child(Label::new(title).size(LabelSize::Small))
                            .child(
                                Label::new(description)
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                            ),
                    ),
            )
    }

    fn render_recent_workspace_error(
        tab_index: usize,
        welcome_page: WeakEntity<Self>,
    ) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_2()
            .child(Self::render_recent_workspace_status(
                "Recent Workspaces unavailable",
                "History could not be read. Retry without leaving Home.",
                IconName::Warning,
            ))
            .child(
                Button::new("retry-recent-workspaces", "Retry")
                    .tab_index(tab_index as isize)
                    .full_width()
                    .label_size(LabelSize::XSmall)
                    .on_click(move |_, window, cx| {
                        welcome_page
                            .update(cx, |welcome_page, cx| {
                                welcome_page.refresh_recent_workspaces(window, cx);
                            })
                            .log_err();
                    }),
            )
    }

    fn render_recent_project(
        &self,
        project_index: usize,
        tab_index: usize,
        location: &SerializedWorkspaceLocation,
        paths: &PathList,
    ) -> impl IntoElement {
        let name = project_name(paths);

        let (icon, title) = match location {
            SerializedWorkspaceLocation::Local => (IconName::Folder, name),
            SerializedWorkspaceLocation::Remote(_) => (IconName::Server, name),
        };

        SectionButton::new(
            title,
            icon,
            &OpenRecentProject {
                index: project_index,
            },
            tab_index,
            self.focus_handle.clone(),
            false,
        )
        .meta(recent_workspace_meta(location, paths))
    }
}

impl Render for WelcomePage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_dez = APP_NAME != "Zed";
        let (has_workspace, local_workspace) = self
            .workspace
            .upgrade()
            .map(|workspace| {
                let workspace = workspace.read(cx);
                let has_workspace = workspace.worktrees(cx).next().is_some();
                let local_workspace =
                    has_workspace && workspace.project_group_key(cx).host().is_none();
                (has_workspace, local_workspace)
            })
            .unwrap_or((false, false));
        let workspace_startup_state = is_dez.then(|| crate::workspace_startup_state(cx));
        let installation_required = matches!(
            &workspace_startup_state,
            Some(crate::WorkspaceStartupState::InstallationRequired { .. })
        );
        let installation_action_count = usize::from(installation_required);
        let action_tab_offset = installation_action_count;
        let (first_section, second_section) = if APP_NAME == "Zed" {
            ZED_CONTENT
        } else if has_workspace {
            DEZ_WORKSPACE_CONTENT
        } else {
            DEZ_CONTENT
        };
        let first_section_entries = first_section.visible_entry_count(local_workspace);
        let second_section_entries = second_section.visible_entry_count(local_workspace);
        let welcome_page = cx.weak_entity();

        let recent_projects = self
            .recent_workspaces
            .as_ref()
            .into_iter()
            .flatten()
            .take(5)
            .enumerate()
            .map(|(index, workspace)| {
                self.render_recent_project(
                    index,
                    action_tab_offset + first_section_entries + index,
                    &workspace.location,
                    &workspace.identity_paths,
                )
            })
            .collect::<Vec<_>>();

        let recent_state = if installation_action_count > 0 {
            WelcomeRecentState::Hidden
        } else {
            welcome_recent_state(
                APP_NAME,
                self.fallback_to_recent_projects,
                self.recent_workspaces.is_some(),
                self.recent_workspaces_load_failed,
                recent_projects.len(),
            )
        };
        let recent_action_count = match recent_state {
            WelcomeRecentState::Ready => recent_projects.len(),
            WelcomeRecentState::Unavailable => 1,
            WelcomeRecentState::Hidden
            | WelcomeRecentState::Loading
            | WelcomeRecentState::Empty => 0,
        };
        let next_tab_index = first_section_entries + recent_action_count + second_section_entries;
        let recent_content = match recent_state {
            WelcomeRecentState::Ready => Some(
                self.render_recent_project_section(recent_projects)
                    .into_any_element(),
            ),
            WelcomeRecentState::Loading => Some(
                Self::render_recent_workspace_status(
                    "Loading recent Workspaces",
                    "Reading your local Workspace history.",
                    IconName::Clock,
                )
                .into_any_element(),
            ),
            WelcomeRecentState::Unavailable => Some(
                Self::render_recent_workspace_error(
                    action_tab_offset + first_section_entries,
                    welcome_page,
                )
                .into_any_element(),
            ),
            WelcomeRecentState::Empty => Some(
                Self::render_recent_workspace_status(
                    "No recent Workspaces",
                    "Open a Workspace and it will appear here.",
                    IconName::Folder,
                )
                .into_any_element(),
            ),
            WelcomeRecentState::Hidden => None,
        };
        let workspace_content = (second_section_entries > 0).then(|| {
            second_section
                .render(
                    action_tab_offset + first_section_entries + recent_action_count,
                    &self.focus_handle,
                    false,
                    true,
                    None,
                    None,
                    local_workspace,
                )
                .into_any_element()
        });
        let secondary_content = if installation_required {
            None
        } else if is_dez {
            match (recent_content, workspace_content) {
                (Some(recent), Some(workspace)) => Some(
                    v_flex()
                        .w_full()
                        .min_w_0()
                        .gap_4()
                        .child(recent)
                        .child(Divider::horizontal().color(DividerColor::BorderVariant))
                        .child(workspace)
                        .into_any_element(),
                ),
                (Some(recent), None) => Some(recent),
                (None, Some(workspace)) => Some(workspace),
                (None, None) => None,
            }
        } else {
            recent_content.or(workspace_content)
        };
        let has_secondary_content = secondary_content.is_some();

        let welcome_label = if is_dez {
            "Dez Home".to_string()
        } else if self.fallback_to_recent_projects {
            format!("Welcome back to {APP_NAME}")
        } else {
            format!("Welcome to {APP_NAME}")
        };
        let welcome_background = if is_dez && theme_is_transparent(cx) {
            gpui::transparent_black()
        } else {
            cx.theme().colors().editor_background
        };
        let installation_notice = match workspace_startup_state {
            Some(crate::WorkspaceStartupState::InstallationRequired { message }) => Some(
                Callout::new()
                    .severity(Severity::Warning)
                    .icon(IconName::FolderOpen)
                    .title("Install Dez to continue")
                    .description(message)
                    .actions_slot(
                        Button::new("home-install-and-relaunch", "Install and Relaunch")
                            .style(ui::ButtonStyle::Filled)
                            .tab_index(0isize)
                            .aria_label("Install Dez in Applications and Relaunch")
                            .on_click(|_, window, cx| {
                                window.dispatch_action(
                                    zed_actions::dez::InstallAndRelaunch.boxed_clone(),
                                    cx,
                                );
                            }),
                    )
                    .into_any_element(),
            ),
            Some(crate::WorkspaceStartupState::Ready) | None => None,
        };
        let section_focus_handle = self.focus_handle.clone();
        let first_entry_meta_override = welcome_terminal_action_meta(
            APP_NAME,
            has_workspace,
            AgentSettings::get_global(cx)
                .terminal_init_command
                .as_deref(),
        )
        .map(SharedString::from);
        let first_entry_icon_override = welcome_terminal_action_icon(
            APP_NAME,
            has_workspace,
            AgentSettings::get_global(cx)
                .terminal_init_command
                .as_deref(),
        );
        let show_onboarding_return = APP_NAME == "Zed" && !self.fallback_to_recent_projects;
        let content_welcome_label = welcome_label.clone();
        let page_title = welcome_title(APP_NAME, has_workspace);
        let page_summary = welcome_summary(APP_NAME, has_workspace);

        h_flex()
            .id("welcome-page")
            .role(gpui::Role::Region)
            .aria_label(welcome_label.clone())
            .key_context("Welcome")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::open_recent_project))
            .size_full()
            .bg(welcome_background)
            .justify_center()
            .when(is_dez, |this| this.items_start())
            .child(container_query(move |available_size, _window, cx| {
                let compact_spacing =
                    dez_welcome_uses_compact_spacing(APP_NAME, available_size.width);
                let split_layout = dez_welcome_uses_split_layout(
                    APP_NAME,
                    available_size.width,
                    has_secondary_content,
                );
                let home_separator = cx.theme().colors().border_variant;
                let mut secondary_content = secondary_content;
                let sections = if split_layout {
                    h_flex()
                        .id("welcome-sections")
                        .w_full()
                        .min_w_0()
                        .items_start()
                        .gap_6()
                        .child(
                            div()
                                .min_w_0()
                                .flex_1()
                                .when(is_dez, |this| this.px_1())
                                .child(first_section.render(
                                    action_tab_offset,
                                    &section_focus_handle,
                                    welcome_emphasizes_first_action(APP_NAME),
                                    true,
                                    first_entry_meta_override.clone(),
                                    first_entry_icon_override,
                                    local_workspace,
                                )),
                        )
                        .when_some(secondary_content.take(), |this, secondary_content| {
                            this.child(
                                div()
                                    .min_w_0()
                                    .flex_1()
                                    .when(is_dez, |this| {
                                        this.pl_6().border_l_1().border_color(home_separator)
                                    })
                                    .child(secondary_content),
                            )
                        })
                        .into_any_element()
                } else {
                    v_flex()
                        .id("welcome-sections")
                        .w_full()
                        .min_w_0()
                        .gap_4()
                        .when(is_dez && !has_secondary_content, |this| {
                            this.max_w(rems_from_px(520.))
                        })
                        .child(first_section.render(
                            action_tab_offset,
                            &section_focus_handle,
                            welcome_emphasizes_first_action(APP_NAME),
                            !compact_spacing,
                            first_entry_meta_override.clone(),
                            first_entry_icon_override,
                            local_workspace,
                        ))
                        .when_some(secondary_content.take(), |this, secondary_content| {
                            this.child(Divider::horizontal().color(DividerColor::BorderVariant))
                                .child(secondary_content)
                        })
                        .into_any_element()
                };

                h_flex().size_full().items_start().justify_center().child(
                    v_flex()
                        .id("welcome-content")
                        .w_full()
                        .h_full()
                        .max_w(rems_from_px(if APP_NAME == "Zed" { 640. } else { 1120. }))
                        .when(APP_NAME == "Zed", |this| this.p_6().gap_5())
                        .when(is_dez && compact_spacing, |this| this.px_3().py_4().gap_4())
                        .when(is_dez && !compact_spacing, |this| {
                            this.px_6().py_6().gap_5()
                        })
                        .overflow_y_scroll()
                        .child(if APP_NAME == "Zed" {
                            h_flex()
                                .w_full()
                                .items_center()
                                .gap_3()
                                .child(
                                    div().flex_none().child(
                                        Icon::new(IconName::Terminal)
                                            .size(IconSize::Medium)
                                            .color(Color::Accent),
                                    ),
                                )
                                .child(
                                    v_flex()
                                        .min_w_0()
                                        .gap_0p5()
                                        .child(
                                            Label::new(page_title)
                                                .size(LabelSize::XSmall)
                                                .color(Color::Muted),
                                        )
                                        .child(Headline::new(content_welcome_label.clone()))
                                        .child(
                                            Label::new(page_summary)
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        ),
                                )
                                .into_any_element()
                        } else {
                            v_flex()
                                .w_full()
                                .min_w_0()
                                .gap_1()
                                .when_some(welcome_identity_label(APP_NAME), |this, identity| {
                                    this.child(
                                        h_flex()
                                            .gap_1()
                                            .child(
                                                Icon::new(IconName::Compass)
                                                    .size(IconSize::XSmall)
                                                    .color(Color::Accent),
                                            )
                                            .child(
                                                Label::new(identity)
                                                    .size(LabelSize::XSmall)
                                                    .color(Color::Muted)
                                                    .weight(FontWeight::MEDIUM),
                                            ),
                                    )
                                })
                                .child(
                                    div()
                                        .font_weight(FontWeight::MEDIUM)
                                        .child(Headline::new(page_title).size(HeadlineSize::Large)),
                                )
                                .when(!installation_required, |this| {
                                    this.child(
                                        Label::new(page_summary)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                })
                                .into_any_element()
                        })
                        .when_some(installation_notice, |this, notice| this.child(notice))
                        .when(!installation_required, |this| this.child(sections))
                        .when(show_onboarding_return, |this| {
                            this.child(
                                v_flex().gap_4().child(Divider::horizontal()).child(
                                    Button::new("welcome-exit", "Return to Onboarding")
                                        .tab_index((action_tab_offset + next_tab_index) as isize)
                                        .full_width()
                                        .label_size(LabelSize::XSmall)
                                        .on_click(|_, window, cx| {
                                            window
                                                .dispatch_action(OpenOnboarding.boxed_clone(), cx);
                                        }),
                                ),
                            )
                        }),
                )
            }))
    }
}

impl EventEmitter<ItemEvent> for WelcomePage {}

impl Focusable for WelcomePage {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for WelcomePage {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        welcome_surface_label(APP_NAME).into()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        welcome_tab_icon(APP_NAME).map(Icon::new)
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some(if APP_NAME == "Zed" {
            "New Welcome Page Opened"
        } else {
            "Home Page Opened"
        })
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn force_show_tab_bar(&self) -> bool {
        welcome_forces_tab_bar(APP_NAME)
    }

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(crate::item::ItemEvent)) {
        f(*event)
    }
}

impl crate::SerializableItem for WelcomePage {
    fn serialized_item_kind() -> &'static str {
        "WelcomePage"
    }

    fn cleanup(
        workspace_id: crate::WorkspaceId,
        alive_items: Vec<crate::ItemId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<gpui::Result<()>> {
        crate::delete_unloaded_items(
            alive_items,
            workspace_id,
            "welcome_pages",
            &persistence::WelcomePagesDb::global(cx),
            cx,
        )
    }

    fn deserialize(
        _project: Entity<project::Project>,
        workspace: gpui::WeakEntity<Workspace>,
        workspace_id: crate::WorkspaceId,
        item_id: crate::ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<gpui::Result<Entity<Self>>> {
        if persistence::WelcomePagesDb::global(cx)
            .get_welcome_page(item_id, workspace_id)
            .ok()
            .is_some_and(|is_open| is_open)
        {
            Task::ready(Ok(
                cx.new(|cx| WelcomePage::new(workspace, false, window, cx))
            ))
        } else {
            Task::ready(Err(anyhow::anyhow!("No welcome page to deserialize")))
        }
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: crate::ItemId,
        _closing: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<gpui::Result<()>>> {
        let workspace_id = workspace.database_id()?;
        let db = persistence::WelcomePagesDb::global(cx);
        Some(cx.background_spawn(
            async move { db.save_welcome_page(item_id, workspace_id, true).await },
        ))
    }

    fn should_serialize(&self, event: &Self::Event) -> bool {
        event == &ItemEvent::UpdateTab
    }
}

mod persistence {
    use crate::WorkspaceDb;
    use db::{
        query,
        sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
        sqlez_macros::sql,
    };

    pub struct WelcomePagesDb(ThreadSafeConnection);

    impl Domain for WelcomePagesDb {
        const NAME: &str = stringify!(WelcomePagesDb);

        const MIGRATIONS: &[&str] = (&[sql!(
                    CREATE TABLE welcome_pages (
                        workspace_id INTEGER,
                        item_id INTEGER UNIQUE,
                        is_open INTEGER DEFAULT FALSE,

                        PRIMARY KEY(workspace_id, item_id),
                        FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                        ON DELETE CASCADE
                    ) STRICT;
        )]);
    }

    db::static_connection!(WelcomePagesDb, [WorkspaceDb]);

    impl WelcomePagesDb {
        query! {
            pub async fn save_welcome_page(
                item_id: crate::ItemId,
                workspace_id: crate::WorkspaceId,
                is_open: bool
            ) -> Result<()> {
                INSERT OR REPLACE INTO welcome_pages(item_id, workspace_id, is_open)
                VALUES (?, ?, ?)
            }
        }

        query! {
            pub fn get_welcome_page(
                item_id: crate::ItemId,
                workspace_id: crate::WorkspaceId
            ) -> Result<bool> {
                SELECT is_open
                FROM welcome_pages
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}

fn project_name(paths: &PathList) -> String {
    let names = paths
        .ordered_paths()
        .filter_map(|path| {
            path.file_name()
                .map(|name| name.to_string_lossy().into_owned())
        })
        .collect::<Vec<_>>();
    match names.as_slice() {
        [] => "Untitled".to_owned(),
        [name] => name.clone(),
        [primary, ..] => format!("{primary} · {} roots", names.len()),
    }
}

fn recent_workspace_meta(location: &SerializedWorkspaceLocation, paths: &PathList) -> SharedString {
    let compact_paths = paths
        .ordered_paths()
        .map(|path| path.compact().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    let path_summary = if compact_paths.is_empty() {
        "Location unavailable".to_owned()
    } else {
        compact_paths.join(" · ")
    };

    match location {
        SerializedWorkspaceLocation::Local => path_summary.into(),
        SerializedWorkspaceLocation::Remote(options) => {
            format!("{} · {path_summary}", options.display_name()).into()
        }
    }
}

fn open_remote_recent_workspace_action(workspace_id: WorkspaceId) -> OpenRecent {
    OpenRecent {
        create_new_window: None,
        workspace_id: Some(workspace_id.to_i64()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_name_empty() {
        let paths = PathList::new::<&str>(&[]);
        assert_eq!(project_name(&paths), "Untitled");
    }

    #[test]
    fn test_project_name_single() {
        let paths = PathList::new(&["/home/user/my-project"]);
        assert_eq!(project_name(&paths), "my-project");
    }

    #[test]
    fn test_project_name_multiple() {
        let paths = PathList::new(&["/home/user/zed", "/home/user/api"]);
        assert_eq!(project_name(&paths), "zed · 2 roots");
    }

    #[test]
    fn test_project_name_root_path_filtered() {
        // A bare root "/" has no file_name(), falls back to "Untitled"
        let paths = PathList::new(&["/"]);
        assert_eq!(project_name(&paths), "Untitled");
    }

    #[test]
    fn recent_workspace_meta_disambiguates_local_roots() {
        let single = PathList::new(&["/home/user/my-project"]);
        assert!(
            recent_workspace_meta(&SerializedWorkspaceLocation::Local, &single)
                .ends_with("/home/user/my-project")
        );

        let multiple = PathList::new(&["/home/user/api", "/home/user/web"]);
        let meta = recent_workspace_meta(&SerializedWorkspaceLocation::Local, &multiple);
        assert!(meta.contains("/home/user/api"));
        assert!(meta.contains("/home/user/web"));
    }

    #[test]
    fn remote_recent_workspace_action_preserves_the_exact_target() {
        let action = open_remote_recent_workspace_action(WorkspaceId::from_i64(42));
        assert_eq!(action.workspace_id, Some(42));
        assert_eq!(action.create_new_window, None);
    }

    #[test]
    fn dez_home_states_the_workflow_without_a_persistent_walkthrough() {
        assert_eq!(
            welcome_summary("Dez", false),
            "Open a Workspace, run an agent, and review changes in one place."
        );
        assert_eq!(
            welcome_summary("Dez", true),
            "Start a terminal or agent in this Workspace, then inspect the code and verify the diff."
        );
        assert_eq!(
            welcome_summary("Zed", true),
            "Write. Delegate. Watch. Verify."
        );
        assert_eq!(welcome_title("Dez", false), "Continue your work");
        assert_eq!(welcome_title("Dez", true), "Continue your work");
        assert_eq!(welcome_identity_label("Dez"), Some("Dez"));
        assert_eq!(welcome_identity_label("Zed"), None);
        assert_eq!(welcome_title("Zed", false), "Terminal-native development");
        assert_eq!(welcome_surface_label("Dez"), "Home");
        assert_eq!(welcome_surface_label("Zed"), "Welcome");
        assert!(welcome_forces_tab_bar("Dez"));
        assert!(!welcome_forces_tab_bar("Zed"));
        assert_eq!(welcome_tab_icon("Dez"), Some(IconName::Compass));
        assert_eq!(welcome_tab_icon("Zed"), None);
        assert_eq!(DEZ_CONTENT.0.entries[0].title, "Open Workspace");
        assert_eq!(DEZ_CONTENT.0.entries[0].meta, Some("Local folder"));
        assert_eq!(DEZ_CONTENT.0.entries[1].title, "Clone Repository");
        assert_eq!(DEZ_CONTENT.0.entries[1].meta, Some("From Git"));
        assert_eq!(
            DEZ_CONTENT.0.entries.len(),
            2,
            "the empty Dez window must not offer a pathless agent-terminal dead end"
        );
        assert_eq!(DEZ_WORKSPACE_CONTENT.0.entries[0].title, "Open Terminal");
        assert_eq!(
            DEZ_WORKSPACE_CONTENT.0.entries[0].meta,
            Some("Default terminal")
        );
        assert_eq!(
            welcome_terminal_action_meta("Dez", true, None).as_deref(),
            Some("Default · Native Shell")
        );
        assert_eq!(
            welcome_terminal_action_meta("Dez", true, Some("codex --yolo")).as_deref(),
            Some("Default · Codex")
        );
        assert_eq!(
            welcome_terminal_action_meta("Dez", true, Some("aider")).as_deref(),
            Some("Default · Aider")
        );
        assert_eq!(
            welcome_terminal_action_meta("Dez", true, Some("tmux")).as_deref(),
            Some("Default · tmux Session")
        );
        assert_eq!(
            welcome_terminal_action_meta("Dez", true, Some("my-agent")).as_deref(),
            Some("Default · Custom Command")
        );
        assert_eq!(
            welcome_terminal_action_meta("Dez", false, Some("codex")),
            None
        );
        assert_eq!(
            welcome_terminal_action_meta("Zed", true, Some("codex")),
            None
        );
        assert_eq!(
            welcome_terminal_action_icon("Dez", true, None),
            Some(IconName::Terminal)
        );
        assert_eq!(
            welcome_terminal_action_icon("Dez", true, Some("codex --yolo")),
            Some(IconName::AiOpenAi)
        );
        assert_eq!(
            welcome_terminal_action_icon("Dez", true, Some("claude")),
            Some(IconName::AiClaude)
        );
        assert_eq!(
            welcome_terminal_action_icon("Dez", true, Some("tmux")),
            Some(IconName::SplitAlt)
        );
        assert_eq!(
            welcome_terminal_action_icon("Dez", false, Some("codex")),
            None
        );
        assert_eq!(
            welcome_terminal_action_icon("Zed", true, Some("codex")),
            None
        );
        assert_eq!(DEZ_WORKSPACE_CONTENT.0.title, "Start with a tool");
        assert_eq!(DEZ_WORKSPACE_CONTENT.0.entries[1].title, "Codex");
        assert_eq!(DEZ_WORKSPACE_CONTENT.0.entries[2].title, "Claude Code");
        assert_eq!(DEZ_WORKSPACE_CONTENT.0.entries[3].title, "OpenCode");
        assert_eq!(DEZ_WORKSPACE_CONTENT.0.entries[4].title, "Workspace tmux");
        assert_eq!(
            DEZ_WORKSPACE_CONTENT.0.entries[5].title,
            "Open Workspace in cmux"
        );
        assert_eq!(DEZ_WORKSPACE_CONTENT.0.visible_entry_count(true), 6);
        assert_eq!(DEZ_WORKSPACE_CONTENT.0.visible_entry_count(false), 5);
        assert_eq!(DEZ_WORKSPACE_CONTENT.1.title, "Inspect and resume");
        assert_eq!(
            DEZ_WORKSPACE_CONTENT.1.entries[0].title,
            "Browse Running Sessions…"
        );
        assert_eq!(DEZ_WORKSPACE_CONTENT.1.entries[1].title, "Open Files");
        assert_eq!(DEZ_WORKSPACE_CONTENT.1.entries[2].title, "Review Changes");
        assert!(
            DEZ_CONTENT.1.entries.is_empty(),
            "Dez Welcome should leave configuration to normal application navigation"
        );
        assert_eq!(DEZ_WORKSPACE_CONTENT.1.entries.len(), 3);
        assert!(
            !ZED_CONTENT.1.entries.is_empty(),
            "official Zed retains its inherited Personalize section"
        );
        assert_eq!(ZED_CONTENT.0.entries[0].title, "New Terminal");
        assert_eq!(OPEN_WORKSPACE.create_new_window, Some(false));
        assert!(welcome_emphasizes_first_action("Dez"));
        assert!(!welcome_emphasizes_first_action("Zed"));
    }

    #[test]
    fn dez_welcome_layout_uses_space_without_becoming_a_dashboard() {
        assert!(dez_welcome_uses_compact_spacing("Dez", px(600.)));
        assert!(!dez_welcome_uses_compact_spacing("Dez", px(760.)));
        assert!(!dez_welcome_uses_compact_spacing("Zed", px(600.)));

        assert!(!dez_welcome_uses_split_layout("Dez", px(979.), true));
        assert!(dez_welcome_uses_split_layout("Dez", px(980.), true));
        assert!(!dez_welcome_uses_split_layout("Dez", px(1400.), false));
        assert!(!dez_welcome_uses_split_layout("Zed", px(1400.), true));
        assert!(welcome_loads_recent_workspaces("Dez", false));
        assert!(welcome_loads_recent_workspaces("Dez", true));
        assert!(welcome_loads_recent_workspaces("Zed", true));
        assert!(!welcome_loads_recent_workspaces("Zed", false));

        assert_eq!(
            welcome_recent_state("Dez", true, false, false, 0),
            WelcomeRecentState::Loading
        );
        assert_eq!(
            welcome_recent_state("Dez", true, true, false, 0),
            WelcomeRecentState::Empty
        );
        assert_eq!(
            welcome_recent_state("Dez", true, false, true, 0),
            WelcomeRecentState::Unavailable
        );
        assert_eq!(
            welcome_recent_state("Dez", true, true, false, 2),
            WelcomeRecentState::Ready
        );
        assert_eq!(
            welcome_recent_state("Dez", false, true, true, 2),
            WelcomeRecentState::Hidden
        );
        assert_eq!(
            welcome_recent_state("Zed", true, false, true, 0),
            WelcomeRecentState::Hidden
        );
    }
}
