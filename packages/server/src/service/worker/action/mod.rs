mod sync_every_tvshow;
mod sync_tvshow;

use std::{sync::Arc, time::Duration};

use entertainarr_database::Database;
use tokio_util::sync::CancellationToken;

use crate::service::tmdb::Tmdb;

pub(super) trait Executable<Action> {
    fn execute(&self, action: &Action) -> impl Future<Output = Result<(), super::Error>>;
}

pub(super) struct Runner {
    cancel: CancellationToken,
    database: Database,
    tmdb: Tmdb,
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
            database,
            tmdb,
            notify,
            tick: tokio::time::interval(tick_period),
        }
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
