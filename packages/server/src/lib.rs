use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod handler;

#[derive(Debug)]
pub struct Config {
    ip_addr: IpAddr,
    port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
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
    pub fn build(&self) -> Server {
        Server {
            socket_addr: SocketAddr::new(self.ip_addr, self.port),
        }
    }
}

pub struct Server {
    socket_addr: SocketAddr,
}

impl Server {
    fn router(&self) -> axum::Router {
        handler::router().layer(tower_http::trace::TraceLayer::new_for_http())
    }

    pub async fn listen(self) -> std::io::Result<()> {
        tracing::debug!("starting server on {}", self.socket_addr);
        let listener = tokio::net::TcpListener::bind(self.socket_addr).await?;
        tracing::info!("server listening on {}", self.socket_addr);
        axum::serve(listener, self.router()).await
    }
}
