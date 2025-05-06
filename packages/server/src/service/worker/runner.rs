use std::time::Duration;

use entertainarr_database::Database;
use tokio_util::sync::CancellationToken;

use crate::service::tmdb::Tmdb;

use super::Action;

pub(super) struct Runner {
    cancel: CancellationToken,
    context: super::Context,
    receiver: super::queue::Receiver<super::Action>,
    tick: tokio::time::Interval,
}

impl Runner {
    pub(super) fn new(
        cancel: CancellationToken,
        sender: super::queue::Sender<super::Action>,
        receiver: super::queue::Receiver<super::Action>,
        tick_period: Duration,
        database: Database,
        tmdb: Tmdb,
    ) -> Self {
        Self {
            cancel,
            context: super::Context {
                sender,
                database,
                tmdb,
            },
            receiver,
            tick: tokio::time::interval(tick_period),
        }
    }

    #[tracing::instrument(skip_all, fields(retry = action.retry))]
    async fn handle_action(&self, action: Action) {
        if action.retry > 10 {
            tracing::error!(message = "too many retry, aborting", action = ?action);
            return;
        }
        match action.params {
            super::ActionParams::SyncEveryTVShow(ref inner) => {
                if let Err(err) = inner.execute(&self.context).await {
                    tracing::warn!(
                        message = "unable to trigger sync of every tvshow",
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
        }
    }

    async fn iterate(&mut self) {
        tokio::select! {
            _ = self.cancel.cancelled() => {
                tracing::info!("worker is being stopped");
            },
            _ = self.tick.tick() => {
                tracing::debug!("tick");
                let _ = self.context.sender.send(Action::sync_every_tvshow());
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
