//! The dez workspace rail: one list of workspace groups, each followed by the
//! live agent sessions tied to it.
//!
//! This is dez's flagship differentiator. It is implemented against
//! [`workspace::Sidebar`] so that `MultiWorkspace` owns its placement, width,
//! and persistence. The rail itself owns only its rendering.
//!
//! The rail is dez's *only* sidebar. Live session supervision happens here
//! rather than in the Agent Threads panel, and Files / Git / Settings stay in
//! Zed's own docked panels instead of being re-hosted as rail tabs.

pub mod filter;
mod settings;
mod sidebar;

use ::settings::Settings as _;
use gpui::App;

pub use settings::DezSidebarSettings;
pub use sidebar::DezSidebar;

/// Registers dez sidebar settings. Call once during app initialization, after
/// `settings::init`.
pub fn init(cx: &mut App) {
    DezSidebarSettings::register(cx);
}
