use crate::domain::auth::prelude::AuthenticationService;
use crate::domain::podcast::prelude::{PodcastEpisodeService, PodcastService};

pub trait ServerState: Send + Sync + 'static {
    fn authentication_service(&self) -> &impl AuthenticationService;
    fn podcast_service(&self) -> &impl PodcastService;
    fn podcast_episode_service(&self) -> &impl PodcastEpisodeService;
}

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use crate::domain::auth::prelude::AuthenticationService;
    use crate::domain::podcast::prelude::{PodcastEpisodeService, PodcastService};

    #[derive(Default)]
    pub struct MockServerStateBuilder {
        pub authentication: Option<crate::domain::auth::prelude::MockAuthenticationService>,
        pub podcast: Option<crate::domain::podcast::prelude::MockPodcastService>,
        pub podcast_episode: Option<crate::domain::podcast::prelude::MockPodcastEpisodeService>,
    }

    impl MockServerStateBuilder {
        pub fn build(self) -> MockServerState {
            MockServerState {
                authentication: Arc::new(self.authentication.unwrap_or_default()),
                podcast: Arc::new(self.podcast.unwrap_or_default()),
                podcast_episode: Arc::new(self.podcast_episode.unwrap_or_default()),
            }
        }

        pub fn authentication(
            mut self,
            item: crate::domain::auth::prelude::MockAuthenticationService,
        ) -> Self {
            self.authentication = Some(item);
            self
        }

        pub fn podcast(
            mut self,
            item: crate::domain::podcast::prelude::MockPodcastService,
        ) -> Self {
            self.podcast = Some(item);
            self
        }
    }

    #[derive(Clone, Default)]
    pub struct MockServerState {
        pub authentication: Arc<crate::domain::auth::prelude::MockAuthenticationService>,
        pub podcast: Arc<crate::domain::podcast::prelude::MockPodcastService>,
        pub podcast_episode: Arc<crate::domain::podcast::prelude::MockPodcastEpisodeService>,
    }

    impl MockServerState {
        pub fn builder() -> MockServerStateBuilder {
            MockServerStateBuilder::default()
        }
    }

    impl super::ServerState for MockServerState {
        fn authentication_service(&self) -> &impl AuthenticationService {
            &self.authentication
        }

        fn podcast_service(&self) -> &impl PodcastService {
            &self.podcast
        }

        fn podcast_episode_service(&self) -> &impl PodcastEpisodeService {
            &self.podcast_episode
        }
    }
}
