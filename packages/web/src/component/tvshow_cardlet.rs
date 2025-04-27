use crate::Route;

use entertainarr_api::tvshow::TVShow;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TVShowCardletProps {
    pub show: TVShow,
}

#[function_component(TVShowCardlet)]
pub fn tv_show_cardlet(props: &TVShowCardletProps) -> Html {
    let TVShowCardletProps { show } = props.clone();

    // Unseen episodes calculation
    let unseen_episodes = show
        .episode_count
        .saturating_sub(show.watched_episode_count);

    // Format first air date nicely
    let first_air_date = show
        .first_air_date
        .map(|date| date.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    // Backdrop image handling
    let backdrop_url = show
        .backdrop_path
        .as_ref()
        .map(|path| format!("https://image.tmdb.org/t/p/w780{}", path));

    html! {
        <Link<Route> to={Route::TvshowView { tvshow_id: props.show.id }} classes="w-full h-[250px] rounded-lg overflow-hidden shadow-md relative hover:shadow-lg transition">
            if let Some(url) = backdrop_url {
                <img src={url} alt={show.name.clone()} class="w-full h-[190px] object-cover" />
            } else {
                <div class="w-full h-[190px] flex items-center justify-center bg-gray-700 text-white">
                    { "No Image" }
                </div>
            }

            <div class="absolute top-2 right-2 flex flex-row gap-2">
                if show.following {
                    if show.episode_count > 0 {
                        <div class="bg-blue-600 text-white text-xs font-bold px-2 py-1 rounded-full shadow">
                            if unseen_episodes > 0 {
                                { unseen_episodes }
                            } else {
                                {"🗸"}
                            }
                        </div>
                    }
                    <div class="bg-white text-xs font-bold px-2 py-1 rounded-full shadow">
                        {"❤️"}
                    </div>
                }
            </div>

            // Bottom info
            <div class="px-3 py-2 h-[60px] flex flex-col justify-center text-center">
                <div class="font-semibold text-sm truncate">{ show.name.clone() }</div>
                <div class="text-gray-800 text-xs">{ first_air_date }</div>
            </div>
        </Link<Route>>
    }
}
