use std::sync::Arc;

use client::TelemetrySettings;
use collections::HashMap;
use fs::Fs;
use gpui::{Action, App, ClipboardItem, IntoElement};
use paths::APP_NAME;
use project::agent_server_store::AllAgentServersSettings;
use project::project_settings::ProjectSettings;
use project::{AgentRegistryStore, RegistryAgent};
use settings::{
    BaseKeymap, CustomAgentServerSettings, Settings, SettingsStore, update_settings_file,
};
use theme::{Appearance, SystemAppearance, ThemeRegistry};
use theme_settings::{ThemeAppearanceMode, ThemeName, ThemeSelection, ThemeSettings};
use ui::{
    AgentSetupButton, Divider, StatefulInteractiveElement, SwitchField, TintColor,
    ToggleButtonGroup, ToggleButtonGroupSize, ToggleButtonSimple, ToggleButtonWithIcon, Tooltip,
    prelude::*,
};
use vim_mode_setting::VimModeSetting;
use workspace::NewCenterTerminal;

use crate::{
    ImportCursorSettings, ImportVsCodeSettings, SettingsImportState,
    theme_preview::{ThemePreviewStyle, ThemePreviewTile},
};

const LIGHT_THEMES: [&str; 3] = ["One Light", "Ayu Light", "Gruvbox Light"];
const DARK_THEMES: [&str; 3] = ["One Dark", "Ayu Dark", "Gruvbox Dark"];
const FAMILY_NAMES: [SharedString; 3] = [
    SharedString::new_static("One"),
    SharedString::new_static("Ayu"),
    SharedString::new_static("Gruvbox"),
];
const CODEX_HOOK_SETUP: &str = include_str!("../../../assets/dez/codex-hooks.json");

fn get_theme_family_themes(theme_name: &str) -> Option<(&'static str, &'static str)> {
    for i in 0..LIGHT_THEMES.len() {
        if LIGHT_THEMES[i] == theme_name || DARK_THEMES[i] == theme_name {
            return Some((LIGHT_THEMES[i], DARK_THEMES[i]));
        }
    }
    None
}

fn render_theme_section(tab_index: &mut isize, cx: &mut App) -> impl IntoElement {
    let theme_selection = ThemeSettings::get_global(cx).theme.clone();
    let system_appearance = theme::SystemAppearance::global(cx);

    let theme_mode = theme_selection
        .mode()
        .unwrap_or_else(|| match *system_appearance {
            Appearance::Light => ThemeAppearanceMode::Light,
            Appearance::Dark => ThemeAppearanceMode::Dark,
        });

    return v_flex()
        .gap_2()
        .child(
            h_flex().justify_between().child(Label::new("Theme")).child(
                ToggleButtonGroup::single_row(
                    "theme-selector-onboarding-dark-light",
                    [
                        ThemeAppearanceMode::Light,
                        ThemeAppearanceMode::Dark,
                        ThemeAppearanceMode::System,
                    ]
                    .map(|mode| {
                        const MODE_NAMES: [SharedString; 3] = [
                            SharedString::new_static("Light"),
                            SharedString::new_static("Dark"),
                            SharedString::new_static("System"),
                        ];
                        ToggleButtonSimple::new(
                            MODE_NAMES[mode as usize].clone(),
                            move |_, _, cx| {
                                write_mode_change(mode, cx);

                                telemetry::event!(
                                    "Welcome Theme mode Changed",
                                    from = theme_mode,
                                    to = mode
                                );
                            },
                        )
                    }),
                )
                .size(ToggleButtonGroupSize::Medium)
                .tab_index(tab_index)
                .selected_index(theme_mode as usize)
                .style(ui::ToggleButtonGroupStyle::Outlined)
                .width(rems_from_px(3. * 64.)),
            ),
        )
        .child(
            h_flex()
                .gap_2()
                .justify_between()
                .children(render_theme_previews(tab_index, &theme_selection, cx)),
        );

    fn render_theme_previews(
        tab_index: &mut isize,
        theme_selection: &ThemeSelection,
        cx: &mut App,
    ) -> [impl IntoElement; 3] {
        let system_appearance = SystemAppearance::global(cx);
        let theme_registry = ThemeRegistry::global(cx);

        let theme_seed = 0xBEEF as f32;
        let theme_mode = theme_selection
            .mode()
            .unwrap_or_else(|| match *system_appearance {
                Appearance::Light => ThemeAppearanceMode::Light,
                Appearance::Dark => ThemeAppearanceMode::Dark,
            });
        let appearance = match theme_mode {
            ThemeAppearanceMode::Light => Appearance::Light,
            ThemeAppearanceMode::Dark => Appearance::Dark,
            ThemeAppearanceMode::System => *system_appearance,
        };
        let current_theme_name: SharedString = theme_selection.name(appearance).0.into();

        let theme_names = match appearance {
            Appearance::Light => LIGHT_THEMES,
            Appearance::Dark => DARK_THEMES,
        };

        let themes = theme_names.map(|theme| theme_registry.get(theme).unwrap());

        [0, 1, 2].map(|index| {
            let theme = &themes[index];
            let is_selected = theme.name == current_theme_name;
            let name = theme.name.clone();
            let colors = cx.theme().colors();

            v_flex()
                .w_full()
                .items_center()
                .gap_1()
                .child(
                    h_flex()
                        .id(name)
                        .relative()
                        .w_full()
                        .border_2()
                        .border_color(colors.border_transparent)
                        .rounded(ThemePreviewTile::ROOT_RADIUS)
                        .map(|this| {
                            if is_selected {
                                this.border_color(colors.border_selected)
                            } else {
                                this.opacity(0.8).hover(|s| s.border_color(colors.border))
                            }
                        })
                        .tab_index({
                            *tab_index += 1;
                            *tab_index - 1
                        })
                        .focus(|mut style| {
                            style.border_color = Some(colors.border_focused);
                            style
                        })
                        .on_click({
                            let theme_name = theme.name.clone();
                            let current_theme_name = current_theme_name.clone();

                            move |_, _, cx| {
                                write_theme_change(theme_name.clone(), theme_mode, cx);
                                telemetry::event!(
                                    "Welcome Theme Changed",
                                    from = current_theme_name,
                                    to = theme_name
                                );
                            }
                        })
                        .map(|this| {
                            if theme_mode == ThemeAppearanceMode::System {
                                let (light, dark) = (
                                    theme_registry.get(LIGHT_THEMES[index]).unwrap(),
                                    theme_registry.get(DARK_THEMES[index]).unwrap(),
                                );
                                this.child(
                                    ThemePreviewTile::new(light, theme_seed)
                                        .style(ThemePreviewStyle::SideBySide(dark)),
                                )
                            } else {
                                this.child(
                                    ThemePreviewTile::new(theme.clone(), theme_seed)
                                        .style(ThemePreviewStyle::Bordered),
                                )
                            }
                        }),
                )
                .child(
                    Label::new(FAMILY_NAMES[index].clone())
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                )
        })
    }

    fn write_mode_change(mode: ThemeAppearanceMode, cx: &mut App) {
        let fs = <dyn Fs>::global(cx);
        update_settings_file(fs, cx, move |settings, _cx| {
            theme_settings::set_mode(settings, mode);
        });
    }

    fn write_theme_change(
        theme: impl Into<Arc<str>>,
        theme_mode: ThemeAppearanceMode,
        cx: &mut App,
    ) {
        let fs = <dyn Fs>::global(cx);
        let theme = theme.into();
        update_settings_file(fs, cx, move |settings, cx| match theme_mode {
            ThemeAppearanceMode::System => {
                let (light_theme, dark_theme) =
                    get_theme_family_themes(&theme).unwrap_or((theme.as_ref(), theme.as_ref()));

                settings.theme.theme = Some(settings::ThemeSelection::Dynamic {
                    mode: ThemeAppearanceMode::System,
                    light: ThemeName(light_theme.into()),
                    dark: ThemeName(dark_theme.into()),
                });
            }
            ThemeAppearanceMode::Light => theme_settings::set_theme(
                settings,
                theme,
                Appearance::Light,
                *SystemAppearance::global(cx),
            ),
            ThemeAppearanceMode::Dark => theme_settings::set_theme(
                settings,
                theme,
                Appearance::Dark,
                *SystemAppearance::global(cx),
            ),
        });
    }
}

fn render_dez_workflow_section(tab_index: &mut isize, cx: &mut App) -> impl IntoElement {
    let colors = cx.theme().colors();
    let steps = [
        (
            IconName::Terminal,
            "Start",
            "Run work in an ordinary terminal or a pane-native agent.",
        ),
        (
            IconName::ListTree,
            "Watch",
            "Use the Session Rail to scan activity and attention without changing context.",
        ),
        (
            IconName::Diff,
            "Review",
            "Open observed commands, checks, and changes beside the owning session.",
        ),
    ];

    *tab_index += 1;
    let copy_hook_tab_index = *tab_index;
    *tab_index += 1;
    let new_terminal_tab_index = *tab_index;
    v_flex()
        .role(gpui::Role::Region)
        .aria_label("Terminal-first workflow")
        .w_full()
        .gap_3()
        .p_4()
        .rounded_md()
        .border_1()
        .border_color(colors.border_variant)
        .bg(colors.panel_background)
        .child(
            v_flex()
                .gap_0p5()
                .child(Label::new("Terminal-first workflow"))
                .child(
                    Label::new(
                        "Write. Delegate. Watch. Verify. Dez keeps the owning surface and its evidence close together.",
                    )
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                ),
        )
        .child(
            div()
                .role(gpui::Role::List)
                .aria_label("Terminal workflow steps")
                .w_full()
                .grid()
                .grid_cols(3)
                .gap_2()
                .children(steps.into_iter().map(|(icon, title, description)| {
                    v_flex()
                        .role(gpui::Role::ListItem)
                        .aria_label(format!("{title}. {description}"))
                        .min_w_0()
                        .gap_1()
                        .p_3()
                        .rounded_sm()
                        .border_1()
                        .border_color(colors.border_variant)
                        .child(
                            h_flex()
                                .gap_1p5()
                                .child(
                                    Icon::new(icon)
                                        .size(IconSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(Label::new(title)),
                        )
                        .child(
                            Label::new(description)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                })),
        )
        .child(
            v_flex()
                .w_full()
                .gap_2()
                .child(
                    Label::new(
                        "Close hides a view. Detach keeps a Host-owned session. Terminate ends the process. Persistence depends on the connected Host. Hooks are never installed automatically.",
                    )
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
                )
                .child(
                    h_flex()
                        .w_full()
                        .flex_wrap()
                        .gap_1()
                        .justify_end()
                        .child(
                            Button::new("onboarding-copy-codex-hook", "Copy Codex Hook")
                                .tab_index(copy_hook_tab_index)
                                .size(ButtonSize::Medium)
                                .style(ButtonStyle::Outlined)
                                .start_icon(Icon::new(IconName::Copy).size(IconSize::Small))
                                .aria_label("Copy Deliberate Codex Hook Setup")
                                .tooltip(Tooltip::text(
                                    "Copy the bundled setup; Dez does not install or modify Codex hooks",
                                ))
                                .on_click(|_, _window, cx| {
                                    cx.write_to_clipboard(ClipboardItem::new_string(
                                        CODEX_HOOK_SETUP.to_owned(),
                                    ));
                                }),
                        )
                        .child(
                            Button::new("onboarding-new-terminal", "New Terminal")
                                .tab_index(new_terminal_tab_index)
                                .size(ButtonSize::Medium)
                                .style(ButtonStyle::Filled)
                                .start_icon(Icon::new(IconName::Terminal).size(IconSize::Small))
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(
                                        Box::new(NewCenterTerminal { local: false }),
                                        cx,
                                    );
                                }),
                        ),
                ),
        )
}

fn render_telemetry_section(tab_index: &mut isize, cx: &App) -> impl IntoElement {
    let fs = <dyn Fs>::global(cx);

    v_flex()
        .gap_4()
        .child(
            SwitchField::new(
                "onboarding-telemetry-metrics",
                None::<&str>,
                Some(format!("Help improve {APP_NAME} by sending anonymous usage data").into()),
                if TelemetrySettings::get_global(cx).metrics {
                    ui::ToggleState::Selected
                } else {
                    ui::ToggleState::Unselected
                },
                {
                    let fs = fs.clone();
                    move |selection, _, cx| {
                        let enabled = match selection {
                            ToggleState::Selected => true,
                            ToggleState::Unselected => false,
                            ToggleState::Indeterminate => {
                                return;
                            }
                        };

                        update_settings_file(fs.clone(), cx, move |setting, _| {
                            setting.telemetry.get_or_insert_default().metrics = Some(enabled);
                        });

                        // This telemetry event shouldn't fire when it's off. If it does we'll be alerted
                        // and can fix it in a timely manner to respect a user's choice.
                        telemetry::event!(
                            "Welcome Page Telemetry Metrics Toggled",
                            options = if enabled { "on" } else { "off" }
                        );
                    }
                },
            )
            .tab_index({
                *tab_index += 1;
                *tab_index
            }),
        )
        .child(
            SwitchField::new(
                "onboarding-telemetry-crash-reports",
                None::<&str>,
                Some(
                    format!("Help fix {APP_NAME} by sending crash reports for critical issues")
                        .into(),
                ),
                if TelemetrySettings::get_global(cx).diagnostics {
                    ui::ToggleState::Selected
                } else {
                    ui::ToggleState::Unselected
                },
                {
                    let fs = fs.clone();
                    move |selection, _, cx| {
                        let enabled = match selection {
                            ToggleState::Selected => true,
                            ToggleState::Unselected => false,
                            ToggleState::Indeterminate => {
                                return;
                            }
                        };

                        update_settings_file(fs.clone(), cx, move |setting, _| {
                            setting.telemetry.get_or_insert_default().diagnostics = Some(enabled);
                        });

                        // This telemetry event shouldn't fire when it's off. If it does we'll be alerted
                        // and can fix it in a timely manner to respect a user's choice.
                        telemetry::event!(
                            "Welcome Page Telemetry Diagnostics Toggled",
                            options = if enabled { "on" } else { "off" }
                        );
                    }
                },
            )
            .tab_index({
                *tab_index += 1;
                *tab_index
            }),
        )
}

fn render_base_keymap_section(tab_index: &mut isize, cx: &mut App) -> impl IntoElement {
    let base_keymap = match BaseKeymap::get_global(cx) {
        BaseKeymap::VSCode => Some(0),
        BaseKeymap::JetBrains => Some(1),
        BaseKeymap::SublimeText => Some(2),
        BaseKeymap::Atom => Some(3),
        BaseKeymap::Emacs => Some(4),
        BaseKeymap::Cursor => Some(5),
        BaseKeymap::TextMate | BaseKeymap::None => None,
    };

    return v_flex().gap_2().child(Label::new("Base Keymap")).child(
        ToggleButtonGroup::two_rows(
            "base_keymap_selection",
            [
                ToggleButtonWithIcon::new("VS Code", IconName::EditorVsCode, |_, _, cx| {
                    write_keymap_base(BaseKeymap::VSCode, cx);
                }),
                ToggleButtonWithIcon::new("JetBrains", IconName::EditorJetBrains, |_, _, cx| {
                    write_keymap_base(BaseKeymap::JetBrains, cx);
                }),
                ToggleButtonWithIcon::new("Sublime Text", IconName::EditorSublime, |_, _, cx| {
                    write_keymap_base(BaseKeymap::SublimeText, cx);
                }),
            ],
            [
                ToggleButtonWithIcon::new("Atom", IconName::EditorAtom, |_, _, cx| {
                    write_keymap_base(BaseKeymap::Atom, cx);
                }),
                ToggleButtonWithIcon::new("Emacs", IconName::EditorEmacs, |_, _, cx| {
                    write_keymap_base(BaseKeymap::Emacs, cx);
                }),
                ToggleButtonWithIcon::new("Cursor", IconName::EditorCursor, |_, _, cx| {
                    write_keymap_base(BaseKeymap::Cursor, cx);
                }),
            ],
        )
        .when_some(base_keymap, |this, base_keymap| {
            this.selected_index(base_keymap)
        })
        .full_width()
        .tab_index(tab_index)
        .size(ui::ToggleButtonGroupSize::Medium)
        .style(ui::ToggleButtonGroupStyle::Outlined),
    );

    fn write_keymap_base(keymap_base: BaseKeymap, cx: &App) {
        let fs = <dyn Fs>::global(cx);

        update_settings_file(fs, cx, move |setting, _| {
            setting.base_keymap = Some(keymap_base.into());
        });

        telemetry::event!("Welcome Keymap Changed", keymap = keymap_base);
    }
}

fn render_vim_mode_switch(tab_index: &mut isize, cx: &mut App) -> impl IntoElement {
    let toggle_state = if VimModeSetting::get_global(cx).0 {
        ui::ToggleState::Selected
    } else {
        ui::ToggleState::Unselected
    };
    SwitchField::new(
        "onboarding-vim-mode",
        Some("Vim Mode"),
        Some("Coming from Neovim? Use our first-class implementation of Vim Mode".into()),
        toggle_state,
        {
            let fs = <dyn Fs>::global(cx);
            move |&selection, _, cx| {
                let vim_mode = match selection {
                    ToggleState::Selected => true,
                    ToggleState::Unselected => false,
                    ToggleState::Indeterminate => {
                        return;
                    }
                };
                update_settings_file(fs.clone(), cx, move |setting, _| {
                    setting.vim_mode = Some(vim_mode);
                });

                telemetry::event!(
                    "Welcome Vim Mode Toggled",
                    options = if vim_mode { "on" } else { "off" },
                );
            }
        },
    )
    .tab_index({
        *tab_index += 1;
        *tab_index - 1
    })
}

fn render_worktree_auto_trust_switch(tab_index: &mut isize, cx: &mut App) -> impl IntoElement {
    let toggle_state = if ProjectSettings::get_global(cx).session.trust_all_worktrees {
        ui::ToggleState::Selected
    } else {
        ui::ToggleState::Unselected
    };

    let tooltip_description = format!(
        "{APP_NAME} only runs language servers, project settings, and MCP servers after you trust a new workspace."
    );

    SwitchField::new(
        "onboarding-auto-trust-worktrees",
        Some("Trust All Projects By Default"),
        Some("Automatically trust new workspaces and allow their developer services to run".into()),
        toggle_state,
        {
            let fs = <dyn Fs>::global(cx);
            move |&selection, _, cx| {
                let trust = match selection {
                    ToggleState::Selected => true,
                    ToggleState::Unselected => false,
                    ToggleState::Indeterminate => {
                        return;
                    }
                };
                update_settings_file(fs.clone(), cx, move |setting, _| {
                    setting.session.get_or_insert_default().trust_all_worktrees = Some(trust);
                });

                telemetry::event!(
                    "Welcome Page Worktree Auto Trust Toggled",
                    options = if trust { "on" } else { "off" }
                );
            }
        },
    )
    .tab_index({
        *tab_index += 1;
        *tab_index - 1
    })
    .tooltip(Tooltip::text(tooltip_description))
}

fn render_setting_import_button(
    tab_index: isize,
    label: SharedString,
    action: &dyn Action,
    imported: bool,
) -> impl IntoElement + 'static {
    let action = action.boxed_clone();

    Button::new(label.clone(), label.clone())
        .style(ButtonStyle::OutlinedGhost)
        .size(ButtonSize::Medium)
        .label_size(LabelSize::Small)
        .selected_style(ButtonStyle::Tinted(TintColor::Accent))
        .toggle_state(imported)
        .tab_index(tab_index)
        .when(imported, |this| {
            this.end_icon(Icon::new(IconName::Check).size(IconSize::Small))
                .color(Color::Success)
        })
        .on_click(move |_, window, cx| {
            telemetry::event!("Welcome Import Settings", import_source = label,);
            window.dispatch_action(action.boxed_clone(), cx);
        })
}

fn render_import_settings_section(tab_index: &mut isize, cx: &mut App) -> impl IntoElement {
    let import_state = SettingsImportState::global(cx);
    let imports: [(SharedString, &dyn Action, bool); 2] = [
        (
            "VS Code".into(),
            &ImportVsCodeSettings { skip_prompt: false },
            import_state.vscode,
        ),
        (
            "Cursor".into(),
            &ImportCursorSettings { skip_prompt: false },
            import_state.cursor,
        ),
    ];

    let [vscode, cursor] = imports.map(|(label, action, imported)| {
        *tab_index += 1;
        render_setting_import_button(*tab_index - 1, label, action, imported)
    });

    h_flex()
        .gap_2()
        .flex_wrap()
        .justify_between()
        .child(
            v_flex()
                .gap_0p5()
                .max_w_5_6()
                .child(Label::new("Import Settings"))
                .child(
                    Label::new("Automatically pull your settings from other editors")
                        .color(Color::Muted),
                ),
        )
        .child(h_flex().gap_1().child(vscode).child(cursor))
}

pub(crate) const FEATURED_AGENT_IDS: &[&str] =
    &["claude-acp", "codex-acp", "github-copilot-cli", "cursor"];

fn render_registry_agent_button(
    agent: &RegistryAgent,
    installed: bool,
    cx: &mut App,
) -> impl IntoElement {
    let agent_id = agent.id().to_string();
    let element_id = format!("{}-onboarding", agent_id);

    let icon = match agent.icon_path() {
        Some(icon_path) => Icon::from_external_svg(icon_path.clone()),
        None => Icon::new(IconName::Sparkle),
    }
    .size(IconSize::XSmall)
    .color(Color::Muted);

    let fs = <dyn Fs>::global(cx);

    let state_element = if installed {
        Icon::new(IconName::Check)
            .size(IconSize::Small)
            .color(Color::Success)
            .into_any_element()
    } else {
        Label::new("Install")
            .size(LabelSize::XSmall)
            .color(Color::Muted)
            .into_any_element()
    };

    AgentSetupButton::new(element_id)
        .icon(icon)
        .name(agent.name().clone())
        .state(state_element)
        .disabled(installed)
        .on_click(move |_, window, cx| {
            telemetry::event!("Welcome Agent Install Clicked", agent = agent_id.as_str());
            update_settings_file(fs.clone(), cx, {
                let agent_id = agent_id.clone();
                move |settings, _| {
                    let agent_servers = settings.agent_servers.get_or_insert_default();
                    agent_servers.entry(agent_id).or_insert_with(|| {
                        CustomAgentServerSettings::Registry {
                            env: Default::default(),
                            default_mode: None,
                            default_config_options: HashMap::default(),
                            favorite_config_option_values: HashMap::default(),
                        }
                    });
                }
            });
            window.dispatch_action(
                Box::new(zed_actions::agent::SelectAgent {
                    agent: agent_id.clone(),
                }),
                cx,
            );
        })
}

fn render_ai_section(cx: &mut App) -> impl IntoElement {
    let registry_agents = AgentRegistryStore::try_global(cx)
        .map(|store| store.read(cx).agents().to_vec())
        .unwrap_or_default();

    let installed_agents = cx
        .global::<SettingsStore>()
        .get::<AllAgentServersSettings>(None)
        .clone();

    let column_count = FEATURED_AGENT_IDS.len() as u16;

    let grid = FEATURED_AGENT_IDS.iter().fold(
        div()
            .w_full()
            .mt_1p5()
            .grid()
            .grid_cols(column_count)
            .gap_2(),
        |grid, agent_id| {
            let Some(agent) = registry_agents
                .iter()
                .find(|a| a.id().as_ref() == *agent_id)
            else {
                return grid;
            };
            let is_installed = installed_agents.contains_key(*agent_id);
            grid.child(render_registry_agent_button(agent, is_installed, cx))
        },
    );

    v_flex()
        .gap_0p5()
        .child(Label::new("Optional ACP Agents"))
        .child(
            Label::new(
                "Connect pane-native agents here. Terminal agents such as Codex remain ordinary terminal sessions and appear in the Session Rail.",
            )
            .color(Color::Muted),
        )
        .child(grid)
}

pub(crate) fn render_basics_page(cx: &mut App) -> impl IntoElement {
    let mut tab_index = 0;

    v_flex()
        .id("basics-page")
        .gap_6()
        .child(render_dez_workflow_section(&mut tab_index, cx))
        .child(render_theme_section(&mut tab_index, cx))
        .child(render_base_keymap_section(&mut tab_index, cx))
        .child(render_ai_section(cx))
        .child(render_import_settings_section(&mut tab_index, cx))
        .child(render_vim_mode_switch(&mut tab_index, cx))
        .child(render_worktree_auto_trust_switch(&mut tab_index, cx))
        .child(Divider::horizontal().color(ui::DividerColor::BorderVariant))
        .child(render_telemetry_section(&mut tab_index, cx))
}
