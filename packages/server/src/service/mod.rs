pub mod fetcher;
pub mod tmdb;
pub mod worker;

#[derive(Debug, Default, serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    pub(crate) database: entertainarr_database::Config,
    #[serde(default)]
    pub(crate) fetcher: fetcher::Config,
    pub(crate) tmdb: tmdb::Config,
    #[serde(default)]
    pub(crate) worker: worker::Config,
}
