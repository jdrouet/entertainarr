use crate::Route;

use entertainarr_api::tvshow_episode::TVShowEpisodeSmall;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub episode: TVShowEpisodeSmall,
}

#[function_component(TVShowEpisodeCardlet)]
pub fn tv_show_cardlet(props: &Props) -> Html {
    let episode = &props.episode;

    let title = format!(
        "{} S{:02}E{:02}",
        episode.tvshow_name, episode.season_number, episode.episode_number
    );
    let air_date = episode.air_date.format("%Y-%m-%d").to_string();

    let route = Route::TvshowEpisodeView {
        tvshow_id: episode.tvshow_id,
        season_number: episode.season_number,
        episode_number: episode.episode_number,
    };
    html! {
        <Link<Route> to={route} classes="w-full h-[250px] bg-white rounded-lg shadow-md overflow-hidden hover:shadow-md relative hover:shadow-lg transition">
            if let Some(poster_path) = &episode.image_path {
                <img
                    src={format!("https://image.tmdb.org/t/p/w780{}", poster_path)}
                    alt={title.clone()}
                    class="w-full h-[190px] object-cover"
                />
            } else {
                <div class="w-full h-[190px] flex items-center justify-center bg-gray-700 text-white">
                    {"No Image"}
                </div>
            }

            <div class="px-3 py-2 h-[60px] flex flex-col justify-center text-center">
                <div class="font-semibold text-sm truncate">{title}</div>
                <div class="text-gray-800 text-xs">{air_date}</div>
            </div>
        </Link<Route>>
    }
}
