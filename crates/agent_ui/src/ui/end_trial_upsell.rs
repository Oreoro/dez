use std::sync::Arc;

use ai_onboarding::{AgentPanelOnboardingCard, PlanDefinitions};
use client::zed_urls;
use gpui::{AnyElement, App, IntoElement, Pixels, RenderOnce, Window};
use settings::Settings as _;
use ui::{Divider, Tooltip, prelude::*};
use workspace::DesignSystemSettings;

fn upsell_section_gap(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(4.),
        settings::CanvasDensity::Balanced => px(6.),
        settings::CanvasDensity::Spacious => px(8.),
    }
}

fn upsell_header_gap(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(6.),
        settings::CanvasDensity::Balanced => px(8.),
        settings::CanvasDensity::Spacious => px(12.),
    }
}

fn upsell_free_section_margin_top(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(4.),
        settings::CanvasDensity::Balanced => px(6.),
        settings::CanvasDensity::Spacious => px(10.),
    }
}

fn upsell_description_margin_bottom(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(6.),
        settings::CanvasDensity::Balanced => px(8.),
        settings::CanvasDensity::Spacious => px(12.),
    }
}

fn upsell_dismiss_offset(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(10.),
        settings::CanvasDensity::Balanced => px(16.),
        settings::CanvasDensity::Spacious => px(20.),
    }
}

fn upsell_current_plan_color(cx: &App) -> Color {
    let colors = cx.theme().colors();
    let opacity = match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => 0.5,
        settings::CanvasContrast::Standard => 0.6,
        settings::CanvasContrast::High => 0.78,
    };
    Color::Custom(colors.text_muted.opacity(opacity))
}

#[derive(IntoElement, RegisterComponent)]
pub struct EndTrialUpsell {
    dismiss_upsell: Arc<dyn Fn(&mut Window, &mut App)>,
}

impl EndTrialUpsell {
    pub fn new(dismiss_upsell: Arc<dyn Fn(&mut Window, &mut App)>) -> Self {
        Self { dismiss_upsell }
    }
}

impl RenderOnce for EndTrialUpsell {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let pro_section = v_flex()
            .gap(upsell_section_gap(cx))
            .child(
                h_flex()
                    .gap(upsell_header_gap(cx))
                    .child(
                        Label::new("Pro")
                            .size(LabelSize::Small)
                            .color(Color::Accent)
                            .buffer_font(cx),
                    )
                    .child(Divider::horizontal()),
            )
            .child(PlanDefinitions.pro_plan())
            .child(
                Button::new("cta-button", "Upgrade to Zed Pro")
                    .full_width()
                    .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                    .on_click(move |_, _window, cx| {
                        telemetry::event!("Upgrade To Pro Clicked", state = "end-of-trial");
                        cx.open_url(&zed_urls::upgrade_to_zed_pro_url(cx))
                    }),
            );

        let free_section = v_flex()
            .mt(upsell_free_section_margin_top(cx))
            .gap(upsell_section_gap(cx))
            .child(
                h_flex()
                    .gap(upsell_header_gap(cx))
                    .child(
                        Label::new("Free")
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .buffer_font(cx),
                    )
                    .child(
                        Label::new("(Current Plan)")
                            .size(LabelSize::Small)
                            .color(upsell_current_plan_color(cx))
                            .buffer_font(cx),
                    )
                    .child(Divider::horizontal()),
            )
            .child(PlanDefinitions.free_plan());

        AgentPanelOnboardingCard::new()
            .child(Headline::new("Your Zed Pro Trial has expired"))
            .child(
                Label::new("You've been automatically reset to the Free plan.")
                    .color(Color::Muted)
                    .mb(upsell_description_margin_bottom(cx)),
            )
            .child(pro_section)
            .child(free_section)
            .child(
                h_flex()
                    .absolute()
                    .top(upsell_dismiss_offset(cx))
                    .right(upsell_dismiss_offset(cx))
                    .child(
                        IconButton::new("dismiss_onboarding", IconName::Close)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Dismiss"))
                            .on_click({
                                let callback = self.dismiss_upsell.clone();
                                move |_, window, cx| {
                                    telemetry::event!("Banner Dismissed", source = "AI Onboarding");
                                    callback(window, cx)
                                }
                            }),
                    ),
            )
    }
}

impl Component for EndTrialUpsell {
    fn scope() -> ComponentScope {
        ComponentScope::Onboarding
    }

    fn name() -> &'static str {
        "End of Trial Upsell Banner"
    }

    fn sort_name() -> &'static str {
        "End of Trial Upsell Banner"
    }

    fn description() -> &'static str {
        "A banner shown in the agent panel when a user's trial has ended, \
        inviting them to upgrade to a paid plan to continue using the agent."
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> AnyElement {
        v_flex()
            .child(EndTrialUpsell {
                dismiss_upsell: Arc::new(|_, _| {}),
            })
            .into_any_element()
    }
}
