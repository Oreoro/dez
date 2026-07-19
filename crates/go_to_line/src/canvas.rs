use gpui::{App, Div, Hsla, Pixels, Stateful, prelude::*, px};
use settings::Settings;
use workspace::DesignSystemSettings;

pub(crate) fn go_to_line_background(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.elevated_surface_background.opacity(0.92),
        settings::CanvasContrast::Standard => colors.elevated_surface_background,
        settings::CanvasContrast::High => colors
            .elevated_surface_background
            .blend(colors.border_focused.opacity(0.06)),
    }
}

pub(crate) fn go_to_line_border(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.border_variant.opacity(0.45),
        settings::CanvasContrast::Standard => colors.border_variant,
        settings::CanvasContrast::High => colors.border_focused,
    }
}

pub(crate) fn go_to_line_padding_x(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(8.),
        settings::CanvasDensity::Balanced => px(10.),
        settings::CanvasDensity::Spacious => px(14.),
    }
}

pub(crate) fn go_to_line_padding_y(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(4.),
        settings::CanvasDensity::Balanced => px(6.),
        settings::CanvasDensity::Spacious => px(8.),
    }
}

pub(crate) fn go_to_line_gap(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(4.),
        settings::CanvasDensity::Balanced => px(6.),
        settings::CanvasDensity::Spacious => px(8.),
    }
}

pub(crate) fn go_to_line_radius(element: Stateful<Div>, cx: &App) -> Stateful<Div> {
    match DesignSystemSettings::get_global(cx).radius {
        settings::CanvasRadius::None => element,
        settings::CanvasRadius::Subtle => element.rounded_md(),
        settings::CanvasRadius::Rounded => element.rounded_lg(),
    }
}
