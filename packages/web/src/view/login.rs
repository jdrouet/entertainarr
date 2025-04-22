use std::sync::Arc;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::hooks::use_navigator;

use crate::Route;

#[derive(serde::Serialize)]
struct LoginPayload {
    username: String,
}

async fn post_login(username: String) -> Result<u16, Arc<gloo_net::Error>> {
    let payload = LoginPayload { username };

    gloo_net::http::Request::post("/api/users/login")
        .header("Content-Type", "application/json")
        .credentials(web_sys::RequestCredentials::Include)
        .json(&payload)
        .unwrap()
        .send()
        .await
        .map(|res| res.status())
        .map_err(Arc::new)
}

#[function_component]
pub fn Login() -> Html {
    let username = use_state(String::default);
    let username_value = (*username).clone();

    let navigator = use_navigator().unwrap();

    let login = {
        let username_value = username_value.clone();

        use_async(async move { post_login(username_value).await })
    };

    let onchange = {
        let inner_username = username.clone();

        Callback::from(move |e: Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                inner_username.set(input.value());
            }
        })
    };

    let onsubmit = {
        let login = login.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            login.run();
        })
    };

    let login_done = login.data.is_some();
    use_effect(move || {
        if login_done {
            navigator.push(&Route::Home);
        }
    });

    html! {
        <form {onsubmit}>
            <h1>{"Login"}</h1>
            <label for="username">{"Username"}</label>
            <input id="username" name="username" {onchange} placeholder="You username here..." value={username_value} />
            <button type="submit">{"Login"}</button>
        </form>
    }
}
