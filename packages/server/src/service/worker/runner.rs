use entertainarr_database::Database;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::service::{storage::Storage, tmdb::Tmdb};

use super::Action;

pub(super) struct Runner {
    cancel: CancellationToken,
    context: super::Context,
    receiver: mpsc::UnboundedReceiver<super::Action>,
}

impl Runner {
    pub(super) fn new(
        cancel: CancellationToken,
        sender: mpsc::UnboundedSender<super::Action>,
        receiver: mpsc::UnboundedReceiver<super::Action>,
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

    #[tracing::instrument(skip(self))]
    async fn handle_action(&self, action: Action) {
        match action.params {
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
