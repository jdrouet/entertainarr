use crate::domain::{
    podcast::entity::PodcastEpisode,
    prelude::{Page, Sort},
};

use super::entity::{Podcast, PodcastInput};

pub trait RssFeedLoader: Send + Sync + 'static {
    fn load(&self, feed_url: &str) -> impl Future<Output = anyhow::Result<PodcastInput>> + Send;
}

pub trait PodcastRepository: Send + Sync + 'static {
    fn find_by_feed_url(
        &self,
        feed_url: &str,
    ) -> impl Future<Output = anyhow::Result<Option<Podcast>>> + Send;
    fn list_by_ids(
        &self,
        podcast_ids: &[u64],
    ) -> impl Future<Output = anyhow::Result<Vec<Podcast>>> + Send;
    fn upsert(&self, entity: &PodcastInput)
    -> impl Future<Output = anyhow::Result<Podcast>> + Send;
}

pub trait PodcastSubscriptionRepository: Send + Sync + 'static {
    fn list(&self, user_id: u64) -> impl Future<Output = anyhow::Result<Vec<Podcast>>> + Send;
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
    ) -> impl Future<Output = anyhow::Result<Vec<Podcast>>> + Send;
    fn subscribe(
        &self,
        user_id: u64,
        feed_url: &str,
    ) -> impl Future<Output = anyhow::Result<Podcast>> + Send;
    fn unsubscribe(
        &self,
        user_id: u64,
        podcast_id: u64,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
    fn list_by_ids(
        &self,
        podcast_ids: &[u64],
    ) -> impl Future<Output = anyhow::Result<Vec<Podcast>>> + Send;
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
    async fn list_by_ids(
        &self,
        podcast_ids: &[u64],
    ) -> anyhow::Result<Vec<super::entity::Podcast>> {
        self.as_ref().list_by_ids(podcast_ids).await
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
        fn list_by_ids(
            &self,
            podcast_ids: &[u64],
        ) -> impl Future<Output = anyhow::Result<Vec<super::entity::Podcast>>> + Send;
    }
}

pub trait PodcastEpisodeRepository: Send + Sync + 'static {
    fn list(
        &self,
        params: ListPodcastEpisodeParams,
    ) -> impl Future<Output = anyhow::Result<Vec<PodcastEpisode>>> + Send;
}

#[derive(Clone, Copy, Debug)]
pub enum PodcastEpisodeField {
    PublishedAt,
}

#[derive(Clone, Copy, Debug)]
pub struct ListPodcastEpisodeFilter {
    pub subscribed: Option<bool>,
    pub watched: Option<bool>,
}

#[derive(Clone, Copy, Debug)]
pub struct ListPodcastEpisodeParams {
    pub user_id: u64,
    pub filter: ListPodcastEpisodeFilter,
    pub sort: Sort<PodcastEpisodeField>,
    pub page: Page,
}

pub trait PodcastEpisodeService: Send + Sync + 'static {
    fn list(
        &self,
        params: ListPodcastEpisodeParams,
    ) -> impl Future<Output = anyhow::Result<Vec<super::entity::PodcastEpisode>>> + Send;
}

#[cfg(test)]
impl<S: PodcastEpisodeService> PodcastEpisodeService for std::sync::Arc<S> {
    async fn list(
        &self,
        params: ListPodcastEpisodeParams,
    ) -> anyhow::Result<Vec<super::entity::PodcastEpisode>> {
        self.as_ref().list(params).await
    }
}

#[cfg(test)]
mockall::mock! {
    pub PodcastEpisodeService {}

    impl PodcastEpisodeService for PodcastEpisodeService {
        fn list(
            &self,
            params: ListPodcastEpisodeParams,
        ) -> impl Future<Output = anyhow::Result<Vec<super::entity::PodcastEpisode>>> + Send;
    }
}
