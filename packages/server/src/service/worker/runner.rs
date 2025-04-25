use entertainarr_database::Database;
use tokio_util::sync::CancellationToken;

use crate::service::{storage::Storage, tmdb::Tmdb};

use super::Action;

pub(super) struct Runner {
    cancel: CancellationToken,
    context: super::Context,
    receiver: super::queue::Receiver<super::Action>,
}

impl Runner {
    pub(super) fn new(
        cancel: CancellationToken,
        sender: super::queue::Sender<super::Action>,
        receiver: super::queue::Receiver<super::Action>,
        database: Database,
        storage: Storage,
        tmdb: Tmdb,
    ) -> Self {
        Self {
            cancel,
            context: super::Context {
                sender,
                database,
                storage,
                tmdb,
            },
            receiver,
        }
    }

    #[tracing::instrument(skip_all, fields(retry = action.retry))]
    async fn handle_action(&self, action: Action) {
        match action.params {
            super::ActionParams::AnalyzeFile(ref inner) => {
                if let Err(err) = inner.execute(&self.context).await {
                    tracing::warn!(
                        message = "unable to analyze file",
                        cause = %err,
                    );
                    let _ = self.context.sender.send(Action {
                        params: action.params,
                        retry: action.retry + 1,
                    });
                }
            }
            super::ActionParams::ScanEveryStorage(ref inner) => {
                if let Err(err) = inner.execute(&self.context).await {
                    tracing::warn!(
                        message = "unable to scan every storage",
                        cause = %err,
                    );
                    let _ = self.context.sender.send(Action {
                        params: action.params,
                        retry: action.retry + 1,
                    });
                }
            }
            super::ActionParams::ScanStoragePath(ref inner) => {
                if let Err(err) = inner.execute(&self.context).await {
                    tracing::warn!(
                        message = "unable to scan storage",
                        cause = %err,
                    );
                    let _ = self.context.sender.send(Action {
                        params: action.params,
                        retry: action.retry + 1,
                    });
                }
            }
            super::ActionParams::SyncTvShow(ref inner) => {
                if let Err(err) = inner.execute(&self.context).await {
                    tracing::warn!(
                        message = "unable to synchronize tvshow",
                        cause = %err,
                    );
                    let _ = self.context.sender.send(Action {
                        params: action.params,
                        retry: action.retry + 1,
                    });
                }
            }
            super::ActionParams::SyncTvShowSeason(ref inner) => {
                if let Err(err) = inner.execute(&self.context).await {
                    tracing::warn!(
                        message = "unable to synchronize tvshow season",
                        cause = %err,
                    );
                    let _ = self.context.sender.send(Action {
                        params: action.params,
                        retry: action.retry + 1,
                    });
                }
            }
        }
    }

    async fn iterate(&mut self) {
        tokio::select! {
            _ = self.cancel.cancelled() => {
                tracing::info!("worker is being stopped");
            },
            maybe_action = self.receiver.recv() => {
                if let Some(action) = maybe_action {
                    self.handle_action(action).await;
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
