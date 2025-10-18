use std::sync::Arc;

mod auth;
mod podcast;
mod podcast_episode;

#[derive(Clone, Debug)]
pub struct Client {
    base_url: Arc<str>,
    inner: reqwest::Client,
    token: Option<String>,
}

impl Client {
    pub fn new(base_url: impl AsRef<str>) -> Self {
        Self {
            base_url: Arc::from(base_url.as_ref()),
            inner: reqwest::Client::new(),
            token: None,
        }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }
}
