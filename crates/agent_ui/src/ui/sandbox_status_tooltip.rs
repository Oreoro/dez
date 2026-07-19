use std::path::PathBuf;

use file_icons::FileIcons;
use gpui::{AnyElement, App, Div, Hsla, Pixels, Stateful};
use settings::Settings;
use ui::{Divider, prelude::*};
use workspace::DesignSystemSettings;

fn sandbox_tooltip_background(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.elevated_surface_background.opacity(0.94),
        settings::CanvasContrast::Standard => colors.elevated_surface_background,
        settings::CanvasContrast::High => colors
            .elevated_surface_background
            .blend(colors.border_focused.opacity(0.08)),
    }
}

fn sandbox_tooltip_border(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.border.opacity(0.42),
        settings::CanvasContrast::Standard => colors.border_variant,
        settings::CanvasContrast::High => colors.border_focused,
    }
}

fn sandbox_section_background(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.editor_background.opacity(0.62),
        settings::CanvasContrast::Standard => colors.editor_background.opacity(0.78),
        settings::CanvasContrast::High => colors.element_background,
    }
}

fn sandbox_tooltip_padding(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(8.),
        settings::CanvasDensity::Balanced => px(10.),
        settings::CanvasDensity::Spacious => px(14.),
    }
}

fn sandbox_tooltip_gap(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(6.),
        settings::CanvasDensity::Balanced => px(8.),
        settings::CanvasDensity::Spacious => px(12.),
    }
}

fn sandbox_row_gap(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(4.),
        settings::CanvasDensity::Balanced => px(6.),
        settings::CanvasDensity::Spacious => px(8.),
    }
}

fn sandbox_tooltip_width(cx: &App) -> Rems {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => rems_from_px(260.),
        settings::CanvasDensity::Balanced => rems_from_px(280.),
        settings::CanvasDensity::Spacious => rems_from_px(340.),
    }
}

fn sandbox_radius(element: Stateful<Div>, cx: &App) -> Stateful<Div> {
    match DesignSystemSettings::get_global(cx).radius {
        settings::CanvasRadius::None => element,
        settings::CanvasRadius::Subtle => element.rounded_md(),
        settings::CanvasRadius::Rounded => element.rounded_lg(),
    }
}

fn sandbox_section_radius(element: Stateful<Div>, cx: &App) -> Stateful<Div> {
    match DesignSystemSettings::get_global(cx).radius {
        settings::CanvasRadius::None => element,
        settings::CanvasRadius::Subtle => element.rounded_sm(),
        settings::CanvasRadius::Rounded => element.rounded_md(),
    }
}

#[derive(Clone)]
pub enum SandboxRow {
    Message(SharedString),
    Path(PathBuf),
    Domain(SharedString),
}

impl SandboxRow {
    pub fn message(message: impl Into<SharedString>) -> Self {
        Self::Message(message.into())
    }

    pub fn path(path: impl Into<PathBuf>) -> Self {
        Self::Path(path.into())
    }

    pub fn domain(domain: impl Into<SharedString>) -> Self {
        Self::Domain(domain.into())
    }

    fn render(self, cx: &App) -> AnyElement {
        let icon_basic = |icon_name: IconName| {
            Icon::new(icon_name)
                .color(Color::Muted)
                .size(IconSize::Small)
        };

        let (icon, label) = match self {
            SandboxRow::Message(message) => {
                return Label::new(message)
                    .size(LabelSize::XSmall)
                    .color(Color::Muted)
                    .into_any_element();
            }
            SandboxRow::Path(path) => {
                let icon = FileIcons::get_icon(&path, cx)
                    .map(|icon| {
                        Icon::from_path(icon)
                            .color(Color::Muted)
                            .size(IconSize::Small)
                    })
                    .unwrap_or_else(|| icon_basic(IconName::Folder));
                (icon, path.display().to_string())
            }
            SandboxRow::Domain(domain) => (icon_basic(IconName::Public), domain.to_string()),
        };

        h_flex()
            .items_start()
            .min_w_0()
            .gap(sandbox_row_gap(cx))
            .child(icon)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .child(Label::new(label).size(LabelSize::XSmall).buffer_font(cx)),
            )
            .into_any_element()
    }
}

#[derive(Clone)]
pub struct SandboxGroup {
    heading: SharedString,
    rows: Vec<SandboxRow>,
}

impl SandboxGroup {
    pub fn new(heading: impl Into<SharedString>) -> Self {
        Self {
            heading: heading.into(),
            rows: Vec::new(),
        }
    }

    pub fn row(mut self, row: SandboxRow) -> Self {
        self.rows.push(row);
        self
    }

    pub fn rows(mut self, rows: impl IntoIterator<Item = SandboxRow>) -> Self {
        self.rows.extend(rows);
        self
    }

    fn render(self, cx: &App) -> impl IntoElement {
        v_flex()
            .gap(sandbox_row_gap(cx))
            .child(
                Label::new(self.heading)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .children(self.rows.into_iter().map(|row| row.render(cx)))
    }
}

#[derive(Clone)]
pub struct SandboxSection {
    title: SharedString,
    groups: Vec<SandboxGroup>,
}

impl SandboxSection {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            groups: Vec::new(),
        }
    }

    pub fn group(mut self, group: SandboxGroup) -> Self {
        self.groups.push(group);
        self
    }

    fn render(self, cx: &App) -> AnyElement {
        v_flex()
            .gap(sandbox_tooltip_gap(cx))
            .p(sandbox_tooltip_padding(cx))
            .bg(sandbox_section_background(cx))
            .border_1()
            .border_color(sandbox_tooltip_border(cx))
            .map(|this| sandbox_section_radius(this, cx))
            .child(Label::new(self.title).size(LabelSize::Small))
            .children(self.groups.into_iter().map(|group| {
                v_flex()
                    .gap(sandbox_tooltip_gap(cx))
                    .child(Divider::horizontal())
                    .child(group.render(cx))
            }))
            .into_any_element()
    }
}

#[derive(Clone, IntoElement, RegisterComponent)]
pub enum SandboxStatusTooltip {
    Enabled {
        settings: SandboxSection,
        thread: Option<SandboxSection>,
    },
    DisabledForThread {
        settings: SandboxSection,
    },
    DisabledInSettings,
}

impl SandboxStatusTooltip {
    pub fn enabled(settings: SandboxSection, thread: Option<SandboxSection>) -> Self {
        Self::Enabled { settings, thread }
    }

    pub fn disabled_for_thread(settings: SandboxSection) -> Self {
        Self::DisabledForThread { settings }
    }

    pub fn disabled_in_settings() -> Self {
        Self::DisabledInSettings
    }
}

impl RenderOnce for SandboxStatusTooltip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let content = match self {
            SandboxStatusTooltip::DisabledInSettings => v_flex()
                .child(
                    Label::new("You have sandboxing disabled in settings.")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element(),
            SandboxStatusTooltip::DisabledForThread { settings } => v_flex()
                .gap(sandbox_tooltip_gap(cx))
                .child(div().opacity(0.5).child(settings.render(cx)))
                .child(Divider::horizontal())
                .child(Label::new("Sandboxing is disabled for this thread").size(LabelSize::Small))
                .into_any_element(),
            SandboxStatusTooltip::Enabled { settings, thread } => v_flex()
                .gap(sandbox_tooltip_gap(cx))
                .child(settings.render(cx))
                .children(thread.map(|thread| {
                    v_flex()
                        .gap(sandbox_tooltip_gap(cx))
                        .child(Divider::horizontal())
                        .child(thread.render(cx))
                }))
                .into_any_element(),
        };

        v_flex()
            .w(sandbox_tooltip_width(cx))
            .p(sandbox_tooltip_padding(cx))
            .gap(sandbox_tooltip_gap(cx))
            .bg(sandbox_tooltip_background(cx))
            .border_1()
            .border_color(sandbox_tooltip_border(cx))
            .map(|this| sandbox_radius(this, cx))
            .child(Label::new("Sandboxing"))
            .child(content)
    }
}

impl Component for SandboxStatusTooltip {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn name() -> &'static str {
        "Sandbox Status Tooltip"
    }

    fn description() -> &'static str {
        "The tooltip shown on the sandboxing lock icon in the agent panel, \
        describing the filesystem, network, and Git access granted to the \
        agent for each of the possible sandbox states."
    }

    fn preview(_window: &mut Window, cx: &mut App) -> AnyElement {
        let settings_section = SandboxSection::new("Defined in your settings:")
            .group(SandboxGroup::new("Write Access").rows([
                SandboxRow::path("/Users/you/project"),
                SandboxRow::path("/tmp (isolated)"),
            ]))
            .group(SandboxGroup::new("Network Access").rows([
                SandboxRow::domain("github.com"),
                SandboxRow::domain("*.npmjs.org"),
            ]));

        let thread_section = SandboxSection::new("Allowed for this thread:")
            .group(
                SandboxGroup::new("Write Access").row(SandboxRow::path("/Users/you/project/build")),
            )
            .group(SandboxGroup::new("Network Access").row(SandboxRow::message("None")));

        let unrestricted_section = SandboxSection::new("Defined in your settings:")
            .group(SandboxGroup::new("Write Access").row(SandboxRow::message(
                "All paths except protected Git metadata",
            )))
            .group(
                SandboxGroup::new("Network Access")
                    .row(SandboxRow::message("All domains (unrestricted)")),
            );

        let container = || div().p_2().elevation_2(cx).max_w_112();

        v_flex()
            .gap_4()
            .child(example_group(vec![
                single_example(
                    "Enabled",
                    container()
                        .child(SandboxStatusTooltip::enabled(
                            settings_section.clone(),
                            Some(thread_section),
                        ))
                        .into_any_element(),
                ),
                single_example(
                    "Enabled (unrestricted, no overrides)",
                    container()
                        .child(SandboxStatusTooltip::enabled(unrestricted_section, None))
                        .into_any_element(),
                ),
                single_example(
                    "Disabled for thread",
                    container()
                        .child(SandboxStatusTooltip::disabled_for_thread(settings_section))
                        .into_any_element(),
                ),
                single_example(
                    "Disabled in settings",
                    container()
                        .child(SandboxStatusTooltip::disabled_in_settings())
                        .into_any_element(),
                ),
            ]))
            .into_any_element()
    }
}
