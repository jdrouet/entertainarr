use yew::prelude::*;
use yew_hooks::{UseAsyncOptions, use_async_with_options};

use crate::component::button::Button;
use crate::component::error_message::ErrorMessage;
use crate::component::follow_button::FollowButton;
use crate::component::header::Header;
use crate::component::loading::Loading;
use crate::component::tvshow_season_cardlet::TVShowSeasonCardlet;
use crate::hook::tvshow::{use_tvshow, use_tvshow_sync};

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub tvshow_id: u64,
}

#[function_component(TVShowView)]
pub fn tvshow_view(props: &Props) -> Html {
    let tvshow = use_tvshow(props.tvshow_id);

    let seasons = use_async_with_options(
        crate::hook::tvshow::list_seasons(props.tvshow_id),
        UseAsyncOptions::enable_auto(),
    );

    let tvshow_follow_loading = tvshow.follow.loading || tvshow.unfollow.loading;

    let on_click_follow = {
        let following = tvshow
            .inner
            .data
            .as_ref()
            .map(|inner| inner.following)
            .unwrap_or(false);
        let follow = tvshow.follow.clone();
        let unfollow = tvshow.unfollow.clone();
        Callback::from(move |_: MouseEvent| {
            if following {
                unfollow.run();
            } else {
                follow.run();
            }
        })
    };

    let tvshow_sync_callback = {
        let tvshow = tvshow.inner.clone();
        let seasons = seasons.clone();
        Callback::from(move |_: ()| {
            tvshow.run();
            seasons.run();
        })
    };

    let tvshow_sync = use_tvshow_sync(props.tvshow_id, tvshow_sync_callback.clone());

    let on_click_refresh = {
        let trigger = tvshow_sync.clone();
        Callback::from(move |_: MouseEvent| {
            trigger.run();
        })
    };

    let on_click_watch = {
        let trigger = tvshow.watch.clone();
        Callback::from(move |_: MouseEvent| {
            trigger.run();
        })
    };

    html! {
        <div class="bg-gray-100 min-h-screen">
            <Header />
            <main class="max-w-6xl mx-auto px-4 py-8">
                if let Some(data) = &tvshow.inner.data {
                    <div class="flex flex-col md:flex-row gap-6">
                        if let Some(path) = data.poster_path.as_ref() {
                            <div class="w-full md:w-1/5">
                                <img
                                    class="max-h-[400px] mx-auto rounded-lg shadow-md"
                                    src={format!("https://image.tmdb.org/t/p/w500{path}")}
                                    alt={data.name.clone()}
                                />
                            </div>
                        }

                        <div class="flex-1">
                            <h1 class="text-3xl font-bold text-gray-900 mb-2">
                                { &data.name }
                                if data.name != data.original_name {
                                    <span class="ml-2 text-sm text-gray-500">
                                        {format!("({})", &data.original_name)}
                                    </span>
                                }
                            </h1>

                            <div class="text-sm text-gray-600 space-y-1 mb-4">
                                <div>{ format!("First Air Date: {}", data.first_air_date.map_or("N/A".to_string(), |d| d.to_string())) }</div>
                                <div>{ format!("Original Language: {}", data.original_language.to_uppercase()) }</div>
                                <div>{ format!("Origin Country: {}", data.origin_country.join(", ")) }</div>
                                <div>{ format!("Vote Average: {:.1} / 10 ({} votes)", data.vote_average, data.vote_count) }</div>
                                <div>{ format!("Popularity: {:.1}", data.popularity) }</div>
                                if data.adult {
                                    <div class="text-red-600 font-semibold">{"Adult Content"}</div>
                                }
                            </div>

                            if data.watch_completed() || data.terminated {
                                <div class="flex gap-2 mb-4 text-xs">
                                    if data.following && data.episode_count > 0 && data.episode_count == data.watched_episode_count {
                                        <span class="bg-green-100 text-green-800 px-2 py-0.5 rounded-full">{"Completed"}</span>
                                    }
                                    if data.terminated {
                                        <span class="bg-red-100 text-red-800 px-2 py-0.5 rounded-full">{"Terminated"}</span>
                                    }
                                </div>
                            }

                            <p class="text-gray-800 leading-relaxed mb-4">
                                { data.overview.clone().unwrap_or_else(|| "No description available.".to_string()) }
                            </p>

                            <div class="flex flex-row gap-2">
                                <FollowButton onclick={on_click_follow} following={data.following} loading={tvshow_follow_loading} />
                                if data.following {
                                    <Button
                                        alt="Refresh TV Show"
                                        disabled={tvshow_sync.loading}
                                        onclick={on_click_refresh}
                                        label={if tvshow_sync.loading {
                                            "Refreshing..."
                                        } else {
                                            "Refresh"
                                        }}
                                    />
                                    <Button alt="Mark all episodes as watched" onclick={on_click_watch} label="Mark all watched" />
                                }
                            </div>
                        </div>
                    </div>
                } else if tvshow.inner.loading {
                    <Loading />
                } else if let Some(err) = &tvshow.inner.error {
                    <div class="text-red-600">{ format!("Error: {}", err) }</div>
                }
                <section class="mt-12">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">{"Seasons"}</h2>
                        if seasons.loading {
                            <Loading />
                        } else if let Some(err) = &seasons.error {
                            <ErrorMessage error={err.to_string()} />
                        } else if let Some(season_list) = &seasons.data {
                            <div class="flex flex-row flex-wrap gap-3">
                                {for season_list.iter().map(|season| {
                                    html! {
                                        <TVShowSeasonCardlet tvshow_id={props.tvshow_id} season={season.clone()} />
                                    }
                                })}
                            </div>
                        }
                </section>
            </main>
        </div>
    }
}
