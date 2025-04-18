use axum::{extract::FromRequestParts, http::StatusCode};
use axum_extra::extract::CookieJar;

use super::error::ApiError;

#[derive(Clone, Debug)]
pub struct Authentication(pub u64);

impl<S> FromRequestParts<S> for Authentication {
    type Rejection = ApiError;

    fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        let jar = CookieJar::from_headers(&parts.headers);

        async move {
            let user_id = jar.get("user_id").ok_or_else(|| {
                ApiError::new(
                    StatusCode::UNAUTHORIZED,
                    "unable to get authentication cookie",
                )
            })?;
            let user_id = user_id.value().parse::<u64>().map_err(|_| {
                ApiError::new(StatusCode::UNAUTHORIZED, "invalid authentication cookie")
            })?;

            Ok(Authentication(user_id))
        }
    }
}
