pub mod list;

impl From<crate::model::tvshow_season::Entity> for entertainarr_api::tvshow_season::TVShowSeason {
    fn from(value: crate::model::tvshow_season::Entity) -> Self {
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
