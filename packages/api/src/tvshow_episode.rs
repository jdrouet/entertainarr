#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Watch {
    pub progress: u64,
    pub completed: bool,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
}
