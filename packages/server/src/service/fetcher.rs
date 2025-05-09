use std::sync::Arc;

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest_middleware::ClientWithMiddleware;

#[derive(Debug, Default, serde::Deserialize)]
pub struct Config {}

impl Config {
    pub fn build(&self) -> std::io::Result<Fetcher> {
        let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
            .with(Cache(HttpCache {
                mode: CacheMode::ForceCache,
                manager: CACacheManager::default(),
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
