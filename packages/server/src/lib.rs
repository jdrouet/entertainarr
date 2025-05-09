use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use axum::Extension;

pub mod prelude;

mod handler;
mod service;
mod view;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default, alias = "data")]
    pub dataset: entertainarr_database::model::Dataset,
    #[serde(default)]
    service: service::Config,
    #[serde(default = "Config::default_ip_addr")]
    ip_addr: IpAddr,
    #[serde(default = "Config::default_port")]
    port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dataset: entertainarr_database::model::Dataset::default(),
            service: service::Config::default(),
            ip_addr: Self::default_ip_addr(),
            port: Self::default_port(),
        }
    }
}

impl Config {
    pub fn parse(path: Option<PathBuf>) -> std::io::Result<Self> {
        let builder = config::Config::builder();
        let builder = match path {
            Some(p) => builder.add_source(config::File::from(p)),
            None => builder,
        };
        let cfg = builder
            .add_source(config::Environment::default())
            .build()
            .map_err(std::io::Error::other)?;
        cfg.try_deserialize().map_err(std::io::Error::other)
    }
}

impl Config {
    const fn default_ip_addr() -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
    }

    const fn default_port() -> u16 {
        3000
    }
}

impl Config {
    pub async fn build(&self) -> std::io::Result<Server> {
        let database = self.service.database.build().await?;
        let fetcher = self.service.fetcher.build()?;
        let tmdb = self.service.tmdb.build()?;
        let worker = self.service.worker.build(database.clone(), tmdb.clone())?;
        Ok(Server {
            database,
            fetcher,
            tmdb,
            worker,
            socket_addr: SocketAddr::new(self.ip_addr, self.port),
        })
    }
}

pub struct Server {
    database: entertainarr_database::Database,
    fetcher: crate::service::fetcher::Fetcher,
    tmdb: crate::service::tmdb::Tmdb,
    worker: crate::service::worker::Worker,
    socket_addr: SocketAddr,
}

impl Server {
    fn router(&self) -> axum::Router {
        handler::router()
            .layer(tower_http::trace::TraceLayer::new_for_http())
            .layer(Extension(self.database.clone()))
            .layer(Extension(self.fetcher.clone()))
            .layer(Extension(self.tmdb.clone()))
            .layer(Extension(self.worker.clone()))
    }

    pub async fn prepare(&self) -> std::io::Result<()> {
        tracing::debug!("preparing server");
        self.database.migrate().await?;
        Ok(())
    }

    pub async fn preload(
        &self,
        dataset: &entertainarr_database::model::Dataset,
    ) -> std::io::Result<()> {
        dataset
            .preload(self.database.as_ref())
            .await
            .map_err(std::io::Error::other)
    }

    pub async fn listen(self) -> std::io::Result<()> {
        tracing::debug!("starting server on {}", self.socket_addr);
        let listener = tokio::net::TcpListener::bind(self.socket_addr).await?;
        tracing::info!("server listening on {}", self.socket_addr);
        axum::serve(listener, self.router()).await
    }
}
