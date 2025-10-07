use std::borrow::Cow;

use axum::{Json, response::IntoResponse, routing::head};

mod auth;
mod status;

pub fn create<AS>() -> axum::Router<super::ServerState<AS>>
where
    AS: crate::domain::auth::prelude::AuthenticationService + Clone,
{
    axum::Router::new()
        .route("/api", head(status::handle))
        .nest("/api/auth", auth::create::<AS>())
}

#[derive(Debug, serde::Serialize)]
pub struct ApiError {
    #[serde(skip)]
    status_code: axum::http::StatusCode,
    message: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<ApiErrorDetail>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, Json(self)).into_response()
    }
}

impl ApiError {
    pub const fn internal() -> ApiError {
        ApiError {
            status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            message: Cow::Borrowed("internal error"),
            detail: None,
        }
    }

    pub fn conflict(message: impl Into<Cow<'static, str>>) -> ApiError {
        ApiError {
            status_code: axum::http::StatusCode::CONFLICT,
            message: message.into(),
            detail: None,
        }
    }

    pub fn bad_request(message: impl Into<Cow<'static, str>>) -> ApiError {
        ApiError {
            status_code: axum::http::StatusCode::BAD_REQUEST,
            message: message.into(),
            detail: None,
        }
    }

    pub fn with_detail(mut self, detail: ApiErrorDetail) -> Self {
        self.detail = Some(detail);
        self
    }
}

#[derive(Debug, serde::Serialize)]
pub struct ApiErrorDetail {
    attribute: Cow<'static, str>,
    reason: Cow<'static, str>,
}

impl ApiErrorDetail {
    pub fn new(
        attribute: impl Into<Cow<'static, str>>,
        reason: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            attribute: attribute.into(),
            reason: reason.into(),
        }
    }
}
