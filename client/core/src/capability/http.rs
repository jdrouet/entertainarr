use crux_http::HttpError;

pub enum ApiError {
    Api(entertainarr_adapter_http::entity::ApiError),
    Http(HttpError),
}

impl From<HttpError> for ApiError {
    fn from(value: HttpError) -> Self {
        match &value {
            HttpError::Http {
                code: _,
                message: _,
                body: Some(body),
            } => match serde_json::from_slice(&body) {
                Ok(value) => Self::Api(value),
                Err(_) => {
                    tracing::warn!("unable to decode error body");
                    Self::Http(value)
                }
            },
            _ => Self::Http(value),
        }
    }
}
