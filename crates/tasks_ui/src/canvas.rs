use gpui::{App, Hsla, Pixels, px};
use picker::{PickerSurfaceContrast, PickerSurfaceDensity, PickerSurfaceRadius};
use settings::Settings;
use ui::{ActiveTheme, ListItemSpacing};
use workspace::DesignSystemSettings;

pub(crate) fn task_picker_density(cx: &App) -> PickerSurfaceDensity {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => PickerSurfaceDensity::Compact,
        settings::CanvasDensity::Balanced => PickerSurfaceDensity::Balanced,
        settings::CanvasDensity::Spacious => PickerSurfaceDensity::Spacious,
    }
}

pub(crate) fn task_picker_radius(cx: &App) -> PickerSurfaceRadius {
    match DesignSystemSettings::get_global(cx).radius {
        settings::CanvasRadius::None => PickerSurfaceRadius::None,
        settings::CanvasRadius::Subtle => PickerSurfaceRadius::Subtle,
        settings::CanvasRadius::Rounded => PickerSurfaceRadius::Rounded,
    }
}

pub(crate) fn task_picker_contrast(cx: &App) -> PickerSurfaceContrast {
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => PickerSurfaceContrast::Low,
        settings::CanvasContrast::Standard => PickerSurfaceContrast::Standard,
        settings::CanvasContrast::High => PickerSurfaceContrast::High,
    }
}

pub(crate) fn task_row_spacing(cx: &App) -> ListItemSpacing {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => ListItemSpacing::ExtraDense,
        settings::CanvasDensity::Balanced => ListItemSpacing::Dense,
        settings::CanvasDensity::Spacious => ListItemSpacing::Sparse,
    }
}

pub(crate) fn task_footer_padding(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(6.),
        settings::CanvasDensity::Balanced => px(8.),
        settings::CanvasDensity::Spacious => px(12.),
    }
}

pub(crate) fn task_footer_gap(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(4.),
        settings::CanvasDensity::Balanced => px(6.),
        settings::CanvasDensity::Spacious => px(8.),
    }
}

pub(crate) fn task_footer_border(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.border_variant.opacity(0.45),
        settings::CanvasContrast::Standard => colors.border_variant,
        settings::CanvasContrast::High => colors.border_focused,
    }
}

pub(crate) fn task_indicator_border(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low | settings::CanvasContrast::Standard => {
            colors.border_transparent
        }
        settings::CanvasContrast::High => colors.elevated_surface_background,
    }
}
