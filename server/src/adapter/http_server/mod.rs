use anyhow::Context;

mod extractor;
mod handler;
mod middleware;
mod prelude;

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

    pub fn builder(self) -> anyhow::Result<HttpServerBuilder<(), ()>> {
        Ok(HttpServerBuilder {
            socket_address: std::net::SocketAddr::from((self.address, self.port)),
            authentication_service: (),
            podcast_service: (),
        })
    }
}

pub struct HttpServerBuilder<AS, PS> {
    socket_address: std::net::SocketAddr,
    authentication_service: AS,
    podcast_service: PS,
}

impl<AS, PS> HttpServerBuilder<AS, PS> {
    pub fn with_authentication_service<AS2>(self, service: AS2) -> HttpServerBuilder<AS2, PS>
    where
        AS2: crate::domain::auth::prelude::AuthenticationService,
    {
        HttpServerBuilder {
            socket_address: self.socket_address,
            authentication_service: service,
            podcast_service: self.podcast_service,
        }
    }

    pub fn with_podcast_service<PS2>(self, service: PS2) -> HttpServerBuilder<AS, PS2>
    where
        PS2: crate::domain::podcast::prelude::PodcastService,
    {
        HttpServerBuilder {
            socket_address: self.socket_address,
            authentication_service: self.authentication_service,
            podcast_service: service,
        }
    }
}

impl<AS, PS> HttpServerBuilder<AS, PS>
where
    AS: crate::domain::auth::prelude::AuthenticationService + Clone,
    PS: crate::domain::podcast::prelude::PodcastService + Clone,
{
    pub fn router(self) -> axum::Router {
        let state = ServerState {
            authentication_service: self.authentication_service,
            podcast_service: self.podcast_service,
        };
        handler::create::<ServerState<AS, PS>>()
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
pub struct ServerState<AS, PS> {
    authentication_service: AS,
    podcast_service: PS,
}

impl<AS, PS> prelude::ServerState for ServerState<AS, PS>
where
    AS: crate::domain::auth::prelude::AuthenticationService,
    PS: crate::domain::podcast::prelude::PodcastService,
{
    fn authentication_service(&self) -> &impl crate::domain::auth::prelude::AuthenticationService {
        &self.authentication_service
    }

    fn podcast_service(&self) -> &impl crate::domain::podcast::prelude::PodcastService {
        &self.podcast_service
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
        tracing::info!(address = ?self.socket_address, "starting server");
        axum::serve(listener, self.router)
            .await
            .context("server shutdown")
    }
}
