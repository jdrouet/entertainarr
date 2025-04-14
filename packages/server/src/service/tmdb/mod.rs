use std::sync::Arc;

use tmdb_api::client::reqwest::ReqwestExecutor;

#[derive(Debug, Default, serde::Deserialize)]
pub struct Config {
    token: Option<String>,
}

impl Config {
    pub(crate) fn build(&self) -> std::io::Result<Tmdb> {
        let token = self
            .token
            .clone()
            .ok_or(std::io::Error::other("missing tmdb token"))?;
        let inner = tmdb_api::client::ClientBuilder::<ReqwestExecutor>::default()
            .with_api_key(token)
            .build()
            .map_err(std::io::Error::other)?;
        Ok(Tmdb(Arc::new(inner)))
    }
}

#[derive(Clone)]
pub struct Tmdb(Arc<tmdb_api::client::Client<ReqwestExecutor>>);

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
