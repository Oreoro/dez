mod application_menu;
pub mod collab;
mod onboarding_banner;
mod plan_chip;
mod sidebar_chrome_settings;
mod update_version;

use crate::application_menu::{ApplicationMenu, show_menus};
use crate::plan_chip::PlanChip;
use agent_settings::{AgentSettings, WindowLayout};
use git_ui::worktree_picker::WorktreePicker;
pub use platform_title_bar::{
    self, DraggedWindowTab, MergeAllWindows, MoveTabToNewWindow, PlatformTitleBar,
    ShowNextWindowTab, ShowPreviousWindowTab,
};
use project::{linked_worktree_short_name, repo_identity_path};

#[cfg(not(target_os = "macos"))]
use crate::application_menu::{
    ActivateDirection, ActivateMenuLeft, ActivateMenuRight, OpenApplicationMenu,
};

use auto_update::AutoUpdateStatus;
use call::ActiveCall;
use client::{Client, UserStore, zed_urls};
use command_palette_hooks::CommandPaletteFilter;

use gpui::{
    Action, Anchor, Animation, AnimationExt, AnyElement, App, Context, Element, Entity, Focusable,
    InteractiveElement, IntoElement, MouseButton, ParentElement, Render,
    StatefulInteractiveElement, Styled, Subscription, TaskExt, WeakEntity, Window, actions, div,
    pulsating_between,
};
use onboarding_banner::OnboardingBanner;
use project::{
    Project, git_store::GitStoreEvent, project_settings::ProjectSettings,
    trusted_worktrees::TrustedWorktrees,
};
use remote::RemoteConnectionOptions;
use settings::{Settings as _, SettingsStore};

use sidebar_chrome_settings::{SidebarChromeSettings, WorkspaceBarSettings};
use std::any::TypeId;
use std::sync::Arc;
use std::time::Duration;
use theme::ActiveTheme;
use ui::{
    Avatar, ButtonLike, ContextMenu, ContextMenuEntry, IconWithIndicator, Indicator, PopoverMenu,
    PopoverMenuHandle, TintColor, Tooltip, prelude::*,
};
use update_version::UpdateVersion;
use util::ResultExt;
use workspace::{
    AccessibleMode, ClearAllSavedCanvasLayouts, ClearSavedCanvasLayoutNamed,
    ClearSavedCanvasLayoutSlot, CopySavedCanvasLayoutsToClipboard, DuplicateSavedCanvasLayoutNamed,
    DuplicateSavedCanvasLayoutSlot, ImportSavedCanvasLayoutsFromClipboard,
    ManageSavedCanvasLayouts, MultiWorkspace, MultiplexerSettings, RenameSavedCanvasLayoutNamed,
    RenameSavedCanvasLayoutSlot, RestoreSavedCanvasLayoutNamed, SaveCurrentCanvasLayoutAs,
    SaveCurrentCanvasLayoutNamed, ToggleWorktreeSecurity, Workspace,
    notifications::{NotifyResultExt, NotifyTaskExt as _},
};

use zed_actions::{OpenRemote, command_palette};

pub use onboarding_banner::restore_banner;

const MAX_PROJECT_NAME_LENGTH: usize = 40;
const MAX_BRANCH_NAME_LENGTH: usize = 40;
const MAX_SHORT_SHA_LENGTH: usize = 8;

actions!(
    collab,
    [
        /// Toggles the user menu dropdown.
        ToggleUserMenu,
        /// Toggles the project menu dropdown.
        ToggleProjectMenu,
        /// Switches to a different git branch.
        SwitchBranch,
        /// A debug action to simulate an update being available to test the update banner UI.
        SimulateUpdateAvailable
    ]
);

actions!(
    workspace,
    [
        /// Switches to the classic, editor-focused panel layout.
        UseClassicLayout,
        /// Switches to the Canvas pane-first agentic layout.
        UseAgenticLayout,
        /// Applies the full Canvas layout with project and agent panes visible.
        ApplyCanvasFullLayout,
        /// Applies the Canvas agent-control layout and focuses the agent pane.
        ApplyCanvasAgentControlLayout,
        /// Applies the Canvas focus-editor layout and hides panel panes.
        ApplyCanvasEditorFocusLayout,
        /// Applies the Canvas even-columns layout.
        ApplyCanvasEvenColumnsLayout,
        /// Applies the Canvas even-rows layout.
        ApplyCanvasEvenRowsLayout,
        /// Applies the Canvas main-and-stack editor layout.
        ApplyCanvasMainStackLayout,
        /// Applies the Canvas main-top editor layout.
        ApplyCanvasMainTopLayout,
        /// Applies the Canvas golden-split editor layout.
        ApplyCanvasGoldenSplitLayout,
        /// Applies the Canvas code-run-observe layout.
        ApplyCanvasCodeRunObserveLayout,
        /// Applies the Canvas review layout.
        ApplyCanvasReviewLayout,
        /// Applies the Canvas debug layout.
        ApplyCanvasDebugLayout,
        /// Applies the Canvas documentation studio layout.
        ApplyCanvasDocumentationStudioLayout,
        /// Applies the Canvas browser development layout.
        ApplyCanvasBrowserDevelopmentLayout,
        /// Applies the Canvas agent operations center layout.
        ApplyCanvasAgentOperationsLayout,
        /// Applies the Canvas four-agent matrix layout.
        ApplyCanvasFourAgentMatrixLayout,
        /// Applies the Canvas six-agent supervisor layout.
        ApplyCanvasSixAgentSupervisorLayout,
        /// Applies the Canvas worktree matrix layout.
        ApplyCanvasWorktreeMatrixLayout,
        /// Applies the Canvas remote operations layout.
        ApplyCanvasRemoteOperationsLayout,
        /// Applies the Canvas pair programming layout.
        ApplyCanvasPairProgrammingLayout,
        /// Applies the Canvas incident response layout.
        ApplyCanvasIncidentResponseLayout,
        /// Applies the Canvas portrait display layout.
        ApplyCanvasPortraitDisplayLayout,
        /// Cycles between Canvas agent-control and focus-editor layouts.
        CycleCanvasLayout,
        /// Saves the current Canvas layout visibility and focus snapshot.
        SaveCurrentCanvasLayout,
        /// Saves the current Canvas layout visibility and focus snapshot to slot 2.
        SaveCurrentCanvasLayoutSlot2,
        /// Saves the current Canvas layout visibility and focus snapshot to slot 3.
        SaveCurrentCanvasLayoutSlot3,
        /// Restores the saved Canvas layout visibility and focus snapshot.
        RestoreSavedCanvasLayout,
        /// Restores Canvas saved layout slot 2.
        RestoreSavedCanvasLayoutSlot2,
        /// Restores Canvas saved layout slot 3.
        RestoreSavedCanvasLayoutSlot3,
        /// Restores the previous Canvas layout visibility and focus snapshot.
        RestorePreviousCanvasLayout,
    ]
);

pub fn init(cx: &mut App) {
    platform_title_bar::PlatformTitleBar::init(cx);

    update_layout_action_filter(cx);

    cx.observe_global::<SettingsStore>(update_layout_action_filter)
        .detach();

    cx.observe_new(|workspace: &mut Workspace, window, _cx| {
        let Some(_window) = window else {
            return;
        };
        workspace.register_action(|_workspace, _: &UseClassicLayout, _window, cx| {
            set_window_layout(WindowLayout::Editor(None), cx);
        });

        workspace.register_action(|workspace, _: &UseAgenticLayout, window, cx| {
            set_window_layout(WindowLayout::Agent(None), cx);
            workspace.apply_canvas_layout(window, cx);
        });

        workspace.register_action(|workspace, _: &ApplyCanvasFullLayout, window, cx| {
            set_window_layout(WindowLayout::Agent(None), cx);
            workspace.apply_canvas_layout(window, cx);
        });

        workspace.register_action(|workspace, _: &ApplyCanvasAgentControlLayout, window, cx| {
            set_window_layout(WindowLayout::Agent(None), cx);
            workspace.apply_canvas_agent_control_layout(window, cx);
        });

        workspace.register_action(|workspace, _: &ApplyCanvasEditorFocusLayout, window, cx| {
            set_window_layout(WindowLayout::Agent(None), cx);
            workspace.apply_canvas_editor_focus_layout(window, cx);
        });

        workspace.register_action(|workspace, _: &ApplyCanvasEvenColumnsLayout, window, cx| {
            set_window_layout(WindowLayout::Agent(None), cx);
            workspace.apply_canvas_even_columns_layout(window, cx);
        });

        workspace.register_action(|workspace, _: &ApplyCanvasEvenRowsLayout, window, cx| {
            set_window_layout(WindowLayout::Agent(None), cx);
            workspace.apply_canvas_even_rows_layout(window, cx);
        });

        workspace.register_action(|workspace, _: &ApplyCanvasMainStackLayout, window, cx| {
            set_window_layout(WindowLayout::Agent(None), cx);
            workspace.apply_canvas_main_stack_layout(window, cx);
        });

        workspace.register_action(|workspace, _: &ApplyCanvasMainTopLayout, window, cx| {
            set_window_layout(WindowLayout::Agent(None), cx);
            workspace.apply_canvas_main_top_layout(window, cx);
        });

        workspace.register_action(|workspace, _: &ApplyCanvasGoldenSplitLayout, window, cx| {
            set_window_layout(WindowLayout::Agent(None), cx);
            workspace.apply_canvas_golden_split_layout(window, cx);
        });

        workspace.register_action(
            |workspace, _: &ApplyCanvasCodeRunObserveLayout, window, cx| {
                set_window_layout(WindowLayout::Agent(None), cx);
                workspace.apply_canvas_code_run_observe_layout(window, cx);
            },
        );

        workspace.register_action(|workspace, _: &ApplyCanvasReviewLayout, window, cx| {
            set_window_layout(WindowLayout::Agent(None), cx);
            workspace.apply_canvas_review_layout(window, cx);
        });

        workspace.register_action(|workspace, _: &ApplyCanvasDebugLayout, window, cx| {
            set_window_layout(WindowLayout::Agent(None), cx);
            workspace.apply_canvas_debug_layout(window, cx);
        });

        workspace.register_action(
            |workspace, _: &ApplyCanvasDocumentationStudioLayout, window, cx| {
                set_window_layout(WindowLayout::Agent(None), cx);
                workspace.apply_canvas_documentation_studio_layout(window, cx);
            },
        );

        workspace.register_action(
            |workspace, _: &ApplyCanvasBrowserDevelopmentLayout, window, cx| {
                set_window_layout(WindowLayout::Agent(None), cx);
                workspace.apply_canvas_browser_development_layout(window, cx);
            },
        );

        workspace.register_action(
            |workspace, _: &ApplyCanvasAgentOperationsLayout, window, cx| {
                set_window_layout(WindowLayout::Agent(None), cx);
                workspace.apply_canvas_agent_operations_layout(window, cx);
            },
        );

        workspace.register_action(
            |workspace, _: &ApplyCanvasFourAgentMatrixLayout, window, cx| {
                set_window_layout(WindowLayout::Agent(None), cx);
                workspace.apply_canvas_four_agent_matrix_layout(window, cx);
            },
        );

        workspace.register_action(
            |workspace, _: &ApplyCanvasSixAgentSupervisorLayout, window, cx| {
                set_window_layout(WindowLayout::Agent(None), cx);
                workspace.apply_canvas_six_agent_supervisor_layout(window, cx);
            },
        );

        workspace.register_action(
            |workspace, _: &ApplyCanvasWorktreeMatrixLayout, window, cx| {
                set_window_layout(WindowLayout::Agent(None), cx);
                workspace.apply_canvas_worktree_matrix_layout(window, cx);
            },
        );

        workspace.register_action(
            |workspace, _: &ApplyCanvasRemoteOperationsLayout, window, cx| {
                set_window_layout(WindowLayout::Agent(None), cx);
                workspace.apply_canvas_remote_operations_layout(window, cx);
            },
        );

        workspace.register_action(
            |workspace, _: &ApplyCanvasPairProgrammingLayout, window, cx| {
                set_window_layout(WindowLayout::Agent(None), cx);
                workspace.apply_canvas_pair_programming_layout(window, cx);
            },
        );

        workspace.register_action(
            |workspace, _: &ApplyCanvasIncidentResponseLayout, window, cx| {
                set_window_layout(WindowLayout::Agent(None), cx);
                workspace.apply_canvas_incident_response_layout(window, cx);
            },
        );

        workspace.register_action(
            |workspace, _: &ApplyCanvasPortraitDisplayLayout, window, cx| {
                set_window_layout(WindowLayout::Agent(None), cx);
                workspace.apply_canvas_portrait_display_layout(window, cx);
            },
        );

        workspace.register_action(|workspace, _: &CycleCanvasLayout, window, cx| {
            set_window_layout(WindowLayout::Agent(None), cx);
            workspace.cycle_canvas_layout(window, cx);
        });

        workspace.register_action(|workspace, _: &SaveCurrentCanvasLayout, window, cx| {
            workspace.save_current_canvas_layout(window, cx);
        });

        workspace.register_action(|workspace, _: &SaveCurrentCanvasLayoutSlot2, window, cx| {
            workspace.save_current_canvas_layout_slot(2, window, cx);
        });

        workspace.register_action(|workspace, _: &SaveCurrentCanvasLayoutSlot3, window, cx| {
            workspace.save_current_canvas_layout_slot(3, window, cx);
        });

        workspace.register_action(|workspace, _: &RestoreSavedCanvasLayout, window, cx| {
            set_window_layout(WindowLayout::Agent(None), cx);
            workspace.restore_saved_canvas_layout(window, cx);
        });

        workspace.register_action(|workspace, _: &RestoreSavedCanvasLayoutSlot2, window, cx| {
            set_window_layout(WindowLayout::Agent(None), cx);
            workspace.restore_saved_canvas_layout_slot(2, window, cx);
        });

        workspace.register_action(|workspace, _: &RestoreSavedCanvasLayoutSlot3, window, cx| {
            set_window_layout(WindowLayout::Agent(None), cx);
            workspace.restore_saved_canvas_layout_slot(3, window, cx);
        });

        workspace.register_action(|workspace, _: &RestorePreviousCanvasLayout, window, cx| {
            set_window_layout(WindowLayout::Agent(None), cx);
            workspace.restore_previous_canvas_layout(window, cx);
        });

        workspace.register_action(|workspace, _: &SimulateUpdateAvailable, _window, cx| {
            if let Some(multi_workspace) = workspace.multi_workspace().cloned() {
                multi_workspace
                    .update(cx, |multi_workspace, cx| {
                        multi_workspace.simulate_update_available(cx);
                    })
                    .log_err();
            }
        });

        #[cfg(not(target_os = "macos"))]
        workspace.register_action(|workspace, action: &OpenApplicationMenu, window, cx| {
            if let Some(multi_workspace) = workspace.multi_workspace().cloned() {
                multi_workspace
                    .update(cx, |multi_workspace, cx| {
                        multi_workspace.open_application_menu(
                            action.menu_name().to_string(),
                            window,
                            cx,
                        );
                    })
                    .log_err();
            }
        });

        #[cfg(not(target_os = "macos"))]
        workspace.register_action(|workspace, _: &ActivateMenuRight, window, cx| {
            if let Some(multi_workspace) = workspace.multi_workspace().cloned() {
                multi_workspace
                    .update(cx, |multi_workspace, cx| {
                        multi_workspace.activate_application_menu(true, window, cx);
                    })
                    .log_err();
            }
        });

        #[cfg(not(target_os = "macos"))]
        workspace.register_action(|workspace, _: &ActivateMenuLeft, window, cx| {
            if let Some(multi_workspace) = workspace.multi_workspace().cloned() {
                multi_workspace
                    .update(cx, |multi_workspace, cx| {
                        multi_workspace.activate_application_menu(false, window, cx);
                    })
                    .log_err();
            }
        });
    })
    .detach();
}

/// Hides or shows the panel layout actions in the command palette based on
/// whether AI is currently disabled.
fn update_layout_action_filter(cx: &mut App) {
    let disable_ai = project::DisableAiSettings::get_global(cx).disable_ai;
    let show_layout = WorkspaceBarSettings::get_global(cx).show_layout();
    let layout_actions = [
        TypeId::of::<UseClassicLayout>(),
        TypeId::of::<UseAgenticLayout>(),
        TypeId::of::<ApplyCanvasFullLayout>(),
        TypeId::of::<ApplyCanvasAgentControlLayout>(),
        TypeId::of::<ApplyCanvasEditorFocusLayout>(),
        TypeId::of::<ApplyCanvasEvenColumnsLayout>(),
        TypeId::of::<ApplyCanvasEvenRowsLayout>(),
        TypeId::of::<ApplyCanvasMainStackLayout>(),
        TypeId::of::<ApplyCanvasMainTopLayout>(),
        TypeId::of::<ApplyCanvasGoldenSplitLayout>(),
        TypeId::of::<ApplyCanvasCodeRunObserveLayout>(),
        TypeId::of::<ApplyCanvasReviewLayout>(),
        TypeId::of::<ApplyCanvasDebugLayout>(),
        TypeId::of::<ApplyCanvasDocumentationStudioLayout>(),
        TypeId::of::<ApplyCanvasBrowserDevelopmentLayout>(),
        TypeId::of::<ApplyCanvasAgentOperationsLayout>(),
        TypeId::of::<ApplyCanvasFourAgentMatrixLayout>(),
        TypeId::of::<ApplyCanvasSixAgentSupervisorLayout>(),
        TypeId::of::<ApplyCanvasWorktreeMatrixLayout>(),
        TypeId::of::<ApplyCanvasRemoteOperationsLayout>(),
        TypeId::of::<ApplyCanvasPairProgrammingLayout>(),
        TypeId::of::<ApplyCanvasIncidentResponseLayout>(),
        TypeId::of::<ApplyCanvasPortraitDisplayLayout>(),
        TypeId::of::<CycleCanvasLayout>(),
        TypeId::of::<SaveCurrentCanvasLayout>(),
        TypeId::of::<SaveCurrentCanvasLayoutSlot2>(),
        TypeId::of::<SaveCurrentCanvasLayoutSlot3>(),
        TypeId::of::<RestoreSavedCanvasLayout>(),
        TypeId::of::<RestoreSavedCanvasLayoutSlot2>(),
        TypeId::of::<RestoreSavedCanvasLayoutSlot3>(),
        TypeId::of::<RenameSavedCanvasLayoutSlot>(),
        TypeId::of::<SaveCurrentCanvasLayoutAs>(),
        TypeId::of::<SaveCurrentCanvasLayoutNamed>(),
        TypeId::of::<ManageSavedCanvasLayouts>(),
        TypeId::of::<ClearAllSavedCanvasLayouts>(),
        TypeId::of::<CopySavedCanvasLayoutsToClipboard>(),
        TypeId::of::<ImportSavedCanvasLayoutsFromClipboard>(),
        TypeId::of::<RestoreSavedCanvasLayoutNamed>(),
        TypeId::of::<RenameSavedCanvasLayoutNamed>(),
        TypeId::of::<DuplicateSavedCanvasLayoutNamed>(),
        TypeId::of::<DuplicateSavedCanvasLayoutSlot>(),
        TypeId::of::<ClearSavedCanvasLayoutNamed>(),
        TypeId::of::<ClearSavedCanvasLayoutSlot>(),
        TypeId::of::<RestorePreviousCanvasLayout>(),
    ];
    CommandPaletteFilter::update_global(cx, |filter, _| {
        if disable_ai || !show_layout {
            filter.hide_action_types(&layout_actions);
        } else {
            filter.show_action_types(layout_actions.iter());
        }
    });
}

fn set_window_layout(layout: WindowLayout, cx: &App) {
    let fs = <dyn fs::Fs>::global(cx);
    drop(AgentSettings::set_layout(layout, fs, cx));
}

pub fn sidebar_button_layout(cx: &App) -> Option<gpui::WindowButtonLayout> {
    SidebarChromeSettings::get_global(cx).button_layout
}

pub struct SidebarChrome {
    platform_titlebar: Entity<PlatformTitleBar>,
    project: Entity<Project>,
    user_store: Entity<UserStore>,
    client: Arc<Client>,
    workspace: WeakEntity<Workspace>,
    multi_workspace: Option<WeakEntity<MultiWorkspace>>,
    application_menu: Option<Entity<ApplicationMenu>>,
    _subscriptions: Vec<Subscription>,
    banner: Option<Entity<OnboardingBanner>>,
    update_version: Entity<UpdateVersion>,
    screen_share_popover_handle: PopoverMenuHandle<ContextMenu>,
    _diagnostics_subscription: Option<gpui::Subscription>,
}

impl Render for SidebarChrome {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.multi_workspace.is_none() {
            if let Some(mw) = self
                .workspace
                .upgrade()
                .and_then(|ws| ws.read(cx).multi_workspace().cloned())
            {
                self.multi_workspace = Some(mw.clone());
                self.platform_titlebar.update(cx, |titlebar, _cx| {
                    titlebar.set_multi_workspace(mw);
                });
            }
        }

        let sidebar_settings = *SidebarChromeSettings::get_global(cx);
        let workspace_bar_settings = *WorkspaceBarSettings::get_global(cx);
        let is_git_enabled = ProjectSettings::get_global(cx).git.enabled.status;
        let show_menus = show_menus(cx);

        let mut project_name = None;
        let mut repository = None;
        let mut linked_worktree_name = None;
        if let Some(worktree) = self.effective_active_worktree(cx) {
            repository = self.get_repository_for_worktree(&worktree, cx);
            let worktree_abs_path = worktree.read(cx).abs_path();
            project_name = worktree
                .read(cx)
                .root_name()
                .file_name()
                .map(|name| SharedString::from(name.to_string()));
            if let Some(repo) = &repository {
                let repo = repo.read(cx);
                linked_worktree_name = repo
                    .main_worktree_abs_path()
                    .and_then(|main_worktree_path| {
                        linked_worktree_short_name(
                            main_worktree_path,
                            repo.work_directory_abs_path.as_ref(),
                        )
                    })
                    .or_else(|| {
                        repo.is_linked_worktree()
                            .then_some(project_name.clone())
                            .flatten()
                    });

                let identity = repo_identity_path(&repo.common_dir_abs_path);

                let display_name = if identity.extension() == Some(std::ffi::OsStr::new("git")) {
                    identity.file_stem()
                } else {
                    identity.file_name()
                };

                if let Some(repo_name) = display_name.and_then(|n| n.to_str()) {
                    let visible_worktrees_in_repo = self.visible_worktrees_in_repository(repo, cx);
                    let name = if visible_worktrees_in_repo == 1 {
                        if let Ok(relative) =
                            worktree_abs_path.strip_prefix(&*repo.work_directory_abs_path)
                        {
                            if relative.as_os_str().is_empty() {
                                repo_name.to_string()
                            } else {
                                format!("{}/{}", repo_name, relative.display())
                            }
                        } else {
                            repo_name.to_string()
                        }
                    } else {
                        repo_name.to_string()
                    };
                    project_name = Some(SharedString::from(name));
                }
            }
        }

        let has_call = ActiveCall::global(cx).read(cx).room().is_some();

        let status = self.client.status();
        let status = &*status.borrow();
        let user = self.user_store.read(cx).current_user();

        let is_signing_in = user.is_none()
            && matches!(
                status,
                client::Status::Authenticating
                    | client::Status::Authenticated
                    | client::Status::Connecting
            );
        let is_signed_out_or_auth_error = user.is_none()
            && matches!(
                status,
                client::Status::SignedOut | client::Status::AuthenticationError
            );

        let mut render_project_items =
            sidebar_settings.show_branch_name || sidebar_settings.show_project_items;
        let application_menu = self.application_menu.clone();

        v_flex()
            .w_full()
            .gap_1()
            .when_some(
                application_menu.clone().filter(|_| show_menus),
                |this, menu| {
                    this.child(
                        div()
                            .w_full()
                            .overflow_x_hidden()
                            .child(menu.into_any_element()),
                    )
                },
            )
            .child(
                h_flex()
                    .w_full()
                    .gap_1()
                    .overflow_x_hidden()
                    .map(|this| {
                        this.when_some(application_menu.filter(|_| !show_menus), |this, menu| {
                            render_project_items &= !menu
                                .update(cx, |menu, cx| menu.all_menus_shown(cx))
                                || cx.accessible_mode();
                            this.child(menu)
                        })
                        .children(self.render_restricted_mode(cx))
                        .when(render_project_items, |this| {
                            this.when(sidebar_settings.show_project_items, |this| {
                                this.children(self.render_project_host(cx))
                                    .child(self.render_project_name(project_name, window, cx))
                            })
                            .when_some(
                                repository.filter(|_| is_git_enabled),
                                |this, repository| {
                                    this.children(self.render_worktree_and_branch(
                                        repository,
                                        linked_worktree_name,
                                        cx,
                                    ))
                                },
                            )
                        })
                    })
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation()),
            )
            .when(has_call, |this| {
                this.child(
                    h_flex()
                        .w_full()
                        .gap_1()
                        .overflow_x_hidden()
                        .child(self.render_collaborator_list(window, cx))
                        .child(self.render_call_controls(window, cx)),
                )
            })
            .when(sidebar_settings.show_onboarding_banner, |this| {
                this.when_some(self.banner.clone(), |this, banner| this.child(banner))
            })
            .child(
                h_flex()
                    .w_full()
                    .map(|this| match workspace_bar_settings.height {
                        settings::WorkspaceBarHeight::Minimal => this.h_6(),
                        settings::WorkspaceBarHeight::Compact => this.h_7(),
                        settings::WorkspaceBarHeight::Comfortable => this.h_8(),
                    })
                    .gap_1()
                    .justify_between()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .child(div().flex_1())
                    .when(workspace_bar_settings.center_command_search(), |this| {
                        this.child(self.render_command_search_button(cx))
                            .child(div().flex_1())
                    })
                    .when_some(
                        self.render_canvas_prefix_indicator(window, cx),
                        |this, indicator| this.child(indicator),
                    )
                    .children(self.render_connection_status(status, cx))
                    .child(self.update_version.clone())
                    .when(
                        user.is_none()
                            && is_signed_out_or_auth_error
                            && sidebar_settings.show_sign_in,
                        |this| this.child(self.render_sign_in_button(cx)),
                    )
                    .when(is_signing_in, |this| {
                        this.child(
                            Label::new("Signing in…")
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .with_animation(
                                    "signing-in",
                                    Animation::new(Duration::from_secs(2))
                                        .repeat()
                                        .with_easing(pulsating_between(0.4, 0.8)),
                                    |label, delta| label.alpha(delta),
                                ),
                        )
                    })
                    .when(sidebar_settings.show_user_menu, |this| {
                        this.child(self.render_user_menu_button(cx))
                    }),
            )
            .into_any_element()
    }
}

impl SidebarChrome {
    pub fn new(
        id: impl Into<ElementId>,
        workspace: Entity<Workspace>,
        multi_workspace: Option<WeakEntity<MultiWorkspace>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let project = workspace.read(cx).project().clone();
        let git_store = project.read(cx).git_store().clone();
        let user_store = workspace.read(cx).app_state().user_store.clone();
        let client = workspace.read(cx).app_state().client.clone();
        let active_call = ActiveCall::global(cx);

        let platform_style = PlatformStyle::platform();
        let application_menu = match platform_style {
            PlatformStyle::Mac => {
                if option_env!("ZED_USE_CROSS_PLATFORM_MENU").is_some() {
                    Some(cx.new(|cx| ApplicationMenu::new(window, cx)))
                } else {
                    None
                }
            }
            PlatformStyle::Linux | PlatformStyle::Windows => {
                Some(cx.new(|cx| ApplicationMenu::new(window, cx)))
            }
        };

        let mut subscriptions = Vec::new();
        subscriptions.push(cx.observe(&workspace, |_, _, cx| cx.notify()));

        subscriptions.push(cx.observe(&active_call, |this, _, cx| this.active_call_changed(cx)));
        subscriptions.push(cx.observe_window_activation(window, Self::window_activation_changed));
        subscriptions.push(
            cx.subscribe(&git_store, move |_, _, event, cx| match event {
                GitStoreEvent::ActiveRepositoryChanged(_)
                | GitStoreEvent::RepositoryUpdated(_, _, true) => {
                    cx.notify();
                }
                _ => {}
            }),
        );
        subscriptions.push(cx.observe(&user_store, |_a, _, cx| cx.notify()));
        subscriptions.push(
            cx.subscribe(&workspace, |_, _, event: &workspace::Event, cx| {
                if matches!(event, workspace::Event::WorktreeCreationChanged) {
                    cx.notify();
                }
            }),
        );
        subscriptions.push(cx.observe_button_layout_changed(window, |_, _, cx| cx.notify()));
        subscriptions.push(cx.observe_pending_input(window, |_, _, cx| cx.notify()));
        if let Some(trusted_worktrees) = TrustedWorktrees::try_get_global(cx) {
            subscriptions.push(cx.subscribe(&trusted_worktrees, |_, _, _, cx| {
                cx.notify();
            }));
        }

        let update_version = cx.new(|cx| UpdateVersion::new(cx));
        let platform_titlebar = cx.new(|cx| {
            let mut titlebar = PlatformTitleBar::new(id, cx);
            if let Some(mw) = multi_workspace.clone() {
                titlebar = titlebar.with_multi_workspace(mw);
            }
            titlebar
        });

        let banner = None;

        let mut this = Self {
            platform_titlebar,
            application_menu,
            workspace: workspace.downgrade(),
            multi_workspace,
            project,
            user_store,
            client,
            _subscriptions: subscriptions,
            banner,
            update_version,
            screen_share_popover_handle: PopoverMenuHandle::default(),
            _diagnostics_subscription: None,
        };

        this.observe_diagnostics(cx);

        this
    }

    fn worktree_count(&self, cx: &App) -> usize {
        self.project.read(cx).visible_worktrees(cx).count()
    }

    pub fn toggle_update_simulation(&mut self, cx: &mut Context<Self>) {
        self.update_version
            .update(cx, |banner, cx| banner.update_simulation(cx));
        cx.notify();
    }

    #[cfg(not(target_os = "macos"))]
    pub fn open_application_menu(&mut self, menu_name: String, cx: &mut Context<Self>) {
        if let Some(menu) = &self.application_menu {
            menu.update(cx, |menu, _| {
                menu.open_menu_name(menu_name);
            });
        }
    }

    #[cfg(not(target_os = "macos"))]
    pub fn activate_application_menu(
        &mut self,
        right: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(menu) = &self.application_menu {
            menu.update(cx, |menu, cx| {
                let direction = if right {
                    ActivateDirection::Right
                } else {
                    ActivateDirection::Left
                };
                menu.navigate_menus_in_direction(direction, window, cx);
            });
        }
    }

    /// Returns the worktree to display in the title bar.
    /// - Prefer the worktree owning the project's active repository
    /// - Fall back to the first visible worktree
    pub fn effective_active_worktree(&self, cx: &App) -> Option<Entity<project::Worktree>> {
        let project = self.project.read(cx);

        if let Some(repo) = project.active_repository(cx) {
            let repo = repo.read(cx);
            let repo_path = &repo.work_directory_abs_path;

            for worktree in project.visible_worktrees(cx) {
                let worktree_path = worktree.read(cx).abs_path();
                if worktree_path == *repo_path || worktree_path.starts_with(repo_path.as_ref()) {
                    return Some(worktree);
                }
            }
        }

        project.visible_worktrees(cx).next()
    }

    fn get_repository_for_worktree(
        &self,
        worktree: &Entity<project::Worktree>,
        cx: &App,
    ) -> Option<Entity<project::git_store::Repository>> {
        let project = self.project.read(cx);
        let git_store = project.git_store().read(cx);
        let worktree_path = worktree.read(cx).abs_path();

        git_store
            .repositories()
            .values()
            .filter(|repo| {
                let repo_path = &repo.read(cx).work_directory_abs_path;
                worktree_path == *repo_path || worktree_path.starts_with(repo_path.as_ref())
            })
            .max_by_key(|repo| repo.read(cx).work_directory_abs_path.as_os_str().len())
            .cloned()
    }

    fn visible_worktrees_in_repository(
        &self,
        repository: &project::git_store::Repository,
        cx: &App,
    ) -> usize {
        let repo_path = &repository.work_directory_abs_path;
        self.project
            .read(cx)
            .visible_worktrees(cx)
            .filter(|worktree| {
                let worktree_path = worktree.read(cx).abs_path();
                worktree_path == *repo_path || worktree_path.starts_with(repo_path.as_ref())
            })
            .count()
    }

    fn render_remote_project_connection(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let workspace = self.workspace.clone();

        let options = self.project.read(cx).remote_connection_options(cx)?;
        let host: SharedString = options.display_name().into();

        let (nickname, tooltip_title, icon) = match options {
            RemoteConnectionOptions::Ssh(options) => (
                options.nickname.map(|nick| nick.into()),
                "Remote Project",
                IconName::Server,
            ),
            RemoteConnectionOptions::Wsl(_) => (None, "Remote Project", IconName::Linux),
            RemoteConnectionOptions::Docker(_dev_container_connection) => {
                (None, "Dev Container", IconName::Box)
            }
            #[cfg(any(test, feature = "test-support"))]
            RemoteConnectionOptions::Mock(_) => (None, "Mock Remote Project", IconName::Server),
        };

        let nickname = nickname.unwrap_or_else(|| host.clone());

        let (indicator_color, meta) = match self.project.read(cx).remote_connection_state(cx)? {
            remote::ConnectionState::Connecting => (Color::Info, format!("Connecting to: {host}")),
            remote::ConnectionState::Connected => (Color::Success, format!("Connected to: {host}")),
            remote::ConnectionState::HeartbeatMissed => (
                Color::Warning,
                format!("Connection attempt to {host} missed. Retrying..."),
            ),
            remote::ConnectionState::Reconnecting => (
                Color::Warning,
                format!("Lost connection to {host}. Reconnecting..."),
            ),
            remote::ConnectionState::Disconnected => {
                (Color::Error, format!("Disconnected from {host}"))
            }
        };

        let icon_color = match self.project.read(cx).remote_connection_state(cx)? {
            remote::ConnectionState::Connecting => Color::Info,
            remote::ConnectionState::Connected => Color::Default,
            remote::ConnectionState::HeartbeatMissed => Color::Warning,
            remote::ConnectionState::Reconnecting => Color::Warning,
            remote::ConnectionState::Disconnected => Color::Error,
        };

        let meta = SharedString::from(meta);

        Some(
            PopoverMenu::new("remote-project-menu")
                .menu(move |window, cx| {
                    let workspace_entity = workspace.upgrade()?;
                    let fs = workspace_entity.read(cx).project().read(cx).fs().clone();
                    Some(recent_projects::RemoteServerProjects::popover(
                        fs,
                        workspace.clone(),
                        None,
                        window,
                        cx,
                    ))
                })
                .trigger_with_tooltip(
                    ButtonLike::new("remote_project")
                        .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                        .child(
                            h_flex()
                                .gap_2()
                                .max_w_32()
                                .child(
                                    IconWithIndicator::new(
                                        Icon::new(icon).size(IconSize::Small).color(icon_color),
                                        Some(Indicator::dot().color(indicator_color)),
                                    )
                                    .indicator_border_color(Some(
                                        cx.theme().colors().editor_background,
                                    ))
                                    .into_any_element(),
                                )
                                .child(Label::new(nickname).size(LabelSize::Small).truncate()),
                        ),
                    move |_window, cx| {
                        Tooltip::with_meta(
                            tooltip_title,
                            Some(&OpenRemote::default()),
                            meta.clone(),
                            cx,
                        )
                    },
                )
                .anchor(gpui::Anchor::TopLeft)
                .into_any_element(),
        )
    }

    pub fn render_restricted_mode(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let has_restricted_worktrees =
            TrustedWorktrees::has_restricted_worktrees(&self.project.read(cx).worktree_store(), cx);
        if !has_restricted_worktrees {
            return None;
        }

        let button = Button::new("restricted_mode_trigger", "Restricted Mode")
            .style(ButtonStyle::Tinted(TintColor::Warning))
            .label_size(LabelSize::Small)
            .color(Color::Warning)
            .start_icon(
                Icon::new(IconName::Warning)
                    .size(IconSize::Small)
                    .color(Color::Warning),
            )
            .tooltip(|_, cx| {
                Tooltip::with_meta(
                    "You're in Restricted Mode",
                    Some(&ToggleWorktreeSecurity),
                    "Mark this project as trusted and unlock all features",
                    cx,
                )
            })
            .on_click({
                cx.listener(move |this, _, window, cx| {
                    this.workspace
                        .update(cx, |workspace, cx| {
                            workspace.show_worktree_trust_security_modal(true, window, cx)
                        })
                        .log_err();
                })
            });

        if ui::utils::MACOS_SDK_26_OR_LATER {
            // Make up for Tahoe's traffic light buttons having less spacing around them
            Some(div().child(button).ml_0p5().into_any_element())
        } else {
            Some(button.into_any_element())
        }
    }

    pub fn render_project_host(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        if self.project.read(cx).is_via_remote_server() {
            return self.render_remote_project_connection(cx);
        }

        if self.project.read(cx).is_disconnected(cx) {
            return Some(
                Button::new("disconnected", "Disconnected")
                    .disabled(true)
                    .color(Color::Disabled)
                    .label_size(LabelSize::Small)
                    .into_any_element(),
            );
        }

        let host = self.project.read(cx).host()?;
        let host_user = self.user_store.read(cx).get_cached_user(host.user_id)?;
        let participant_index = self
            .user_store
            .read(cx)
            .participant_indices()
            .get(&host_user.legacy_id)?;

        Some(
            Button::new("project_owner_trigger", host_user.username.clone())
                .color(Color::Player(participant_index.0))
                .label_size(LabelSize::Small)
                .tab_index(0isize)
                .tooltip(move |_, cx| {
                    let tooltip_title = format!(
                        "{} is sharing this project. Click to follow.",
                        host_user.username
                    );

                    Tooltip::with_meta(tooltip_title, None, "Click to Follow", cx)
                })
                .on_click({
                    let host_peer_id = host.peer_id;
                    cx.listener(move |this, _, window, cx| {
                        this.workspace
                            .update(cx, |workspace, cx| {
                                workspace.follow(host_peer_id, window, cx);
                            })
                            .log_err();
                    })
                })
                .into_any_element(),
        )
    }

    fn render_project_name(
        &self,
        name: Option<SharedString>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let workspace = self.workspace.clone();

        let is_project_selected = name.is_some();

        let display_name = if let Some(ref name) = name {
            util::truncate_and_trailoff(name, MAX_PROJECT_NAME_LENGTH)
        } else {
            "Open Recent Project".to_string()
        };

        let is_sidebar_open = self
            .multi_workspace
            .as_ref()
            .and_then(|mw| mw.upgrade())
            .map(|mw| mw.read(cx).sidebar_open())
            .unwrap_or(false)
            && PlatformTitleBar::is_multi_workspace_enabled(cx);

        let is_threads_list_view_active = self
            .multi_workspace
            .as_ref()
            .and_then(|mw| mw.upgrade())
            .map(|mw| mw.read(cx).is_threads_list_view_active(cx))
            .unwrap_or(false);

        if is_sidebar_open && is_threads_list_view_active {
            return self
                .render_recent_projects_popover(display_name, is_project_selected, cx)
                .into_any_element();
        }

        let focus_handle = workspace
            .upgrade()
            .map(|w| w.read(cx).focus_handle(cx))
            .unwrap_or_else(|| cx.focus_handle());

        let window_project_groups: Vec<_> = self
            .multi_workspace
            .as_ref()
            .and_then(|mw| mw.upgrade())
            .map(|mw| mw.read(cx).project_group_keys())
            .unwrap_or_default();

        PopoverMenu::new("recent-projects-menu")
            .menu(move |window, cx| {
                Some(recent_projects::RecentProjects::popover(
                    workspace.clone(),
                    window_project_groups.clone(),
                    None,
                    focus_handle.clone(),
                    window,
                    cx,
                ))
            })
            .trigger_with_tooltip(
                Button::new("project_name_trigger", display_name)
                    .label_size(LabelSize::Small)
                    .tab_index(0isize)
                    .when(self.worktree_count(cx) > 1, |this| {
                        this.end_icon(
                            Icon::new(IconName::ChevronDown)
                                .size(IconSize::XSmall)
                                .color(Color::Muted),
                        )
                    })
                    .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                    .when(!is_project_selected, |s| s.color(Color::Muted)),
                move |_window, cx| {
                    Tooltip::for_action("Recent Projects", &zed_actions::OpenRecent::default(), cx)
                },
            )
            .anchor(gpui::Anchor::TopLeft)
            .into_any_element()
    }

    fn render_recent_projects_popover(
        &self,
        display_name: String,
        is_project_selected: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let workspace = self.workspace.clone();

        let focus_handle = workspace
            .upgrade()
            .map(|w| w.read(cx).focus_handle(cx))
            .unwrap_or_else(|| cx.focus_handle());

        let window_project_groups: Vec<_> = self
            .multi_workspace
            .as_ref()
            .and_then(|mw| mw.upgrade())
            .map(|mw| mw.read(cx).project_group_keys())
            .unwrap_or_default();

        PopoverMenu::new("sidebar-title-recent-projects-menu")
            .menu(move |window, cx| {
                Some(recent_projects::RecentProjects::popover(
                    workspace.clone(),
                    window_project_groups.clone(),
                    None,
                    focus_handle.clone(),
                    window,
                    cx,
                ))
            })
            .trigger_with_tooltip(
                Button::new("project_name_trigger", display_name)
                    .label_size(LabelSize::Small)
                    .tab_index(0isize)
                    .when(self.worktree_count(cx) > 1, |this| {
                        this.end_icon(
                            Icon::new(IconName::ChevronDown)
                                .size(IconSize::XSmall)
                                .color(Color::Muted),
                        )
                    })
                    .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                    .when(!is_project_selected, |s| s.color(Color::Muted)),
                move |_window, cx| {
                    Tooltip::for_action("Recent Projects", &zed_actions::OpenRecent::default(), cx)
                },
            )
            .anchor(gpui::Anchor::TopLeft)
    }

    fn render_worktree_and_branch(
        &self,
        repository: Entity<project::git_store::Repository>,
        linked_worktree_name: Option<SharedString>,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let workspace = self.workspace.upgrade()?;

        let (branch_name, icon_info, is_detached_head) = {
            let repo = repository.read(cx);

            let is_detached_head = repo.branch.is_none();

            let branch_name = repo
                .branch
                .as_ref()
                .map(|branch| branch.name())
                .map(|name| util::truncate_and_trailoff(name, MAX_BRANCH_NAME_LENGTH))
                .or_else(|| {
                    repo.head_commit.as_ref().map(|commit| {
                        commit
                            .sha
                            .chars()
                            .take(MAX_SHORT_SHA_LENGTH)
                            .collect::<String>()
                    })
                });

            let status = repo.status_summary();
            let tracked = status.index + status.worktree;
            let icon_info = if status.conflict > 0 {
                (IconName::Warning, Color::VersionControlConflict)
            } else if tracked.modified > 0 {
                (IconName::SquareDot, Color::VersionControlModified)
            } else if tracked.added > 0 || status.untracked > 0 {
                (IconName::SquarePlus, Color::VersionControlAdded)
            } else if tracked.deleted > 0 {
                (IconName::SquareMinus, Color::VersionControlDeleted)
            } else {
                (IconName::GitBranch, Color::Muted)
            };

            (branch_name, icon_info, is_detached_head)
        };

        let settings = SidebarChromeSettings::get_global(cx);
        let effective_repository = Some(repository);

        let worktree_label: SharedString = linked_worktree_name.unwrap_or_else(|| "main".into());

        let (creation_in_progress, is_switch) = self
            .workspace
            .upgrade()
            .map(|ws| {
                let creation = ws.read(cx).active_worktree_creation();
                (creation.label.clone(), creation.is_switch)
            })
            .unwrap_or((None, false));
        let is_creating = creation_in_progress.is_some();

        let display_label: SharedString = if let Some(ref name) = creation_in_progress {
            if is_switch {
                format!("Loading {}…", name).into()
            } else {
                format!("Creating {}…", name).into()
            }
        } else {
            worktree_label.clone()
        };

        let worktree_button = {
            let project = self.project.clone();
            let workspace_handle = workspace.downgrade();
            PopoverMenu::new("worktree-picker-menu")
                .menu(move |window, cx| {
                    // When opened from the title bar, focus is on the trigger
                    // button (not a dock), so `focused_dock` is `None`. That's
                    // fine — there's no prior dock focus to restore.
                    Some(cx.new(|cx| {
                        WorktreePicker::new(project.clone(), workspace_handle.clone(), window, cx)
                    }))
                })
                .trigger_with_tooltip(
                    Button::new("worktree_picker_trigger", display_label)
                        .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                        .label_size(LabelSize::Small)
                        .color(Color::Muted)
                        .tab_index(0isize)
                        .loading(is_creating)
                        .start_icon(
                            Icon::new(IconName::GitWorktree)
                                .size(IconSize::XSmall)
                                .color(Color::Muted),
                        ),
                    move |_window, cx| {
                        Tooltip::with_meta(
                            "Worktree",
                            Some(&zed_actions::git::Worktree),
                            format!("Currently In Use: {}", worktree_label),
                            cx,
                        )
                    },
                )
                .anchor(gpui::Anchor::TopLeft)
        };

        let branch_picker = branch_name.and_then(|branch_name| {
            settings.show_branch_name.then(|| {
                let branch_tooltip_label = branch_name.clone();
                let (branch_icon, branch_icon_color) = if settings.show_branch_status_icon {
                    icon_info
                } else {
                    (IconName::GitBranch, Color::Muted)
                };

                let trigger = if is_detached_head {
                    Button::new("project_branch_trigger", "Create Branch")
                        .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                        .label_size(LabelSize::Small)
                        .tab_index(0isize)
                        .start_icon(
                            Icon::new(IconName::GitBranchPlus)
                                .size(IconSize::XSmall)
                                .color(Color::Muted),
                        )
                } else {
                    Button::new("project_branch_trigger", branch_name)
                        .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                        .label_size(LabelSize::Small)
                        .color(Color::Muted)
                        .tab_index(0isize)
                        .start_icon(
                            Icon::new(branch_icon)
                                .size(IconSize::XSmall)
                                .color(branch_icon_color),
                        )
                };

                PopoverMenu::new("branch-menu")
                    .menu(move |window, cx| {
                        Some(git_ui::git_picker::popover(
                            workspace.downgrade(),
                            effective_repository.clone(),
                            git_ui::git_picker::GitPickerTab::Branches,
                            gpui::rems(34.),
                            window,
                            cx,
                        ))
                    })
                    .trigger_with_tooltip(trigger, move |_window, cx| {
                        let meta = if is_detached_head {
                            format!("Detached HEAD: {}", branch_tooltip_label)
                        } else {
                            format!("Currently Checked Out: {}", branch_tooltip_label)
                        };
                        Tooltip::with_meta(
                            "Branch & Stash",
                            Some(&zed_actions::git::Branch),
                            meta,
                            cx,
                        )
                    })
                    .anchor(gpui::Anchor::TopLeft)
            })
        });

        Some(
            h_flex()
                .gap_px()
                .child(worktree_button)
                .when_some(branch_picker, |this, branch_picker| {
                    this.child(
                        Label::new("/")
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .alpha(0.25),
                    )
                    .child(branch_picker)
                })
                .into_any_element(),
        )
    }

    fn window_activation_changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if window.is_window_active() {
            ActiveCall::global(cx)
                .update(cx, |call, cx| call.set_location(Some(&self.project), cx))
                .detach_and_log_err(cx);
        } else if cx.active_window().is_none() {
            ActiveCall::global(cx)
                .update(cx, |call, cx| call.set_location(None, cx))
                .detach_and_log_err(cx);
        }
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.update_active_view_for_followers(window, cx);
            })
            .ok();
    }

    fn active_call_changed(&mut self, cx: &mut Context<Self>) {
        self.observe_diagnostics(cx);
        cx.notify();
    }

    fn observe_diagnostics(&mut self, cx: &mut Context<Self>) {
        let diagnostics = ActiveCall::global(cx)
            .read(cx)
            .room()
            .and_then(|room| room.read(cx).diagnostics().cloned());

        if let Some(diagnostics) = diagnostics {
            self._diagnostics_subscription = Some(cx.observe(&diagnostics, |_, _, cx| cx.notify()));
        } else {
            self._diagnostics_subscription = None;
        }
    }

    fn share_project(&mut self, cx: &mut Context<Self>) {
        let active_call = ActiveCall::global(cx);
        let project = self.project.clone();
        active_call
            .update(cx, |call, cx| call.share_project(project, cx))
            .detach_and_log_err(cx);
    }

    fn unshare_project(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        let active_call = ActiveCall::global(cx);
        let project = self.project.clone();
        active_call
            .update(cx, |call, cx| call.unshare_project(project, cx))
            .log_err();
    }

    fn render_connection_status(
        &self,
        status: &client::Status,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        match status {
            client::Status::ConnectionError
            | client::Status::ConnectionLost
            | client::Status::Reauthenticating
            | client::Status::Reconnecting
            | client::Status::ReconnectionError { .. } => Some(
                div()
                    .id("disconnected")
                    .child(Icon::new(IconName::Disconnected).size(IconSize::Small))
                    .tooltip(Tooltip::text("Disconnected"))
                    .into_any_element(),
            ),
            client::Status::UpgradeRequired => {
                let auto_updater = auto_update::AutoUpdater::get(cx);
                let label = match auto_updater.map(|auto_update| auto_update.read(cx).status()) {
                    Some(AutoUpdateStatus::Updated { .. }) => "Please restart Zed to Collaborate",
                    Some(AutoUpdateStatus::Installing { .. })
                    | Some(AutoUpdateStatus::Downloading { .. })
                    | Some(AutoUpdateStatus::Checking) => "Updating...",
                    Some(AutoUpdateStatus::Idle)
                    | Some(AutoUpdateStatus::Errored { .. })
                    | None => "Please update Zed to Collaborate",
                };

                Some(
                    Button::new("connection-status", label)
                        .label_size(LabelSize::Small)
                        .on_click(|_, window, cx| {
                            if let Some(auto_updater) = auto_update::AutoUpdater::get(cx)
                                && auto_updater.read(cx).status().is_updated()
                            {
                                workspace::reload(cx);
                                return;
                            }
                            auto_update::check(&Default::default(), window, cx);
                        })
                        .into_any_element(),
                )
            }
            _ => None,
        }
    }

    fn render_canvas_prefix_indicator(
        &self,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let multiplexer_settings = MultiplexerSettings::get_global(cx);
        if !multiplexer_settings.prefix_mode || !window.has_pending_keystrokes() {
            return None;
        }

        Some(
            h_flex()
                .id("canvas-prefix-indicator")
                .h_5()
                .gap_1()
                .px_1p5()
                .rounded_sm()
                .border_1()
                .border_color(cx.theme().colors().border_focused)
                .bg(cx.theme().colors().element_active.opacity(0.35))
                .child(Indicator::dot().color(Color::Accent))
                .child(
                    Label::new(format!("PREFIX {}", multiplexer_settings.prefix))
                        .size(LabelSize::XSmall)
                        .color(Color::Accent),
                )
                .tooltip(Tooltip::text("Prefix mode is awaiting the next key"))
                .into_any_element(),
        )
    }

    pub fn render_sign_in_button(&mut self, _: &mut Context<Self>) -> Button {
        let client = self.client.clone();
        let workspace = self.workspace.clone();
        Button::new("sign_in", "Sign In")
            .label_size(LabelSize::Small)
            .tab_index(0isize)
            .on_click(move |_, window, cx| {
                let client = client.clone();
                let workspace = workspace.clone();
                window
                    .spawn(cx, async move |mut cx| {
                        client
                            .sign_in_with_optional_connect(true, cx)
                            .await
                            .notify_workspace_async_err(workspace, &mut cx);
                    })
                    .detach();
            })
    }

    pub fn render_command_search_button(&mut self, _: &mut Context<Self>) -> Button {
        Button::new("workspace-command-search", "Command Search")
            .label_size(LabelSize::Small)
            .color(Color::Muted)
            .start_icon(
                Icon::new(IconName::MagnifyingGlass)
                    .size(IconSize::Small)
                    .color(Color::Muted),
            )
            .tooltip(Tooltip::text("Open Command Palette"))
            .on_click(|_, window, cx| {
                window.dispatch_action(command_palette::Toggle.boxed_clone(), cx);
            })
    }

    pub fn render_user_menu_button(&mut self, cx: &mut Context<Self>) -> impl Element {
        let show_update_button = self.update_version.read(cx).show_update_in_menu_bar();

        let user_store = self.user_store.clone();
        let workspace = self.workspace.clone();
        let user = user_store.read(cx).current_user();

        let user_avatar = user.as_ref().map(|u| u.avatar_uri.clone());
        let username = user.as_ref().map(|u| u.username.clone());

        let is_signed_in = user.is_some();

        let current_organization = user_store.read(cx).current_organization();
        let business_organization = current_organization
            .as_ref()
            .filter(|organization| !organization.is_personal);
        let organizations: Vec<_> = user_store
            .read(cx)
            .organizations()
            .iter()
            .map(|organization| {
                let plan = user_store.read(cx).plan_for_organization(&organization.id);
                (organization.clone(), plan)
            })
            .collect();

        let show_user_picture = SidebarChromeSettings::get_global(cx).show_user_picture;
        let show_layout = WorkspaceBarSettings::get_global(cx).show_layout();

        let trigger = if is_signed_in && show_user_picture {
            let avatar = user_avatar.map(|avatar| Avatar::new(avatar)).map(|avatar| {
                if show_update_button {
                    avatar.indicator(
                        div()
                            .absolute()
                            .bottom_0()
                            .right_0()
                            .child(Indicator::dot().color(Color::Accent)),
                    )
                } else {
                    avatar
                }
            });

            ButtonLike::new("user-menu")
                .aria_label("User menu")
                .tab_index(0isize)
                .child(
                    h_flex()
                        .when_some(business_organization, |this, organization| {
                            this.gap_2()
                                .child(Label::new(&organization.name).size(LabelSize::Small))
                        })
                        .children(avatar),
                )
        } else {
            ButtonLike::new("user-menu")
                .aria_label("User menu")
                .tab_index(0isize)
                .child(Icon::new(IconName::ChevronDown).size(IconSize::Small))
        };

        PopoverMenu::new("user-menu")
            .trigger(trigger)
            .menu(move |window, cx| {
                let username = username.clone();
                let current_organization = current_organization.clone();
                let organizations = organizations.clone();
                let user_store = user_store.clone();
                let workspace = workspace.clone();

                let ai_enabled = !project::DisableAiSettings::get_global(cx).disable_ai;
                let current_layout = AgentSettings::get_layout(cx);
                let is_editor = matches!(current_layout, WindowLayout::Editor(_));
                let is_agent = matches!(current_layout, WindowLayout::Agent(_));
                let is_custom = matches!(current_layout, WindowLayout::Custom(_));
                let (
                    active_canvas_layout_recipe,
                    canvas_layout_history_len,
                    has_saved_canvas_layout_slot_1,
                    has_saved_canvas_layout_slot_2,
                    has_saved_canvas_layout_slot_3,
                    saved_canvas_layout_count,
                    saved_canvas_layout_slot_1_label,
                    saved_canvas_layout_slot_2_label,
                    saved_canvas_layout_slot_3_label,
                    saved_canvas_named_layouts,
                ) = workspace.upgrade().map_or(
                    (None, 0, false, false, false, 0, None, None, None, Vec::new()),
                    |workspace| {
                        let workspace = workspace.read(cx);
                        (
                            workspace.active_canvas_layout_recipe_id(),
                            workspace.canvas_layout_history_len(),
                            workspace.has_saved_canvas_layout_slot(1),
                            workspace.has_saved_canvas_layout_slot(2),
                            workspace.has_saved_canvas_layout_slot(3),
                            workspace.saved_canvas_layout_count(),
                            workspace
                                .saved_canvas_layout_slot_label(1)
                                .map(str::to_string),
                            workspace
                                .saved_canvas_layout_slot_label(2)
                                .map(str::to_string),
                            workspace
                                .saved_canvas_layout_slot_label(3)
                                .map(str::to_string),
                            workspace.saved_canvas_named_layouts(),
                        )
                    },
                );
                let active_canvas_layout_recipe =
                    is_agent.then_some(active_canvas_layout_recipe).flatten();
                let has_previous_canvas_layout = canvas_layout_history_len > 0;
                let canvas_layout_history_label = if canvas_layout_history_len == 1 {
                    "Layout History: 1 snapshot".to_string()
                } else {
                    format!("Layout History: {canvas_layout_history_len} snapshots")
                };
                let saved_canvas_layout_label = if saved_canvas_layout_count == 1 {
                    "Saved Layouts: 1 saved layout".to_string()
                } else {
                    format!("Saved Layouts: {saved_canvas_layout_count} saved layouts")
                };
                let restore_saved_canvas_layout_slot_1_label = saved_canvas_layout_slot_1_label
                    .map_or_else(
                        || "Restore Canvas Layout: Slot 1".to_string(),
                        |label| format!("Restore Canvas Layout: Slot 1 — {label}"),
                    );
                let restore_saved_canvas_layout_slot_2_label = saved_canvas_layout_slot_2_label
                    .map_or_else(
                        || "Restore Canvas Layout: Slot 2".to_string(),
                        |label| format!("Restore Canvas Layout: Slot 2 — {label}"),
                    );
                let restore_saved_canvas_layout_slot_3_label = saved_canvas_layout_slot_3_label
                    .map_or_else(
                        || "Restore Canvas Layout: Slot 3".to_string(),
                        |label| format!("Restore Canvas Layout: Slot 3 — {label}"),
                    );
                let multiplexer_hint = {
                    let multiplexer_settings = MultiplexerSettings::get_global(cx);
                    multiplexer_settings.prefix_mode.then(|| {
                        let confirmation = match multiplexer_settings.broadcast_confirmation {
                            settings::BroadcastConfirmation::Always => "always",
                            settings::BroadcastConfirmation::Risky => "risky",
                            settings::BroadcastConfirmation::Never => "never",
                        };
                        let prefix = multiplexer_settings.prefix.clone();
                        let timeout = multiplexer_settings.prefix_timeout.map_or_else(
                            || "off".to_string(),
                            |timeout| format!("{}ms", timeout.as_millis()),
                        );
                        vec![
                            format!(
                                "Prefix mode: {prefix} · timeout: {timeout} · broadcast confirmation: {confirmation}"
                            ),
                            "Prefix commands: ctrl-b space · Cycle Layout".to_string(),
                            "Prefix commands: ctrl-b a · Agent Control".to_string(),
                            "Prefix commands: ctrl-b f · Focus Editor".to_string(),
                            "Prefix commands: ctrl-b m · Four-Agent Matrix".to_string(),
                            "Prefix commands: ctrl-b s/r/p · Save, Restore, Previous".to_string(),
                            "Prefix commands: ctrl-b 1/2/3 · Restore saved slots".to_string(),
                            "Prefix commands: ctrl-b shift-1/2/3 · Save slots".to_string(),
                            "Prefix commands: ctrl-b n m/s/1/2/3 · Manage, Save as, Rename slots"
                                .to_string(),
                            "Prefix commands: ctrl-b arrows · Focus adjacent panes".to_string(),
                            "Prefix commands: ctrl-b shift-arrows · Swap adjacent panes"
                                .to_string(),
                            "Prefix commands: ctrl-b alt-arrows · Move pane to edge".to_string(),
                            "Prefix commands: ctrl-b v/enter · Split right, Split down".to_string(),
                            "Prefix commands: ctrl-b h/j/k/l/= · Resize, Equalize".to_string(),
                            "Prefix commands: ctrl-b ctrl-b · Send prefix".to_string(),
                        ]
                    })
                };

                ContextMenu::build(window, cx, |menu, _, _cx| {
                    menu.when(is_signed_in, |this| {
                        let username = username.clone();
                        this.custom_entry(
                            move |_window, _cx| {
                                let username = username.clone().unwrap_or_default();

                                h_flex()
                                    .w_full()
                                    .justify_between()
                                    .child(Label::new(username))
                                    .into_any_element()
                            },
                            move |_, cx| {
                                cx.open_url(&zed_urls::account_url(cx));
                            },
                        )
                        .separator()
                    })
                    .when(show_update_button, |this| {
                        this.custom_entry(
                            move |_window, _cx| {
                                h_flex()
                                    .w_full()
                                    .gap_1()
                                    .justify_between()
                                    .child(Label::new("Restart to update Zed").color(Color::Accent))
                                    .child(
                                        Icon::new(IconName::Download)
                                            .size(IconSize::Small)
                                            .color(Color::Accent),
                                    )
                                    .into_any_element()
                            },
                            move |_, cx| {
                                workspace::reload(cx);
                            },
                        )
                        .separator()
                    })
                    .map(|this| {
                        let mut this = this.header("Organization");

                        for (organization, plan) in &organizations {
                            let organization = organization.clone();
                            let plan = *plan;

                            let is_current =
                                current_organization
                                    .as_ref()
                                    .is_some_and(|current_organization| {
                                        current_organization.id == organization.id
                                    });

                            this = this.custom_entry(
                                {
                                    let organization = organization.clone();
                                    move |_window, _cx| {
                                        h_flex()
                                            .w_full()
                                            .gap_4()
                                            .justify_between()
                                            .child(
                                                h_flex()
                                                    .gap_1()
                                                    .child(Label::new(&organization.name))
                                                    .when(is_current, |this| {
                                                        this.child(
                                                            Icon::new(IconName::Check)
                                                                .color(Color::Accent),
                                                        )
                                                    }),
                                            )
                                            .children(plan.map(|plan| PlanChip::new(plan)))
                                            .into_any_element()
                                    }
                                },
                                {
                                    let user_store = user_store.clone();
                                    let organization = organization.clone();
                                    let workspace = workspace.clone();
                                    move |window, cx| {
                                        let task = user_store.update(cx, |user_store, cx| {
                                            user_store
                                                .set_current_organization(organization.clone(), cx)
                                        });
                                        task.detach_and_notify_err(workspace.clone(), window, cx);
                                    }
                                },
                            );
                        }

                        this.separator()
                    })
                    .action("Settings", zed_actions::OpenSettings.boxed_clone())
                    .action("Keymap", Box::new(zed_actions::OpenKeymap))
                    .action(
                        "Themes…",
                        zed_actions::theme_selector::Toggle::default().boxed_clone(),
                    )
                    .action(
                        "Icon Themes…",
                        zed_actions::icon_theme_selector::Toggle::default().boxed_clone(),
                    )
                    .action(
                        "Extensions",
                        zed_actions::Extensions::default().boxed_clone(),
                    )
                    .when(ai_enabled && show_layout, |menu| {
                        menu.separator()
                            .submenu("Panel Layout", move |menu, _window, _cx| {
                                menu.toggleable_entry(
                                    "Classic",
                                    is_editor,
                                    IconPosition::Start,
                                    Some(UseClassicLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(UseClassicLayout.boxed_clone(), cx);
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas",
                                    is_agent,
                                    IconPosition::Start,
                                    Some(UseAgenticLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(UseAgenticLayout.boxed_clone(), cx);
                                    },
                                )
                                .separator()
                                .toggleable_entry(
                                    "Canvas: Full",
                                    active_canvas_layout_recipe == Some("full"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasFullLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasFullLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Agent Control",
                                    active_canvas_layout_recipe == Some("agent_control"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasAgentControlLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasAgentControlLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Focus Editor",
                                    active_canvas_layout_recipe == Some("editor_focus"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasEditorFocusLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasEditorFocusLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Even Columns",
                                    active_canvas_layout_recipe == Some("even_columns"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasEvenColumnsLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasEvenColumnsLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Even Rows",
                                    active_canvas_layout_recipe == Some("even_rows"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasEvenRowsLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasEvenRowsLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Main + Stack",
                                    active_canvas_layout_recipe == Some("main_stack"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasMainStackLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasMainStackLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Main Top",
                                    active_canvas_layout_recipe == Some("main_top"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasMainTopLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasMainTopLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Golden Split",
                                    active_canvas_layout_recipe == Some("golden_split"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasGoldenSplitLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasGoldenSplitLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Code, Run, Observe",
                                    active_canvas_layout_recipe == Some("code_run_observe"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasCodeRunObserveLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasCodeRunObserveLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Review",
                                    active_canvas_layout_recipe == Some("review"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasReviewLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasReviewLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Debug",
                                    active_canvas_layout_recipe == Some("debug"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasDebugLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasDebugLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Documentation Studio",
                                    active_canvas_layout_recipe == Some("documentation_studio"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasDocumentationStudioLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasDocumentationStudioLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Browser Development",
                                    active_canvas_layout_recipe == Some("browser_development"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasBrowserDevelopmentLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasBrowserDevelopmentLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Agent Operations Center",
                                    active_canvas_layout_recipe == Some("agent_operations"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasAgentOperationsLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasAgentOperationsLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Four-Agent Matrix",
                                    active_canvas_layout_recipe == Some("four_agent_matrix"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasFourAgentMatrixLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasFourAgentMatrixLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Six-Agent Supervisor",
                                    active_canvas_layout_recipe == Some("six_agent_supervisor"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasSixAgentSupervisorLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasSixAgentSupervisorLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Worktree Matrix",
                                    active_canvas_layout_recipe == Some("worktree_matrix"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasWorktreeMatrixLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasWorktreeMatrixLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Remote Operations",
                                    active_canvas_layout_recipe == Some("remote_operations"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasRemoteOperationsLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasRemoteOperationsLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Pair Programming",
                                    active_canvas_layout_recipe == Some("pair_programming"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasPairProgrammingLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasPairProgrammingLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Incident Response",
                                    active_canvas_layout_recipe == Some("incident_response"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasIncidentResponseLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasIncidentResponseLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .toggleable_entry(
                                    "Canvas: Portrait Display",
                                    active_canvas_layout_recipe == Some("portrait_display"),
                                    IconPosition::Start,
                                    Some(ApplyCanvasPortraitDisplayLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            ApplyCanvasPortraitDisplayLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .entry(
                                    "Cycle Canvas Layout",
                                    Some(CycleCanvasLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(CycleCanvasLayout.boxed_clone(), cx);
                                    },
                                )
                                .action(
                                    "Save Canvas Layout As…",
                                    SaveCurrentCanvasLayoutAs.boxed_clone(),
                                )
                                .action(
                                    "Manage Canvas Saved Layouts…",
                                    ManageSavedCanvasLayouts.boxed_clone(),
                                )
                                .action_checked_with_disabled(
                                    "Clear All Canvas Saved Layouts…",
                                    ClearAllSavedCanvasLayouts.boxed_clone(),
                                    false,
                                    saved_canvas_layout_count == 0,
                                )
                                .action_checked_with_disabled(
                                    "Copy Canvas Saved Layouts JSON",
                                    CopySavedCanvasLayoutsToClipboard.boxed_clone(),
                                    false,
                                    saved_canvas_layout_count == 0,
                                )
                                .action(
                                    "Import Canvas Saved Layouts JSON from Clipboard…",
                                    ImportSavedCanvasLayoutsFromClipboard.boxed_clone(),
                                )
                                .entry(
                                    "Save Canvas Layout: Slot 1",
                                    Some(SaveCurrentCanvasLayout.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            SaveCurrentCanvasLayout.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .action_checked_with_disabled(
                                    restore_saved_canvas_layout_slot_1_label,
                                    RestoreSavedCanvasLayout.boxed_clone(),
                                    false,
                                    !has_saved_canvas_layout_slot_1,
                                )
                                .action_checked_with_disabled(
                                    "Rename Canvas Layout: Slot 1",
                                    RenameSavedCanvasLayoutSlot { slot: 1 }.boxed_clone(),
                                    false,
                                    !has_saved_canvas_layout_slot_1,
                                )
                                .action_checked_with_disabled(
                                    "Duplicate Canvas Layout: Slot 1",
                                    DuplicateSavedCanvasLayoutSlot { slot: 1 }.boxed_clone(),
                                    false,
                                    !has_saved_canvas_layout_slot_1,
                                )
                                .action_checked_with_disabled(
                                    "Clear Canvas Layout: Slot 1",
                                    ClearSavedCanvasLayoutSlot { slot: 1 }.boxed_clone(),
                                    false,
                                    !has_saved_canvas_layout_slot_1,
                                )
                                .entry(
                                    "Save Canvas Layout: Slot 2",
                                    Some(SaveCurrentCanvasLayoutSlot2.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            SaveCurrentCanvasLayoutSlot2.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .action_checked_with_disabled(
                                    restore_saved_canvas_layout_slot_2_label,
                                    RestoreSavedCanvasLayoutSlot2.boxed_clone(),
                                    false,
                                    !has_saved_canvas_layout_slot_2,
                                )
                                .action_checked_with_disabled(
                                    "Rename Canvas Layout: Slot 2",
                                    RenameSavedCanvasLayoutSlot { slot: 2 }.boxed_clone(),
                                    false,
                                    !has_saved_canvas_layout_slot_2,
                                )
                                .action_checked_with_disabled(
                                    "Duplicate Canvas Layout: Slot 2",
                                    DuplicateSavedCanvasLayoutSlot { slot: 2 }.boxed_clone(),
                                    false,
                                    !has_saved_canvas_layout_slot_2,
                                )
                                .action_checked_with_disabled(
                                    "Clear Canvas Layout: Slot 2",
                                    ClearSavedCanvasLayoutSlot { slot: 2 }.boxed_clone(),
                                    false,
                                    !has_saved_canvas_layout_slot_2,
                                )
                                .entry(
                                    "Save Canvas Layout: Slot 3",
                                    Some(SaveCurrentCanvasLayoutSlot3.boxed_clone()),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            SaveCurrentCanvasLayoutSlot3.boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                                .action_checked_with_disabled(
                                    restore_saved_canvas_layout_slot_3_label,
                                    RestoreSavedCanvasLayoutSlot3.boxed_clone(),
                                    false,
                                    !has_saved_canvas_layout_slot_3,
                                )
                                .action_checked_with_disabled(
                                    "Rename Canvas Layout: Slot 3",
                                    RenameSavedCanvasLayoutSlot { slot: 3 }.boxed_clone(),
                                    false,
                                    !has_saved_canvas_layout_slot_3,
                                )
                                .action_checked_with_disabled(
                                    "Duplicate Canvas Layout: Slot 3",
                                    DuplicateSavedCanvasLayoutSlot { slot: 3 }.boxed_clone(),
                                    false,
                                    !has_saved_canvas_layout_slot_3,
                                )
                                .action_checked_with_disabled(
                                    "Clear Canvas Layout: Slot 3",
                                    ClearSavedCanvasLayoutSlot { slot: 3 }.boxed_clone(),
                                    false,
                                    !has_saved_canvas_layout_slot_3,
                                )
                                .when(!saved_canvas_named_layouts.is_empty(), |menu| {
                                    let mut menu = menu.separator();
                                    for (name, label) in saved_canvas_named_layouts.clone() {
                                        menu = menu
                                            .action_checked_with_disabled(
                                                format!("Restore Saved Canvas Layout — {label}"),
                                                RestoreSavedCanvasLayoutNamed {
                                                    name: name.clone(),
                                                }
                                                .boxed_clone(),
                                                false,
                                                false,
                                            )
                                            .action_checked_with_disabled(
                                                format!("Rename Saved Canvas Layout — {label}"),
                                                RenameSavedCanvasLayoutNamed {
                                                    name: name.clone(),
                                                }
                                                .boxed_clone(),
                                                false,
                                                false,
                                            )
                                            .action_checked_with_disabled(
                                                format!("Duplicate Saved Canvas Layout — {label}"),
                                                DuplicateSavedCanvasLayoutNamed {
                                                    name: name.clone(),
                                                }
                                                .boxed_clone(),
                                                false,
                                                false,
                                            )
                                            .action_checked_with_disabled(
                                                format!("Clear Saved Canvas Layout — {label}"),
                                                ClearSavedCanvasLayoutNamed { name }.boxed_clone(),
                                                false,
                                                false,
                                            );
                                    }
                                    menu
                                })
                                .action_checked_with_disabled(
                                    "Restore Previous Canvas Layout",
                                    RestorePreviousCanvasLayout.boxed_clone(),
                                    false,
                                    !has_previous_canvas_layout,
                                )
                                .when(is_agent, |menu| {
                                    menu.item(
                                        ContextMenuEntry::new(canvas_layout_history_label.clone())
                                            .disabled(true),
                                    )
                                })
                                .when(is_agent, |menu| {
                                    menu.item(
                                        ContextMenuEntry::new(saved_canvas_layout_label.clone())
                                            .disabled(true),
                                    )
                                })
                                .when_some(multiplexer_hint.clone(), |menu, hints| {
                                    let mut menu = menu.separator();
                                    for hint in hints {
                                        menu =
                                            menu.item(ContextMenuEntry::new(hint).disabled(true));
                                    }
                                    menu
                                })
                                .when(is_agent && active_canvas_layout_recipe.is_none(), |menu| {
                                    menu.item(
                                        ContextMenuEntry::new("Custom Canvas Layout")
                                            .toggleable(IconPosition::Start, true)
                                            .disabled(true),
                                    )
                                })
                                .when(is_custom, |menu| {
                                    menu.item(
                                        ContextMenuEntry::new("Custom")
                                            .toggleable(IconPosition::Start, true)
                                            .disabled(true),
                                    )
                                })
                            })
                    })
                    .when(is_signed_in, |this| {
                        this.separator()
                            .action("Sign Out", client::SignOut.boxed_clone())
                    })
                })
                .into()
            })
            .anchor(Anchor::TopRight)
    }
}
