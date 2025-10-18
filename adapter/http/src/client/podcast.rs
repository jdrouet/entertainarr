use anyhow::Context;

use crate::entity::podcast::SubscriptionRequest;

impl super::Client {
    pub async fn podcast_subscribe(&self, feed_url: &str) -> anyhow::Result<()> {
        let Some(token) = self.token.as_deref() else {
            anyhow::bail!("unauthorized")
        };
        let url = format!("{}/api/users/me/podcasts", self.base_url);
        let res = self
            .inner
            .post(&url)
            .header("Authorization", format!("Bearer {token}"))
            .json(&SubscriptionRequest {
                feed_url: feed_url.into(),
            })
            .send()
            .await
            .context("unable to send request")?;
        res.error_for_status_ref()?;
        Ok(())
    }
}
