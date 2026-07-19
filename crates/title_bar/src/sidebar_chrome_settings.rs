use gpui::WindowButtonLayout;
use settings::{RegisterSetting, Settings, SettingsContent};

#[derive(Copy, Clone, Debug, RegisterSetting)]
pub struct SidebarChromeSettings {
    pub show_branch_status_icon: bool,
    pub show_onboarding_banner: bool,
    pub show_user_picture: bool,
    pub show_branch_name: bool,
    pub show_project_items: bool,
    pub show_sign_in: bool,
    pub show_user_menu: bool,
    pub show_menus: bool,
    pub button_layout: Option<WindowButtonLayout>,
}

#[derive(Copy, Clone, Debug, RegisterSetting)]
pub struct WorkspaceBarSettings {
    pub visibility: settings::CanvasVisibility,
    pub height: settings::WorkspaceBarHeight,
    pub center_command_search: bool,
    pub show_layout: bool,
    pub show_agent_attention: bool,
}

impl WorkspaceBarSettings {
    pub fn is_visible(&self) -> bool {
        self.visibility != settings::CanvasVisibility::Hidden
    }

    pub fn show_layout(&self) -> bool {
        self.is_visible() && self.show_layout
    }
}

impl Settings for SidebarChromeSettings {
    fn from_settings(s: &SettingsContent) -> Self {
        let content = s.sidebar.clone().unwrap();
        SidebarChromeSettings {
            show_branch_status_icon: content.show_branch_status_icon.unwrap(),
            show_onboarding_banner: content.show_onboarding_banner.unwrap(),
            show_user_picture: content.show_user_picture.unwrap(),
            show_branch_name: content.show_branch_name.unwrap(),
            show_project_items: content.show_project_items.unwrap(),
            show_sign_in: content.show_sign_in.unwrap(),
            show_user_menu: content.show_user_menu.unwrap(),
            show_menus: content.show_menus.unwrap(),
            button_layout: content.button_layout.unwrap_or_default().into_layout(),
        }
    }
}

impl Settings for WorkspaceBarSettings {
    fn from_settings(s: &SettingsContent) -> Self {
        let content = s.workspace_bar.clone().unwrap();
        WorkspaceBarSettings {
            visibility: content.visibility.unwrap(),
            height: content.height.unwrap(),
            center_command_search: content.center_command_search.unwrap(),
            show_layout: content.show_layout.unwrap(),
            show_agent_attention: content.show_agent_attention.unwrap(),
        }
    }
}
