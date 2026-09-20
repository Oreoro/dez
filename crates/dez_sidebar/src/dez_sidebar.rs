//! The dez workspace sidebar: a navigator of workspace surfaces and agent
//! sessions that sits beside the editor.
//!
//! This is dez's flagship differentiator. It is implemented against
//! [`workspace::Sidebar`] so that `MultiWorkspace` owns its placement, width,
//! and persistence. The sidebar itself owns only its view state and rendering.

mod sidebar;

pub use sidebar::{DezSidebar, DezSidebarView};