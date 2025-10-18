use anyhow::Context;

mod extractor;
mod handler;
mod middleware;
mod prelude;

/// HTTP server configuration
#[derive(serde::Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_address")]
    address: std::net::IpAddr,
    #[serde(default = "Config::default_port")]
    port: u16,
}

const DEFAULT_ADDRESS: std::net::IpAddr = std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED);

impl Default for Config {
    fn default() -> Self {
        Self {
            address: Self::default_address(),
            port: Self::default_port(),
        }
    }
}

impl Config {
    pub const fn default_address() -> std::net::IpAddr {
        DEFAULT_ADDRESS
    }

    pub const fn default_port() -> u16 {
        3000
    }

    pub fn builder(self) -> anyhow::Result<HttpServerBuilder<(), (), ()>> {
        Ok(HttpServerBuilder {
            socket_address: std::net::SocketAddr::from((self.address, self.port)),
            authentication_service: (),
            podcast_service: (),
            podcast_episode_service: (),
        })
    }
}

pub struct HttpServerBuilder<AS, PS, PES> {
    socket_address: std::net::SocketAddr,
    authentication_service: AS,
    podcast_service: PS,
    podcast_episode_service: PES,
}

impl<AS, PS, PES> HttpServerBuilder<AS, PS, PES> {
    pub fn with_authentication_service<AS2>(self, service: AS2) -> HttpServerBuilder<AS2, PS, PES>
    where
        AS2: crate::domain::auth::prelude::AuthenticationService,
    {
        HttpServerBuilder {
            socket_address: self.socket_address,
            authentication_service: service,
            podcast_service: self.podcast_service,
            podcast_episode_service: self.podcast_episode_service,
        }
    }

    pub fn with_podcast_service<PS2>(self, service: PS2) -> HttpServerBuilder<AS, PS2, PES>
    where
        PS2: crate::domain::podcast::prelude::PodcastService,
    {
        HttpServerBuilder {
            socket_address: self.socket_address,
            authentication_service: self.authentication_service,
            podcast_service: service,
            podcast_episode_service: self.podcast_episode_service,
        }
    }

    pub fn with_podcast_episode_service<PES2>(
        self,
        service: PES2,
    ) -> HttpServerBuilder<AS, PS, PES2>
    where
        PES2: crate::domain::podcast::prelude::PodcastEpisodeService,
    {
        HttpServerBuilder {
            socket_address: self.socket_address,
            authentication_service: self.authentication_service,
            podcast_service: self.podcast_service,
            podcast_episode_service: service,
        }
    }
}

impl<AS, PS, PES> HttpServerBuilder<AS, PS, PES>
where
    AS: crate::domain::auth::prelude::AuthenticationService + Clone,
    PS: crate::domain::podcast::prelude::PodcastService + Clone,
    PES: crate::domain::podcast::prelude::PodcastEpisodeService + Clone,
{
    pub fn router(self) -> axum::Router {
        let state = ServerState {
            authentication_service: self.authentication_service,
            podcast_service: self.podcast_service,
            podcast_episode_service: self.podcast_episode_service,
        };
        handler::create::<ServerState<AS, PS, PES>>()
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
pub struct ServerState<AS, PS, PES> {
    authentication_service: AS,
    podcast_service: PS,
    podcast_episode_service: PES,
}

impl<AS, PS, PES> prelude::ServerState for ServerState<AS, PS, PES>
where
    AS: crate::domain::auth::prelude::AuthenticationService,
    PS: crate::domain::podcast::prelude::PodcastService,
    PES: crate::domain::podcast::prelude::PodcastEpisodeService,
{
    fn authentication_service(&self) -> &impl crate::domain::auth::prelude::AuthenticationService {
        &self.authentication_service
    }

    fn podcast_service(&self) -> &impl crate::domain::podcast::prelude::PodcastService {
        &self.podcast_service
    }

    fn podcast_episode_service(
        &self,
    ) -> &impl crate::domain::podcast::prelude::PodcastEpisodeService {
        &self.podcast_episode_service
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
