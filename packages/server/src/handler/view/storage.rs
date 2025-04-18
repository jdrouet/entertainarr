use axum::Extension;
use axum::extract::Path;
use entertainarr_storage::source::AnySource;
use entertainarr_storage::source::prelude::Source;
use entertainarr_web::storage::StorageView;

use crate::handler::view::error::ViewError;
use crate::service::storage::Storage;

async fn handle(
    storage: Storage,
    name: String,
    path: String,
) -> Result<super::View<StorageView>, ViewError> {
    println!("handling name={name:?} path={path:?}");
    let source: &AnySource = storage
        .source(name.as_str())
        .ok_or(ViewError::not_found("source not found"))?;

    let source_path = path.as_str().strip_suffix('/').unwrap_or(path.as_str());
    let entries = source.list(source_path).await?;

    let view = StorageView::new(name, path, entries);

    Ok(super::View(view))
}

pub(super) async fn handle_root(
    Extension(storage): Extension<Storage>,
    Path(name): Path<String>,
) -> Result<super::View<StorageView>, ViewError> {
    handle(storage, name, String::new()).await
}

pub(super) async fn handle_path(
    Extension(storage): Extension<Storage>,
    Path((name, path)): Path<(String, String)>,
) -> Result<super::View<StorageView>, ViewError> {
    handle(storage, name, path).await
}
