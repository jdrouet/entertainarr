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

    html! {
        <Link<Route> to={Route::TvshowSeasonView { tvshow_id: props.tvshow_id, season_number: props.season.season_number }} classes="bg-white rounded-lg shadow-md overflow-hidden hover:shadow-md transition">
            <div class="bg-white rounded-lg shadow-md overflow-hidden">
                {
                    if let Some(poster_path) = &season.poster_path {
                        html! {
                            <img
                                src={format!("https://image.tmdb.org/t/p/w300{}", poster_path)}
                                alt={season.name.clone()}
                                class="w-full h-40 object-cover"
                            />
                        }
                    } else {
                        html! {
                            <div class="w-full h-40 bg-gray-300 flex items-center justify-center text-gray-600">
                                {"No Image"}
                            </div>
                        }
                    }
                }
                <div class="px-4 pt-2 pb-3">
                    <h3 class="text-lg font-semibold text-gray-800 mb-1">{ &season.name }</h3>
                    <div class="text-xs text-gray-500">
                        { format!("Air Date: {}", season.air_date.map_or("Unknown".to_string(), |d| d.to_string())) }
                    </div>
                </div>
            </div>
        </Link<Route>>
    }
}
