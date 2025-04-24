use std::sync::Arc;

use entertainarr_database::Database;
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_util::sync::CancellationToken;

use super::{storage::Storage, tmdb::Tmdb};

mod runner;

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

    pub fn build(
        &self,
        database: Database,
        storage: Storage,
        tmdb: Tmdb,
    ) -> std::io::Result<Worker> {
        Worker::new(self.size, database, storage, tmdb)
    }
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct Worker {
    cancel: CancellationToken,
    sender: mpsc::Sender<Action>,
    task: Arc<JoinHandle<()>>,
}

impl Worker {
    pub fn new(
        queue_size: usize,
        database: Database,
        storage: Storage,
        tmdb: Tmdb,
    ) -> std::io::Result<Self> {
        let cancel = CancellationToken::new();
        let (sender, receiver) = mpsc::channel(queue_size);
        let task = runner::Runner::new(
            cancel.clone(),
            sender.clone(),
            receiver,
            database,
            storage,
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
        if let Err(err) = self.sender.send(action).await {
            tracing::error!(message = "unable to send action to worker queue", cause = %err);
        }
    }
}

#[derive(Debug)]
enum ActionParams {
    SyncTvShow { tvshow_id: u64 },
    SyncTvShowSeason { tvshow_id: u64, season_number: u64 },
}

#[derive(Debug)]
pub struct Action {
    params: ActionParams,
    retry: u8,
}

impl Action {
    pub fn sync_tvshow(tvshow_id: u64) -> Self {
        Self {
            params: ActionParams::SyncTvShow { tvshow_id },
            retry: 0,
        }
    }

    pub fn sync_tvshow_season(tvshow_id: u64, season_number: u64) -> Self {
        Self {
            params: ActionParams::SyncTvShowSeason {
                tvshow_id,
                season_number,
            },
            retry: 0,
        }
    }
}
