use std::sync::Arc;

use entertainarr_api::tvshow_season::TVShowSeason;
use yew::prelude::*;
use yew_hooks::{UseAsyncHandle, use_async, use_async_with_options};

async fn list_seasons(tvshow_id: u64) -> Result<Vec<TVShowSeason>, Arc<gloo_net::Error>> {
    let url = format!("/api/tvshows/{tvshow_id}/seasons");
    let res = gloo_net::http::Request::get(url.as_str())
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

#[hook]
pub fn use_tvshow_seasons(
    tvshow_id: u64,
) -> UseAsyncHandle<Vec<TVShowSeason>, Arc<gloo_net::Error>> {
    use_async_with_options(
        list_seasons(tvshow_id),
        yew_hooks::UseAsyncOptions::enable_auto(),
    )
}

async fn get_season(
    tvshow_id: u64,
    season_number: u64,
) -> Result<TVShowSeason, Arc<gloo_net::Error>> {
    let url = format!("/api/tvshows/{tvshow_id}/seasons/{season_number}");
    let res = gloo_net::http::Request::get(url.as_str())
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

#[hook]
pub fn use_tvshow_season(
    tvshow_id: u64,
    season_number: u64,
) -> UseAsyncHandle<TVShowSeason, Arc<gloo_net::Error>> {
    use_async_with_options(
        get_season(tvshow_id, season_number),
        yew_hooks::UseAsyncOptions::enable_auto(),
    )
}

async fn watch_season(
    tvshow_id: u64,
    season_number: u64,
) -> Result<TVShowSeason, Arc<gloo_net::Error>> {
    let url = format!("/api/tvshows/{tvshow_id}/seasons/{season_number}/watch");
    let res = gloo_net::http::Request::post(url.as_str())
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

#[hook]
pub fn use_watch_tvshow_season(
    tvshow_id: u64,
    season_number: u64,
    callback: Callback<TVShowSeason>,
) -> UseAsyncHandle<(), Arc<gloo_net::Error>> {
    use_async(async move {
        let season = watch_season(tvshow_id, season_number).await?;
        callback.emit(season);
        Ok(())
    })
}

async fn unwatch_season(
    tvshow_id: u64,
    season_number: u64,
) -> Result<TVShowSeason, Arc<gloo_net::Error>> {
    let url = format!("/api/tvshows/{tvshow_id}/seasons/{season_number}/watch");
    let res = gloo_net::http::Request::delete(url.as_str())
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

#[hook]
pub fn use_unwatch_tvshow_season(
    tvshow_id: u64,
    season_number: u64,
    callback: Callback<TVShowSeason>,
) -> UseAsyncHandle<(), Arc<gloo_net::Error>> {
    use_async(async move {
        let season = unwatch_season(tvshow_id, season_number).await?;
        callback.emit(season);
        Ok(())
    })
}
