use gpui::{App, Div, Hsla, Pixels, Stateful, prelude::*, px};
use settings::Settings;
use ui::ActiveTheme;
use workspace::DesignSystemSettings;

pub(crate) fn debugger_background(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.editor_background,
        settings::CanvasContrast::Standard => colors.editor_background,
        settings::CanvasContrast::High => colors.element_background,
    }
}

pub(crate) fn debugger_panel_background(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.panel_background,
        settings::CanvasContrast::Standard => colors.panel_background,
        settings::CanvasContrast::High => colors.element_background,
    }
}

pub(crate) fn debugger_row_background(selected: bool, cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match (selected, DesignSystemSettings::get_global(cx).contrast) {
        (true, settings::CanvasContrast::Low) => colors.element_selected.opacity(0.72),
        (true, settings::CanvasContrast::Standard) => colors.element_selected,
        (true, settings::CanvasContrast::High) => colors
            .element_selected
            .blend(colors.border_focused.opacity(0.16)),
        (false, settings::CanvasContrast::Low) => colors.editor_background,
        (false, settings::CanvasContrast::Standard) => colors.editor_background,
        (false, settings::CanvasContrast::High) => colors.element_background.opacity(0.66),
    }
}

pub(crate) fn debugger_row_hover_background(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.element_hover.opacity(0.75),
        settings::CanvasContrast::Standard => colors.element_hover,
        settings::CanvasContrast::High => colors
            .element_hover
            .blend(colors.border_focused.opacity(0.14)),
    }
}

pub(crate) fn debugger_row_border_color(background: Hsla, selected: bool, cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match (selected, DesignSystemSettings::get_global(cx).contrast) {
        (_, settings::CanvasContrast::Low) => background,
        (true, settings::CanvasContrast::Standard) => colors.border.opacity(0.36),
        (true, settings::CanvasContrast::High) => colors.border_variant,
        (false, settings::CanvasContrast::Standard) => background,
        (false, settings::CanvasContrast::High) => colors.border.opacity(0.28),
    }
}

pub(crate) fn debugger_row_hover_border_color(background: Hsla, selected: bool, cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match (selected, DesignSystemSettings::get_global(cx).contrast) {
        (_, settings::CanvasContrast::Low) => background,
        (true, settings::CanvasContrast::Standard) => colors.border.opacity(0.46),
        (true, settings::CanvasContrast::High) => colors.border_focused,
        (false, settings::CanvasContrast::Standard) => background,
        (false, settings::CanvasContrast::High) => colors.border_variant,
    }
}

pub(crate) fn debugger_row_padding(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(4.),
        settings::CanvasDensity::Balanced => px(6.),
        settings::CanvasDensity::Spacious => px(8.),
    }
}

pub(crate) fn debugger_panel_padding(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(4.),
        settings::CanvasDensity::Balanced => px(6.),
        settings::CanvasDensity::Spacious => px(8.),
    }
}

pub(crate) fn debugger_radius(element: Div, cx: &App) -> Div {
    match DesignSystemSettings::get_global(cx).radius {
        settings::CanvasRadius::None => element,
        settings::CanvasRadius::Subtle => element.rounded_sm(),
        settings::CanvasRadius::Rounded => element.rounded_md(),
    }
}

pub(crate) fn debugger_stateful_radius(element: Stateful<Div>, cx: &App) -> Stateful<Div> {
    match DesignSystemSettings::get_global(cx).radius {
        settings::CanvasRadius::None => element,
        settings::CanvasRadius::Subtle => element.rounded_sm(),
        settings::CanvasRadius::Rounded => element.rounded_md(),
    }
}

pub(crate) fn debugger_row_surface(element: Div, selected: bool, cx: &App) -> Div {
    let background = debugger_row_background(selected, cx);
    let hover_background = debugger_row_hover_background(cx);
    let border_color = debugger_row_border_color(background, selected, cx);
    let hover_border_color = debugger_row_hover_border_color(background, selected, cx);

    debugger_radius(
        element
            .border_1()
            .border_color(border_color)
            .bg(background)
            .hover(move |this| this.bg(hover_background).border_color(hover_border_color)),
        cx,
    )
}
