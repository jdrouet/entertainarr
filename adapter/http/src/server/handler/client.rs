pub mod prelude {
    pub trait ClientService: Send + Sync + 'static {
        fn get_file<P: AsRef<std::path::Path>>(&self, path: P) -> Option<&[u8]>;
    }

    #[cfg(test)]
    #[derive(Clone, Debug, Default)]
    pub struct MockClientService;

    #[cfg(test)]
    impl ClientService for MockClientService {
        fn get_file<P: AsRef<std::path::Path>>(&self, _path: P) -> Option<&[u8]> {
            None
        }
    }
}

use std::borrow::Cow;

use axum::{
    extract::{Path, State},
    http::{HeaderName, HeaderValue},
    response::AppendHeaders,
};

use crate::server::handler::client::prelude::ClientService;

pub async fn handle_index<S>(
    State(state): State<S>,
) -> Result<(AppendHeaders<[(HeaderName, HeaderValue); 1]>, Vec<u8>), axum::http::StatusCode>
where
    S: crate::server::prelude::ServerState,
{
    handle(State(state), Path(Cow::Borrowed("index.html"))).await
}

pub async fn handle<'a, S>(
    State(state): State<S>,
    Path(fname): Path<Cow<'static, str>>,
) -> Result<(AppendHeaders<[(HeaderName, HeaderValue); 1]>, Vec<u8>), axum::http::StatusCode>
where
    S: crate::server::prelude::ServerState,
{
    let Some(content_type) = content_type(&fname) else {
        return Err(axum::http::StatusCode::NOT_FOUND);
    };
    let Some(slice) = state.client_service().get_file(fname.as_ref()) else {
        return Err(axum::http::StatusCode::NOT_FOUND);
    };
    let headers = AppendHeaders([(HeaderName::from_static("Content-Type"), content_type)]);
    Ok((headers, slice.to_vec()))
}

const CONTENT_TYPE_HTML: HeaderValue = HeaderValue::from_static("text/html");
const CONTENT_TYPE_CSS: HeaderValue = HeaderValue::from_static("text/css");
const CONTENT_TYPE_JS: HeaderValue = HeaderValue::from_static("text/javascript");
const CONTENT_TYPE_WASM: HeaderValue = HeaderValue::from_static("application/wasm");

fn content_type(fname: &str) -> Option<HeaderValue> {
    if fname.ends_with("html") {
        Some(CONTENT_TYPE_HTML)
    } else if fname.ends_with("css") {
        Some(CONTENT_TYPE_CSS)
    } else if fname.ends_with("js") {
        Some(CONTENT_TYPE_JS)
    } else if fname.ends_with("wasm") {
        Some(CONTENT_TYPE_WASM)
    } else {
        None
    }
}
