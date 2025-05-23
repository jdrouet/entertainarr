use std::{sync::Arc, time::Duration};

use entertainarr_database::Database;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use self::action::Action;
use super::tmdb::Tmdb;

pub mod action;
mod queue;
mod runner;

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
    sender: queue::Sender<Action>,
    task: Arc<JoinHandle<()>>,
}

impl Worker {
    pub fn new(tick_interval: Duration, database: Database, tmdb: Tmdb) -> std::io::Result<Self> {
        let cancel = CancellationToken::new();
        let (sender, receiver) = queue::channel();
        let task = runner::Runner::new(
            cancel.clone(),
            sender.clone(),
            receiver,
            tick_interval,
            database,
            tmdb,
        );
        let task = tokio::spawn(async move { task.run().await });
        Ok(Self {
            cancel,
            sender,
            task: Arc::new(task),
        })
    }

    pub async fn push(&self, action: Action) {
        let _ = self.sender.send(action);
    }
}

#[derive(Debug)]
struct Context {
    sender: queue::Sender<Action>,
    database: Database,
    tmdb: Tmdb,
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
