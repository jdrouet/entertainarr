use std::borrow::Cow;

use sqlx::SqlitePool;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_url")]
    url: Cow<'static, str>,
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

    pub(crate) async fn build(&self) -> std::io::Result<Database> {
        SqlitePool::connect(&self.url)
            .await
            .map(Database)
            .map_err(std::io::Error::other)
    }
}

#[derive(Clone, Debug)]
pub struct Database(SqlitePool);

impl Database {
    pub async fn migrate(&self) -> std::io::Result<()> {
        sqlx::migrate!("./migrations")
            .run(&self.0)
            .await
            .map_err(std::io::Error::other)
    }

    pub async fn ping(&self) -> sqlx::Result<()> {
        sqlx::query("select 1").execute(&self.0).await?;
        Ok(())
    }
}

impl AsRef<SqlitePool> for Database {
    fn as_ref(&self) -> &SqlitePool {
        &self.0
    }
}
