use anyhow::Context;

mod handler;
mod middleware;

/// HTTP server configuration
pub struct Config {
    address: std::net::IpAddr,
    port: u16,
}

const DEFAULT_ADDRESS: std::net::IpAddr = std::net::IpAddr::V4(std::net::Ipv4Addr::BROADCAST);

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            address: super::with_env_as_or("HTTP_ADDRESS", DEFAULT_ADDRESS)?,
            port: super::with_env_as_or("HTTP_PORT", 3000)?,
        })
    }

    pub fn builder(self) -> anyhow::Result<HttpServerBuilder> {
        Ok(HttpServerBuilder {
            socket_address: std::net::SocketAddr::from((self.address, self.port)),
        })
    }
}

pub struct HttpServerBuilder {
    socket_address: std::net::SocketAddr,
}

impl HttpServerBuilder {
    pub fn router(self) -> axum::Router {
        handler::create().layer(middleware::tracing::layer())
    }

    pub fn build(self) -> anyhow::Result<HttpServer> {
        let socket_address = self.socket_address;
        let router = self.router();

        Ok(HttpServer {
            router,
            socket_address,
        })
    }
}

pub struct HttpServer {
    router: axum::Router,
    socket_address: std::net::SocketAddr,
}

impl HttpServer {
    pub async fn run(self) -> anyhow::Result<()> {
        let listener = tokio::net::TcpListener::bind(self.socket_address)
            .await
            .context("unable to bind socket")?;
        axum::serve(listener, self.router)
            .await
            .context("server shutdown")
    }
}
