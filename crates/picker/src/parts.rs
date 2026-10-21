//! Components used in multiple pickers

use gpui::Entity;
use project::Project;
use ui::{CommonAnimationExt, Tooltip, prelude::*};

fn project_scan_status_label(app_name: &str) -> &'static str {
    if app_name == "Zed" {
        "Project scan in progress…"
    } else {
        "Workspace scan in progress…"
    }
}

pub fn project_scan_indicator(
    has_query: bool,
    project: &Entity<Project>,
    cx: &App,
) -> Option<impl IntoElement> {
    let is_project_scan_running = {
        let worktree_store = project.read(cx).worktree_store();
        !worktree_store.read(cx).initial_scan_completed()
    };
    (has_query && is_project_scan_running).then(|| {
        let status_label = project_scan_status_label(paths::APP_NAME);
        h_flex()
            .id("project-scan-indicator")
            .role(gpui::Role::Status)
            .aria_label(status_label)
            .tooltip(Tooltip::text(status_label))
            .child(
                Icon::new(IconName::LoadCircle)
                    .color(Color::Accent)
                    .size(IconSize::Small)
                    .with_rotate_animation(2),
            )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_status_preserves_product_vocabulary() {
        assert_eq!(
            project_scan_status_label("Dez"),
            "Workspace scan in progress…"
        );
        assert_eq!(
            project_scan_status_label("Zed"),
            "Project scan in progress…"
        );
    }
}
