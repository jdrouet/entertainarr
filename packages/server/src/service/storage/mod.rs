use std::sync::Arc;

#[derive(Debug, Default, serde::Deserialize)]
#[serde(transparent)]
pub struct Config(entertainarr_storage::Config);

impl Config {
    pub fn build(&self) -> std::io::Result<Storage> {
        self.0.build().map(Arc::new)
    }
}

pub type Storage = Arc<entertainarr_storage::Storage>;
