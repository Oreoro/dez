use gpui::{App, Div, Hsla, Pixels, Stateful, prelude::*, px};
use settings::Settings;
use workspace::DesignSystemSettings;

pub(crate) fn search_background(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.editor_background,
        settings::CanvasContrast::Standard => colors.editor_background,
        settings::CanvasContrast::High => colors.element_background,
    }
}

pub(crate) fn search_border(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.border.opacity(0.5),
        settings::CanvasContrast::Standard => colors.border,
        settings::CanvasContrast::High => colors.border_variant,
    }
}

pub(crate) fn search_subtle_border(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.border_variant.opacity(0.45),
        settings::CanvasContrast::Standard => colors.border_variant,
        settings::CanvasContrast::High => colors.border,
    }
}

pub(crate) fn search_accent_background(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.text_accent.opacity(0.04),
        settings::CanvasContrast::Standard => colors.text_accent.opacity(0.05),
        settings::CanvasContrast::High => colors.text_accent.opacity(0.09),
    }
}

pub(crate) fn search_input_min_height(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(28.),
        settings::CanvasDensity::Balanced => px(32.),
        settings::CanvasDensity::Spacious => px(36.),
    }
}

pub(crate) fn search_input_padding_left(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(6.),
        settings::CanvasDensity::Balanced => px(8.),
        settings::CanvasDensity::Spacious => px(10.),
    }
}

pub(crate) fn search_input_padding_right(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(3.),
        settings::CanvasDensity::Balanced => px(4.),
        settings::CanvasDensity::Spacious => px(6.),
    }
}

pub(crate) fn search_radius(element: Stateful<Div>, cx: &App) -> Stateful<Div> {
    match DesignSystemSettings::get_global(cx).radius {
        settings::CanvasRadius::None => element,
        settings::CanvasRadius::Subtle => element.rounded_sm(),
        settings::CanvasRadius::Rounded => element.rounded_md(),
    }
}
