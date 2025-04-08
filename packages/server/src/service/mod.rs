pub mod database;
pub mod storage;

#[derive(Debug, Default, serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    pub(crate) database: database::Config,
    #[serde(default)]
    pub(crate) storage: storage::Config,
}
