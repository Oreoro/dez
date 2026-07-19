use std::rc::Rc;

use gpui::{
    DismissEvent, Div, Entity, EventEmitter, FocusHandle, Focusable, Hsla, IntoElement, Pixels,
    Stateful,
};
use settings::Settings;
use ui::{Tooltip, prelude::*};
use workspace::{DesignSystemSettings, ToastAction, ToastView};
use zed_actions::toast;

fn status_toast_background(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.surface_background.opacity(0.94),
        settings::CanvasContrast::Standard => colors.surface_background,
        settings::CanvasContrast::High => colors
            .elevated_surface_background
            .blend(colors.border_focused.opacity(0.08)),
    }
}

fn status_toast_border(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.border.opacity(0.42),
        settings::CanvasContrast::Standard => colors.border_variant,
        settings::CanvasContrast::High => colors.border_focused,
    }
}

fn status_toast_gap(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(6.),
        settings::CanvasDensity::Balanced => px(8.),
        settings::CanvasDensity::Spacious => px(12.),
    }
}

fn status_toast_padding_y(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(4.),
        settings::CanvasDensity::Balanced => px(6.),
        settings::CanvasDensity::Spacious => px(8.),
    }
}

fn status_toast_padding_x(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(8.),
        settings::CanvasDensity::Balanced => px(10.),
        settings::CanvasDensity::Spacious => px(14.),
    }
}

fn status_toast_trailing_padding(has_action_or_dismiss: bool, cx: &App) -> Pixels {
    let base = match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(6.),
        settings::CanvasDensity::Balanced => px(10.),
        settings::CanvasDensity::Spacious => px(14.),
    };

    if has_action_or_dismiss {
        base
    } else {
        status_toast_padding_x(cx)
    }
}

fn status_toast_radius(element: Stateful<Div>, cx: &App) -> Stateful<Div> {
    match DesignSystemSettings::get_global(cx).radius {
        settings::CanvasRadius::None => element,
        settings::CanvasRadius::Subtle => element.rounded_md(),
        settings::CanvasRadius::Rounded => element.rounded_lg(),
    }
}

#[derive(RegisterComponent)]
pub struct StatusToast {
    icon: Option<Icon>,
    text: SharedString,
    action: Option<ToastAction>,
    show_dismiss: bool,
    auto_dismiss: bool,
    this_handle: Entity<Self>,
    focus_handle: FocusHandle,
}

impl StatusToast {
    pub fn new(
        text: impl Into<SharedString>,
        cx: &mut App,
        f: impl FnOnce(Self, &mut Context<Self>) -> Self,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let focus_handle = cx.focus_handle();

            f(
                Self {
                    text: text.into(),
                    icon: None,
                    action: None,
                    show_dismiss: false,
                    auto_dismiss: true,
                    this_handle: cx.entity(),
                    focus_handle,
                },
                cx,
            )
        })
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn auto_dismiss(mut self, auto_dismiss: bool) -> Self {
        self.auto_dismiss = auto_dismiss;
        self
    }

    pub fn action(
        mut self,
        label: impl Into<SharedString>,
        f: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        let this_handle = self.this_handle.clone();
        self.action = Some(ToastAction::new(
            label.into(),
            Some(Rc::new(move |window, cx| {
                this_handle.update(cx, |_, cx| {
                    cx.emit(DismissEvent);
                });
                f(window, cx);
            })),
        ));
        self
    }

    pub fn dismiss_button(mut self, show: bool) -> Self {
        self.show_dismiss = show;
        self
    }
}

impl Render for StatusToast {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_action_or_dismiss = self.action.is_some() || self.show_dismiss;

        h_flex()
            .id("status-toast")
            .elevation_3(cx)
            .gap(status_toast_gap(cx))
            .py(status_toast_padding_y(cx))
            .pl(status_toast_padding_x(cx))
            .pr(status_toast_trailing_padding(has_action_or_dismiss, cx))
            .flex_none()
            .bg(status_toast_background(cx))
            .border_1()
            .border_color(status_toast_border(cx))
            .map(|this| status_toast_radius(this, cx))
            .shadow_lg()
            .when_some(self.icon.clone(), |this, icon| this.child(icon))
            .child(Label::new(self.text.clone()).color(Color::Default))
            .when_some(self.action.as_ref(), |this, action| {
                this.child(
                    Button::new(action.id.clone(), action.label.clone())
                        .tooltip(Tooltip::for_action_title(
                            action.label.clone(),
                            &toast::RunAction,
                        ))
                        .color(Color::Muted)
                        .when_some(action.on_click.clone(), |el, handler| {
                            el.on_click(move |_click_event, window, cx| handler(window, cx))
                        }),
                )
            })
            .when(self.show_dismiss, |this| {
                let handle = self.this_handle.clone();
                this.child(
                    IconButton::new("dismiss", IconName::Close)
                        .shape(ui::IconButtonShape::Square)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted)
                        .tooltip(Tooltip::text("Dismiss"))
                        .on_click(move |_click_event, _window, cx| {
                            handle.update(cx, |_, cx| {
                                cx.emit(DismissEvent);
                            });
                        }),
                )
            })
    }
}

impl ToastView for StatusToast {
    fn action(&self) -> Option<ToastAction> {
        self.action.clone()
    }

    fn auto_dismiss(&self) -> bool {
        self.auto_dismiss
    }
}

impl Focusable for StatusToast {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for StatusToast {}

impl Component for StatusToast {
    fn scope() -> ComponentScope {
        ComponentScope::Notification
    }

    fn description() -> &'static str {
        "A compact, transient toast used to surface status updates \
        such as completed operations or pending updates, with optional icon, \
        action, and dismiss affordances."
    }

    fn preview(_window: &mut Window, cx: &mut App) -> AnyElement {
        let text_example = StatusToast::new("Operation completed", cx, |this, _| this);

        let action_example = StatusToast::new("Update ready to install", cx, |this, _cx| {
            this.action("Restart", |_, _| {})
        });

        let dismiss_button_example =
            StatusToast::new("Dismiss Button", cx, |this, _| this.dismiss_button(true));

        let icon_example = StatusToast::new(
            "Nathan Sobo accepted your contact request",
            cx,
            |this, _| {
                this.icon(
                    Icon::new(IconName::Check)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
            },
        );

        let success_example = StatusToast::new("Pushed 4 changes to `zed/main`", cx, |this, _| {
            this.icon(
                Icon::new(IconName::Check)
                    .size(IconSize::Small)
                    .color(Color::Success),
            )
        });

        let error_example = StatusToast::new(
            "git push: Couldn't find remote origin `iamnbutler/zed`",
            cx,
            |this, _cx| {
                this.icon(
                    Icon::new(IconName::XCircle)
                        .size(IconSize::Small)
                        .color(Color::Error),
                )
                .action("More Info", |_, _| {})
            },
        );

        let warning_example = StatusToast::new("You have outdated settings", cx, |this, _cx| {
            this.icon(
                Icon::new(IconName::Warning)
                    .size(IconSize::Small)
                    .color(Color::Warning),
            )
            .action("More Info", |_, _| {})
        });

        let pr_example =
            StatusToast::new("`zed/new-notification-system` created!", cx, |this, _cx| {
                this.icon(
                    Icon::new(IconName::GitBranch)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .action("Open Pull Request", |_, cx| {
                    cx.open_url("https://github.com/")
                })
            });

        v_flex()
            .gap_6()
            .p_4()
            .children(vec![
                example_group_with_title(
                    "Basic Toast",
                    vec![
                        single_example("Text", div().child(text_example).into_any_element()),
                        single_example("Action", div().child(action_example).into_any_element()),
                        single_example("Icon", div().child(icon_example).into_any_element()),
                        single_example(
                            "Dismiss Button",
                            div().child(dismiss_button_example).into_any_element(),
                        ),
                    ],
                ),
                example_group_with_title(
                    "Examples",
                    vec![
                        single_example("Success", div().child(success_example).into_any_element()),
                        single_example("Error", div().child(error_example).into_any_element()),
                        single_example("Warning", div().child(warning_example).into_any_element()),
                        single_example("Create PR", div().child(pr_example).into_any_element()),
                    ],
                )
                .vertical(),
            ])
            .into_any_element()
    }
}
