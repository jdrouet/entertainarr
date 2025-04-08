use std::{ffi::OsStr, str::FromStr};

pub fn from_env<K, V>(name: K) -> std::io::Result<V>
where
    K: AsRef<OsStr>,
    V: FromStr,
    <V as FromStr>::Err: std::fmt::Debug,
{
    let Ok(found) = std::env::var(name.as_ref()) else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("unable to find environment variable {:?}", name.as_ref()),
        ));
    };
    V::from_str(found.as_str()).map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("unable to convert {found:?} to the requested type: {err:?}"),
        )
    })
}

pub fn from_env_or<K, V>(name: K, fallback: V) -> std::io::Result<V>
where
    K: AsRef<OsStr>,
    V: FromStr,
    <V as FromStr>::Err: std::fmt::Debug,
{
    let Ok(found) = std::env::var(name.as_ref()) else {
        return Ok(fallback);
    };
    V::from_str(found.as_str()).map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("unable to convert {found:?} to the requested type: {err:?}"),
        )
    })
}

pub fn from_env_or_else<K, F, V>(name: K, callback: F) -> std::io::Result<V>
where
    K: AsRef<OsStr>,
    F: FnOnce() -> V,
    V: FromStr,
    <V as FromStr>::Err: std::fmt::Debug,
{
    let Ok(found) = std::env::var(name.as_ref()) else {
        return Ok(callback());
    };
    V::from_str(found.as_str()).map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("unable to convert {found:?} to the requested type: {err:?}"),
        )
    })
}
