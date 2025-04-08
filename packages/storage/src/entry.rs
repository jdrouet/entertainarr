#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EntryKind {
    File,
    Directory,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EntryInfo {
    Directory(DirectoryInfo),
    File(FileInfo),
}

impl EntryInfo {
    pub fn kind(&self) -> EntryKind {
        match self {
            Self::Directory(_) => EntryKind::Directory,
            Self::File(_) => EntryKind::File,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct DirectoryInfo {
    pub name: String,
    pub created_at: u64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
    pub created_at: u64,
    pub modified_at: u64,
}
