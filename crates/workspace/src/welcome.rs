use crate::{
    NewCenterTerminal, NewFile, Open, OpenFolder, OpenMode, PathList, RecentWorkspace, RevealFiles,
    SerializedWorkspaceLocation, Workspace, WorkspaceSettings,
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
use ui::{ButtonLike, Divider, DividerColor, KeyBinding, prelude::*, theme_is_transparent};
use util::ResultExt;
use zed_actions::{Extensions, OpenKeymap, OpenOnboarding, OpenSettings, command_palette};

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, JsonSchema, Action)]
#[action(namespace = welcome)]
#[serde(transparent)]
pub struct OpenRecentProject {
    pub index: usize,
}

actions!(
    zed,
    [
        /// Show the Dez welcome screen
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
                        .buffer_font(cx)
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
        }
    }
}

impl RenderOnce for SectionButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let id = format!("onb-button-{}-{}", self.label, self.tab_index);
        let action_ref: &dyn Action = &*self.action;
        let icon_color = if self.primary {
            Color::Accent
        } else {
            Color::Muted
        };

        ButtonLike::new(id)
            .tab_index(self.tab_index as isize)
            .aria_label(self.label.clone())
            .when(APP_NAME != "Zed", |this| this.style(ButtonStyle::Subtle))
            .when(self.primary && APP_NAME == "Zed", |this| {
                this.style(ButtonStyle::Filled)
                    .aria_description("Recommended first step")
            })
            .when(self.primary && APP_NAME != "Zed", |this| {
                this.aria_description("Recommended first step")
            })
            .full_width()
            .size(ButtonSize::Medium)
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Icon::new(self.icon).color(icon_color).size(IconSize::Small))
                            .child(
                                Label::new(self.label)
                                    .when(self.primary, |label| label.weight(FontWeight::MEDIUM)),
                            ),
                    )
                    .child(
                        KeyBinding::for_action_in(action_ref, &self.focus_handle, cx)
                            .size(rems_from_px(12.)),
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

const NEW_CENTER_TERMINAL: NewCenterTerminal = NewCenterTerminal { local: false };
const OPEN_WORKSPACE: OpenFolder = OpenFolder {
    create_new_window: Some(false),
};
const REVEAL_FILES: RevealFiles = RevealFiles;

fn welcome_summary(app_name: &str, has_workspace: bool) -> &'static str {
    if app_name == "Zed" {
        "Write. Delegate. Watch. Verify."
    } else if has_workspace {
        "Run an agent here. Supervise it in Agent Sessions. Review its work in Files and Git."
    } else {
        "Open a project, run an agent in its terminal, and review the work in one place."
    }
}

fn welcome_surface_label(app_name: &str) -> &'static str {
    if app_name == "Zed" { "Welcome" } else { "Home" }
}

fn welcome_emphasizes_first_action(app_name: &str) -> bool {
    app_name != "Zed"
}

fn welcome_run_step_description(app_name: &str, has_workspace: bool) -> &'static str {
    if app_name == "Zed" {
        if has_workspace {
            "Start a Terminal Session in this Workspace."
        } else {
            "Open a Workspace, then start a Terminal Session in its codebase."
        }
    } else if has_workspace {
        "Open an Agent Terminal here, then run Codex, Claude Code, or OpenCode."
    } else {
        "Open a Workspace, then run an agent in its Main Work Area."
    }
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
                title: "Customize Keymaps",
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
}

const DEZ_WELCOME_COMPACT_BREAKPOINT: Pixels = px(760.);
const DEZ_WELCOME_SPLIT_BREAKPOINT: Pixels = px(980.);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WelcomeRecentState {
    Hidden,
    Loading,
    Empty,
    Ready,
}

fn welcome_recent_state(
    app_name: &str,
    fallback_to_recent_projects: bool,
    recent_workspaces_loaded: bool,
    recent_workspace_count: usize,
) -> WelcomeRecentState {
    if !fallback_to_recent_projects {
        WelcomeRecentState::Hidden
    } else if recent_workspace_count > 0 {
        WelcomeRecentState::Ready
    } else if app_name == "Zed" {
        WelcomeRecentState::Hidden
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

        if fallback_to_recent_projects {
            let fs = workspace
                .upgrade()
                .map(|ws| ws.read(cx).app_state().fs.clone());
            let db = WorkspaceDb::global(cx);
            cx.spawn_in(window, async move |this: WeakEntity<Self>, cx| {
                let Some(fs) = fs else { return };
                let workspaces = db
                    .recent_project_workspaces(fs.as_ref())
                    .await
                    .log_err()
                    .unwrap_or_default();

                this.update(cx, |this, cx| {
                    this.recent_workspaces = Some(workspaces);
                    cx.notify();
                })
                .ok();
            })
            .detach();
        }

        WelcomePage {
            workspace,
            focus_handle,
            fallback_to_recent_projects,
            recent_workspaces: None,
        }
    }

    fn select_next(&mut self, _: &SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_next(cx);
        cx.notify();
    }

    fn select_previous(&mut self, _: &SelectPrevious, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_prev(cx);
        cx.notify();
    }

    fn open_recent_project(
        &mut self,
        action: &OpenRecentProject,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(recent_workspaces) = &self.recent_workspaces {
            if let Some(workspace) = recent_workspaces.get(action.index) {
                let is_local = matches!(workspace.location, SerializedWorkspaceLocation::Local);

                if is_local {
                    let paths = workspace.paths.paths().to_vec();
                    let open_mode = match WorkspaceSettings::get_global(cx).default_open_behavior {
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
                } else {
                    use zed_actions::OpenRecent;
                    window.dispatch_action(OpenRecent::default().boxed_clone(), cx);
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

        let welcome_label = if is_dez {
            "Dez Home".to_string()
        } else if self.fallback_to_recent_projects {
            format!("Welcome back to {APP_NAME}")
        } else {
            format!("Welcome to {APP_NAME}")
        };
        let supervise_surface = if APP_NAME == "Zed" {
            "Sessions"
        } else {
            "Agent Sessions"
        };
        let supervise_description = if APP_NAME == "Zed" {
            "Sessions keeps live state, attention, and recovery visible."
        } else {
            "Agent Sessions surfaces active agent work, attention, and recovery without moving ordinary terminals out of the Main Work Area."
        };
        let workflow_steps = [
            (
                IconName::Terminal,
                "Run",
                "Terminal",
                welcome_run_step_description(APP_NAME, has_workspace),
            ),
            (
                IconName::ListTree,
                "Supervise",
                supervise_surface,
                supervise_description,
            ),
            (
                IconName::Diff,
                "Review",
                "Files & Git",
                "Files, Git, diffs, diagnostics, and Debug stay in the Main Work Area.",
            ),
        ];
        let sections = if split_layout {
            h_flex()
                .id("welcome-sections")
                .w_full()
                .min_w_0()
                .items_start()
                .gap_6()
                .child(div().min_w_0().flex_1().child(first_section.render(
                    0,
                    &self.focus_handle,
                    welcome_emphasizes_first_action(APP_NAME),
                )))
                .when_some(secondary_content, |this, secondary_content| {
                    this.child(div().min_w_0().flex_1().child(secondary_content))
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
                    this.child(secondary_content)
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
            .when(is_dez, |this| this.items_start().justify_start())
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
                                        Label::new("Terminal-native development")
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
                                            Headline::new(if has_workspace {
                                                "Start in this Workspace"
                                            } else {
                                                "Open a Workspace"
                                            })
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
                    .when(APP_NAME != "Zed", |this| {
                        this.child(
                            h_flex()
                                .id("dez-workflow")
                                .role(gpui::Role::List)
                                .aria_label("How Dez works: Run, Supervise, Review")
                                .w_full()
                                .flex_wrap()
                                .items_center()
                                .gap_x_2()
                                .gap_y_1()
                                .when(compact_spacing, |this| {
                                    this.flex_col().items_start().gap_y_2()
                                })
                                .children(workflow_steps.into_iter().enumerate().map(
                                    |(index, (icon, title, destination, description))| {
                                        h_flex()
                                            .id(("dez-workflow-step", index))
                                            .role(gpui::Role::ListItem)
                                            .aria_label(format!("{title}. {description}"))
                                            .items_center()
                                            .gap_1()
                                            .when(index > 0 && !compact_spacing, |this| {
                                                this.child(
                                                    Icon::new(IconName::ArrowRight)
                                                        .size(IconSize::XSmall)
                                                        .color(Color::Muted),
                                                )
                                            })
                                            .child(Icon::new(icon).size(IconSize::XSmall))
                                            .child(
                                                Label::new(format!("{title} in {destination}"))
                                                    .size(LabelSize::XSmall)
                                                    .color(Color::Muted),
                                            )
                                    },
                                )),
                        )
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

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("New Welcome Page Opened")
    }

    fn show_toolbar(&self) -> bool {
        false
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
    let joined = paths
        .paths()
        .iter()
        .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .collect::<Vec<_>>()
        .join(", ");
    if joined.is_empty() {
        "Untitled".to_string()
    } else {
        joined
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
        // PathList sorts lexicographically, so filenames appear in alpha order
        let paths = PathList::new(&["/home/user/zed", "/home/user/api"]);
        assert_eq!(project_name(&paths), "api, zed");
    }

    #[test]
    fn test_project_name_root_path_filtered() {
        // A bare root "/" has no file_name(), falls back to "Untitled"
        let paths = PathList::new(&["/"]);
        assert_eq!(project_name(&paths), "Untitled");
    }

    #[test]
    fn dez_welcome_summary_teaches_the_workflow_without_a_promotion_card() {
        assert_eq!(
            welcome_summary("Dez", false),
            "Open a project, run an agent in its terminal, and review the work in one place."
        );
        assert_eq!(
            welcome_summary("Dez", true),
            "Run an agent here. Supervise it in Agent Sessions. Review its work in Files and Git."
        );
        assert_eq!(
            welcome_summary("Zed", true),
            "Write. Delegate. Watch. Verify."
        );
        assert_eq!(welcome_surface_label("Dez"), "Home");
        assert_eq!(welcome_surface_label("Zed"), "Welcome");
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
        assert_eq!(
            welcome_run_step_description("Dez", true),
            "Open an Agent Terminal here, then run Codex, Claude Code, or OpenCode."
        );
        assert_eq!(
            welcome_run_step_description("Dez", false),
            "Open a Workspace, then run an agent in its Main Work Area."
        );
        assert_eq!(
            welcome_run_step_description("Zed", true),
            "Start a Terminal Session in this Workspace."
        );
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
            welcome_recent_state("Dez", true, false, 0),
            WelcomeRecentState::Loading
        );
        assert_eq!(
            welcome_recent_state("Dez", true, true, 0),
            WelcomeRecentState::Empty
        );
        assert_eq!(
            welcome_recent_state("Dez", true, true, 2),
            WelcomeRecentState::Ready
        );
        assert_eq!(
            welcome_recent_state("Dez", false, true, 2),
            WelcomeRecentState::Hidden
        );
        assert_eq!(
            welcome_recent_state("Zed", true, true, 0),
            WelcomeRecentState::Hidden
        );
    }
}
