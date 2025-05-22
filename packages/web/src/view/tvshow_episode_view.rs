use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::component::button::Button;
use crate::component::header::Header;
use crate::component::text::{Text, TextColor, TextSize};
use crate::component::text_placeholder::TextPlaceholder;
use crate::hook::tvshow::*;
use crate::hook::tvshow_episode::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tvshow_id: u64,
    pub season_number: u64,
    pub episode_number: u64,
}

#[function_component(TVShowEpisodeView)]
pub fn tvshow_episode_view(props: &Props) -> Html {
    let tvshow = use_tvshow(props.tvshow_id);
    let episode = use_tvshow_episode(props.tvshow_id, props.season_number, props.episode_number);

    html! {
        <div class="bg-gray-100 min-h-screen">
            <Header />
            <main class="max-w-6xl mx-auto p-4">
                <div class="flex flex-row items-center gap-3 mb-4">
                    <Link<Route> to={Route::TvshowView { tvshow_id: props.tvshow_id }} classes="text-sm">{"TV Show"}</Link<Route>>
                    {" / "}
                    <Link<Route> to={Route::TvshowSeasonView { tvshow_id: props.tvshow_id, season_number: props.season_number }} classes="text-sm">{"Season"}</Link<Route>>
                    {" / "}
                    <Text size={TextSize::Sm} value="Episode" />
                </div>
                <video class="w-full" controls={true}>
                    // {episode.data.iter().flat_map(|item| item.files.clone().into_iter()).map(|item| html! {
                    //     <source
                    //         src={format!("/api/storages/tvshows/{}?format=mp4", item.path.to_string_lossy())}
                    //         type="video/mp4"
                    //     />
                    // }).collect::<Html>()}
                </video>
                <div class="flex flex-row items-center justify-between">
                    <div class="flex flex-row items-center gap-4 my-4">
                        <TextPlaceholder bold=true color={TextColor::Black} tag="h1" size={TextSize::Xxl} value={tvshow.data.as_ref().map(|value| format!("{} S{:02}E{:02}", value.name, props.season_number, props.episode_number))} />
                        <TextPlaceholder color={TextColor::Gray} tag="h3" size={TextSize::Md} value={episode.data.as_ref().and_then(|item| item.inner.air_date.map(|date| date.format("%Y-%m-%d").to_string()))} width={Some("w-[10rem]")} />
                    </div>
                    <Button
                        alt="Mark as watched"
                        disabled=true
                        label="Mark as watched"
                        onclick={move |_: MouseEvent| {}}
                    />
                </div>
                <TextPlaceholder tag="div" full_width=true value={episode.data.as_ref().and_then(|v| v.inner.overview.clone())} />
            </main>
        </div>
    }
}
