use std::sync::Arc;

use entertainarr_api::user::LoginPayload;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::hooks::use_navigator;

use crate::Route;

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

#[derive(Properties, PartialEq)]
pub struct Props {
    pub redirect: Option<Route>,
}

#[function_component(Login)]
pub fn login(props: &Props) -> Html {
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
    let redirect_route = props.redirect.clone().unwrap_or(Route::Home);
    use_effect(move || {
        if login_done {
            navigator.push(&redirect_route);
        }
    });

    html! {
        <div class="flex items-center justify-center min-h-screen bg-gray-100">
            <div class="bg-white p-8 rounded-lg shadow-lg w-full max-w-sm">
                <h1 class="text-2xl font-semibold text-gray-800 mb-6 text-center">{"Login"}</h1>
                <form {onsubmit}>
                    <div class="mb-4">
                        <label for="username" class="block text-gray-700 mb-2">{"Username"}</label>
                        <input
                            id="username"
                            type="text"
                            class="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-400"
                            value={username_value}
                            {onchange}
                            placeholder="Enter your username"
                            required=true
                        />
                    </div>
                    <button
                        type="submit"
                        class="w-full bg-indigo-600 text-white py-2 rounded-md hover:bg-indigo-700 transition duration-200"
                    >
                        {"Sign In"}
                    </button>
                </form>
            </div>
        </div>
    }
}
