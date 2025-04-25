use std::collections::HashMap;

use source::prelude::Source;

pub mod entry;
pub mod source;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Config {
    #[serde(default)]
    sources: HashMap<Box<str>, source::Config>,
}

impl Config {
    pub fn add_source(&mut self, name: impl Into<Box<str>>, source: impl Into<source::Config>) {
        self.sources.insert(name.into(), source.into());
    }

    pub fn with_source(
        mut self,
        name: impl Into<Box<str>>,
        source: impl Into<source::Config>,
    ) -> Self {
        self.add_source(name, source);
        self
    }

    pub fn build(&self) -> std::io::Result<Storage> {
        let mut sources = HashMap::default();
        for (name, source) in self.sources.iter() {
            sources.insert(name.clone(), source.build()?);
        }
        Ok(Storage { sources })
    }
}

#[derive(Debug, Default)]
pub struct Storage {
    sources: HashMap<Box<str>, source::AnySource>,
}

impl Storage {
    pub async fn healthcheck(&self) -> std::io::Result<()> {
        for (_name, source) in self.sources.iter() {
            source.healthcheck().await?;
        }
        Ok(())
    }

    pub fn sources(&self) -> impl Iterator<Item = (&Box<str>, &source::AnySource)> {
        self.sources.iter()
    }

    pub fn source(&self, name: &str) -> Option<&source::AnySource> {
        self.sources.get(name)
    }
}
