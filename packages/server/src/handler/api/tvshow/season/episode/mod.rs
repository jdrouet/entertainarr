pub mod list;

impl From<crate::model::tvshow_episode::Entity>
    for entertainarr_api::tvshow_episode::TVShowEpisode
{
    fn from(value: crate::model::tvshow_episode::Entity) -> Self {
        Self {
            id: value.id,
            name: value.name,
            air_date: value.air_date,
            overview: value.overview,
            episode_number: value.episode_number,
        }
    }
}
