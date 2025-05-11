use std::{path::PathBuf, sync::Arc};

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest_middleware::ClientWithMiddleware;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_path")]
    path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: Self::default_path(),
        }
    }
}

impl Config {
    fn default_path() -> PathBuf {
        PathBuf::from(".cache")
    }

    fn manager(&self) -> CACacheManager {
        CACacheManager {
            path: self.path.clone(),
        }
    }

    pub fn build(&self) -> std::io::Result<Fetcher> {
        let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
            .with(Cache(HttpCache {
                mode: CacheMode::ForceCache,
                manager: self.manager(),
                options: HttpCacheOptions::default(),
            }))
            .build();
        Ok(Fetcher(Arc::new(client)))
    }
}

#[derive(Clone, Debug)]
pub struct Fetcher(Arc<ClientWithMiddleware>);

impl Fetcher {
    pub async fn fetch_into_body(
        &self,
        url: &str,
    ) -> reqwest_middleware::Result<(axum::http::HeaderMap, axum::body::Body)> {
        let res = self.0.get(url).send().await?;
        res.error_for_status_ref()?;
        let headers = res.headers().clone();
        let stream = res.bytes_stream();
        Ok((headers, axum::body::Body::from_stream(stream)))
    }
}
