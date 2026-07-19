use gpui::{
    AnyElement, Hsla, IntoElement, ParentElement, Pixels, linear_color_stop, linear_gradient,
};
use smallvec::SmallVec;
use ui::prelude::*;

#[derive(IntoElement)]
pub struct AgentPanelOnboardingCard {
    children: SmallVec<[AnyElement; 2]>,
    outer_padding: Option<Pixels>,
    frame_padding: Option<Pixels>,
    content_padding_x: Option<Pixels>,
    content_padding_y: Option<Pixels>,
    content_gap: Option<Pixels>,
    outer_background: Option<Hsla>,
    frame_background: Option<Hsla>,
    content_background: Option<Hsla>,
    content_border: Option<Hsla>,
    frame_radius: Option<Pixels>,
    content_radius: Option<Pixels>,
    gradient_start: Option<Hsla>,
    gradient_end: Option<Hsla>,
}

impl AgentPanelOnboardingCard {
    pub fn new() -> Self {
        Self {
            children: SmallVec::new(),
            outer_padding: None,
            frame_padding: None,
            content_padding_x: None,
            content_padding_y: None,
            content_gap: None,
            outer_background: None,
            frame_background: None,
            content_background: None,
            content_border: None,
            frame_radius: None,
            content_radius: None,
            gradient_start: None,
            gradient_end: None,
        }
    }

    pub fn outer_padding(mut self, padding: Pixels) -> Self {
        self.outer_padding = Some(padding);
        self
    }

    pub fn frame_padding(mut self, padding: Pixels) -> Self {
        self.frame_padding = Some(padding);
        self
    }

    pub fn content_padding(mut self, x: Pixels, y: Pixels) -> Self {
        self.content_padding_x = Some(x);
        self.content_padding_y = Some(y);
        self
    }

    pub fn content_gap(mut self, gap: Pixels) -> Self {
        self.content_gap = Some(gap);
        self
    }

    pub fn backgrounds(mut self, outer: Hsla, frame: Hsla, content: Hsla) -> Self {
        self.outer_background = Some(outer);
        self.frame_background = Some(frame);
        self.content_background = Some(content);
        self
    }

    pub fn content_border(mut self, border: Hsla) -> Self {
        self.content_border = Some(border);
        self
    }

    pub fn radii(mut self, frame: Pixels, content: Pixels) -> Self {
        self.frame_radius = Some(frame);
        self.content_radius = Some(content);
        self
    }

    pub fn gradient(mut self, start: Hsla, end: Hsla) -> Self {
        self.gradient_start = Some(start);
        self.gradient_end = Some(end);
        self
    }
}

impl ParentElement for AgentPanelOnboardingCard {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for AgentPanelOnboardingCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = cx.theme().colors();
        let Self {
            children,
            outer_padding,
            frame_padding,
            content_padding_x,
            content_padding_y,
            content_gap,
            outer_background,
            frame_background,
            content_background,
            content_border,
            frame_radius,
            content_radius,
            gradient_start,
            gradient_end,
        } = self;

        let outer_padding = outer_padding.unwrap_or(px(10.));
        let frame_padding = frame_padding.unwrap_or(px(3.));
        let content_padding_x = content_padding_x.unwrap_or(px(16.));
        let content_padding_y = content_padding_y.unwrap_or(px(12.));
        let content_gap = content_gap.unwrap_or(px(8.));
        let outer_background = outer_background.unwrap_or(color.editor_background);
        let frame_background = frame_background.unwrap_or(color.background.opacity(0.5));
        let content_background = content_background.unwrap_or(color.panel_background);
        let content_border = content_border.unwrap_or(color.text.opacity(0.1));
        let content_radius = content_radius.unwrap_or(px(5.));
        let gradient_start = gradient_start.unwrap_or(color.panel_background);
        let gradient_end = gradient_end.unwrap_or(color.editor_background);

        let frame = div()
            .min_w_0()
            .p(frame_padding)
            .elevation_2(cx)
            .bg(frame_background);
        let frame = if let Some(radius) = frame_radius {
            frame.rounded(radius)
        } else {
            frame.rounded_lg()
        };

        div().min_w_0().p(outer_padding).bg(outer_background).child(
            frame.child(
                v_flex()
                    .relative()
                    .size_full()
                    .min_w_0()
                    .px(content_padding_x)
                    .py(content_padding_y)
                    .gap(content_gap)
                    .border_1()
                    .rounded(content_radius)
                    .border_color(content_border)
                    .bg(content_background)
                    .overflow_hidden()
                    .child(
                        div()
                            .absolute()
                            .inset_0()
                            .size_full()
                            .rounded(content_radius)
                            .overflow_hidden()
                            .bg(linear_gradient(
                                360.,
                                linear_color_stop(gradient_start, 1.0),
                                linear_color_stop(gradient_end, 0.45),
                            )),
                    )
                    .children(children),
            ),
        )
    }
}
