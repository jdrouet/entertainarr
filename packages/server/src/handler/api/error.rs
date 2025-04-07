use std::borrow::Cow;

use axum::{Json, http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub struct ApiError {
    code: StatusCode,
    message: Cow<'static, str>,
}

impl From<sqlx::Error> for ApiError {
    fn from(value: sqlx::Error) -> Self {
        tracing::error!(message = "database error", cause = %value);
        ApiError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: Cow::Borrowed("internal database error"),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.code, Json(self.message)).into_response()
    }
}
