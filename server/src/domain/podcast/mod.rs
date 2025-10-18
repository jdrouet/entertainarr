use crate::domain::podcast::prelude::PodcastEpisodeRepository;

pub mod entity;
pub mod prelude;

#[derive(Clone, Debug, bon::Builder)]
pub struct PodcastService<RFL, PR, PSR> {
    rss_feed_loader: RFL,
    podcast_repository: PR,
    podcast_subscription_repository: PSR,
}

impl<RFL, PR, PSR> PodcastService<RFL, PR, PSR>
where
    RFL: Clone + prelude::RssFeedLoader,
    PR: Clone + prelude::PodcastRepository,
    PSR: Clone + prelude::PodcastSubscriptionRepository,
{
    async fn find_or_sync_by_feed_url(
        &self,
        feed_url: &str,
    ) -> anyhow::Result<self::entity::Podcast> {
        if let Some(item) = self.podcast_repository.find_by_feed_url(feed_url).await? {
            return Ok(item);
        }

        let loaded = self.rss_feed_loader.load(feed_url).await?;
        self.podcast_repository.upsert(&loaded).await
    }
}

impl<RFL, PR, PSR> prelude::PodcastService for PodcastService<RFL, PR, PSR>
where
    RFL: Clone + prelude::RssFeedLoader,
    PR: Clone + prelude::PodcastRepository,
    PSR: Clone + prelude::PodcastSubscriptionRepository,
{
    async fn list_by_ids(&self, podcast_ids: &[u64]) -> anyhow::Result<Vec<entity::Podcast>> {
        self.podcast_repository.list_by_ids(podcast_ids).await
    }

    async fn subscriptions(&self, user_id: u64) -> anyhow::Result<Vec<self::entity::Podcast>> {
        self.podcast_subscription_repository.list(user_id).await
    }

    async fn subscribe(
        &self,
        user_id: u64,
        feed_url: &str,
    ) -> anyhow::Result<self::entity::Podcast> {
        let subscription = self.find_or_sync_by_feed_url(feed_url).await?;
        self.podcast_subscription_repository
            .create(user_id, subscription.id)
            .await?;
        Ok(subscription)
    }

    async fn unsubscribe(&self, user_id: u64, podcast_id: u64) -> anyhow::Result<()> {
        self.podcast_subscription_repository
            .delete(user_id, podcast_id)
            .await
    }
}

#[derive(Clone, Debug, bon::Builder)]
pub struct PodcastEpisodeService<PER> {
    podcast_episode_repository: PER,
}

impl<PER> prelude::PodcastEpisodeService for PodcastEpisodeService<PER>
where
    PER: PodcastEpisodeRepository,
{
    async fn list(
        &self,
        params: prelude::ListPodcastEpisodeParams,
    ) -> anyhow::Result<Vec<self::entity::PodcastEpisode>> {
        self.podcast_episode_repository.list(params).await
    }
}
