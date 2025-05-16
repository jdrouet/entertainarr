use entertainarr_api::tvshow_episode::TVShowEpisode;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::component::button::Button;
use crate::hook::tvshow_episode::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tvshow_id: u64,
    pub season_number: u64,
    pub episode: TVShowEpisode,
    pub onchange: Callback<TVShowEpisode>,
}

#[function_component(TVShowEpisodeListItem)]
pub fn tv_show_episode_list_item(props: &Props) -> Html {
    let episode = &props.episode;

    let watch = use_watch_tvshow_episode(
        props.tvshow_id,
        props.season_number,
        episode.episode_number,
        props.onchange.clone(),
    );
    let unwatch = use_unwatch_tvshow_episode(
        props.tvshow_id,
        props.season_number,
        episode.episode_number,
        props.onchange.clone(),
    );

    let watch_status = if episode.watched() {
        html! { <span class="text-green-600 font-medium text-sm">{"Watched"}</span> }
    } else {
        html! { <span class="text-gray-500 text-sm">{"Not Watched"}</span> }
    };

    let file_status = if episode.file_count > 0 {
        html! { <span class="text-blue-600 font-medium text-sm">{"Available"}</span> }
    } else {
        html! { <span class="text-red-500 font-medium text-sm">{"Not Available"}</span> }
    };

    let href = Route::TvshowEpisodeView {
        tvshow_id: props.tvshow_id,
        season_number: props.season_number,
        episode_number: props.episode.episode_number,
    };

    html! {
        <Link<Route> to={href} classes="bg-white rounded shadow p-4 flex flex-col md:flex-row md:justify-between md:items-start">
            <div class="flex-1 pr-4">
                <h3 class="text-lg font-semibold text-gray-800">
                    { format!("Episode {}: {}", episode.episode_number, episode.name) }
                </h3>
                {
                    if let Some(date) = &episode.air_date {
                        html! {
                            <p class="text-sm text-gray-500 mt-1">
                                { format!("Aired: {}", date.format("%B %d, %Y")) }
                            </p>
                        }
                    } else {
                        html!()
                    }
                }
                {
                    if let Some(overview) = &episode.overview {
                        html! {
                            <p class="mt-2 text-sm text-gray-700">{ overview }</p>
                        }
                    } else {
                        html!()
                    }
                }
                <div class="mt-2 space-x-4">
                    { watch_status }
                    { file_status }
                </div>
            </div>
            <div class="mt-4 md:mt-0">
                if episode.watched() {
                    <Button
                        alt="Mark episode as not watched"
                        disabled={unwatch.loading}
                        onclick={move |_| unwatch.run()}
                        label="Unwatch"
                    />
                } else {
                    <Button
                        alt="Mark all episodes as watched"
                        disabled={watch.loading}
                        onclick={move |_| watch.run()}
                        label="Mark watched"
                    />
                }
            </div>
        </Link<Route>>
    }
}
