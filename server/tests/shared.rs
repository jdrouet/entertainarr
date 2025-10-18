use std::{borrow::Cow, time::Duration};

use reqwest::StatusCode;

#[allow(unused, reason = "only for testing")]
pub struct Client {
    pub client: reqwest::Client,
    handler: tokio::task::JoinHandle<anyhow::Result<()>>,
    tmpdir: tempfile::TempDir,
}

impl Client {
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

        Self {
            client: reqwest::Client::new(),
            handler,
            tmpdir,
        }
    }

    pub async fn auth_signup(&self, email: &str, password: &str) -> String {
        let res = self
            .client
            .post("http://localhost:3000/api/auth/signup")
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "email": email,
                "password": password,
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let payload: serde_json::Value = res.json().await.unwrap();
        let token = payload
            .as_object()
            .unwrap()
            .get("token")
            .unwrap()
            .as_str()
            .unwrap();
        token.to_string()
    }
}
