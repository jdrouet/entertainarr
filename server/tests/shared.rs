use std::{borrow::Cow, time::Duration};

#[allow(unused, reason = "only for testing")]
pub struct Server {
    handler: tokio::task::JoinHandle<anyhow::Result<()>>,
    tmpdir: tempfile::TempDir,
}

impl Server {
    pub async fn new() -> Self {
        let tmpdir = tempfile::tempdir().unwrap();
        let config = entertainarr::Config {
            http_server: entertainarr::HttpServerConfig {
                address: std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
                port: 3000,
            },
            jsonwebtoken: Default::default(),
            rss: Default::default(),
            sqlite: entertainarr::SqliteConfig {
                url: Cow::Owned(
                    tmpdir
                        .path()
                        .join("database.db")
                        .to_string_lossy()
                        .to_string(),
                ),
            },
        };
        let app = config.build().await.unwrap();
        let handler = tokio::spawn(app.run());

        tokio::time::sleep(Duration::from_millis(500)).await;

        Self { handler, tmpdir }
    }

    pub fn client(&self) -> entertainarr_adapter_http::client::Client {
        entertainarr_adapter_http::client::Client::new("http://localhost:3000")
    }
}
