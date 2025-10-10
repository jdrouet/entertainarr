use std::borrow::Cow;

pub mod http_server;
pub mod jsonwebtoken;
pub mod rss;
pub mod sqlite;

#[inline]
pub(crate) fn with_env_or(name: &str, value: &'static str) -> Cow<'static, str> {
    std::env::var(name)
        .ok()
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed(value))
}

#[inline]
pub(crate) fn with_env_as_or<T>(name: &str, value: T) -> anyhow::Result<T>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    let Ok(value) = std::env::var(name) else {
        return Ok(value);
    };
    value.parse::<T>().map_err(anyhow::Error::from)
}
