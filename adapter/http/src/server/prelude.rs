use entertainarr_domain::auth::prelude::AuthenticationService;
use entertainarr_domain::podcast::prelude::{PodcastEpisodeService, PodcastService};

use crate::server::handler::client::prelude::ClientService;

pub trait ServerState: Send + Sync + 'static {
    fn authentication_service(&self) -> &impl AuthenticationService;
    fn client_service(&self) -> &impl ClientService;
    fn podcast_service(&self) -> &impl PodcastService;
    fn podcast_episode_service(&self) -> &impl PodcastEpisodeService;
}

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use entertainarr_domain::auth::prelude::AuthenticationService;
    use entertainarr_domain::podcast::prelude::{PodcastEpisodeService, PodcastService};

    use crate::server::handler::client::prelude::{ClientService, MockClientService};

    #[derive(Default)]
    pub struct MockServerStateBuilder {
        pub authentication: Option<entertainarr_domain::auth::prelude::MockAuthenticationService>,
        pub podcast: Option<entertainarr_domain::podcast::prelude::MockPodcastService>,
        pub podcast_episode:
            Option<entertainarr_domain::podcast::prelude::MockPodcastEpisodeService>,
    }

    impl MockServerStateBuilder {
        pub fn build(self) -> MockServerState {
            MockServerState {
                authentication: Arc::new(self.authentication.unwrap_or_default()),
                client: MockClientService,
                podcast: Arc::new(self.podcast.unwrap_or_default()),
                podcast_episode: Arc::new(self.podcast_episode.unwrap_or_default()),
            }
        }

        pub fn authentication(
            mut self,
            item: entertainarr_domain::auth::prelude::MockAuthenticationService,
        ) -> Self {
            self.authentication = Some(item);
            self
        }

        pub fn podcast(
            mut self,
            item: entertainarr_domain::podcast::prelude::MockPodcastService,
        ) -> Self {
            self.podcast = Some(item);
            self
        }
    }

    #[derive(Clone, Default)]
    pub struct MockServerState {
        pub authentication: Arc<entertainarr_domain::auth::prelude::MockAuthenticationService>,
        pub client: MockClientService,
        pub podcast: Arc<entertainarr_domain::podcast::prelude::MockPodcastService>,
        pub podcast_episode: Arc<entertainarr_domain::podcast::prelude::MockPodcastEpisodeService>,
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

        fn client_service(&self) -> &impl ClientService {
            &self.client
        }

        fn podcast_service(&self) -> &impl PodcastService {
            &self.podcast
        }

        fn podcast_episode_service(&self) -> &impl PodcastEpisodeService {
            &self.podcast_episode
        }
    }
}
