use std::borrow::Cow;

use sqlx::SqlitePool;

#[derive(Debug)]
pub struct Config {
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
