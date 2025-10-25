use crux_core::{Command, render::render};

impl crate::Application {
    pub fn update_home(
        &self,
        event: super::Event,
        root: &mut crate::Model,
    ) -> Command<crate::Effect, crate::Event> {
        let crate::Model::Authenticated {
            authentication_token,
            model: crate::domain::AuthenticatedModel::Home(model),
            server_url,
        } = root
        else {
            return render();
        };

        match event {
            super::Event::Initialize => Command::all([Command::event(
                super::Event::ListPodcastEpisodesRequest.into(),
            )]),
            super::Event::ListPodcastEpisodesRequest => {
                model.podcast_episodes_loading = true;
                Command::all([
                    super::execute::list_podcast_episodes(&server_url, &authentication_token),
                    render(),
                ])
            }
            super::Event::ListPodcastEpisodesSuccess(podcasts) => {
                model.podcast_episodes_loading = false;
                model.podcast_episodes = podcasts;
                render()
            }
            super::Event::ListPodcastEpisodesError(_) => {
                model.podcast_episodes_loading = false;
                render()
            }
        }
    }
}
