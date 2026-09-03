use gpui::{InteractiveElement, SharedString, Styled};

/// A trait for elements that can be made visible on hover by
/// tracking a specific group.
pub trait VisibleOnHover {
    /// Sets the element to only be visible when the specified group is hovered.
    ///
    /// Pass `""` as the `group_name` to use the global group.
    fn visible_on_hover(self, group_name: impl Into<SharedString>) -> Self;

    /// Sets the element to be visible when the specified group is hovered
    /// *or* when the element itself has keyboard focus.
    ///
    /// Prefer this over [`VisibleOnHover::visible_on_hover`] for focusable
    /// controls: while hidden, GPUI skips painting hitboxes and accessibility
    /// nodes entirely, so a purely hover-revealed control is unreachable by
    /// keyboard and invisible to screen readers.
    fn visible_on_hover_or_focus(self, group_name: impl Into<SharedString>) -> Self;
}

impl<E: InteractiveElement + Styled> VisibleOnHover for E {
    fn visible_on_hover(self, group_name: impl Into<SharedString>) -> Self {
        self.invisible()
            .group_hover(group_name, |style| style.visible())
    }

    fn visible_on_hover_or_focus(self, group_name: impl Into<SharedString>) -> Self {
        self.invisible()
            .group_hover(group_name, |style| style.visible())
            .focus_visible(|style| style.visible())
    }
}
