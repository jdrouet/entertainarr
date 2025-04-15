use std::{borrow::Cow, io::ErrorKind};

use axum::{Json, http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub struct ViewError {
    code: StatusCode,
    message: Cow<'static, str>,
}

impl From<axum::http::Error> for ViewError {
    fn from(value: axum::http::Error) -> Self {
        tracing::error!(message = "http error", cause = ?value);
        ViewError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: Cow::Borrowed("internal response error"),
        }
    }
}

impl From<std::io::Error> for ViewError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            ErrorKind::InvalidInput => ViewError {
                code: StatusCode::BAD_REQUEST,
                message: value.to_string().into(),
            },
            _ => ViewError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Cow::Borrowed("internal storage error"),
            },
        }
    }
}

impl From<sqlx::Error> for ViewError {
    fn from(value: sqlx::Error) -> Self {
        tracing::error!(message = "database error", cause = %value);
        ViewError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: Cow::Borrowed("internal database error"),
        }
    }
}

impl From<tmdb_api::error::Error> for ViewError {
    fn from(value: tmdb_api::error::Error) -> Self {
        tracing::error!(message = "tmdb error", cause = ?value);
        ViewError {
            code: StatusCode::BAD_GATEWAY,
            message: Cow::Borrowed("unable to fetch data from tmdb"),
        }
    }
}

impl IntoResponse for ViewError {
    fn into_response(self) -> axum::response::Response {
        (self.code, Json(self.message)).into_response()
    }
}
