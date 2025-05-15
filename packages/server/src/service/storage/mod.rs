use std::sync::Arc;

#[derive(Debug, serde::Deserialize)]
pub struct TVShowStorageConfig {
    pub store: any_storage::any::AnyStoreConfig,
}

impl TVShowStorageConfig {
    fn build(&self) -> std::io::Result<TVShowStorage> {
        Ok(TVShowStorage {
            store: self
                .store
                .build()
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))?,
        })
    }
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    pub tvshow: Option<TVShowStorageConfig>,
}

impl Config {
    pub fn build(&self) -> std::io::Result<Storage> {
        let tvshow = match self.tvshow {
            Some(ref inner) => Some(Arc::new(inner.build()?)),
            None => None,
        };
        Ok(Storage { tvshow })
    }
}

#[derive(Debug)]
pub struct TVShowStorage {
    pub(crate) store: any_storage::any::AnyStore,
}

#[derive(Clone, Debug)]
pub struct Storage {
    pub(crate) tvshow: Option<Arc<TVShowStorage>>,
}
