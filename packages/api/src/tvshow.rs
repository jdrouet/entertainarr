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
    pub adult: bool,

    #[serde(default)]
    pub following: bool,
}

impl From<tmdb_api::tvshow::TVShowBase> for TVShow {
    fn from(value: tmdb_api::tvshow::TVShowBase) -> Self {
        Self {
            id: value.id,
            name: value.name,
            original_name: value.original_name,
            original_language: value.original_language,
            origin_country: value.origin_country,
            overview: value.overview,
            first_air_date: value.first_air_date,
            poster_path: value.poster_path,
            backdrop_path: value.backdrop_path,
            popularity: value.popularity,
            vote_count: value.vote_count,
            vote_average: value.vote_average,
            adult: value.adult,
            //
            following: false,
        }
    }
}
