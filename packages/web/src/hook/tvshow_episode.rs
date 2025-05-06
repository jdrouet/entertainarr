use std::sync::Arc;

use entertainarr_api::tvshow_episode::TVShowEpisode;
use yew::prelude::*;
use yew_hooks::{UseAsyncHandle, use_async_with_options};

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
