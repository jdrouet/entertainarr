pub trait RssFeedLoader: Send + Sync + 'static {
    fn load(
        &self,
        feed_url: &str,
    ) -> impl Future<Output = anyhow::Result<super::entity::PodcastWithEpisodes>> + Send;
}

pub trait PodcastRepository: Send + Sync + 'static {
    fn find_by_feed_url(
        &self,
        feed_url: &str,
    ) -> impl Future<Output = anyhow::Result<Option<super::entity::Podcast>>> + Send;
    fn upsert(
        &self,
        entity: &super::entity::PodcastWithEpisodes,
    ) -> impl Future<Output = anyhow::Result<super::entity::Podcast>> + Send;
}

pub trait PodcastSubscriptionRepository: Send + Sync + 'static {
    fn list(
        &self,
        user_id: u64,
    ) -> impl Future<Output = anyhow::Result<Vec<super::entity::Podcast>>> + Send;
    fn create(
        &self,
        user_id: u64,
        subscription_id: u64,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
    fn delete(
        &self,
        user_id: u64,
        subscription_id: u64,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

pub trait PodcastService: Send + Sync + 'static {
    fn subscriptions(
        &self,
        user_id: u64,
    ) -> impl Future<Output = anyhow::Result<Vec<super::entity::Podcast>>> + Send;
    fn subscribe(
        &self,
        user_id: u64,
        feed_url: &str,
    ) -> impl Future<Output = anyhow::Result<super::entity::Podcast>> + Send;
    fn unsubscribe(
        &self,
        user_id: u64,
        podcast_id: u64,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

#[cfg(test)]
impl<S: PodcastService> PodcastService for std::sync::Arc<S> {
    async fn subscriptions(&self, user_id: u64) -> anyhow::Result<Vec<super::entity::Podcast>> {
        self.as_ref().subscriptions(user_id).await
    }
    async fn subscribe(
        &self,
        user_id: u64,
        feed_url: &str,
    ) -> anyhow::Result<super::entity::Podcast> {
        self.as_ref().subscribe(user_id, feed_url).await
    }
    async fn unsubscribe(&self, user_id: u64, podcast_id: u64) -> anyhow::Result<()> {
        self.as_ref().unsubscribe(user_id, podcast_id).await
    }
}

#[cfg(test)]
mockall::mock! {
    pub PodcastService {}

    impl PodcastService for PodcastService {
        fn subscriptions(
            &self,
            user_id: u64,
        ) -> impl Future<Output = anyhow::Result<Vec<super::entity::Podcast>>> + Send;
        fn subscribe(
            &self,
            user_id: u64,
            feed_url: &str,
        ) -> impl Future<Output = anyhow::Result<super::entity::Podcast>> + Send;
        fn unsubscribe(
            &self,
            user_id: u64,
            podcast_id: u64,
        ) -> impl Future<Output = anyhow::Result<()>> + Send;
    }
}
