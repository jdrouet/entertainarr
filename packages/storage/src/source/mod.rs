pub mod local;
pub mod prelude;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Config {
    Local(local::Config),
}

impl Config {
    pub fn build(&self) -> std::io::Result<AnySource> {
        match self {
            Self::Local(inner) => Ok(AnySource::Local(inner.build()?)),
        }
    }
}

#[derive(Debug)]
pub enum AnySource {
    Local(local::Source),
}

impl crate::source::prelude::Source for AnySource {
    async fn healthcheck(&self) -> std::io::Result<()> {
        match self {
            Self::Local(inner) => inner.healthcheck().await,
        }
    }

    async fn list(&self, path: &str) -> std::io::Result<Vec<crate::entry::EntryInfo>> {
        match self {
            Self::Local(inner) => inner.list(path).await,
        }
    }
}
