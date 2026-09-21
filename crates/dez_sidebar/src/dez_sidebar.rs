//! The dez workspace sidebar: a navigator of workspace surfaces and agent
//! sessions that sits beside the editor.
//!
//! This is dez's flagship differentiator. It is implemented against
//! [`workspace::Sidebar`] so that `MultiWorkspace` owns its placement, width,
//! and persistence. The sidebar itself owns only its view state and rendering.

mod settings;
mod sidebar;

use gpui::App;
use settings::Settings as _;

pub use settings::DezSidebarSettings;
pub use sidebar::{DezSidebar, DezSidebarView};

/// Registers dez sidebar settings. Call once during app initialization, after
/// `settings::init`.
pub fn init(cx: &mut App) {
    DezSidebarSettings::register(cx);
}
