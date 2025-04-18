use std::{borrow::Cow, io::ErrorKind};

use axum::{Json, http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub struct ApiError {
    code: StatusCode,
    message: Cow<'static, str>,
}

impl ApiError {
    pub fn new(code: StatusCode, message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    pub fn not_found(message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            code: StatusCode::NOT_FOUND,
            message: message.into(),
        }
    }
}

impl From<axum::http::Error> for ApiError {
    fn from(value: axum::http::Error) -> Self {
        tracing::error!(message = "http error", cause = ?value);
        ApiError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: Cow::Borrowed("internal response error"),
        }
    }
}

impl From<std::io::Error> for ApiError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            ErrorKind::InvalidInput => ApiError {
                code: StatusCode::BAD_REQUEST,
                message: value.to_string().into(),
            },
            _ => ApiError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Cow::Borrowed("internal storage error"),
            },
        }
    }
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

impl From<tmdb_api::error::Error> for ApiError {
    fn from(value: tmdb_api::error::Error) -> Self {
        tracing::error!(message = "tmdb error", cause = ?value);
        ApiError {
            code: StatusCode::BAD_GATEWAY,
            message: Cow::Borrowed("unable to fetch data from tmdb"),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.code, Json(self.message)).into_response()
    }
}
