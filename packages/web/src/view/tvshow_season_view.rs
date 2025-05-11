use entertainarr_api::tvshow_episode::TVShowEpisode;
use entertainarr_api::tvshow_season::TVShowSeason;
use yew::prelude::*;

use crate::component::button::Button;
use crate::component::error_message::ErrorMessage;
use crate::component::header::Header;
use crate::component::loading::Loading;
use crate::component::tvshow_episode_list_item::TVShowEpisodeListItem;
use crate::hook::tvshow::*;
use crate::hook::tvshow_episode::*;
use crate::hook::tvshow_season::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tvshow_id: u64,
    pub season_number: u64,
}

#[function_component(TVShowSeasonView)]
pub fn tvshow_season_view(props: &Props) -> Html {
    let tvshow = use_tvshow(props.tvshow_id);
    let season = use_tvshow_season(props.tvshow_id, props.season_number);
    let episodes = use_tvshow_episodes(props.tvshow_id, props.season_number);

    let on_change_episode = {
        let episodes = episodes.clone();
        Callback::from(move |episode: TVShowEpisode| {
            if let Some(ref data) = episodes.data {
                let data = data
                    .iter()
                    .map(|item| {
                        if item.episode_number == episode.episode_number {
                            episode.clone()
                        } else {
                            item.clone()
                        }
                    })
                    .collect::<Vec<_>>();
                episodes.update(data);
            }
        })
    };

    let watch = {
        let season = season.clone();
        let episodes = episodes.clone();

        let callback = Callback::from(move |value: TVShowSeason| {
            season.update(value);
            episodes.run();
        });

        use_watch_tvshow_season(props.tvshow_id, props.season_number, callback)
    };

    let unwatch = {
        let season = season.clone();
        let episodes = episodes.clone();

        let callback = Callback::from(move |value: TVShowSeason| {
            season.update(value);
            episodes.run();
        });

        use_unwatch_tvshow_season(props.tvshow_id, props.season_number, callback)
    };

    let tvshow_name = tvshow.data.as_ref().map(|inner| inner.name.as_str());

    html! {
        <div class="bg-gray-100 min-h-screen">
            <Header />
            <main class="max-w-6xl mx-auto px-4 py-8">
                if let Some(err) = &season.error {
                    <ErrorMessage error={err.to_string()} />
                } else if let Some(season) = &season.data {
                    <div class="flex flex-col md:flex-row gap-6 mb-6">
                        if let Some(path) = season.poster_path.as_ref() {
                            <div class="w-full md:w-1/6">
                                <img
                                    class="max-h-[300px] mx-auto rounded-lg shadow-md"
                                    src={format!("/resources/tmdb/w500{path}")}
                                    alt={season.name.clone()}
                                />
                            </div>
                        }

                        <div class="flex-1">
                            <h1 class="text-3xl font-bold text-gray-900 mb-1">
                                if let Some(name) = tvshow_name {
                                    {name}
                                }
                            </h1>
                            <h2 class="text-2xl font-bold text-gray-800">
                                {season.name.as_str()}
                            </h2>
                            if let Some(date) = season.air_date {
                                <p class="text-sm text-gray-500 mt-1">
                                    { format!("Aired on {}", date.format("%B %d, %Y")) }
                                </p>
                            }
                            if let Some(overview) = &season.overview {
                                <p class="mt-4 text-gray-700">{ overview }</p>
                            }
                            <p class="text-sm text-gray-600 my-4">
                                { format!("{} episodes • {} watched", season.episode_count, season.watched_episode_count) }
                            </p>
                            <div>
                                if season.watch_completed() {
                                    <Button
                                        alt="Mark all episodes as not watched"
                                        label="Unwatched all"
                                        onclick={move |_: MouseEvent| unwatch.run()}
                                    />
                                } else {
                                    <Button
                                        alt="Mark all episodes as watched"
                                        label="Mark all watched"
                                        onclick={move |_: MouseEvent| watch.run()}
                                    />
                                }

                            </div>
                        </div>
                    </div>
                } else {
                    <Loading />
                }

                if let Some(err) = &episodes.error {
                    <ErrorMessage error={err.to_string()} />
                } else if let Some(episodes) = &episodes.data {
                    <div class="space-y-4">
                        { for episodes.iter().map(|episode| {
                            html! {
                                <TVShowEpisodeListItem
                                    tvshow_id={props.tvshow_id}
                                    season_number={props.season_number}
                                    episode={episode.clone()}
                                    onchange={on_change_episode.clone()}
                                />
                            }
                        }) }
                    </div>
                } else {
                    <Loading />
                }
            </main>
        </div>
    }
}
