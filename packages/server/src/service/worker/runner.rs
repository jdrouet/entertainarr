use entertainarr_database::Database;
use tmdb_api::{
    prelude::Command,
    tvshow::{details::TVShowDetails, season::details::TVShowSeasonDetails},
};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::service::{storage::Storage, tmdb::Tmdb};

use super::Action;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Database(#[from] entertainarr_database::sqlx::Error),
    #[error(transparent)]
    Tmdb(#[from] tmdb_api::error::Error),
}

pub(super) struct Runner {
    cancel: CancellationToken,
    sender: mpsc::Sender<super::Action>,
    receiver: mpsc::Receiver<super::Action>,
    database: Database,
    #[allow(unused)]
    storage: Storage,
    tmdb: Tmdb,
}

impl Runner {
    pub(super) fn new(
        cancel: CancellationToken,
        sender: mpsc::Sender<super::Action>,
        receiver: mpsc::Receiver<super::Action>,
        database: Database,
        storage: Storage,
        tmdb: Tmdb,
    ) -> Self {
        Self {
            cancel,
            sender,
            receiver,
            database,
            storage,
            tmdb,
        }
    }

    async fn sync_tvshow(&self, tvshow_id: u64) -> Result<(), Error> {
        tracing::debug!("fetching details");
        let tvshow = TVShowDetails::new(tvshow_id)
            .execute(self.tmdb.as_ref())
            .await?;

        tracing::debug!("storing in database");
        entertainarr_database::model::tvshow::upsert_all(
            self.database.as_ref(),
            std::iter::once(&tvshow.inner),
        )
        .await?;

        tracing::debug!("delegating new tasks");
        for action in tvshow
            .seasons
            .into_iter()
            .map(|season| Action::sync_tvshow_season(tvshow_id, season.inner.season_number))
        {
            let _ = self.sender.send(action).await;
        }

        tracing::debug!("done");
        Ok(())
    }

    async fn sync_tvshow_season(&self, tvshow_id: u64, season_number: u64) -> Result<(), Error> {
        tracing::debug!("fetching details");
        let season = TVShowSeasonDetails::new(tvshow_id, season_number)
            .execute(self.tmdb.as_ref())
            .await?;

        tracing::debug!("storing in database");
        entertainarr_database::model::tvshow_season::upsert_all(
            self.database.as_ref(),
            tvshow_id,
            std::iter::once(&season.inner),
        )
        .await?;

        entertainarr_database::model::tvshow_episode::upsert_all(
            self.database.as_ref(),
            season.inner.id,
            season.episodes.iter().map(|item| &item.inner),
        )
        .await?;
        tracing::debug!("completed");
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn handle_action(&self, action: Action) {
        match action.params {
            super::ActionParams::SyncTvShow { tvshow_id } => {
                match self.sync_tvshow(tvshow_id).await {
                    Ok(_) => {}
                    Err(err) => {
                        tracing::warn!(message = "unable to synchronize tvshow", cause = %err);
                        let _ = self
                            .sender
                            .send(Action {
                                params: action.params,
                                retry: action.retry + 1,
                            })
                            .await;
                    }
                }
            }
            super::ActionParams::SyncTvShowSeason {
                tvshow_id,
                season_number,
            } => match self.sync_tvshow_season(tvshow_id, season_number).await {
                Ok(_) => {}
                Err(err) => {
                    tracing::warn!(
                        message = "unable to synchronize tvshow season",
                        cause = %err,
                    );
                    let _ = self
                        .sender
                        .send(Action {
                            params: action.params,
                            retry: action.retry + 1,
                        })
                        .await;
                }
            },
        }
    }

    async fn iterate(&mut self) {
        tokio::select! {
            _ = self.cancel.cancelled() => {
                tracing::info!("worker is being stopped");
                self.receiver.close();
            },
            maybe = self.receiver.recv() => {
                match maybe {
                    Some(action) => {
                        self.handle_action(action).await;
                    }
                    None => {
                        tracing::debug!("receiver is closed");
                    }
                }
            },
        };
    }

    pub(super) async fn run(mut self) {
        while !self.cancel.is_cancelled() || !self.receiver.is_empty() {
            self.iterate().await;
        }
        tracing::info!("runner shutdown");
    }
}
