use axum::routing::{get, post};

mod follow;
mod get_by_id;
mod list;
mod search;
mod season;

pub(crate) fn router() -> axum::Router {
    axum::Router::new()
        .route("/", get(list::handle))
        .route("/search", get(search::handle))
        .route("/{tvshow_id}", get(get_by_id::handle))
        .route(
            "/{tvshow_id}/follow",
            post(follow::create).delete(follow::delete),
        )
        .route("/{tvshow_id}/seasons", get(season::list::handle))
}

impl From<crate::model::tvshow::Entity> for entertainarr_api::tvshow::TVShow {
    fn from(value: crate::model::tvshow::Entity) -> Self {
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

            following: false,
        }
    }
}
