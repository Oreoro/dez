use gpui::{App, Div, Hsla, Pixels, prelude::*, px};
use settings::Settings;
use ui::ActiveTheme;
use workspace::DesignSystemSettings;

pub(crate) fn prompt_backdrop(cx: &App) -> Hsla {
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => gpui::black().opacity(0.16),
        settings::CanvasContrast::Standard => gpui::black().opacity(0.2),
        settings::CanvasContrast::High => gpui::black().opacity(0.28),
    }
}

pub(crate) fn prompt_background(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.elevated_surface_background.opacity(0.94),
        settings::CanvasContrast::Standard => colors.elevated_surface_background,
        settings::CanvasContrast::High => colors
            .elevated_surface_background
            .blend(colors.border_focused.opacity(0.08)),
    }
}

pub(crate) fn prompt_border(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.border_variant.opacity(0.45),
        settings::CanvasContrast::Standard => colors.border_variant,
        settings::CanvasContrast::High => colors.border_focused,
    }
}

pub(crate) fn prompt_padding(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(12.),
        settings::CanvasDensity::Balanced => px(16.),
        settings::CanvasDensity::Spacious => px(24.),
    }
}

pub(crate) fn prompt_gap(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(12.),
        settings::CanvasDensity::Balanced => px(16.),
        settings::CanvasDensity::Spacious => px(20.),
    }
}

pub(crate) fn prompt_action_gap(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(4.),
        settings::CanvasDensity::Balanced => px(6.),
        settings::CanvasDensity::Spacious => px(8.),
    }
}

pub(crate) fn prompt_radius(element: Div, cx: &App) -> Div {
    match DesignSystemSettings::get_global(cx).radius {
        settings::CanvasRadius::None => element,
        settings::CanvasRadius::Subtle => element.rounded_md(),
        settings::CanvasRadius::Rounded => element.rounded_lg(),
    }
}
