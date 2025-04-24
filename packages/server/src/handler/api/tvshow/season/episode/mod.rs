pub mod list;

fn episode_to_view(
    value: entertainarr_database::model::tvshow_episode::Entity,
) -> entertainarr_api::tvshow_episode::TVShowEpisode {
    entertainarr_api::tvshow_episode::TVShowEpisode {
        id: value.id,
        name: value.name,
        air_date: value.air_date,
        overview: value.overview,
        episode_number: value.episode_number,
    }
}
