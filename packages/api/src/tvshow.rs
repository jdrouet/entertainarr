#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TVShow {
    pub id: u64,
    pub name: String,
    pub original_name: String,
    pub original_language: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origin_country: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_air_date: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poster_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub vote_count: u64,
    pub vote_average: f64,
    #[serde(default, skip_serializing_if = "crate::is_false")]
    pub adult: bool,

    #[serde(default)]
    pub following: bool,
    #[serde(default)]
    pub episode_count: u32,
    #[serde(default)]
    pub watched_episode_count: u32,
    /// when there will be no more episodes
    #[serde(default)]
    pub terminated: bool,
}

impl TVShow {
    pub fn watch_completed(&self) -> bool {
        self.episode_count > 0 && self.episode_count == self.watched_episode_count
    }
}
