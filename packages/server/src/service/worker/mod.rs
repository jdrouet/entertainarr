use std::{sync::Arc, time::Duration};

use entertainarr_database::{Database, model::task};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use super::tmdb::Tmdb;

pub mod action;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_size")]
    size: usize,
    #[serde(default = "Config::default_tick_interval")]
    tick_interval: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            size: Self::default_size(),
            tick_interval: Self::default_tick_interval(),
        }
    }
}

impl Config {
    pub const fn default_size() -> usize {
        100
    }

    pub const fn default_tick_interval() -> u64 {
        60
    }

    pub fn build(&self, database: Database, tmdb: Tmdb) -> std::io::Result<Worker> {
        Worker::new(Duration::from_secs(self.tick_interval), database, tmdb)
    }
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct Worker {
    cancel: CancellationToken,
    database: Database,
    notify: Arc<tokio::sync::Notify>,
    task: Arc<JoinHandle<()>>,
}

impl Worker {
    pub fn new(tick_interval: Duration, database: Database, tmdb: Tmdb) -> std::io::Result<Self> {
        let cancel = CancellationToken::new();
        let notify = Arc::new(tokio::sync::Notify::new());
        let task = action::Runner::new(
            cancel.clone(),
            notify.clone(),
            tick_interval,
            database.clone(),
            tmdb,
        );
        let task = tokio::spawn(async move { task.run().await });
        Ok(Self {
            cancel,
            database,
            notify,
            task: Arc::new(task),
        })
    }

    pub async fn push(&self, action: task::Action) {
        if let Err(err) = task::CreateTask::new(action, task::Status::Waiting)
            .save(self.database.as_ref())
            .await
        {
            tracing::error!(message = "unable to create task", cause = %err);
        }
        self.notify.notify_waiters();
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Database(#[from] entertainarr_database::sqlx::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Tmdb(#[from] tmdb_api::error::Error),
}

impl From<crate::view::Error> for Error {
    fn from(value: crate::view::Error) -> Self {
        match value {
            crate::view::Error::Database(inner) => Error::Database(inner),
            crate::view::Error::Io(inner) => Error::Io(inner),
            crate::view::Error::Tmdb(inner) => Error::Tmdb(inner),
        }
    }
}
