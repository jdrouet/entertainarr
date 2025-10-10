use anyhow::Context;
use jsonwebtoken::TokenData;

use crate::domain::auth::entity::Profile;
use crate::domain::auth::prelude::VerifyError;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    // aud: String, // Optional. Audience
    exp: u64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: u64, // Optional. Issued at (as UTC timestamp)
    // iss: String, // Optional. Issuer
    // nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: u64, // Optional. Subject (whom token refers to)
}

impl crate::domain::auth::prelude::TokenRepository for super::JsonWebToken {
    #[tracing::instrument(skip_all, fields(user_id = profile.id), err(Debug))]
    async fn create_token(
        &self,
        profile: &crate::domain::auth::entity::Profile,
    ) -> anyhow::Result<String> {
        let now = chrono::Utc::now();
        let exp = now + self.0.duration;
        let claims = Claims {
            exp: exp.timestamp() as u64,
            iat: now.timestamp() as u64,
            sub: profile.id,
        };
        jsonwebtoken::encode(&self.0.header, &claims, &self.0.encoding)
            .context("unable to create token")
    }

    #[tracing::instrument(skip_all, err(Debug))]
    async fn decode_token(&self, token: &str) -> Result<Profile, VerifyError> {
        jsonwebtoken::decode(token, &self.0.decoding, &self.0.validation)
            .map_err(|err| match err.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => VerifyError::ExpiredToken,
                jsonwebtoken::errors::ErrorKind::InvalidToken => VerifyError::InvalidToken,
                _ => VerifyError::Internal(err.into()),
            })
            .map(|res: TokenData<Claims>| Profile { id: res.claims.sub })
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::domain::auth::{
        entity::Profile,
        prelude::{TokenRepository, VerifyError},
    };

    #[tokio::test]
    async fn should_encode_token() {
        let config = crate::adapter::jsonwebtoken::Config {
            secret: Cow::Borrowed("secret"),
            duration: 10,
        };
        let client = config.build().unwrap();
        let _token = client.create_token(&Profile { id: 1 }).await.unwrap();
    }

    #[tokio::test]
    async fn should_decode_token() {
        let config = crate::adapter::jsonwebtoken::Config {
            secret: Cow::Borrowed("secret"),
            duration: 10,
        };
        let client = config.build().unwrap();
        let token = client.create_token(&Profile { id: 1 }).await.unwrap();
        let profile = client.decode_token(&token).await.unwrap();
        assert_eq!(profile.id, 1);
    }

    #[tokio::test]
    async fn should_fail_decoding_if_invalid() {
        let config = crate::adapter::jsonwebtoken::Config {
            secret: Cow::Borrowed("secret"),
            duration: 10,
        };
        let client = config.build().unwrap();
        let err = client.decode_token("foo").await.unwrap_err();
        assert!(matches!(err, VerifyError::InvalidToken));
    }

    #[tokio::test]
    async fn should_fail_decoding_if_expired() {
        let config = crate::adapter::jsonwebtoken::Config {
            secret: Cow::Borrowed("secret"),
            duration: 10,
        };
        let client = config.build().unwrap();

        let now = chrono::Utc::now();
        let claims = super::Claims {
            exp: (now.timestamp() - 1000) as u64,
            iat: (now.timestamp() - 2000) as u64,
            sub: 1,
        };
        let token = jsonwebtoken::encode(&client.0.header, &claims, &client.0.encoding).unwrap();

        let err = client.decode_token(&token).await.unwrap_err();
        assert!(matches!(err, VerifyError::ExpiredToken));
    }
}
