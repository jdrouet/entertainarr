use std::sync::Arc;

use entertainarr_api::{tvshow::TVShow, tvshow_season::TVShowSeason};

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

pub async fn follow(tvshow_id: u64) -> Result<(), Arc<gloo_net::Error>> {
    let url = format!("/api/tvshows/{tvshow_id}/follow");
    let res = gloo_net::http::Request::post(url.as_str())
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

pub async fn unfollow(tvshow_id: u64) -> Result<(), Arc<gloo_net::Error>> {
    let url = format!("/api/tvshows/{tvshow_id}/follow");
    let res = gloo_net::http::Request::delete(url.as_str())
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}
