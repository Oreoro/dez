use gpui::{Action, ClickEvent, FocusHandle, Hsla, Pixels, prelude::*};
use language_model::DisabledReason;
use settings::Settings;
use ui::{Chip, ElevationIndex, KeyBinding, ListItem, ListItemSpacing, Tooltip, prelude::*};
use workspace::DesignSystemSettings;
use zed_actions::agent::ToggleModelSelector;

use crate::CycleFavoriteModels;

fn model_selector_border(cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match DesignSystemSettings::get_global(cx).contrast {
        settings::CanvasContrast::Low => colors.border_variant.opacity(0.42),
        settings::CanvasContrast::Standard => colors.border_variant,
        settings::CanvasContrast::High => colors.border_focused,
    }
}

fn model_selector_row_spacing(cx: &App) -> ListItemSpacing {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => ListItemSpacing::ExtraDense,
        settings::CanvasDensity::Balanced => ListItemSpacing::Dense,
        settings::CanvasDensity::Spacious => ListItemSpacing::Sparse,
    }
}

fn model_selector_header_padding_x(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(6.),
        settings::CanvasDensity::Balanced => px(8.),
        settings::CanvasDensity::Spacious => px(12.),
    }
}

fn model_selector_header_padding_bottom(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(2.),
        settings::CanvasDensity::Balanced => px(4.),
        settings::CanvasDensity::Spacious => px(6.),
    }
}

fn model_selector_section_margin_top(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(2.),
        settings::CanvasDensity::Balanced => px(4.),
        settings::CanvasDensity::Spacious => px(6.),
    }
}

fn model_selector_section_padding_top(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(6.),
        settings::CanvasDensity::Balanced => px(8.),
        settings::CanvasDensity::Spacious => px(10.),
    }
}

fn model_selector_gap(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(4.),
        settings::CanvasDensity::Balanced => px(6.),
        settings::CanvasDensity::Spacious => px(8.),
    }
}

fn model_selector_end_padding(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(4.),
        settings::CanvasDensity::Balanced => px(8.),
        settings::CanvasDensity::Spacious => px(10.),
    }
}

fn model_selector_hover_padding(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(4.),
        settings::CanvasDensity::Balanced => px(6.),
        settings::CanvasDensity::Spacious => px(8.),
    }
}

fn model_selector_footer_padding(cx: &App) -> Pixels {
    match DesignSystemSettings::get_global(cx).density {
        settings::CanvasDensity::Compact => px(4.),
        settings::CanvasDensity::Balanced => px(6.),
        settings::CanvasDensity::Spacious => px(10.),
    }
}

enum ModelIcon {
    Name(IconName),
    Path(SharedString),
}

#[derive(IntoElement)]
pub struct ModelSelectorHeader {
    title: SharedString,
    has_border: bool,
}

impl ModelSelectorHeader {
    pub fn new(title: impl Into<SharedString>, has_border: bool) -> Self {
        Self {
            title: title.into(),
            has_border,
        }
    }
}

impl RenderOnce for ModelSelectorHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .px(model_selector_header_padding_x(cx))
            .pb(model_selector_header_padding_bottom(cx))
            .when(self.has_border, |this| {
                this.mt(model_selector_section_margin_top(cx))
                    .pt(model_selector_section_padding_top(cx))
                    .border_t_1()
                    .border_color(model_selector_border(cx))
            })
            .child(
                Label::new(self.title)
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
            )
    }
}

#[derive(IntoElement)]
pub struct ModelSelectorListItem {
    index: usize,
    title: SharedString,
    icon: Option<ModelIcon>,
    is_selected: bool,
    is_focused: bool,
    is_latest: bool,
    is_favorite: bool,
    disabled: Option<DisabledReason>,
    on_toggle_favorite: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    cost_info: Option<SharedString>,
}

impl ModelSelectorListItem {
    pub fn new(index: usize, title: impl Into<SharedString>) -> Self {
        Self {
            index,
            title: title.into(),
            icon: None,
            is_selected: false,
            is_focused: false,
            is_latest: false,
            is_favorite: false,
            disabled: None,
            on_toggle_favorite: None,
            cost_info: None,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(ModelIcon::Name(icon));
        self
    }

    pub fn icon_path(mut self, path: SharedString) -> Self {
        self.icon = Some(ModelIcon::Path(path));
        self
    }

    pub fn is_selected(mut self, is_selected: bool) -> Self {
        self.is_selected = is_selected;
        self
    }

    pub fn disabled(mut self, disabled: Option<DisabledReason>) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn is_focused(mut self, is_focused: bool) -> Self {
        self.is_focused = is_focused;
        self
    }

    pub fn is_latest(mut self, is_latest: bool) -> Self {
        self.is_latest = is_latest;
        self
    }

    pub fn is_favorite(mut self, is_favorite: bool) -> Self {
        self.is_favorite = is_favorite;
        self
    }

    pub fn on_toggle_favorite(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle_favorite = Some(Box::new(handler));
        self
    }

    pub fn cost_info(mut self, cost_info: Option<SharedString>) -> Self {
        self.cost_info = cost_info;
        self
    }
}

impl RenderOnce for ModelSelectorListItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_disabled = self.disabled.is_some();

        let model_icon_color = if self.is_selected {
            Color::Accent
        } else if is_disabled {
            Color::Disabled
        } else {
            Color::Muted
        };

        let is_favorite = self.is_favorite;

        ListItem::new(self.index)
            .inset(true)
            .spacing(model_selector_row_spacing(cx))
            .toggle_state(self.is_focused)
            .when_some(self.disabled, |this, disabled_reason| {
                this.disabled(true)
                    .tooltip(Tooltip::text(disabled_reason.0))
            })
            .child(
                h_flex()
                    .w_full()
                    .gap(model_selector_gap(cx))
                    .when_some(self.icon, |this, icon| {
                        this.child(
                            match icon {
                                ModelIcon::Name(icon_name) => Icon::new(icon_name),
                                ModelIcon::Path(icon_path) => Icon::from_external_svg(icon_path),
                            }
                            .color(model_icon_color)
                            .size(IconSize::Small),
                        )
                    })
                    .child(
                        Label::new(self.title)
                            .when(is_disabled, |this| this.color(Color::Disabled))
                            .truncate(),
                    )
                    .when(self.is_latest, |parent| parent.child(Chip::new("Latest")))
                    .when_some(self.cost_info, |this, cost_info| {
                        let tooltip_text = if cost_info.ends_with('×') {
                            format!("Cost Multiplier: {}", cost_info)
                        } else if cost_info.contains('$') {
                            format!("Cost per Million Tokens: {}", cost_info)
                        } else {
                            format!("Cost: {}", cost_info)
                        };

                        this.child(Chip::new(cost_info).tooltip(Tooltip::text(tooltip_text)))
                    }),
            )
            .end_slot(
                h_flex()
                    .pr(model_selector_end_padding(cx))
                    .gap(model_selector_gap(cx))
                    .when(self.is_selected, |this| {
                        this.child(Icon::new(IconName::Check).color(Color::Accent))
                    })
                    .when(is_disabled, |this| {
                        this.child(Icon::new(IconName::Info).color(Color::Muted))
                    }),
            )
            .when(!is_disabled, |this| {
                this.end_slot_on_hover(div().pr(model_selector_hover_padding(cx)).when_some(
                    self.on_toggle_favorite,
                    {
                        |this, handle_click| {
                            let (icon, color, tooltip) = if is_favorite {
                                (IconName::StarFilled, Color::Accent, "Unfavorite Model")
                            } else {
                                (IconName::Star, Color::Default, "Favorite Model")
                            };
                            this.child(
                                IconButton::new(("toggle-favorite", self.index), icon)
                                    .layer(ElevationIndex::ElevatedSurface)
                                    .icon_color(color)
                                    .icon_size(IconSize::Small)
                                    .tooltip(Tooltip::text(tooltip))
                                    .on_click(move |event, window, cx| {
                                        (handle_click)(event, window, cx)
                                    }),
                            )
                        }
                    },
                ))
            })
    }
}

#[derive(IntoElement)]
pub struct ModelSelectorFooter {
    action: Box<dyn Action>,
    focus_handle: FocusHandle,
}

impl ModelSelectorFooter {
    pub fn new(action: Box<dyn Action>, focus_handle: FocusHandle) -> Self {
        Self {
            action,
            focus_handle,
        }
    }
}

impl RenderOnce for ModelSelectorFooter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let action = self.action;
        let focus_handle = self.focus_handle;

        h_flex()
            .w_full()
            .p(model_selector_footer_padding(cx))
            .border_t_1()
            .border_color(model_selector_border(cx))
            .child(
                Button::new("configure", "Configure")
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .key_binding(
                        KeyBinding::for_action_in(action.as_ref(), &focus_handle, cx)
                            .map(|kb| kb.size(rems_from_px(12.))),
                    )
                    .on_click(move |_, window, cx| {
                        window.dispatch_action(action.boxed_clone(), cx);
                    }),
            )
    }
}

#[derive(IntoElement)]
pub struct ModelSelectorTooltip {
    show_cycle_row: bool,
}

impl ModelSelectorTooltip {
    pub fn new() -> Self {
        Self {
            show_cycle_row: true,
        }
    }

    pub fn show_cycle_row(mut self, show: bool) -> Self {
        self.show_cycle_row = show;
        self
    }
}

impl RenderOnce for ModelSelectorTooltip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .gap(model_selector_gap(cx))
            .child(
                h_flex()
                    .gap(model_selector_gap(cx))
                    .justify_between()
                    .child(Label::new("Change Model"))
                    .child(KeyBinding::for_action(&ToggleModelSelector, cx)),
            )
            .when(self.show_cycle_row, |this| {
                this.child(
                    h_flex()
                        .pt(model_selector_footer_padding(cx))
                        .gap(model_selector_gap(cx))
                        .border_t_1()
                        .border_color(model_selector_border(cx))
                        .justify_between()
                        .child(Label::new("Cycle Favorite Models"))
                        .child(KeyBinding::for_action(&CycleFavoriteModels, cx)),
                )
            })
    }
}
