use std::rc::Rc;

use gpui::ParentElement;
use gpui::Styled;
use gpui::{App, ElementId, IntoElement, RenderOnce, SharedString};
use heck::ToTitleCase as _;
use ui::{
    ButtonSize, ContextMenu, ContextMenuEntry, Disableable as _, DropdownMenu, DropdownStyle,
    FluentBuilder as _, Icon, IconName, IconPosition, IconSize, h_flex, px,
};

#[derive(IntoElement)]
pub struct EnumVariantDropdown<T>
where
    T: strum::VariantArray + strum::VariantNames + Copy + PartialEq + Send + Sync + 'static,
{
    id: ElementId,
    current_value: T,
    variants: &'static [T],
    labels: &'static [&'static str],
    should_do_title_case: bool,
    tab_index: Option<isize>,
    disabled: bool,
    aria_label: Option<SharedString>,
    aria_description: Option<SharedString>,
    icon_for_value: Option<Rc<dyn Fn(T) -> IconName + 'static>>,
    on_change: Rc<dyn Fn(T, &mut ui::Window, &mut App) + 'static>,
}

impl<T> EnumVariantDropdown<T>
where
    T: strum::VariantArray + strum::VariantNames + Copy + PartialEq + Send + Sync + 'static,
{
    pub fn new(
        id: impl Into<ElementId>,
        current_value: T,
        variants: &'static [T],
        labels: &'static [&'static str],
        on_change: impl Fn(T, &mut ui::Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            current_value,
            variants,
            labels,
            should_do_title_case: true,
            tab_index: None,
            disabled: false,
            aria_label: None,
            aria_description: None,
            icon_for_value: None,
            on_change: Rc::new(on_change),
        }
    }

    pub fn title_case(mut self, title_case: bool) -> Self {
        self.should_do_title_case = title_case;
        self
    }

    pub fn tab_index(mut self, tab_index: isize) -> Self {
        self.tab_index = Some(tab_index);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn icon_for_value(mut self, icon_for_value: impl Fn(T) -> IconName + 'static) -> Self {
        self.icon_for_value = Some(Rc::new(icon_for_value));
        self
    }

    /// Sets the label announced by assistive technology.
    /// Defaults to the currently selected value's label.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    /// Sets the supplementary description announced by assistive technology
    /// after the combobox's name, role, and value (e.g. a setting subtitle).
    pub fn aria_description(mut self, description: impl Into<SharedString>) -> Self {
        self.aria_description = Some(description.into());
        self
    }
}

impl<T> RenderOnce for EnumVariantDropdown<T>
where
    T: strum::VariantArray + strum::VariantNames + Copy + PartialEq + Send + Sync + 'static,
{
    fn render(self, window: &mut ui::Window, cx: &mut ui::App) -> impl gpui::IntoElement {
        let Self {
            id,
            current_value,
            variants,
            labels,
            should_do_title_case,
            tab_index,
            disabled,
            aria_label,
            aria_description,
            icon_for_value,
            on_change,
        } = self;

        let current_value_index = variants
            .iter()
            .position(|value| *value == current_value)
            .unwrap_or_default();
        let current_value_label = labels.get(current_value_index).copied().unwrap_or_default();
        let visible_label = if should_do_title_case {
            current_value_label.to_title_case()
        } else {
            current_value_label.to_string()
        };
        let menu_icon_for_value = icon_for_value.clone();

        let context_menu = window.use_keyed_state(current_value_label, cx, |window, cx| {
            ContextMenu::new(window, cx, move |mut menu, _, _| {
                for (&value, &label) in std::iter::zip(variants, labels) {
                    let on_change = on_change.clone();
                    let entry = ContextMenuEntry::new(if should_do_title_case {
                        label.to_title_case()
                    } else {
                        label.to_string()
                    })
                    .toggleable(IconPosition::End, value == current_value)
                    .handler(move |window, cx| {
                        on_change(value, window, cx);
                    });
                    menu = if let Some(icon_for_value) = menu_icon_for_value.as_ref() {
                        menu.item(entry.icon(icon_for_value(value)))
                    } else {
                        menu.item(entry)
                    };
                }
                menu
            })
        });

        let dropdown = if let Some(icon_for_value) = icon_for_value {
            DropdownMenu::new_with_element(
                id,
                h_flex()
                    .gap_1p5()
                    .child(Icon::new(icon_for_value(current_value)).size(IconSize::Small))
                    .child(visible_label.clone())
                    .into_any_element(),
                context_menu,
            )
            .aria_value(visible_label)
        } else {
            DropdownMenu::new(id, visible_label, context_menu)
        };

        dropdown
            .when_some(aria_label, |this, label| this.aria_label(label))
            .when_some(aria_description, |this, description| {
                this.aria_description(description)
            })
            .disabled(disabled)
            .when_some(tab_index, |elem, tab_index| elem.tab_index(tab_index))
            .trigger_size(ButtonSize::Medium)
            .style(DropdownStyle::Outlined)
            .offset(gpui::Point {
                x: px(0.0),
                y: px(2.0),
            })
            .into_any_element()
    }
}
