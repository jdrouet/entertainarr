use axum::Extension;
use axum::extract::Path;
use entertainarr_storage::source::AnySource;
use entertainarr_storage::source::prelude::{File, Source};
use entertainarr_web::watch::{Source as WatchSource, WatchView};

use crate::handler::view::error::ViewError;
use crate::service::storage::Storage;

pub(super) async fn handle(
    Extension(storage): Extension<Storage>,
    Path((name, path)): Path<(String, String)>,
) -> Result<super::View, ViewError> {
    let source: &AnySource = storage
        .source(name.as_str())
        .ok_or(ViewError::not_found("source not found"))?;

    let file = source.file(&path).await?;

    let sources = vec![WatchSource::new(
        file.content_type().essence_str().to_string(),
        format!("/api/files/{name}/stream/{path}"),
    )];
    let view = WatchView::new(sources);

    Ok(super::View::from(view))
}
