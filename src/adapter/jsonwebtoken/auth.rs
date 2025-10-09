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
            .map(|res: TokenData<Claims>| Profile { id: res.claims.sub })
            .map_err(|err| match err.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => VerifyError::ExpiredToken,
                jsonwebtoken::errors::ErrorKind::InvalidToken => VerifyError::InvalidToken,
                _ => VerifyError::Internal(err.into()),
            })
    }
}
