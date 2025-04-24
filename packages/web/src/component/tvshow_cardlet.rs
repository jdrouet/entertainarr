use crate::Route;

use entertainarr_api::tvshow::TVShow;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub show: TVShow,
}

#[function_component(TVShowCardlet)]
pub fn tv_show_cardlet(props: &Props) -> Html {
    let show = &props.show;

    let poster_url = show
        .poster_path
        .as_ref()
        .map(|path| format!("https://image.tmdb.org/t/p/w185{}", path));

    html! {
        <Link<Route> to={Route::TvshowView { tvshow_id: props.show.id }} classes="flex bg-white shadow-sm rounded-md overflow-hidden hover:shadow-md transition">
            if let Some(url) = poster_url {
                <img
                    src={url}
                    alt={format!("Poster for {}", show.name)}
                    class="w-24 h-auto object-cover"
                />
            } else {
                <div class="w-24 bg-gray-200 flex items-center justify-center text-gray-500 text-sm">
                    {"No Image"}
                </div>
            }

            <div class="p-4 flex-1">
                <h3 class="text-lg font-semibold text-gray-800">{ &show.name }</h3>
                <div class="flex gap-2 mt-1 text-xs">
                    if show.following {
                        <span class="bg-blue-100 text-blue-800 px-2 py-0.5 rounded-full">{"Following"}</span>
                    }
                    if show.completed {
                        <span class="bg-green-100 text-green-800 px-2 py-0.5 rounded-full">{"Completed"}</span>
                    }
                    if show.terminated {
                        <span class="bg-red-100 text-red-800 px-2 py-0.5 rounded-full">{"Terminated"}</span>
                    }
                </div>
                if let Some(overview) = &show.overview {
                    <p class="text-sm text-gray-600 line-clamp-3 mt-1">{ overview }</p>
                }
                <div class="text-xs text-gray-500 mt-2 flex justify-between">
                    <span>{ show.first_air_date.map(|d| d.to_string()).unwrap_or_else(|| "Unknown date".to_string()) }</span>
                    <span>{ format!("⭐ {:.1} ({})", show.vote_average, show.vote_count) }</span>
                </div>
            </div>
        </Link<Route>>
    }
}
