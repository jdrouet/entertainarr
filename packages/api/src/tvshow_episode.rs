#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TVShowEpisode {
    pub id: u64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub air_date: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    pub episode_number: u64,
}

impl From<tmdb_api::tvshow::EpisodeShort> for TVShowEpisode {
    fn from(value: tmdb_api::tvshow::EpisodeShort) -> Self {
        Self {
            id: value.id,
            name: value.name,
            air_date: value.air_date,
            overview: value.overview,
            episode_number: value.episode_number,
        }
    }
}
