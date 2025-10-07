use std::borrow::Cow;

use anyhow::Context;

mod auth;

pub struct Config {
    url: Cow<'static, str>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            url: super::with_env_or("DATABASE_URL", ":memory:"),
        })
    }

    pub async fn build(self) -> anyhow::Result<Pool> {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .min_connections(1)
            .connect(self.url.as_ref())
            .await?;

        sqlx::migrate!()
            .run(&pool)
            .await
            .context("unable to run migrations")?;

        Ok(Pool(pool))
    }
}

#[derive(Debug, Clone)]
pub struct Pool(sqlx::SqlitePool);

struct Wrapper<T>(T);

impl<T> Wrapper<T> {
    fn maybe_inner(this: Option<Self>) -> Option<T> {
        this.map(Wrapper::inner)
    }

    fn inner(self) -> T {
        self.0
    }
}
