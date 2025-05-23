use axum::routing::{get, post};

mod get_by_number;
mod list;
mod transcode;
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
        file_count: value.file_count,
    }
}

pub(super) fn router() -> axum::Router {
    axum::Router::new()
        .route("/", get(list::handle))
        .route("/{episode_number}", get(get_by_number::handle))
        .route(
            "/{episode_number}/watch",
            post(watch::create).delete(watch::delete),
        )
        .route("/{episode_number}/transcode", post(transcode::handle))
}
