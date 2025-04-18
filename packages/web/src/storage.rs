use askama::Template;
use entertainarr_storage::entry::EntryInfo;

#[derive(Debug, Template)]
#[template(path = "view/storage.html")]
pub struct StorageView {
    source: String,
    path: String,
    entries: Vec<EntryInfo>,
}

impl StorageView {
    pub fn new(source: String, path: String, entries: Vec<EntryInfo>) -> Self {
        Self {
            source,
            path,
            entries,
        }
    }
}
