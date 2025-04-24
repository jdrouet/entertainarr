use axum::routing::{get, post};

mod episode;
mod list;
mod watch;

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
        episode_count: value.episode_count,
        watched_episode_count: value.watched_episode_count,
    }
}

pub(super) fn router() -> axum::Router {
    axum::Router::new()
        .route("/", get(list::handle))
        .nest("/{season_number}/episodes", episode::router())
        .route(
            "/{season_number}/watch",
            post(watch::create).delete(watch::delete),
        )
}
