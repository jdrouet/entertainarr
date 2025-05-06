use std::sync::Arc;

use entertainarr_api::{MetaCount, Response, tvshow::TVShow};
use yew::prelude::*;
use yew_hooks::{UseAsyncHandle, use_async, use_async_with_options};

async fn get_followed_tvshows() -> Result<Response<Vec<TVShow>, (), MetaCount>, Arc<gloo_net::Error>>
{
    let res = gloo_net::http::Request::get("/api/tvshows")
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

#[hook]
pub fn use_followed_tvshows()
-> UseAsyncHandle<Response<Vec<TVShow>, (), MetaCount>, Arc<gloo_net::Error>> {
    use_async_with_options(
        get_followed_tvshows(),
        yew_hooks::UseAsyncOptions::enable_auto(),
    )
}

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
