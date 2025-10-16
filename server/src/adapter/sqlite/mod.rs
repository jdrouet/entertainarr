use std::borrow::Cow;

use anyhow::Context;

mod auth;
mod podcast;

#[derive(serde::Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_url")]
    url: Cow<'static, str>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            url: Self::default_url(),
        }
    }
}

impl Config {
    pub const fn default_url() -> Cow<'static, str> {
        Cow::Borrowed(":memory:")
    }

    pub async fn build(self) -> anyhow::Result<Pool> {
        let options = sqlx::sqlite::SqliteConnectOptions::new();
        let options = match self.url.as_ref() {
            ":memory:" => options.in_memory(true).create_if_missing(true),
            path => options.filename(path),
        };
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .min_connections(1)
            .connect_with(options)
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

#[cfg(test)]
impl Pool {
    pub async fn test() -> Self {
        Config {
            url: Cow::Borrowed(":memory:"),
        }
        .build()
        .await
        .unwrap()
    }
}

struct Wrapper<T>(T);

impl<T> Wrapper<T> {
    fn maybe_inner(this: Option<Self>) -> Option<T> {
        this.map(Wrapper::inner)
    }

    fn inner(self) -> T {
        self.0
    }

    fn list(values: Vec<Wrapper<T>>) -> Vec<T> {
        values.into_iter().map(Wrapper::inner).collect()
    }
}

fn record_one<T>(_: &T) {
    let span = tracing::Span::current();
    span.record("db.response.returned_rows", 1);
}

fn record_optional<T>(item: &Option<T>) {
    let span = tracing::Span::current();
    span.record(
        "db.response.returned_rows",
        if item.is_some() { 1 } else { 0 },
    );
}

#[allow(clippy::ptr_arg, reason = "needed by sqlx")]
fn record_all<T>(list: &Vec<T>) {
    let span = tracing::Span::current();
    span.record("db.response.returned_rows", list.len());
}

fn record_error(err: &sqlx::Error) {
    let span = tracing::Span::current();
    span.record(
        "error.type",
        if err.as_database_error().is_some() {
            "client"
        } else {
            "server"
        },
    );
    span.record("error.message", err.to_string());
    span.record("error.stacktrace", format!("{err:?}"));
}
