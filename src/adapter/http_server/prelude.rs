use crate::domain::auth::prelude::AuthenticationService;
use crate::domain::podcast::prelude::PodcastService;

pub trait ServerState: Send + Sync + 'static {
    fn authentication_service(&self) -> &impl AuthenticationService;
    fn podcast_service(&self) -> &impl PodcastService;
}

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use crate::domain::auth::prelude::AuthenticationService;
    use crate::domain::podcast::prelude::PodcastService;

    #[derive(Default)]
    pub struct MockServerStateBuilder {
        pub authentication: Option<crate::domain::auth::prelude::MockAuthenticationService>,
        pub podcast: Option<crate::domain::podcast::prelude::MockPodcastService>,
    }

    impl MockServerStateBuilder {
        pub fn build(self) -> MockServerState {
            MockServerState {
                authentication: Arc::new(self.authentication.unwrap_or_default()),
                podcast: Arc::new(self.podcast.unwrap_or_default()),
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
    }
}
