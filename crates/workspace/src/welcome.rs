use crate::{
    NewCenterTerminal, NewFile, Open, OpenFolder, OpenMode, PathList, RecentWorkspace, RevealFiles,
    SerializedWorkspaceLocation, Workspace, WorkspaceId, WorkspaceSettings,
    item::{Item, ItemEvent},
    persistence::WorkspaceDb,
};
use git::Clone as GitClone;
use gpui::WeakEntity;
use gpui::{
    Action, App, Context, Entity, EventEmitter, FocusHandle, Focusable, FontWeight,
    InteractiveElement, ParentElement, Pixels, Render, Styled, Task, TaskExt, Window, actions, px,
};
use menu::{SelectNext, SelectPrevious};
use paths::APP_NAME;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{DefaultOpenBehavior, Settings};
use ui::{
    ButtonLike, Divider, DividerColor, KeyBinding, Tooltip, prelude::*, theme_is_transparent,
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
        h_flex()
            .w_full()
            .min_w_0()
            .px_1()
            .mb_1()
            .gap_2()
            .child(
                div().flex_none().child(
                    Label::new(self.title.to_ascii_uppercase())
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
        }
    }

    fn meta(mut self, meta: impl Into<SharedString>) -> Self {
        self.meta = Some(meta.into());
        self
    }
}

impl RenderOnce for SectionButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let id = format!("home-action-{}-{}", self.label, self.tab_index);
        let action_ref: &dyn Action = &*self.action;
        let meta = self.meta.clone();
        let icon_color = if self.primary {
            Color::Accent
        } else {
            Color::Muted
        };

        ButtonLike::new(id)
            .tab_index(self.tab_index as isize)
            .aria_label(self.label.clone())
            .when(APP_NAME != "Zed", |this| this.style(ButtonStyle::Subtle))
            .when(self.primary, |this| {
                this.style(ButtonStyle::Filled)
                    .aria_description("Recommended first step")
            })
            .when_some(meta.clone(), |this, meta| {
                this.aria_description(meta.clone())
                    .tooltip(Tooltip::text(meta))
            })
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
                            .when_some(meta, |this, meta| {
                                this.child(
                                    div().max_w(rems_from_px(220.)).overflow_hidden().child(
                                        Label::new(meta)
                                            .truncate()
                                            .size(LabelSize::XSmall)
                                            .color(Color::Muted),
                                    ),
                                )
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
}

impl SectionVisibility {
    fn is_visible(&self) -> bool {
        match self {
            SectionVisibility::Always => true,
        }
    }
}

struct SectionEntry {
    icon: IconName,
    title: &'static str,
    action: &'static dyn Action,
    visibility_guard: SectionVisibility,
}

impl SectionEntry {
    fn render(
        &self,
        button_index: usize,
        focus: &FocusHandle,
        primary: bool,
    ) -> Option<impl IntoElement> {
        self.visibility_guard.is_visible().then(|| {
            SectionButton::new(
                self.title,
                self.icon,
                self.action,
                button_index,
                focus.clone(),
                primary,
            )
        })
    }
}

const NEW_CENTER_TERMINAL: NewCenterTerminal = NewCenterTerminal {
    local: false,
    startup_command: None,
};
const OPEN_WORKSPACE: OpenFolder = OpenFolder {
    create_new_window: Some(false),
};
const REVEAL_FILES: RevealFiles = RevealFiles;

fn welcome_summary(app_name: &str, has_workspace: bool) -> &'static str {
    if app_name == "Zed" {
        "Write. Delegate. Watch. Verify."
    } else if has_workspace {
        "Run an agent in a native terminal, follow it in Workspaces, and review its files and Git changes here."
    } else {
        "Open a codebase to run agents in native terminals and review their work without leaving the Workspace."
    }
}

fn welcome_title(app_name: &str, has_workspace: bool) -> &'static str {
    if app_name == "Zed" {
        "Terminal-native development"
    } else if has_workspace {
        "This Workspace"
    } else {
        "Open a Workspace"
    }
}

fn welcome_surface_label(app_name: &str) -> &'static str {
    if app_name == "Zed" { "Welcome" } else { "Home" }
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

const ZED_CONTENT: (Section, Section) = (
    Section {
        title: "Start Working",
        entries: &[
            SectionEntry {
                icon: IconName::Terminal,
                title: "New Terminal",
                action: &NEW_CENTER_TERMINAL,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::FolderOpen,
                title: "Open Workspace",
                action: &Open::DEFAULT,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::CloudDownload,
                title: "Clone Repository",
                action: &GitClone,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::Plus,
                title: "New File",
                action: &NewFile,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::ListCollapse,
                title: "Open Command Palette",
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
                action: &OpenSettings,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::Keyboard,
                title: "Keyboard Shortcuts",
                action: &OpenKeymap,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::Blocks,
                title: "Explore Extensions",
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
        title: "Start",
        entries: &[
            SectionEntry {
                icon: IconName::FolderOpen,
                title: "Open Workspace",
                action: &OPEN_WORKSPACE,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::CloudDownload,
                title: "Clone Repository",
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
        title: "This Workspace",
        entries: &[
            SectionEntry {
                icon: IconName::Terminal,
                title: "Open Agent Terminal",
                action: &NEW_CENTER_TERMINAL,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::FolderOpen,
                title: "Open Files",
                action: &REVEAL_FILES,
                visibility_guard: SectionVisibility::Always,
            },
            SectionEntry {
                icon: IconName::File,
                title: "New File",
                action: &NewFile,
                visibility_guard: SectionVisibility::Always,
            },
        ],
    },
    Section {
        title: "",
        entries: &[],
    },
);

#[derive(Clone, Copy)]
struct Section {
    title: &'static str,
    entries: &'static [SectionEntry],
}

impl Section {
    fn render(
        self,
        index_offset: usize,
        focus: &FocusHandle,
        emphasize_first: bool,
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
                        entry.render(index_offset + index, focus, emphasize_first && index == 0)
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_dez = APP_NAME != "Zed";
        let viewport_width = window.viewport_size().width;
        let compact_spacing = dez_welcome_uses_compact_spacing(APP_NAME, viewport_width);
        let has_workspace = self
            .workspace
            .upgrade()
            .is_some_and(|workspace| workspace.read(cx).worktrees(cx).next().is_some());
        let (first_section, second_section) = if APP_NAME == "Zed" {
            ZED_CONTENT
        } else if has_workspace {
            DEZ_WORKSPACE_CONTENT
        } else {
            DEZ_CONTENT
        };
        let first_section_entries = first_section.entries.len();
        let next_tab_index = first_section_entries + second_section.entries.len();
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
                    first_section_entries + index,
                    &workspace.location,
                    &workspace.identity_paths,
                )
            })
            .collect::<Vec<_>>();

        let recent_state = welcome_recent_state(
            APP_NAME,
            self.fallback_to_recent_projects,
            self.recent_workspaces.is_some(),
            self.recent_workspaces_load_failed,
            recent_projects.len(),
        );
        let secondary_content = match recent_state {
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
                Self::render_recent_workspace_error(first_section_entries, welcome_page)
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
            WelcomeRecentState::Hidden if !second_section.entries.is_empty() => Some(
                second_section
                    .render(first_section_entries, &self.focus_handle, false)
                    .into_any_element(),
            ),
            WelcomeRecentState::Hidden => None,
        };
        let has_secondary_content = secondary_content.is_some();
        let split_layout =
            dez_welcome_uses_split_layout(APP_NAME, viewport_width, has_secondary_content);
        let home_separator = cx.theme().colors().border_variant;

        let welcome_label = if is_dez {
            "Dez Home".to_string()
        } else if self.fallback_to_recent_projects {
            format!("Welcome back to {APP_NAME}")
        } else {
            format!("Welcome to {APP_NAME}")
        };
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
                            0,
                            &self.focus_handle,
                            welcome_emphasizes_first_action(APP_NAME),
                        )),
                )
                .when_some(secondary_content, |this, secondary_content| {
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
                    0,
                    &self.focus_handle,
                    welcome_emphasizes_first_action(APP_NAME),
                ))
                .when_some(secondary_content, |this, secondary_content| {
                    this.child(Divider::horizontal().color(DividerColor::BorderVariant))
                        .child(secondary_content)
                })
                .into_any_element()
        };
        let welcome_background = if is_dez && theme_is_transparent(cx) {
            gpui::transparent_black()
        } else {
            cx.theme().colors().editor_background
        };

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
            .child(
                v_flex()
                    .id("welcome-content")
                    .w_full()
                    .h_full()
                    .max_w(rems_from_px(if APP_NAME == "Zed" { 640. } else { 1040. }))
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
                                        Label::new(welcome_title(APP_NAME, has_workspace))
                                            .size(LabelSize::XSmall)
                                            .color(Color::Muted),
                                    )
                                    .child(Headline::new(welcome_label.clone()))
                                    .child(
                                        Label::new(welcome_summary(APP_NAME, has_workspace))
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    ),
                            )
                            .into_any_element()
                    } else {
                        h_flex()
                            .w_full()
                            .items_start()
                            .gap_3()
                            .child(
                                div()
                                    .size_8()
                                    .flex_none()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        Icon::new(if has_workspace {
                                            IconName::Terminal
                                        } else {
                                            IconName::FolderOpen
                                        })
                                        .size(IconSize::Small)
                                        .color(Color::Accent),
                                    ),
                            )
                            .child(
                                v_flex()
                                    .min_w_0()
                                    .gap_1()
                                    .child(
                                        div().font_weight(FontWeight::MEDIUM).child(
                                            Headline::new(welcome_title(APP_NAME, has_workspace))
                                                .size(HeadlineSize::Large),
                                        ),
                                    )
                                    .child(
                                        Label::new(welcome_summary(APP_NAME, has_workspace))
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    ),
                            )
                            .into_any_element()
                    })
                    .child(sections)
                    .when(
                        APP_NAME == "Zed" && !self.fallback_to_recent_projects,
                        |this| {
                            this.child(
                                v_flex().gap_4().child(Divider::horizontal()).child(
                                    Button::new("welcome-exit", "Return to Onboarding")
                                        .tab_index(next_tab_index as isize)
                                        .full_width()
                                        .label_size(LabelSize::XSmall)
                                        .on_click(|_, window, cx| {
                                            window
                                                .dispatch_action(OpenOnboarding.boxed_clone(), cx);
                                        }),
                                ),
                            )
                        },
                    ),
            )
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
            "Open a codebase to run agents in native terminals and review their work without leaving the Workspace."
        );
        assert_eq!(
            welcome_summary("Dez", true),
            "Run an agent in a native terminal, follow it in Workspaces, and review its files and Git changes here."
        );
        assert_eq!(
            welcome_summary("Zed", true),
            "Write. Delegate. Watch. Verify."
        );
        assert_eq!(welcome_title("Dez", false), "Open a Workspace");
        assert_eq!(welcome_title("Dez", true), "This Workspace");
        assert_eq!(welcome_title("Zed", false), "Terminal-native development");
        assert_eq!(welcome_surface_label("Dez"), "Home");
        assert_eq!(welcome_surface_label("Zed"), "Welcome");
        assert!(welcome_forces_tab_bar("Dez"));
        assert!(!welcome_forces_tab_bar("Zed"));
        assert_eq!(welcome_tab_icon("Dez"), Some(IconName::Compass));
        assert_eq!(welcome_tab_icon("Zed"), None);
        assert_eq!(DEZ_CONTENT.0.entries[0].title, "Open Workspace");
        assert_eq!(DEZ_CONTENT.0.entries[1].title, "Clone Repository");
        assert_eq!(
            DEZ_CONTENT.0.entries.len(),
            2,
            "the empty Dez window must not offer a pathless agent-terminal dead end"
        );
        assert_eq!(
            DEZ_WORKSPACE_CONTENT.0.entries[0].title,
            "Open Agent Terminal"
        );
        assert_eq!(DEZ_WORKSPACE_CONTENT.0.entries[1].title, "Open Files");
        assert_eq!(DEZ_WORKSPACE_CONTENT.0.entries[2].title, "New File");
        assert!(
            DEZ_CONTENT.1.entries.is_empty(),
            "Dez Welcome should leave configuration to normal application navigation"
        );
        assert!(
            DEZ_WORKSPACE_CONTENT.1.entries.is_empty(),
            "an active Workspace should not restore unrelated configuration launchers"
        );
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
