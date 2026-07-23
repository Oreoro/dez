use crate::{Divider, DividerColor, KeyBinding, prelude::*};
use gpui::{ClickEvent, FocusHandle};

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct ProjectEmptyState {
    label: SharedString,
    title: Option<SharedString>,
    description: Option<SharedString>,
    open_project_label: SharedString,
    clone_repo_label: SharedString,
    top_aligned: bool,
    focus_handle: FocusHandle,
    open_project_key_binding: KeyBinding,
    on_open_project: Option<ClickHandler>,
    on_clone_repo: Option<ClickHandler>,
}

impl ProjectEmptyState {
    pub fn new(
        label: impl Into<SharedString>,
        focus_handle: FocusHandle,
        open_project_key_binding: KeyBinding,
    ) -> Self {
        Self {
            label: label.into(),
            title: None,
            description: None,
            open_project_label: "Open Project".into(),
            clone_repo_label: "Clone Repository".into(),
            top_aligned: false,
            focus_handle,
            open_project_key_binding,
            on_open_project: None,
            on_clone_repo: None,
        }
    }

    /// Replaces the inherited project-centric copy with wording tailored to
    /// the region using this empty state.
    pub fn with_copy(
        mut self,
        title: impl Into<SharedString>,
        description: impl Into<SharedString>,
        open_project_label: impl Into<SharedString>,
        clone_repo_label: impl Into<SharedString>,
    ) -> Self {
        self.title = Some(title.into());
        self.description = Some(description.into());
        self.open_project_label = open_project_label.into();
        self.clone_repo_label = clone_repo_label.into();
        self
    }

    /// Keeps recovery guidance near the region header instead of floating it
    /// in the middle of an otherwise empty pane.
    pub fn top_aligned(mut self) -> Self {
        self.top_aligned = true;
        self
    }

    pub fn on_open_project(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_project = Some(Box::new(handler));
        self
    }

    pub fn on_clone_repo(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_clone_repo = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for ProjectEmptyState {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let id = format!("empty-state-{}", self.label);
        let description = self.description.unwrap_or_else(|| {
            format!("Choose one of the options below to use the {}", self.label).into()
        });
        let has_title = self.title.is_some();
        let open_project_button = Button::new("open_project", self.open_project_label)
            .full_width()
            .key_binding(self.open_project_key_binding)
            .when(has_title, |button| {
                button
                    .style(ButtonStyle::Filled)
                    .start_icon(Icon::new(IconName::FolderOpen).size(IconSize::Small))
            })
            .when_some(self.on_open_project, |button, handler| {
                button.on_click(handler)
            });
        let clone_repo_button = Button::new("clone_repo", self.clone_repo_label)
            .full_width()
            .when(has_title, |button| {
                button
                    .style(ButtonStyle::Outlined)
                    .start_icon(Icon::new(IconName::GitBranch).size(IconSize::Small))
            })
            .when_some(self.on_clone_repo, |button, handler| {
                button.on_click(handler)
            });

        v_flex()
            .id(id)
            .p_4()
            .size_full()
            .items_center()
            .role(gpui::Role::Region)
            .aria_label(self.title.clone().unwrap_or_else(|| self.label.clone()))
            .when(self.top_aligned, |this| this.justify_start().pt_8())
            .when(!self.top_aligned, |this| this.justify_center())
            .track_focus(&self.focus_handle)
            .child(
                v_flex()
                    .max_w_full()
                    .when(has_title, |this| this.w_64().gap_2())
                    .when(!has_title, |this| this.w_48().gap_1())
                    .children(self.title.map(|title| {
                        h_flex()
                            .gap_1p5()
                            .child(
                                Icon::new(IconName::FolderOpen)
                                    .size(IconSize::Small)
                                    .color(Color::Accent),
                            )
                            .child(Label::new(title).size(LabelSize::Large))
                    }))
                    .child(
                        div()
                            .when(!has_title, |this| this.text_center().mb_2())
                            .child(
                                Label::new(description)
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    )
                    .child(open_project_button)
                    .when(!has_title, |this| {
                        this.child(
                            h_flex()
                                .gap_2()
                                .child(Divider::horizontal().color(DividerColor::Border))
                                .child(Label::new("or").size(LabelSize::XSmall).color(Color::Muted))
                                .child(Divider::horizontal().color(DividerColor::Border)),
                        )
                    })
                    .child(clone_repo_button),
            )
    }
}
