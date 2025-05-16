use axum::routing::{get, post};

mod follow;
mod get_by_id;
mod list;
mod search;
mod sync;
mod watch;
mod watchlist;

pub(super) fn router() -> axum::Router {
    axum::Router::new()
        .route("/", get(list::handle))
        .route("/search", get(search::handle))
        .route("/sync", post(sync::all))
        .route("/watchlist", get(watchlist::handle))
        .route("/{tvshow_id}", get(get_by_id::handle))
        .route("/{tvshow_id}/sync", post(sync::single))
        .route(
            "/{tvshow_id}/follow",
            post(follow::create).delete(follow::delete),
        )
        .route(
            "/{tvshow_id}/watch",
            post(watch::create).delete(watch::delete),
        )
        .nest("/{tvshow_id}/seasons", super::tvshow_season::router())
}

fn tvshow_to_view(
    value: entertainarr_database::model::tvshow::Entity,
) -> entertainarr_api::tvshow::TVShow {
    entertainarr_api::tvshow::TVShow {
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

        following: value.following,
        episode_count: value.episode_count,
        watched_episode_count: value.watched_episode_count,
        terminated: false,
    }
}
