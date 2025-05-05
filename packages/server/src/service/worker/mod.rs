use std::sync::Arc;

use entertainarr_database::Database;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use super::tmdb::Tmdb;

mod queue;
mod runner;

mod sync_every_tvshow;
mod sync_tvshow;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_size")]
    size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            size: Self::default_size(),
        }
    }
}

impl Config {
    pub const fn default_size() -> usize {
        100
    }

    pub fn build(&self, database: Database, tmdb: Tmdb) -> std::io::Result<Worker> {
        Worker::new(database, tmdb)
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
    pub fn new(database: Database, tmdb: Tmdb) -> std::io::Result<Self> {
        let cancel = CancellationToken::new();
        let (sender, receiver) = queue::channel();
        let task = runner::Runner::new(cancel.clone(), sender.clone(), receiver, database, tmdb);
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

#[derive(Debug)]
enum ActionParams {
    SyncEveryTVShow(sync_every_tvshow::SyncEveryTVShow),
    SyncTvShow(sync_tvshow::SyncTVShow),
}

#[derive(Debug)]
pub struct Action {
    params: ActionParams,
    retry: u8,
}

impl Action {
    pub fn sync_every_tvshow() -> Self {
        Self {
            params: ActionParams::SyncEveryTVShow(sync_every_tvshow::SyncEveryTVShow),
            retry: 0,
        }
    }

    pub fn sync_tvshow(tvshow_id: u64) -> Self {
        Self {
            params: ActionParams::SyncTvShow(sync_tvshow::SyncTVShow { tvshow_id }),
            retry: 0,
        }
    }
}
