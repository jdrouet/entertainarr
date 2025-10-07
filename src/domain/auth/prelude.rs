#[cfg(test)]
use std::sync::Arc;

use crate::domain::auth::entity::Profile;

#[derive(Debug)]
pub struct LoginRequest {
    pub email: super::entity::Email,
    #[allow(unused)]
    pub password: super::entity::Password,
}

#[derive(Debug)]
pub struct LoginSuccess {
    pub token: String,
}

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub trait AuthenticationService: Send + Sync + 'static {
    fn login(
        &self,
        request: LoginRequest,
    ) -> impl Future<Output = Result<LoginSuccess, LoginError>> + Send;
}

#[cfg(test)]
impl<S: AuthenticationService> AuthenticationService for Arc<S> {
    async fn login(&self, request: LoginRequest) -> Result<LoginSuccess, LoginError> {
        self.as_ref().login(request).await
    }
}

#[cfg(test)]
mockall::mock! {
    pub AuthenticationService {}

    impl AuthenticationService for AuthenticationService {
        fn login(
            &self,
            request: LoginRequest,
        ) -> impl Future<Output = Result<LoginSuccess, LoginError>> + Send;
    }
}

pub trait AuthenticationRepository: Send + Sync + 'static {
    fn find_by_email(
        &self,
        email: &str,
    ) -> impl Future<Output = anyhow::Result<Option<Profile>>> + Send;
}

pub trait TokenRepository: Send + Sync + 'static {
    fn create_token(
        &self,
        profile: &Profile,
    ) -> impl Future<Output = anyhow::Result<String>> + Send;
}
