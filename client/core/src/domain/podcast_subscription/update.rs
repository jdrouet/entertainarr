use crux_core::{Command, render::render};

use crate::domain::AuthenticatedModel;

impl crate::Application {
    pub fn update_podcast_subscription(
        &self,
        event: super::Event,
        root: &mut crate::Model,
    ) -> Command<crate::Effect, crate::Event> {
        let crate::Model::Authenticated {
            authentication_token: _,
            model,
            server_url: _,
        } = root
        else {
            return render();
        };
        if matches!(event, super::Event::Open) {
            let _ = std::mem::replace(
                model,
                AuthenticatedModel::PodcastSubscription(super::Model { loading: true }),
            );
            return render();
        }
        let crate::domain::AuthenticatedModel::PodcastSubscription(model) = model else {
            return render();
        };

        match event {
            super::Event::Open => {} // already handled
            super::Event::Submit { url: _ } => {
                model.loading = true;
            }
            super::Event::Success => {
                model.loading = false;
            }
            super::Event::Error(_) => {
                model.loading = false;
            }
        }

        render()
    }
}
