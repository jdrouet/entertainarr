use anyhow::Context;

use crate::domain::{auth::AuthenticationService, podcast::PodcastService};

mod adapter;
pub(crate) mod domain;
pub mod tracing;

/// Entertainarr main configuration
#[derive(serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    http_server: adapter::http_server::Config,
    #[serde(default)]
    jsonwebtoken: adapter::jsonwebtoken::Config,
    #[serde(default)]
    rss: adapter::rss::Config,
    #[serde(default)]
    sqlite: adapter::sqlite::Config,
}

impl Config {
    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self> {
        let data = std::fs::read(path).context("unable to open configuration file")?;
        toml::from_slice(data.as_ref()).context("unable to deserialize config")
    }

    pub async fn build(self) -> anyhow::Result<Application> {
        let http_server = self.http_server.builder()?;
        let jsonwebtoken = self.jsonwebtoken.build()?;
        let rss_client = self.rss.build()?;
        let sqlite_pool = self.sqlite.build().await?;
        let authentication_service = AuthenticationService::builder()
            .authentication_repository(sqlite_pool.clone())
            .token_repository(jsonwebtoken)
            .build();
        let podcast_service = PodcastService::builder()
            .rss_feed_loader(rss_client)
            .podcast_repository(sqlite_pool.clone())
            .podcast_subscription_repository(sqlite_pool)
            .build();
        let http_server = http_server
            .with_authentication_service(authentication_service)
            .with_podcast_service(podcast_service)
            .build()?;
        Ok(Application { http_server })
    }
}

/// Entertainarr application
pub struct Application {
    http_server: adapter::http_server::HttpServer,
}

impl Application {
    pub async fn run(self) -> anyhow::Result<()> {
        self.http_server.run().await
    }
}
