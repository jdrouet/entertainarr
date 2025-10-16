use std::{borrow::Cow, sync::Arc};

mod auth;

#[derive(serde::Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_secret")]
    secret: Cow<'static, str>,
    #[serde(default = "Config::default_duration")]
    duration: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            secret: Self::default_secret(),
            duration: Self::default_duration(),
        }
    }
}

impl Config {
    pub const fn default_secret() -> Cow<'static, str> {
        Cow::Borrowed("this-is-a-secret")
    }

    pub const fn default_duration() -> u64 {
        60 * 60 * 12
    }

    pub fn build(self) -> anyhow::Result<JsonWebToken> {
        let algorithm = jsonwebtoken::Algorithm::HS512;
        let decoding = jsonwebtoken::DecodingKey::from_secret(self.secret.as_ref().as_bytes());
        let encoding = jsonwebtoken::EncodingKey::from_secret(self.secret.as_ref().as_bytes());
        let header = jsonwebtoken::Header::new(algorithm);
        let validation = jsonwebtoken::Validation::new(algorithm);
        Ok(JsonWebToken(Arc::new(Inner {
            decoding,
            duration: std::time::Duration::from_secs(self.duration),
            encoding,
            header,
            validation,
        })))
    }
}

struct Inner {
    decoding: jsonwebtoken::DecodingKey,
    duration: std::time::Duration,
    encoding: jsonwebtoken::EncodingKey,
    header: jsonwebtoken::Header,
    validation: jsonwebtoken::Validation,
}

#[derive(Clone)]
pub struct JsonWebToken(Arc<Inner>);
