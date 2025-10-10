use std::{borrow::Cow, sync::Arc};

mod auth;

pub struct Config {
    secret: Cow<'static, str>,
    duration: u64,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            secret: super::with_env_or("JWT_SECRET", "this-is-a-secret"),
            duration: super::with_env_as_or("JWT_DURATION", 60 * 60 * 12)?,
        })
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
