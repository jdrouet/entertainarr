use anyhow::Context;
use entertainarr_domain::{
    auth::AuthenticationService,
    podcast::{PodcastEpisodeService, PodcastService},
};

mod client;
pub mod tracing;

/// Entertainarr main configuration
#[derive(serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    pub http_server: entertainarr_adapter_http::server::Config,
    #[serde(default)]
    pub jsonwebtoken: entertainarr_adapter_jsonwebtoken::Config,
    #[serde(default)]
    pub rss: entertainarr_adapter_rss::Config,
    #[serde(default)]
    pub sqlite: entertainarr_adapter_sqlite::Config,
}

impl Config {
    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self> {
        let data = std::fs::read(path.as_ref())
            .with_context(|| format!("unable to open configuration file on {:?}", path.as_ref()))?;
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
            .podcast_subscription_repository(sqlite_pool.clone())
            .build();
        let podcast_episode_service = PodcastEpisodeService::builder()
            .podcast_episode_repository(sqlite_pool)
            .build();
        let http_server = http_server
            .with_authentication_service(authentication_service)
            .with_client_service(crate::client::ClientService)
            .with_podcast_service(podcast_service)
            .with_podcast_episode_service(podcast_episode_service)
            .build()?;
        Ok(Application { http_server })
    }
}

/// Entertainarr application
pub struct Application {
    http_server: entertainarr_adapter_http::server::HttpServer,
}

impl Application {
    pub async fn run(self) -> anyhow::Result<()> {
        self.http_server.run().await
    }
}
