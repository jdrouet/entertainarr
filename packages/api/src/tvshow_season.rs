#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
}

impl From<tmdb_api::tvshow::SeasonBase> for TVShowSeason {
    fn from(value: tmdb_api::tvshow::SeasonBase) -> Self {
        Self {
            id: value.id,
            name: value.name,
            air_date: value.air_date,
            overview: value.overview,
            poster_path: value.poster_path,
            season_number: value.season_number,
        }
    }
}
