use any_storage::{Entry, Store, StoreDirectory, StoreFile, StoreMetadata, any::AnyStoreDirectory};
use entertainarr_database::model::file;
use tokio_stream::StreamExt;

use crate::service::worker::Context;

#[derive(Debug)]
pub(super) struct ScanTVShowStorage;

impl ScanTVShowStorage {
    #[tracing::instrument(name = "scan_tvshow_storage", skip_all)]
    pub(super) async fn execute(&self, ctx: &Context) -> Result<(), crate::service::worker::Error> {
        let mut tx = ctx.database.as_ref().begin().await?;
        let mut stack: Vec<AnyStoreDirectory> = vec![];
        if let Some(ref tvshow) = ctx.storage.tvshow {
            stack.push(tvshow.store.root().await?);
        }
        while let Some(dir) = stack.pop() {
            tracing::debug!(message = "reading directory", path = ?dir.path());
            let mut reader = dir.read().await?;
            while let Some(Ok(item)) = reader.next().await {
                match item {
                    Entry::File(file) => {
                        let content_type = mime_guess::from_path(file.path())
                            .first()
                            .map(|item| item.essence_str().to_string());
                        let meta = file.metadata().await?;
                        file::upsert(
                            &mut *tx,
                            file::Store::TVShow,
                            file.path(),
                            meta.size(),
                            content_type.as_deref(),
                            meta.created(),
                            meta.modified(),
                        )
                        .await?;
                    }
                    Entry::Directory(item) => {
                        stack.push(item);
                    }
                }
            }
        }
        tx.commit().await?;
        Ok(())
    }
}
