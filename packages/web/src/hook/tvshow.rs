use std::sync::Arc;

use entertainarr_api::tvshow::TVShow;

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
