use entertainarr_api::tvshow_episode::TVShowEpisode;
use yew::prelude::*;

use crate::component::button::Button;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub episode: TVShowEpisode,
}

#[function_component(TVShowEpisodeListItem)]
pub fn tv_show_episode_list_item(props: &Props) -> Html {
    let episode = &props.episode;

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

    let on_mark_watched = {
        let episode_id = episode.id;
        Callback::from(move |_| {
            // TODO: implement real mutation call
            web_sys::console::log_1(&format!("Mark episode {} as watched", episode_id).into());
        })
    };

    html! {
        <div class="bg-white rounded shadow p-4 flex flex-col md:flex-row md:justify-between md:items-start">
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
                if !episode.watched() {
                    <Button
                        alt="Mark episode as watched"
                        onclick={on_mark_watched}
                        label="Mark as Watched"
                    />
                }
            </div>
        </div>
    }
}
