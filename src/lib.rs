/// Entertainarr main configuration
pub struct Config {
    // TODO
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {})
    }

    pub fn build(self) -> anyhow::Result<Application> {
        Ok(Application {})
    }
}

/// Entertainarr application
pub struct Application {
    // TODO
}

impl Application {
    pub async fn run(self) -> anyhow::Result<()> {
        // TODO
        Ok(())
    }
}
