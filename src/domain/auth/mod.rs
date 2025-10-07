pub mod entity;
pub mod prelude;

#[derive(Clone, Debug, bon::Builder)]
pub struct AuthenticationService<AR, TR> {
    authentication_repository: AR,
    token_repository: TR,
}

impl<AR, TR> prelude::AuthenticationService for AuthenticationService<AR, TR>
where
    AR: prelude::AuthenticationRepository,
    TR: prelude::TokenRepository,
{
    async fn login(
        &self,
        req: prelude::LoginRequest,
    ) -> Result<prelude::LoginSuccess, prelude::LoginError> {
        let email = req.email.into_inner();
        let password = req.password.into_inner();
        let password_hash = hash_password(&email, &password);
        let profile = self
            .authentication_repository
            .find_by_credentials(email.as_str(), password_hash.as_str())
            .await?
            .ok_or(prelude::LoginError::InvalidCredentials)?;

        let token = self.token_repository.create_token(&profile).await?;
        Ok(prelude::LoginSuccess { token })
    }

    async fn signup(
        &self,
        req: prelude::SignupRequest,
    ) -> Result<prelude::LoginSuccess, prelude::SignupError> {
        let email = req.email.into_inner();
        let password = req.password.into_inner();
        let password_hash = hash_password(&email, &password);
        let profile = self
            .authentication_repository
            .create(email.as_str(), password_hash.as_str())
            .await?;

        let token = self.token_repository.create_token(&profile).await?;
        Ok(prelude::LoginSuccess { token })
    }
}

fn hash_password(email: &str, password: &str) -> String {
    use base64ct::Encoding;
    use sha2::Digest;

    let mut hasher = sha2::Sha256::new();
    hasher.update(email.as_bytes());
    hasher.update(password.as_bytes());
    let hash = hasher.finalize();
    base64ct::Base64::encode_string(&hash)
}
