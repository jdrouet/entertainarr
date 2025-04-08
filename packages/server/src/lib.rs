use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use axum::Extension;

pub mod prelude;

mod handler;
mod model;
mod service;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
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
        Ok(Server {
            database: self.service.database.build().await?,
            storage: self.service.storage.build()?,
            socket_addr: SocketAddr::new(self.ip_addr, self.port),
        })
    }
}

pub struct Server {
    database: crate::service::database::Database,
    storage: crate::service::storage::Storage,
    socket_addr: SocketAddr,
}

impl Server {
    fn router(&self) -> axum::Router {
        handler::router()
            .layer(tower_http::trace::TraceLayer::new_for_http())
            .layer(Extension(self.database.clone()))
            .layer(Extension(self.storage.clone()))
    }

    pub async fn prepare(&self) -> std::io::Result<()> {
        tracing::debug!("preparing server");
        self.database.migrate().await?;
        self.storage.healthcheck().await?;
        Ok(())
    }

    pub async fn listen(self) -> std::io::Result<()> {
        tracing::debug!("starting server on {}", self.socket_addr);
        let listener = tokio::net::TcpListener::bind(self.socket_addr).await?;
        tracing::info!("server listening on {}", self.socket_addr);
        axum::serve(listener, self.router()).await
    }
}
