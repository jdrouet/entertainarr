use crate::{
    adapter::http_server::handler::ApiError,
    domain::auth::prelude::{AuthenticationService, VerifyError},
};

#[derive(Clone, Copy, Debug)]
pub struct CurrentUser(pub u64);

impl<S> axum::extract::FromRequestParts<S> for CurrentUser
where
    S: crate::adapter::http_server::prelude::ServerState,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        if let Some(authorization) = parts.headers.get(axum::http::header::AUTHORIZATION) {
            let authorization = authorization.to_str().map_err(|err| {
                tracing::warn!(error = ?err, "unable to read authorization token");
                ApiError::unauthorized("unable to read authorization header")
            })?;
            let authorization = authorization
                .strip_prefix("Bearer ")
                .ok_or_else(|| ApiError::unauthorized("authorization header format invalid"))?;
            state
                .authentication_service()
                .verify(authorization)
                .await
                .map(|profile| CurrentUser(profile.id))
                .map_err(|err| match err {
                    VerifyError::ExpiredToken => {
                        ApiError::unauthorized("authorization token expired")
                    }
                    VerifyError::InvalidToken => {
                        ApiError::unauthorized("authorization token invalid")
                    }
                    VerifyError::Internal(inner) => {
                        tracing::error!(error = ?inner, "unable to verify token");
                        ApiError::internal()
                    }
                })
        } else {
            Err(ApiError::unauthorized("authorization header not found"))
        }
    }
}
