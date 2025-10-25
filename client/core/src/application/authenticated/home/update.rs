use crux_core::render::render;

use crate::application::{ApplicationState, authenticated::AuthenticatedModel};

impl crate::application::ApplicationModel {
    pub fn handle_home_event(&mut self, event: super::HomeEvent) -> crate::ApplicationCommand {
        let Some(token) = self.session.as_ref().map(|session| session.token.as_str()) else {
            return render();
        };
        let Some(server_url) = self.server_url.as_deref() else {
            return render();
        };

        let ApplicationState::Authenticated(AuthenticatedModel::Home(model)) = &mut self.state
        else {
            return render();
        };
        match event {
            super::HomeEvent::ListPodcastEpisodesRequest => {
                model.podcast_episodes_loading = true;
                model.podcast_episodes_error = false;
                crate::ApplicationCommand::all([
                    super::execute::list_podcast_episodes(server_url, token),
                    render(),
                ])
            }
            super::HomeEvent::ListPodcastEpisodesSuccess(list) => {
                model.podcast_episodes = list;
                model.podcast_episodes_loading = false;
                render()
            }
            super::HomeEvent::ListPodcastEpisodesError => {
                model.podcast_episodes_loading = false;
                model.podcast_episodes_error = true;
                render()
            }
        }
    }
}
