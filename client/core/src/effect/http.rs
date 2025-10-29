#[derive(Debug, Clone, PartialEq, Eq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub enum HttpResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T> From<crux_http::Result<crux_http::Response<T>>>
    for HttpResult<crux_http::Response<T>, crux_http::HttpError>
{
    fn from(value: crux_http::Result<crux_http::Response<T>>) -> Self {
        match value {
            Ok(response) => HttpResult::Ok(response),
            Err(error) => HttpResult::Err(error),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub enum HttpError {
    Api(crux_http::http::StatusCode, ApiError),
    Http(crux_http::HttpError),
}

impl From<crux_http::HttpError> for HttpError {
    fn from(value: crux_http::HttpError) -> Self {
        match value {
            crux_http::HttpError::Http {
                code,
                message,
                body: Some(body),
            } => match serde_json::from_slice::<'_, ApiError>(&body) {
                Ok(err) => HttpError::Api(code, err),
                Err(_) => HttpError::Http(crux_http::HttpError::Http {
                    code,
                    message,
                    body: Some(body),
                }),
            },
            other => HttpError::Http(other),
        }
    }
}

impl HttpError {
    pub const fn is_token_expired(&self) -> bool {
        matches!(
            self,
            HttpError::Api(crux_http::http::StatusCode::Unauthorized, _)
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub struct ApiError {
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<ApiErrorDetail>,
}

#[derive(Debug, Clone, PartialEq, Eq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub struct ApiErrorDetail {
    pub attribute: String,
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub enum Operation<Req, Res> {
    Request(Req),
    Success(Res),
    Error(HttpError),
}
