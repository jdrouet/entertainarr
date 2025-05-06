use std::sync::Arc;

use entertainarr_api::tvshow_episode::{TVShowEpisode, TVShowEpisodeSmall};
use yew::prelude::*;
use yew_hooks::{UseAsyncHandle, UseAsyncOptions, use_async, use_async_with_options};

async fn list_episodes(
    tvshow_id: u64,
    season_number: u64,
) -> Result<Vec<TVShowEpisode>, Arc<gloo_net::Error>> {
    let url = format!("/api/tvshows/{tvshow_id}/seasons/{season_number}/episodes");
    let res = gloo_net::http::Request::get(url.as_str())
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

#[hook]
pub fn use_tvshow_episodes(
    tvshow_id: u64,
    season_number: u64,
) -> UseAsyncHandle<Vec<TVShowEpisode>, Arc<gloo_net::Error>> {
    use_async_with_options(
        list_episodes(tvshow_id, season_number),
        yew_hooks::UseAsyncOptions::enable_auto(),
    )
}

async fn watch_episode(
    tvshow_id: u64,
    season_number: u64,
    episode_number: u64,
) -> Result<TVShowEpisode, Arc<gloo_net::Error>> {
    let url =
        format!("/api/tvshows/{tvshow_id}/seasons/{season_number}/episodes/{episode_number}/watch");
    let res = gloo_net::http::Request::post(url.as_str())
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

#[hook]
pub fn use_watch_tvshow_episode(
    tvshow_id: u64,
    season_number: u64,
    episode_number: u64,
    callback: Callback<TVShowEpisode>,
) -> UseAsyncHandle<(), Arc<gloo_net::Error>> {
    use_async(async move {
        let episode = watch_episode(tvshow_id, season_number, episode_number).await?;
        callback.emit(episode);
        Ok(())
    })
}

async fn unwatch_episode(
    tvshow_id: u64,
    season_number: u64,
    episode_number: u64,
) -> Result<TVShowEpisode, Arc<gloo_net::Error>> {
    let url =
        format!("/api/tvshows/{tvshow_id}/seasons/{season_number}/episodes/{episode_number}/watch");
    let res = gloo_net::http::Request::delete(url.as_str())
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

#[hook]
pub fn use_unwatch_tvshow_episode(
    tvshow_id: u64,
    season_number: u64,
    episode_number: u64,
    callback: Callback<TVShowEpisode>,
) -> UseAsyncHandle<(), Arc<gloo_net::Error>> {
    use_async(async move {
        let episode = unwatch_episode(tvshow_id, season_number, episode_number).await?;
        callback.emit(episode);
        Ok(())
    })
}

async fn episode_watchlist() -> Result<Vec<TVShowEpisodeSmall>, Arc<gloo_net::Error>> {
    let url = "/api/tvshows/watchlist";
    let res = gloo_net::http::Request::get(url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

#[hook]
pub fn use_episode_watchlist() -> UseAsyncHandle<Vec<TVShowEpisodeSmall>, Arc<gloo_net::Error>> {
    use_async_with_options(episode_watchlist(), UseAsyncOptions::enable_auto())
}
