#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TVShowSeason {
    pub id: u64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub air_date: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poster_path: Option<String>,
    pub season_number: u64,
    #[serde(default, skip_serializing_if = "crate::is_u32_zero")]
    pub episode_count: u32,
    #[serde(default, skip_serializing_if = "crate::is_u32_zero")]
    pub watched_episode_count: u32,
}
