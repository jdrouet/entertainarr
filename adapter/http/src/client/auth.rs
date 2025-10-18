use anyhow::Context;

use crate::entity::auth::{AuthenticationRequest, AuthenticationResponse};

impl super::Client {
    pub async fn auth(&self, kind: &str, email: &str, password: &str) -> anyhow::Result<String> {
        let url = format!("{}/api/auth/{kind}", self.base_url);
        let res = self
            .inner
            .post(&url)
            .json(&AuthenticationRequest {
                email: email.into(),
                password: password.into(),
            })
            .send()
            .await
            .context("unable to send request")?;
        res.error_for_status_ref()?;
        let res: AuthenticationResponse = res.json().await.context("unable to read response")?;
        Ok(res.token)
    }

    pub async fn auth_login(&self, email: &str, password: &str) -> anyhow::Result<String> {
        self.auth("login", email, password).await
    }

    pub async fn auth_signup(&self, email: &str, password: &str) -> anyhow::Result<String> {
        self.auth("signup", email, password).await
    }
}
