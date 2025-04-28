use std::sync::Arc;

use entertainarr_api::{
    tvshow::TVShow, tvshow_episode::TVShowEpisode, tvshow_season::TVShowSeason,
};
use yew::prelude::*;
use yew_hooks::{UseAsyncHandle, use_async, use_async_with_options};

pub async fn search(query: String) -> Result<Vec<TVShow>, Arc<gloo_net::Error>> {
    if query.is_empty() {
        return Ok(Vec::new());
    }
    let params = [("q", query)];
    let res = gloo_net::http::Request::get("/api/tvshows/search")
        .query(params)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

pub async fn get_by_id(tvshow_id: u64) -> Result<TVShow, Arc<gloo_net::Error>> {
    let url = format!("/api/tvshows/{tvshow_id}");
    let res = gloo_net::http::Request::get(url.as_str())
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

pub async fn list_seasons(tvshow_id: u64) -> Result<Vec<TVShowSeason>, Arc<gloo_net::Error>> {
    let url = format!("/api/tvshows/{tvshow_id}/seasons");
    let res = gloo_net::http::Request::get(url.as_str())
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

pub async fn follow_tvshow(tvshow_id: u64) -> Result<TVShow, Arc<gloo_net::Error>> {
    let url = format!("/api/tvshows/{tvshow_id}/follow");
    let res = gloo_net::http::Request::post(url.as_str())
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

#[hook]
pub fn use_follow_tvshow(
    tvshow_id: u64,
    callback: Callback<TVShow>,
) -> UseAsyncHandle<(), Arc<gloo_net::Error>> {
    use_async(async move {
        let tvshow = follow_tvshow(tvshow_id).await?;
        callback.emit(tvshow);
        Ok(())
    })
}

pub async fn unfollow_tvshow(tvshow_id: u64) -> Result<TVShow, Arc<gloo_net::Error>> {
    let url = format!("/api/tvshows/{tvshow_id}/follow");
    let res = gloo_net::http::Request::delete(url.as_str())
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

#[hook]
pub fn use_unfollow_tvshow(
    tvshow_id: u64,
    callback: Callback<TVShow>,
) -> UseAsyncHandle<(), Arc<gloo_net::Error>> {
    use_async(async move {
        let tvshow = unfollow_tvshow(tvshow_id).await?;
        callback.emit(tvshow);
        Ok(())
    })
}

pub async fn watch_tvshow(tvshow_id: u64) -> Result<TVShow, Arc<gloo_net::Error>> {
    let url = format!("/api/tvshows/{tvshow_id}/watch");
    let res = gloo_net::http::Request::post(url.as_str())
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

#[hook]
pub fn use_watch_tvshow(
    tvshow_id: u64,
    callback: Callback<TVShow>,
) -> UseAsyncHandle<(), Arc<gloo_net::Error>> {
    use_async(async move {
        let tvshow = watch_tvshow(tvshow_id).await?;
        callback.emit(tvshow);
        Ok(())
    })
}

pub async fn unwatch_tvshow(tvshow_id: u64) -> Result<TVShow, Arc<gloo_net::Error>> {
    let url = format!("/api/tvshows/{tvshow_id}/watch");
    let res = gloo_net::http::Request::delete(url.as_str())
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

#[hook]
pub fn use_unwatch_tvshow(
    tvshow_id: u64,
    callback: Callback<TVShow>,
) -> UseAsyncHandle<(), Arc<gloo_net::Error>> {
    use_async(async move {
        let tvshow = unwatch_tvshow(tvshow_id).await?;
        callback.emit(tvshow);
        Ok(())
    })
}

#[hook]
pub fn use_tvshow(tvshow_id: u64) -> UseAsyncHandle<TVShow, Arc<gloo_net::Error>> {
    use_async_with_options(
        get_by_id(tvshow_id),
        yew_hooks::UseAsyncOptions::enable_auto(),
    )
}

pub async fn get_season(
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

pub async fn list_episodes(
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

pub async fn tvshow_sync(tvshow_id: u64) -> Result<(), Arc<gloo_net::Error>> {
    let url = format!("/api/tvshows/{tvshow_id}/sync");
    let res = gloo_net::http::Request::post(url.as_str())
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    if res.status() >= 200 && res.status() < 300 {
        Ok(())
    } else {
        res.json().await.map_err(Arc::new)
    }
}

#[hook]
pub fn use_tvshow_sync(
    tvshow_id: u64,
    callback: Callback<()>,
) -> UseAsyncHandle<(), Arc<gloo_net::Error>> {
    use_async(async move {
        tvshow_sync(tvshow_id).await?;
        callback.emit(());
        Ok(())
    })
}

pub async fn watch_season(
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

pub async fn unwatch_season(
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
