use std::borrow::Cow;

pub use sqlx;
pub mod model;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_url")]
    pub url: Cow<'static, str>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            url: Cow::Borrowed(":memory:"),
        }
    }
}

impl Config {
    pub fn default_url() -> Cow<'static, str> {
        Cow::Borrowed(":memory:")
    }

    pub async fn build(&self) -> std::io::Result<Database> {
        sqlx::SqlitePool::connect(&self.url)
            .await
            .map(Database)
            .map_err(std::io::Error::other)
    }
}

#[derive(Clone, Debug)]
pub struct Database(sqlx::SqlitePool);

impl Database {
    pub async fn migrate(&self) -> std::io::Result<()> {
        sqlx::migrate!()
            .run(&self.0)
            .await
            .map_err(std::io::Error::other)
    }

    pub async fn ping(&self) -> sqlx::Result<()> {
        sqlx::query("select 1").execute(&self.0).await?;
        Ok(())
    }
}

impl AsRef<sqlx::SqlitePool> for Database {
    fn as_ref(&self) -> &sqlx::SqlitePool {
        &self.0
    }
}

#[cfg(test)]
fn enable_tracing() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    let _ = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "entertainarr_database=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init();
}
