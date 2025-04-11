use axum::Extension;
use axum::Json;
use axum::extract::Path;
use entertainarr_storage::entry::EntryInfo;
use entertainarr_storage::source::prelude::Source;

use crate::handler::api::error::ApiError;
use crate::service::storage::Storage;

async fn handle(
    storage: Storage,
    name: &str,
    path: &str,
) -> Result<Json<Vec<EntryInfo>>, ApiError> {
    let source = storage
        .source(name)
        .ok_or_else(|| ApiError::not_found(format!("source {name:?} does not exist")))?;
    let list = source.list(path).await?;
    Ok(Json(list))
}

pub(super) async fn handle_path(
    Extension(storage): Extension<Storage>,
    Path((name, path)): Path<(String, String)>,
) -> Result<Json<Vec<EntryInfo>>, ApiError> {
    handle(storage, name.as_str(), path.as_str()).await
}

pub(super) async fn handle_root(
    Extension(storage): Extension<Storage>,
    Path(name): Path<String>,
) -> Result<Json<Vec<EntryInfo>>, ApiError> {
    handle(storage, name.as_str(), "").await
}
