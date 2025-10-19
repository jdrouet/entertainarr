use anyhow::Context;

use crate::entity::{
    ApiResource,
    podcast_episode::{PodcastEpisodeDocument, PodcastEpisodeRelation},
};

impl super::Client {
    pub async fn podcast_episode_list(
        &self,
    ) -> anyhow::Result<ApiResource<Vec<PodcastEpisodeDocument>, PodcastEpisodeRelation>> {
        let Some(token) = self.token.as_deref() else {
            anyhow::bail!("unauthorized")
        };
        let url = format!("{}/api/podcast-episodes", self.base_url);
        let res = self
            .inner
            .get(&url)
            .query(&[("include", "podcast")])
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await
            .context("unable to send request")?;
        if let Err(err) = res.error_for_status_ref() {
            let body = res.text().await.context("unable to read payload body")?;
            Err(anyhow::Error::from(err).context(body))
        } else {
            res.json().await.context("unable to deserialize payload")
        }
    }
}
