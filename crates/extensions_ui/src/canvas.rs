use gpui::{App, Div, Hsla, Pixels, prelude::*, px};
use settings::Settings;
use ui::ActiveTheme;
use workspace::DesignSystemSettings;

pub(crate) fn extensions_background(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.editor_background,
        settings::CanvasContrast::Standard => colors.editor_background,
        settings::CanvasContrast::High => colors.element_background,
    }
}

pub(crate) fn extensions_panel_background(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.editor_background,
        settings::CanvasContrast::Standard => colors.editor_background,
        settings::CanvasContrast::High => colors.panel_background,
    }
}

pub(crate) fn extensions_card_background(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.elevated_surface_background.opacity(0.4),
        settings::CanvasContrast::Standard => colors.elevated_surface_background.opacity(0.5),
        settings::CanvasContrast::High => colors.elevated_surface_background.opacity(0.75),
    }
}

pub(crate) fn extensions_card_overlay_background(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.elevated_surface_background.alpha(0.72),
        settings::CanvasContrast::Standard => colors.elevated_surface_background.alpha(0.8),
        settings::CanvasContrast::High => colors.element_background.alpha(0.88),
    }
}

pub(crate) fn extensions_border(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.border_variant.opacity(0.45),
        settings::CanvasContrast::Standard => colors.border_variant,
        settings::CanvasContrast::High => colors.border,
    }
}

pub(crate) fn extensions_padding(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(12.),
        settings::CanvasDensity::Balanced => px(16.),
        settings::CanvasDensity::Spacious => px(24.),
    }
}

pub(crate) fn extensions_card_padding(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(8.),
        settings::CanvasDensity::Balanced => px(12.),
        settings::CanvasDensity::Spacious => px(16.),
    }
}

pub(crate) fn extensions_gap(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(8.),
        settings::CanvasDensity::Balanced => px(12.),
        settings::CanvasDensity::Spacious => px(16.),
    }
}

pub(crate) fn extensions_search_height(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(28.),
        settings::CanvasDensity::Balanced => px(32.),
        settings::CanvasDensity::Spacious => px(36.),
    }
}

pub(crate) fn extensions_radius(element: Div, cx: &App) -> Div {
    match DesignSystemSettings::get_global(cx).radius {
        settings::CanvasRadius::None => element,
        settings::CanvasRadius::Subtle => element.rounded_sm(),
        settings::CanvasRadius::Rounded => element.rounded_md(),
    }
}
