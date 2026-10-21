use gpui::{App, Div, Hsla, Pixels, prelude::*, px};
use settings::Settings;
use ui::ActiveTheme;
use workspace::DesignSystemSettings;

pub(crate) fn preview_background(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.editor_background,
        settings::CanvasContrast::Standard => colors.editor_background,
        settings::CanvasContrast::High => colors.element_background,
    }
}

pub(crate) fn preview_panel_background(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.surface_background,
        settings::CanvasContrast::Standard => colors.surface_background,
        settings::CanvasContrast::High => colors.panel_background,
    }
}

pub(crate) fn preview_border(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.border.opacity(0.5),
        settings::CanvasContrast::Standard => colors.border,
        settings::CanvasContrast::High => colors.border_variant,
    }
}

pub(crate) fn preview_subtle_border(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.border_variant.opacity(0.45),
        settings::CanvasContrast::Standard => colors.border_variant,
        settings::CanvasContrast::High => colors.border,
    }
}

pub(crate) fn preview_toolbar_padding(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(6.),
        settings::CanvasDensity::Balanced => px(8.),
        settings::CanvasDensity::Spacious => px(12.),
    }
}

pub(crate) fn preview_toolbar_gap(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(8.),
        settings::CanvasDensity::Balanced => px(12.),
        settings::CanvasDensity::Spacious => px(16.),
    }
}

pub(crate) fn preview_cell_padding_x(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(3.),
        settings::CanvasDensity::Balanced => px(4.),
        settings::CanvasDensity::Spacious => px(6.),
    }
}

pub(crate) fn preview_radius(element: Div, cx: &App) -> Div {
    match DesignSystemSettings::get_global(cx).radius {
        settings::CanvasRadius::None => element,
        settings::CanvasRadius::Subtle => element.rounded_sm(),
        settings::CanvasRadius::Rounded => element.rounded_md(),
    }
}
