use axum::routing::{get, post};

mod list;
mod watch;

fn episode_to_view(
    value: entertainarr_database::model::tvshow_episode::Entity,
) -> entertainarr_api::tvshow_episode::TVShowEpisode {
    entertainarr_api::tvshow_episode::TVShowEpisode {
        id: value.id,
        name: value.name,
        air_date: value.air_date,
        overview: value.overview,
        episode_number: value.episode_number,
        watch: match (value.watch_progress, value.watch_completed) {
            (Some(progress), Some(completed)) => Some(entertainarr_api::tvshow_episode::Watch {
                progress,
                completed,
            }),
            _ => None,
        },
    }
}

pub(super) fn router() -> axum::Router {
    axum::Router::new().route("/", get(list::handle)).route(
        "/{episode_number}/watch",
        post(watch::create).delete(watch::delete),
    )
}
