use axum::Json;
use axum::{Extension, http::StatusCode};

use crate::service::worker::{Action, Worker};

#[derive(Debug, serde::Deserialize)]
pub struct Payload {
    source: String,
    #[serde(default)]
    path: String,
}

pub(super) async fn handle(
    Extension(worker): Extension<Worker>,
    Json(payload): Json<Payload>,
) -> StatusCode {
    worker
        .push(Action::scan_storage_path(payload.source, payload.path))
        .await;
    StatusCode::CREATED
}
