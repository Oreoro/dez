use std::sync::Arc;

use client::{Client, UserStore};
use gpui::{Entity, IntoElement, ParentElement};
use language_model::{LanguageModelRegistry, ZED_CLOUD_PROVIDER_ID};
use ui::prelude::*;

use crate::{AgentPanelOnboardingCard, ApiKeysWithoutProviders};

pub struct AgentPanelOnboarding {
    has_configured_providers: bool,
    continue_to_agent: Arc<dyn Fn(&mut Window, &mut App)>,
}

impl AgentPanelOnboarding {
    pub fn new(
        _user_store: Entity<UserStore>,
        _client: Arc<Client>,
        continue_to_agent: impl Fn(&mut Window, &mut App) + 'static,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.subscribe(
            &LanguageModelRegistry::global(cx),
            |this: &mut Self, _registry, event: &language_model::Event, cx| match event {
                language_model::Event::ProviderStateChanged(_)
                | language_model::Event::AddedProvider(_)
                | language_model::Event::RemovedProvider(_)
                | language_model::Event::ProvidersChanged => {
                    this.has_configured_providers = Self::has_configured_providers(cx)
                }
                _ => {}
            },
        )
        .detach();

        Self {
            has_configured_providers: Self::has_configured_providers(cx),
            continue_to_agent: Arc::new(continue_to_agent),
        }
    }

    fn has_configured_providers(cx: &App) -> bool {
        LanguageModelRegistry::read_global(cx)
            .visible_providers()
            .iter()
            .any(|provider| provider.is_authenticated(cx) && provider.id() != ZED_CLOUD_PROVIDER_ID)
    }
}

impl Render for AgentPanelOnboarding {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let continue_to_agent = self.continue_to_agent.clone();
        AgentPanelOnboardingCard::new()
            .child(
                v_flex()
                    .relative()
                    .gap_2()
                    .child(Headline::new(if self.has_configured_providers {
                        "Agent ready"
                    } else {
                        "Connect an AI provider"
                    }))
                    .child(
                        Label::new(if self.has_configured_providers {
                            "Use your configured provider for a native agent thread."
                        } else {
                            "Dez keeps provider choice explicit. Add a provider to start a native agent thread."
                        })
                        .color(Color::Muted),
                    )
                    .when(!self.has_configured_providers, |this| {
                        this.child(ApiKeysWithoutProviders::new())
                    })
                    .when(self.has_configured_providers, |this| {
                        this.child(
                            Button::new("continue-to-agent", "Start Agent")
                                .full_width()
                                .style(ButtonStyle::Filled)
                                .on_click(move |_, window, cx| {
                                    continue_to_agent(window, cx);
                                }),
                        )
                    }),
            )
    }
}
