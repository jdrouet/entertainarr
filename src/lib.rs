use crate::domain::auth::AuthenticationService;

mod adapter;
pub(crate) mod domain;

/// Entertainarr main configuration
pub struct Config {
    http_server: adapter::http_server::Config,
    jsonwebtoken: adapter::jsonwebtoken::Config,
    sqlite: adapter::sqlite::Config,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            http_server: adapter::http_server::Config::from_env()?,
            jsonwebtoken: adapter::jsonwebtoken::Config::from_env()?,
            sqlite: adapter::sqlite::Config::from_env()?,
        })
    }

    pub async fn build(self) -> anyhow::Result<Application> {
        let http_server = self.http_server.builder()?;
        let jsonwebtoken = self.jsonwebtoken.build()?;
        let sqlite_pool = self.sqlite.build().await?;
        let authentication_servie = AuthenticationService::builder()
            .authentication_repository(sqlite_pool)
            .token_repository(jsonwebtoken)
            .build();
        let http_server = http_server
            .with_authentication_service(authentication_servie)
            .build()?;
        Ok(Application { http_server })
    }
}

/// Entertainarr application
pub struct Application {
    http_server: adapter::http_server::HttpServer,
}

impl Application {
    pub async fn run(self) -> anyhow::Result<()> {
        self.http_server.run().await
    }
}
