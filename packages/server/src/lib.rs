use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::Extension;

mod handler;
mod service;

#[derive(Debug)]
pub struct Config {
    service: service::Config,
    ip_addr: IpAddr,
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
            socket_addr: SocketAddr::new(self.ip_addr, self.port),
        })
    }
}

pub struct Server {
    database: crate::service::database::Database,
    socket_addr: SocketAddr,
}

impl Server {
    fn router(&self) -> axum::Router {
        handler::router()
            .layer(tower_http::trace::TraceLayer::new_for_http())
            .layer(Extension(self.database.clone()))
    }

    pub async fn listen(self) -> std::io::Result<()> {
        tracing::debug!("preparing server");
        self.database.migrate().await?;
        tracing::debug!("starting server on {}", self.socket_addr);
        let listener = tokio::net::TcpListener::bind(self.socket_addr).await?;
        tracing::info!("server listening on {}", self.socket_addr);
        axum::serve(listener, self.router()).await
    }
}
