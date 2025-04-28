#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Watch {
    #[serde(default, skip_serializing_if = "crate::is_u64_zero")]
    pub progress: u64,
    #[serde(default, skip_serializing_if = "crate::is_false")]
    pub completed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TVShowEpisode {
    pub id: u64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub air_date: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    pub episode_number: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watch: Option<Watch>,
    #[serde(default, skip_serializing_if = "crate::is_u16_zero")]
    pub file_count: u16,
}

impl TVShowEpisode {
    pub fn watched(&self) -> bool {
        self.watch.as_ref().map(|w| w.completed).unwrap_or(false)
    }
}
