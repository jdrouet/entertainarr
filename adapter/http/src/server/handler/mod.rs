use std::borrow::Cow;

use axum::Json;
use axum::response::IntoResponse;
use axum::routing::head;

mod auth;
mod podcast;
mod podcast_episode;
pub(crate) mod prelude;
mod status;

pub fn create<S>() -> axum::Router<S>
where
    S: crate::server::prelude::ServerState + Clone,
{
    let api = axum::Router::new()
        .merge(auth::create::<S>())
        .merge(podcast::create::<S>())
        .merge(podcast_episode::create::<S>());
    axum::Router::new()
        .route("/api", head(status::handle))
        .nest("/api", api)
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

    pub fn bad_request(message: impl Into<Cow<'static, str>>) -> ApiError {
        ApiError {
            status_code: axum::http::StatusCode::BAD_REQUEST,
            message: message.into(),
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

    pub fn unauthorized(message: impl Into<Cow<'static, str>>) -> ApiError {
        ApiError {
            status_code: axum::http::StatusCode::UNAUTHORIZED,
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
