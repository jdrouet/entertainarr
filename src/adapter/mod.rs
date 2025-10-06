pub mod http_server;

#[inline]
fn with_env_as_or<T>(name: &str, value: T) -> anyhow::Result<T>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    let Ok(value) = std::env::var(name) else {
        return Ok(value);
    };
    value.parse::<T>().map_err(anyhow::Error::from)
}
