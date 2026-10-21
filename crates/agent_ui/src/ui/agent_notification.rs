use gpui::{
    App, Context, Div, EventEmitter, Hsla, IntoElement, Pixels, PlatformDisplay, Size, Stateful,
    Window, WindowBackgroundAppearance, WindowBounds, WindowDecorations, WindowKind, WindowOptions,
    linear_color_stop, linear_gradient, point,
};
use release_channel::ReleaseChannel;
use settings::Settings;
use std::rc::Rc;
use ui::{Render, prelude::*};
use workspace::DesignSystemSettings;

fn notification_window_size(cx: &App) -> Size<Pixels> {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => Size {
            width: px(420.),
            height: px(64.),
        },
        settings::CanvasDensity::Balanced => Size {
            width: px(450.),
            height: px(72.),
        },
        settings::CanvasDensity::Spacious => Size {
            width: px(520.),
            height: px(88.),
        },
    }
}

fn notification_window_margin(cx: &App) -> (Pixels, Pixels) {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => (px(12.), px(-40.)),
        settings::CanvasDensity::Balanced => (px(16.), px(-48.)),
        settings::CanvasDensity::Spacious => (px(24.), px(-56.)),
    }
}

fn notification_background(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.elevated_surface_background.opacity(0.94),
        settings::CanvasContrast::Standard => colors.elevated_surface_background,
        settings::CanvasContrast::High => colors
            .elevated_surface_background
            .blend(colors.border_focused.opacity(0.08)),
    }
}

fn notification_border(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.border.opacity(0.45),
        settings::CanvasContrast::Standard => colors.border,
        settings::CanvasContrast::High => colors.border_focused,
    }
}

fn notification_padding(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(8.),
        settings::CanvasDensity::Balanced => px(12.),
        settings::CanvasDensity::Spacious => px(16.),
    }
}

fn notification_gap(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(10.),
        settings::CanvasDensity::Balanced => px(16.),
        settings::CanvasDensity::Spacious => px(20.),
    }
}

fn notification_inner_gap(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(6.),
        settings::CanvasDensity::Balanced => px(8.),
        settings::CanvasDensity::Spacious => px(10.),
    }
}

fn notification_button_gap(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(4.),
        settings::CanvasDensity::Balanced => px(4.),
        settings::CanvasDensity::Spacious => px(6.),
    }
}

fn notification_text_width(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(260.),
        settings::CanvasDensity::Balanced => px(300.),
        settings::CanvasDensity::Spacious => px(340.),
    }
}

fn notification_radius(element: Stateful<Div>, cx: &App) -> Stateful<Div> {
    match DesignSystemSettings::get_global(cx).radius {
        settings::CanvasRadius::None => element,
        settings::CanvasRadius::Subtle => element.rounded_lg(),
        settings::CanvasRadius::Rounded => element.rounded_xl(),
    }
}

pub struct AgentNotification {
    title: SharedString,
    caption: Option<SharedString>,
    icon: IconName,
    project_name: Option<SharedString>,
}

impl AgentNotification {
    pub fn new(
        title: impl Into<SharedString>,
        caption: Option<SharedString>,
        icon: IconName,
        project_name: Option<impl Into<SharedString>>,
    ) -> Self {
        Self {
            title: title.into(),
            caption: caption,
            icon,
            project_name: project_name.map(|name| name.into()),
        }
    }

    pub fn window_options(screen: Rc<dyn PlatformDisplay>, cx: &App) -> WindowOptions {
        let size = notification_window_size(cx);
        let (notification_margin_width, notification_margin_height) =
            notification_window_margin(cx);

        let bounds = gpui::Bounds::<Pixels> {
            origin: screen.bounds().top_right()
                - point(
                    size.width + notification_margin_width,
                    notification_margin_height,
                ),
            size,
        };

        let app_id = ReleaseChannel::global(cx).app_id();

        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: None,
            focus: false,
            show: true,
            kind: WindowKind::PopUp,
            is_movable: false,
            display_id: Some(screen.id()),
            window_background: WindowBackgroundAppearance::Transparent,
            app_id: Some(app_id.to_owned()),
            window_min_size: None,
            window_decorations: Some(WindowDecorations::Client),
            tabbing_identifier: None,
            ..Default::default()
        }
    }
}

pub enum AgentNotificationEvent {
    Accepted,
    Dismissed,
}

impl EventEmitter<AgentNotificationEvent> for AgentNotification {}

impl AgentNotification {
    pub fn accept(&mut self, cx: &mut Context<Self>) {
        cx.emit(AgentNotificationEvent::Accepted);
    }

    pub fn dismiss(&mut self, cx: &mut Context<Self>) {
        cx.emit(AgentNotificationEvent::Dismissed);
    }
}

impl Render for AgentNotification {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font = theme_settings::setup_ui_font(window, cx);
        let line_height = window.line_height();

        let bg = notification_background(cx);
        let gradient_overflow = || {
            div()
                .h_full()
                .absolute()
                .w_8()
                .bottom_0()
                .right_0()
                .bg(linear_gradient(
                    90.,
                    linear_color_stop(bg, 1.),
                    linear_color_stop(bg.opacity(0.2), 0.),
                ))
        };

        h_flex()
            .id("agent-notification")
            .size_full()
            .p(notification_padding(cx))
            .gap(notification_gap(cx))
            .justify_between()
            .elevation_3(cx)
            .text_ui(cx)
            .font(ui_font)
            .bg(bg)
            .border_1()
            .border_color(notification_border(cx))
            .map(|this| notification_radius(this, cx))
            .child(
                h_flex()
                    .items_start()
                    .gap(notification_inner_gap(cx))
                    .flex_1()
                    .child(
                        h_flex().h(line_height).justify_center().child(
                            Icon::new(self.icon)
                                .color(Color::Muted)
                                .size(IconSize::Small),
                        ),
                    )
                    .child(
                        v_flex()
                            .flex_1()
                            .max_w(notification_text_width(cx))
                            .child(
                                div()
                                    .relative()
                                    .text_size(px(14.))
                                    .text_color(cx.theme().colors().text)
                                    .truncate()
                                    .child(self.title.clone())
                                    .child(gradient_overflow()),
                            )
                            .child(
                                h_flex()
                                    .relative()
                                    .gap(notification_button_gap(cx))
                                    .text_size(px(12.))
                                    .text_color(cx.theme().colors().text_muted)
                                    .truncate()
                                    .when_some(
                                        self.project_name.clone(),
                                        |description, project_name| {
                                            let has_caption = self.caption.is_some();
                                            let project = div()
                                                .truncate()
                                                .when(has_caption, |this| this.max_w_16())
                                                .child(project_name);
                                            let mut row = h_flex().gap_1p5().child(project);
                                            if has_caption {
                                                row = row.child(
                                                    div().size(px(3.)).rounded_full().bg(cx
                                                        .theme()
                                                        .colors()
                                                        .text
                                                        .opacity(0.5)),
                                                );
                                            }
                                            description.child(row)
                                        },
                                    )
                                    .when_some(self.caption.clone(), |description, caption| {
                                        description.child(caption)
                                    })
                                    .child(gradient_overflow()),
                            ),
                    ),
            )
            .child(
                v_flex()
                    .gap(notification_button_gap(cx))
                    .items_center()
                    .child(
                        Button::new("open", "View")
                            .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                            .full_width()
                            .on_click({
                                cx.listener(move |this, _event, _, cx| {
                                    this.accept(cx);
                                })
                            }),
                    )
                    .child(Button::new("dismiss", "Dismiss").full_width().on_click({
                        cx.listener(move |this, _event, _, cx| {
                            this.dismiss(cx);
                        })
                    })),
            )
    }
}
