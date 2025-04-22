use std::sync::Arc;

use yew::prelude::*;
use yew_hooks::prelude::*;

async fn fetch_tvshows() -> Result<u16, Arc<gloo_net::Error>> {
    gloo_net::http::Request::get("/api/tvshows")
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map(|res| res.status())
        .map_err(Arc::new)
}

#[function_component]
pub fn Home() -> Html {
    let tvshows = use_async_with_options(fetch_tvshows(), UseAsyncOptions::enable_auto());

    html! {
        <>
            <h1>{"Home"}</h1>

            <div>
                {if tvshows.loading {
                    html! { <p>{"LOADING"}</p>}
                } else {
                    html! { <p>{"LOADED"}</p>}
                }}
            </div>
        </>
    }
}
