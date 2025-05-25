use entertainarr_database::model;

impl super::Executable<model::task::SynchronizeEveryTVShow> for super::Runner {
    async fn execute(
        &self,
        _action: &model::task::SynchronizeEveryTVShow,
    ) -> Result<(), crate::service::worker::Error> {
        let mut tx = self.database.as_ref().begin().await?;
        let ids = model::tvshow::list_needing_update(&mut *tx).await?;
        tracing::debug!(message = "triggering tvshow sync", count = %ids.len());
        for tvshow_id in ids {
            if let Err(err) = model::task::CreateTask::new(
                model::task::Action::SynchronizeTVShow(model::task::SynchronizeTVShow {
                    tvshow_id,
                }),
                model::task::Status::Waiting,
            )
            .save(&mut *tx)
            .await
            {
                tracing::error!(message = "unable to create task", cause = %err);
            }
        }
        tx.commit().await?;
        Ok(())
    }
}

#[derive(Debug)]
pub(super) struct SyncEveryTVShow;

impl SyncEveryTVShow {
    #[tracing::instrument(name = "sync_every_tvshow", skip_all)]
    pub(super) async fn execute(&self, ctx: &Context) -> Result<(), crate::service::worker::Error> {
        let mut tx = ctx.database.as_ref().begin().await?;
        let ids = model::tvshow::list_needing_update(ctx.database.as_ref()).await?;
        tracing::debug!(message = "triggering tvshow sync", count = %ids.len());
        for tvshow_id in ids {
            if let Err(err) = model::task::CreateTask::new(
                model::task::Action::SynchronizeTVShow(model::task::SynchronizeTVShow {
                    tvshow_id,
                }),
                model::task::Status::Waiting,
            )
            .save(&mut *tx)
            .await
            {
                tracing::error!(message = "unable to create task", cause = %err);
            }
        }
        tx.commit().await?;
        Ok(())
    }
}
