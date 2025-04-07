pub mod database;

#[derive(Debug, Default)]
pub struct Config {
    pub(crate) database: database::Config,
}
