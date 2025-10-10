use std::str::FromStr;

use anyhow::Context;

use crate::domain::podcast::entity::{PodcastEpisodeInput, PodcastInput};

impl TryFrom<super::ChannelItem> for PodcastEpisodeInput {
    type Error = anyhow::Error;

    fn try_from(value: super::ChannelItem) -> Result<Self, Self::Error> {
        Ok(PodcastEpisodeInput {
            guid: value.guid,
            published_at: value.publication_date,
            title: value
                .title
                .ok_or_else(|| anyhow::anyhow!("title attribute not specified"))?,
            description: value.description,
            link: value.link,
            duration: value.itunes_duration,
            file_url: value
                .enclosure_url
                .ok_or_else(|| anyhow::anyhow!("enclure_url attribute not specified"))?,
            file_size: value.enclosure_length,
            file_type: value.enclosure_type,
        })
    }
}

impl TryFrom<super::Channel> for PodcastInput {
    type Error = anyhow::Error;

    fn try_from(value: super::Channel) -> Result<Self, Self::Error> {
        let episodes = value
            .items
            .into_iter()
            .filter_map(|item| {
                PodcastEpisodeInput::try_from(item)
                    .inspect_err(|err| tracing::warn!(error = ?err, "unable to parse episode"))
                    .ok()
            })
            .collect::<Vec<_>>();
        Ok(PodcastInput {
            feed_url: value
                .atom_link_href
                .ok_or_else(|| anyhow::anyhow!("atom link attribute not specified"))?,
            title: value
                .title
                .ok_or_else(|| anyhow::anyhow!("title attribute not specified"))?,
            description: value.description,
            image_url: value.image_url.or(value.itunes_image_href),
            language: value.language,
            website: value.link,
            episodes,
        })
    }
}

impl crate::domain::podcast::prelude::RssFeedLoader for super::RssClient {
    #[tracing::instrument(skip(self), err(Debug))]
    async fn load(&self, feed_url: &str) -> anyhow::Result<PodcastInput> {
        let res = self
            .client
            .get(feed_url)
            .send()
            .await
            .context("unable to query server")?;
        let body = res.text().await.context("unable to query payload")?;
        let mut rss = super::Rss::from_str(body.as_str()).context("unable to parse rss result")?;
        let channel = rss
            .channels
            .pop()
            .ok_or_else(|| anyhow::anyhow!("empty rss feed"))?;
        PodcastInput::try_from(channel)
    }
}
