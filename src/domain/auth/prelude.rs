#[cfg(test)]
use std::sync::Arc;

use crate::domain::auth::entity::Profile;

#[derive(Debug)]
pub struct LoginRequest {
    pub email: super::entity::Email,
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

#[derive(Debug)]
pub struct SignupRequest {
    pub email: super::entity::Email,
    pub password: super::entity::Password,
}

#[derive(Debug, thiserror::Error)]
pub enum SignupError {
    #[error("email address already used")]
    EmailConflict,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub trait AuthenticationService: Send + Sync + 'static {
    fn login(
        &self,
        request: LoginRequest,
    ) -> impl Future<Output = Result<LoginSuccess, LoginError>> + Send;
    fn signup(
        &self,
        request: SignupRequest,
    ) -> impl Future<Output = Result<LoginSuccess, SignupError>> + Send;
}

#[cfg(test)]
impl<S: AuthenticationService> AuthenticationService for Arc<S> {
    async fn login(&self, request: LoginRequest) -> Result<LoginSuccess, LoginError> {
        self.as_ref().login(request).await
    }
    async fn signup(&self, request: SignupRequest) -> Result<LoginSuccess, SignupError> {
        self.as_ref().signup(request).await
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
        fn signup(
            &self,
            request: SignupRequest,
        ) -> impl Future<Output = Result<LoginSuccess, SignupError>> + Send;
    }
}

pub trait AuthenticationRepository: Send + Sync + 'static {
    fn find_by_credentials(
        &self,
        email: &str,
        password: &str,
    ) -> impl Future<Output = anyhow::Result<Option<Profile>>> + Send;
    fn create(
        &self,
        email: &str,
        password: &str,
    ) -> impl Future<Output = Result<Profile, SignupError>> + Send;
}

pub trait TokenRepository: Send + Sync + 'static {
    fn create_token(
        &self,
        profile: &Profile,
    ) -> impl Future<Output = anyhow::Result<String>> + Send;
}
