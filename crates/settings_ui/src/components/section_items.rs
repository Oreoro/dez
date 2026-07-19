use gpui::{App, IntoElement, ParentElement, Pixels, Role, Styled, px};
use settings::Settings;
use ui::{Divider, DividerColor, prelude::*};
use workspace::DesignSystemSettings;

fn canvas_section_padding_x(density: settings::CanvasDensity) -> Pixels {
    match density {
        settings::CanvasDensity::Compact => px(18.),
        settings::CanvasDensity::Balanced => px(24.),
        settings::CanvasDensity::Spacious => px(30.),
    }
}

fn canvas_section_gap(density: settings::CanvasDensity) -> Pixels {
    match density {
        settings::CanvasDensity::Compact => px(4.),
        settings::CanvasDensity::Balanced => px(6.),
        settings::CanvasDensity::Spacious => px(8.),
    }
}

fn canvas_section_divider_color(contrast: settings::CanvasContrast) -> DividerColor {
    match contrast {
        settings::CanvasContrast::Low => DividerColor::BorderFaded,
        settings::CanvasContrast::Standard => DividerColor::BorderFaded,
        settings::CanvasContrast::High => DividerColor::Border,
    }
}

#[derive(IntoElement)]
pub struct SettingsSectionHeader {
    icon: Option<IconName>,
    label: SharedString,
    no_padding: bool,
}

impl SettingsSectionHeader {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            no_padding: false,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn no_padding(mut self, no_padding: bool) -> Self {
        self.no_padding = no_padding;
        self
    }
}

impl RenderOnce for SettingsSectionHeader {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let design_system = DesignSystemSettings::get_global(cx);
        let label_text = self.label.clone();
        let label = Label::new(self.label)
            .size(LabelSize::Small)
            .color(Color::Muted)
            .buffer_font(cx);

        v_flex()
            .id(label_text.clone())
            .role(Role::Heading)
            .aria_level(2)
            .aria_label(label_text)
            .w_full()
            .when(!self.no_padding, |this| {
                this.px(canvas_section_padding_x(design_system.density))
            })
            .gap(canvas_section_gap(design_system.density))
            .map(|this| {
                if let Some(icon) = self.icon {
                    this.child(
                        h_flex()
                            .gap_1p5()
                            .child(Icon::new(icon).color(Color::Muted))
                            .child(label),
                    )
                } else {
                    this.child(label)
                }
            })
            .child(
                Divider::horizontal().color(canvas_section_divider_color(design_system.contrast)),
            )
    }
}
