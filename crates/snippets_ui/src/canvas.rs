use gpui::{App, Pixels, px};
use picker::{PickerSurfaceContrast, PickerSurfaceDensity, PickerSurfaceRadius};
use settings::Settings;
use ui::ListItemSpacing;
use workspace::DesignSystemSettings;

pub(crate) fn snippets_picker_density(cx: &App) -> PickerSurfaceDensity {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => PickerSurfaceDensity::Compact,
        settings::CanvasDensity::Balanced => PickerSurfaceDensity::Balanced,
        settings::CanvasDensity::Spacious => PickerSurfaceDensity::Spacious,
    }
}

pub(crate) fn snippets_picker_radius(cx: &App) -> PickerSurfaceRadius {
    match DesignSystemSettings::get_global(cx).radius {
        settings::CanvasRadius::None => PickerSurfaceRadius::None,
        settings::CanvasRadius::Subtle => PickerSurfaceRadius::Subtle,
        settings::CanvasRadius::Rounded => PickerSurfaceRadius::Rounded,
    }
}

pub(crate) fn snippets_picker_contrast(cx: &App) -> PickerSurfaceContrast {
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => PickerSurfaceContrast::Low,
        settings::CanvasContrast::Standard => PickerSurfaceContrast::Standard,
        settings::CanvasContrast::High => PickerSurfaceContrast::High,
    }
}

pub(crate) fn snippets_row_spacing(cx: &App) -> ListItemSpacing {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => ListItemSpacing::ExtraDense,
        settings::CanvasDensity::Balanced => ListItemSpacing::Dense,
        settings::CanvasDensity::Spacious => ListItemSpacing::Sparse,
    }
}

pub(crate) fn snippets_row_gap(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(4.),
        settings::CanvasDensity::Balanced => px(6.),
        settings::CanvasDensity::Spacious => px(8.),
    }
}
