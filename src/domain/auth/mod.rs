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
        res: prelude::LoginRequest,
    ) -> Result<prelude::LoginSuccess, prelude::LoginError> {
        let email = res.email.into_inner();
        let profile = self
            .authentication_repository
            .find_by_email(email.as_str())
            .await?
            .ok_or(prelude::LoginError::InvalidCredentials)?;
        let password = res.password.into_inner();
        if !profile.compare_password(&password) {
            tracing::warn!("invalid password");
            return Err(prelude::LoginError::InvalidCredentials);
        }
        let token = self.token_repository.create_token(&profile).await?;
        Ok(prelude::LoginSuccess { token })
    }
}
