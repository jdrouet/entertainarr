use crux_core::render::render;

use crate::application::{ApplicationState, authenticated::AuthenticatedModel, router::Route};

impl crate::application::ApplicationModel {
    pub fn handle_podcast_subscribe_event(
        &mut self,
        event: super::PodcastSubscribeEvent,
    ) -> crate::ApplicationCommand {
        let Some(token) = self.session.as_ref().map(|session| session.token.as_str()) else {
            return render();
        };
        let Some(server_url) = self.server_url.as_deref() else {
            return render();
        };
        let ApplicationState::Authenticated(AuthenticatedModel::PodcastSubscribe(model)) =
            &mut self.state
        else {
            return render();
        };
        match event {
            super::PodcastSubscribeEvent::Submit(req) => {
                model.loading = true;
                crate::ApplicationCommand::all([req.execute(server_url, token), render()])
            }
            super::PodcastSubscribeEvent::Success => {
                model.loading = false;
                Route::Home.into()
            }
            super::PodcastSubscribeEvent::Error(err) => {
                model.loading = false;
                model.error = Some(err);
                render()
            }
        }
    }
}
