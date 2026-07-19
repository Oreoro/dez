use gpui::{AnyElement, prelude::*};
use smallvec::SmallVec;
use ui::prelude::*;

use crate::canvas;

#[derive(IntoElement)]
pub struct ExtensionCard {
    overridden_by_dev_extension: bool,
    children: SmallVec<[AnyElement; 2]>,
}

impl ExtensionCard {
    pub fn new() -> Self {
        Self {
            overridden_by_dev_extension: false,
            children: SmallVec::new(),
        }
    }

    pub fn overridden_by_dev_extension(mut self, overridden: bool) -> Self {
        self.overridden_by_dev_extension = overridden;
        self
    }
}

impl ParentElement for ExtensionCard {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for ExtensionCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div().w_full().child(
            v_flex()
                .mt_4()
                .w_full()
                .h(rems_from_px(110.))
                .p(canvas::extensions_card_padding(cx))
                .gap(canvas::extensions_gap(cx))
                .bg(canvas::extensions_card_background(cx))
                .border_1()
                .border_color(canvas::extensions_border(cx))
                .map(|card| canvas::extensions_radius(card, cx))
                .children(self.children)
                .when(self.overridden_by_dev_extension, |card| {
                    card.child(
                        h_flex()
                            .absolute()
                            .top_0()
                            .left_0()
                            .block_mouse_except_scroll()
                            .cursor_default()
                            .size_full()
                            .justify_center()
                            .bg(canvas::extensions_card_overlay_background(cx))
                            .child(Label::new("Overridden by dev extension.")),
                    )
                }),
        )
    }
}
