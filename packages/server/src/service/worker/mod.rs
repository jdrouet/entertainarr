use std::sync::Arc;

use entertainarr_database::Database;
use entertainarr_storage::entry::FileInfo;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use super::{storage::Storage, tmdb::Tmdb};

mod queue;
mod runner;

mod analyse_file;
mod scan_every_storage;
mod scan_storage_path;
mod sync_tvshow;
mod sync_tvshow_season;

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
        Worker::new(database, storage, tmdb)
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
    pub fn new(database: Database, storage: Storage, tmdb: Tmdb) -> std::io::Result<Self> {
        let cancel = CancellationToken::new();
        let (sender, receiver) = queue::channel();
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
        let _ = self.sender.send(action);
    }
}

#[derive(Debug)]
struct Context {
    sender: queue::Sender<Action>,
    database: Database,
    storage: Storage,
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
    AnalyzeFile(analyse_file::AnalyseFile),
    ScanEveryStorage(scan_every_storage::ScanEveryStorage),
    ScanStoragePath(scan_storage_path::ScanStoragePath),
    SyncTvShow(sync_tvshow::SyncTVShow),
    SyncTvShowSeason(sync_tvshow_season::SyncTVShowSeason),
}

#[derive(Debug)]
pub struct Action {
    params: ActionParams,
    retry: u8,
}

impl Action {
    pub fn analyze_file(source: String, path: String, file: FileInfo) -> Self {
        Self {
            params: ActionParams::AnalyzeFile(analyse_file::AnalyseFile { source, path, file }),
            retry: 0,
        }
    }

    pub fn scan_storage_full() -> Self {
        Self {
            params: ActionParams::ScanEveryStorage(scan_every_storage::ScanEveryStorage),
            retry: 0,
        }
    }

    pub fn scan_storage_path(name: String, path: String) -> Self {
        Self {
            params: ActionParams::ScanStoragePath(scan_storage_path::ScanStoragePath {
                name,
                path,
            }),
            retry: 0,
        }
    }

    pub fn sync_tvshow(tvshow_id: u64) -> Self {
        Self {
            params: ActionParams::SyncTvShow(sync_tvshow::SyncTVShow { tvshow_id }),
            retry: 0,
        }
    }

    pub fn sync_tvshow_season(tvshow_id: u64, season_number: u64) -> Self {
        Self {
            params: ActionParams::SyncTvShowSeason(sync_tvshow_season::SyncTVShowSeason {
                tvshow_id,
                season_number,
            }),
            retry: 0,
        }
    }
}
