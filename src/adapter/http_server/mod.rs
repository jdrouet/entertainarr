use anyhow::Context;

mod handler;
mod middleware;

/// HTTP server configuration
pub struct Config {
    address: std::net::IpAddr,
    port: u16,
}

const DEFAULT_ADDRESS: std::net::IpAddr = std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED);

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            address: super::with_env_as_or("HTTP_ADDRESS", DEFAULT_ADDRESS)?,
            port: super::with_env_as_or("HTTP_PORT", 3000)?,
        })
    }

    pub fn builder(self) -> anyhow::Result<HttpServerBuilder<()>> {
        Ok(HttpServerBuilder {
            socket_address: std::net::SocketAddr::from((self.address, self.port)),
            authentication_service: (),
        })
    }
}

pub struct HttpServerBuilder<AS> {
    socket_address: std::net::SocketAddr,
    authentication_service: AS,
}

impl<AS> HttpServerBuilder<AS> {
    pub fn with_authentication_service<AS2>(self, service: AS2) -> HttpServerBuilder<AS2>
    where
        AS2: crate::domain::auth::prelude::AuthenticationService,
    {
        HttpServerBuilder {
            socket_address: self.socket_address,
            authentication_service: service,
        }
    }
}

impl<AS> HttpServerBuilder<AS>
where
    AS: crate::domain::auth::prelude::AuthenticationService + Clone,
{
    pub fn router(self) -> axum::Router {
        let state = ServerState {
            authentication_service: self.authentication_service,
        };
        handler::create::<AS>()
            .layer(middleware::tracing::layer())
            .with_state(state)
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

#[derive(Clone, Debug)]
pub struct ServerState<AS> {
    authentication_service: AS,
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
