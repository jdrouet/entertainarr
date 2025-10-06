use anyhow::Context;

/// HTTP server configuration
pub struct Config {
    address: std::net::IpAddr,
    port: u16,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            address: super::with_env_as_or(
                "HTTP_ADDRESS",
                std::net::IpAddr::V4(std::net::Ipv4Addr::BROADCAST),
            )?,
            port: super::with_env_as_or("HTTP_PORT", 3000)?,
        })
    }

    pub fn build(self) -> anyhow::Result<HttpServer> {
        Ok(HttpServer {
            socket_address: std::net::SocketAddr::from((self.address, self.port)),
        })
    }
}

pub struct HttpServer {
    socket_address: std::net::SocketAddr,
}

impl HttpServer {
    pub async fn run(self) -> anyhow::Result<()> {
        let listener = tokio::net::TcpListener::bind(self.socket_address)
            .await
            .context("unable to bind socket")?;
        let app = axum::Router::new();
        axum::serve(listener, app).await.context("server shutdown")
    }
}
