use std::{sync::Arc, time::Duration};

use entertainarr_database::Database;
use tokio_util::sync::CancellationToken;

use crate::service::tmdb::Tmdb;

pub(super) trait Executable<Action> {
    fn execute(&self, action: &Action) -> impl Future<Output = Result<(), super::Error>>;
}

pub(super) struct Runner {
    cancel: CancellationToken,
    context: super::Context,
    notify: Arc<tokio::sync::Notify>,
    tick: tokio::time::Interval,
}

impl Runner {
    pub(super) fn new(
        cancel: CancellationToken,
        notify: Arc<tokio::sync::Notify>,
        tick_period: Duration,
        database: Database,
        tmdb: Tmdb,
    ) -> Self {
        Self {
            cancel,
            context: super::Context { database, tmdb },
            notify,
            tick: tokio::time::interval(tick_period),
        }
    }

    #[tracing::instrument(skip_all, fields(retry = action.retry))]
    async fn handle_action(&self, action: super::action::Action) {
        // if let Err(err) = action.execute(&self.context).await {
        //     tracing::warn!(message = "unable to execute action", cause = %err);
        //     if let Some(retry) = action.retry() {
        //         let _ = self.context.sender.send(retry);
        //     }
        // }
    }

    async fn tick(&mut self) {}

    async fn iterate(&mut self) {}

    pub(super) async fn run(mut self) {
        while !self.cancel.is_cancelled() {
            tokio::select! {
                _ = self.cancel.cancelled() => {
                    tracing::info!("worker is being stopped");
                },
                _ = self.tick.tick() => {
                    self.tick().await;
                },
                _ = self.notify.notified() => {
                    self.iterate().await;
                },
            }
        }
        tracing::info!("runner shutdown");
    }
}
