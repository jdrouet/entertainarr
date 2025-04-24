use std::{borrow::Cow, sync::Arc};

use tmdb_api::client::reqwest::ReqwestExecutor;

#[derive(Debug, Default, serde::Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_base_url")]
    pub base_url: Cow<'static, str>,
    pub token: Option<String>,
}

impl Config {
    pub fn default_base_url() -> Cow<'static, str> {
        Cow::Borrowed("https://api.themoviedb.org/3")
    }
}

impl Config {
    pub(crate) fn build(&self) -> std::io::Result<Tmdb> {
        let token = self
            .token
            .clone()
            .ok_or(std::io::Error::other("missing tmdb token"))?;
        let inner = tmdb_api::client::ClientBuilder::<ReqwestExecutor>::default()
            .with_base_url(self.base_url.clone())
            .with_api_key(token)
            .build()
            .map_err(std::io::Error::other)?;
        Ok(Tmdb(Arc::new(inner)))
    }
}

#[derive(Clone)]
pub struct Tmdb(Arc<tmdb_api::client::Client<ReqwestExecutor>>);

#[cfg(test)]
impl Tmdb {
    pub fn test(base_url: String) -> Self {
        let inner = tmdb_api::client::ClientBuilder::<ReqwestExecutor>::default()
            .with_api_key(String::from("token"))
            .with_base_url(base_url)
            .build()
            .map_err(std::io::Error::other)
            .unwrap();
        Tmdb(Arc::new(inner))
    }
}

impl std::fmt::Debug for Tmdb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Tmdb)).finish()
    }
}

impl AsRef<tmdb_api::client::Client<ReqwestExecutor>> for Tmdb {
    fn as_ref(&self) -> &tmdb_api::client::Client<ReqwestExecutor> {
        &self.0
    }
}
