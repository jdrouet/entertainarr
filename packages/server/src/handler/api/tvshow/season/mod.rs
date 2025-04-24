pub mod episode;
pub mod list;

fn season_to_view(
    value: entertainarr_database::model::tvshow_season::Entity,
) -> entertainarr_api::tvshow_season::TVShowSeason {
    entertainarr_api::tvshow_season::TVShowSeason {
        id: value.id,
        name: value.name,
        air_date: value.air_date,
        overview: value.overview,
        poster_path: value.poster_path,
        season_number: value.season_number,
    }
}
