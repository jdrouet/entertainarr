mod adapter;

/// Entertainarr main configuration
pub struct Config {
    http_server: adapter::http_server::Config,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            http_server: adapter::http_server::Config::from_env()?,
        })
    }

    pub fn build(self) -> anyhow::Result<Application> {
        Ok(Application {
            http_server: self.http_server.build()?,
        })
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
