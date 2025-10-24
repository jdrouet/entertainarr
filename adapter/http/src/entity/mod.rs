use std::borrow::Cow;

pub mod auth;
pub mod podcast;
pub mod podcast_episode;

fn default_includes<T>() -> Vec<T> {
    Vec::new()
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResource<T, I = ()> {
    pub data: T,
    #[serde(default = "default_includes", skip_serializing_if = "Vec::is_empty")]
    pub includes: Vec<I>,
}

impl<T> ApiResource<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            includes: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Relation<T> {
    pub data: T,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiError {
    #[serde(skip)]
    #[cfg(feature = "server")]
    pub status_code: axum::http::StatusCode,
    pub message: Cow<'static, str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<ApiErrorDetail>,
}

#[cfg(feature = "server")]
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiErrorDetail {
    pub attribute: Cow<'static, str>,
    pub code: Cow<'static, str>,
}

impl ApiErrorDetail {
    pub fn new(
        attribute: impl Into<Cow<'static, str>>,
        code: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            attribute: attribute.into(),
            code: code.into(),
        }
    }
}
