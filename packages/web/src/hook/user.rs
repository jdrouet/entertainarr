use std::sync::Arc;

use entertainarr_api::user::User;
use yew::prelude::*;
use yew_hooks::{UseAsyncHandle, UseAsyncOptions, use_async_with_options};

async fn get_me() -> Result<User, Arc<gloo_net::Error>> {
    let res = gloo_net::http::Request::get("/api/users/me")
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

#[hook]
pub fn use_me() -> UseAsyncHandle<User, Arc<gloo_net::Error>> {
    use_async_with_options(get_me(), UseAsyncOptions::enable_auto())
}
