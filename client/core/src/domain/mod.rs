pub mod authentication;
pub mod home;
pub mod init;
pub mod podcast_subscription;

pub enum AuthenticatedModel {
    Home(home::Model),
    PodcastSubscription(podcast_subscription::Model),
}

impl Default for AuthenticatedModel {
    fn default() -> Self {
        Self::Home(Default::default())
    }
}
