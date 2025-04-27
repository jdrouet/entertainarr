use crate::Route;

use entertainarr_api::tvshow_season::TVShowSeason;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tvshow_id: u64,
    pub season: TVShowSeason,
}

#[function_component(TVShowSeasonCardlet)]
pub fn tv_show_cardlet(props: &Props) -> Html {
    let season = &props.season;

    let air_date = season
        .air_date
        .map(|date| date.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    html! {
        <Link<Route> to={Route::TvshowSeasonView { tvshow_id: props.tvshow_id, season_number: props.season.season_number }} classes="w-[180px] bg-white rounded-lg shadow-md overflow-hidden hover:shadow-md relative hover:shadow-lg transition">
            if let Some(poster_path) = &season.poster_path {
                <img
                    src={format!("https://image.tmdb.org/t/p/w300{}", poster_path)}
                    alt={season.name.clone()}
                    class="w-full h-[250px] object-cover"
                />
            } else {
                <div class="w-full h-[250px] flex items-center justify-center bg-gray-700 text-white">
                    {"No Image"}
                </div>
            }


            <div class="absolute top-2 right-2 flex flex-row gap-2">
                if season.episode_count > 0 {
                    <div class="bg-blue-600 text-white text-xs font-bold px-2 py-1 rounded-full shadow">
                        if season.episode_count > season.watched_episode_count {
                            { season.episode_count - season.watched_episode_count }
                        } else {
                            {"🗸"}
                        }
                    </div>
                }
            </div>

            <div class="px-3 py-2 h-[60px] flex flex-col justify-center text-center">
                <div class="font-semibold text-sm truncate">{ &season.name }</div>
                <div class="text-gray-800 text-xs">{ air_date }</div>
            </div>
        </Link<Route>>
    }
}
