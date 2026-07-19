use std::cmp::Ordering;

use gpui::{AnyElement, IntoElement, Stateful};
use smallvec::SmallVec;

use crate::prelude::*;

const START_TAB_SLOT_SIZE: Pixels = px(12.);
const END_TAB_SLOT_SIZE: Pixels = px(14.);

/// The position of a [`Tab`] within a list of tabs.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TabPosition {
    /// The tab is first in the list.
    First,

    /// The tab is in the middle of the list (i.e., it is not the first or last tab).
    ///
    /// The [`Ordering`] is where this tab is positioned with respect to the selected tab.
    Middle(Ordering),

    /// The tab is last in the list.
    Last,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TabCloseSide {
    Start,
    End,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabDensity {
    Compact,
    #[default]
    Balanced,
    Spacious,
}

impl TabDensity {
    pub fn content_height(self, cx: &App) -> Pixels {
        self.container_height(cx) - px(1.)
    }

    pub fn container_height(self, cx: &App) -> Pixels {
        match self {
            Self::Compact => DynamicSpacing::Base24.px(cx),
            Self::Balanced => DynamicSpacing::Base32.px(cx),
            Self::Spacious => DynamicSpacing::Base40.px(cx),
        }
    }

    fn content_padding(self, cx: &App) -> (Pixels, Pixels) {
        match self {
            Self::Compact => (DynamicSpacing::Base06.px(cx), DynamicSpacing::Base02.px(cx)),
            Self::Balanced => (DynamicSpacing::Base08.px(cx), DynamicSpacing::Base04.px(cx)),
            Self::Spacious => (DynamicSpacing::Base12.px(cx), DynamicSpacing::Base06.px(cx)),
        }
    }

    pub(crate) fn slot_padding(self, cx: &App) -> Rems {
        match self {
            Self::Compact => DynamicSpacing::Base04.rems(cx),
            Self::Balanced => DynamicSpacing::Base06.rems(cx),
            Self::Spacious => DynamicSpacing::Base08.rems(cx),
        }
    }

    pub(crate) fn gap(self, cx: &App) -> Rems {
        match self {
            Self::Compact => DynamicSpacing::Base02.rems(cx),
            Self::Balanced => DynamicSpacing::Base04.rems(cx),
            Self::Spacious => DynamicSpacing::Base06.rems(cx),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabRadius {
    #[default]
    None,
    Subtle,
    Rounded,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabContrast {
    Low,
    #[default]
    Standard,
    High,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TabInsertionIndicator {
    Left,
    Right,
}

#[derive(IntoElement, RegisterComponent)]
pub struct Tab {
    div: Stateful<Div>,
    selected: bool,
    position: TabPosition,
    close_side: TabCloseSide,
    density: TabDensity,
    radius: TabRadius,
    contrast: TabContrast,
    insertion_indicator: Option<TabInsertionIndicator>,
    start_slot: Option<AnyElement>,
    end_slot: Option<AnyElement>,
    children: SmallVec<[AnyElement; 2]>,
}

impl Tab {
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            div: div()
                .id(id.clone())
                .debug_selector(|| format!("TAB-{}", id)),
            selected: false,
            position: TabPosition::First,
            close_side: TabCloseSide::End,
            density: TabDensity::default(),
            radius: TabRadius::default(),
            contrast: TabContrast::default(),
            insertion_indicator: None,
            start_slot: None,
            end_slot: None,
            children: SmallVec::new(),
        }
    }

    pub fn position(mut self, position: TabPosition) -> Self {
        self.position = position;
        self
    }

    pub fn close_side(mut self, close_side: TabCloseSide) -> Self {
        self.close_side = close_side;
        self
    }

    pub fn density(mut self, density: TabDensity) -> Self {
        self.density = density;
        self
    }

    pub fn radius(mut self, radius: TabRadius) -> Self {
        self.radius = radius;
        self
    }

    pub fn contrast(mut self, contrast: TabContrast) -> Self {
        self.contrast = contrast;
        self
    }

    pub fn insertion_indicator_left(mut self) -> Self {
        self.insertion_indicator = Some(TabInsertionIndicator::Left);
        self
    }

    pub fn insertion_indicator_right(mut self) -> Self {
        self.insertion_indicator = Some(TabInsertionIndicator::Right);
        self
    }

    pub fn start_slot<E: IntoElement>(mut self, element: impl Into<Option<E>>) -> Self {
        self.start_slot = element.into().map(IntoElement::into_any_element);
        self
    }

    pub fn end_slot<E: IntoElement>(mut self, element: impl Into<Option<E>>) -> Self {
        self.end_slot = element.into().map(IntoElement::into_any_element);
        self
    }

    pub fn content_height(cx: &App) -> Pixels {
        TabDensity::Balanced.content_height(cx)
    }

    pub fn container_height(cx: &App) -> Pixels {
        TabDensity::Balanced.container_height(cx)
    }
}

impl InteractiveElement for Tab {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.div.interactivity()
    }
}

impl StatefulInteractiveElement for Tab {}

impl Toggleable for Tab {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl ParentElement for Tab {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Tab {
    #[allow(refining_impl_trait)]
    fn render(self, _: &mut Window, cx: &mut App) -> Stateful<Div> {
        let insertion_indicator = self.insertion_indicator;
        let colors = cx.theme().colors();
        let (text_color, tab_bg, _tab_hover_bg, _tab_active_bg) = match self.selected {
            false => (
                colors.text_muted,
                colors.tab_inactive_background,
                colors.ghost_element_hover,
                colors.ghost_element_active,
            ),
            true => (
                colors.text,
                colors.tab_active_background,
                colors.element_hover,
                colors.element_active,
            ),
        };
        let tab_bg = match (self.contrast, self.selected) {
            (TabContrast::Low, false) => colors.tab_bar_background,
            (TabContrast::Low, true) => colors
                .tab_bar_background
                .blend(colors.tab_active_background.opacity(0.7)),
            (TabContrast::Standard, _) => tab_bg,
            (TabContrast::High, false) => colors
                .tab_inactive_background
                .blend(colors.border_variant.opacity(0.12)),
            (TabContrast::High, true) => colors
                .tab_active_background
                .blend(colors.border_focused.opacity(0.12)),
        };
        let border_color = match self.contrast {
            TabContrast::Low => colors.border.opacity(0.55),
            TabContrast::Standard => colors.border,
            TabContrast::High => colors.border_variant,
        };
        let (padding_left, padding_right) = self.density.content_padding(cx);

        let (start_slot, end_slot) = {
            let start_slot = self.start_slot.map(|slot| {
                h_flex()
                    .size(START_TAB_SLOT_SIZE)
                    .justify_center()
                    .child(slot)
            });

            let end_slot = self.end_slot.map(|slot| {
                h_flex()
                    .size(END_TAB_SLOT_SIZE)
                    .justify_center()
                    .child(slot)
            });

            match self.close_side {
                TabCloseSide::End => (start_slot, end_slot),
                TabCloseSide::Start => (end_slot, start_slot),
            }
        };

        self.div
            .relative()
            .h(self.density.container_height(cx))
            .bg(tab_bg)
            .border_color(border_color)
            .when(self.selected && self.radius == TabRadius::Subtle, |this| {
                this.rounded_t_sm()
            })
            .when(self.selected && self.radius == TabRadius::Rounded, |this| {
                this.rounded_t_md()
            })
            .map(|this| match self.position {
                TabPosition::First => {
                    if self.selected {
                        this.pl_px().border_r_1().pb_px()
                    } else {
                        this.pl_px().pr_px().border_b_1()
                    }
                }
                TabPosition::Last => {
                    if self.selected {
                        this.border_l_1().border_r_1().pb_px()
                    } else {
                        this.pl_px().border_b_1().border_r_1()
                    }
                }
                TabPosition::Middle(Ordering::Equal) => this.border_l_1().border_r_1().pb_px(),
                TabPosition::Middle(Ordering::Less) => this.border_l_1().pr_px().border_b_1(),
                TabPosition::Middle(Ordering::Greater) => this.border_r_1().pl_px().border_b_1(),
            })
            .cursor_pointer()
            .child(
                h_flex()
                    .group("")
                    .relative()
                    .h(self.density.content_height(cx))
                    .pl(padding_left)
                    .pr(padding_right)
                    .gap(self.density.gap(cx))
                    .text_color(text_color)
                    .children(start_slot)
                    .children(self.children)
                    .children(end_slot),
            )
            .when_some(insertion_indicator, |this, insertion_indicator| {
                this.child(
                    div()
                        .absolute()
                        .top(px(4.))
                        .bottom(px(4.))
                        .w(px(2.))
                        .bg(cx.theme().colors().drop_target_border)
                        .map(|indicator| match insertion_indicator {
                            TabInsertionIndicator::Left => indicator.left_0(),
                            TabInsertionIndicator::Right => indicator.right_0(),
                        }),
                )
            })
    }
}

impl Component for Tab {
    fn scope() -> ComponentScope {
        ComponentScope::Navigation
    }

    fn description() -> &'static str {
        "A tab component that can be used in a tabbed interface, \
        supporting different positions and states."
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> AnyElement {
        v_flex()
            .gap_6()
            .children(vec![example_group_with_title(
                "Variations",
                vec![
                    single_example(
                        "Default",
                        Tab::new("default").child("Default Tab").into_any_element(),
                    ),
                    single_example(
                        "Selected",
                        Tab::new("selected")
                            .toggle_state(true)
                            .child("Selected Tab")
                            .into_any_element(),
                    ),
                    single_example(
                        "First",
                        Tab::new("first")
                            .position(TabPosition::First)
                            .child("First Tab")
                            .into_any_element(),
                    ),
                    single_example(
                        "Middle",
                        Tab::new("middle")
                            .position(TabPosition::Middle(Ordering::Equal))
                            .child("Middle Tab")
                            .into_any_element(),
                    ),
                    single_example(
                        "Last",
                        Tab::new("last")
                            .position(TabPosition::Last)
                            .child("Last Tab")
                            .into_any_element(),
                    ),
                ],
            )])
            .into_any_element()
    }
}
