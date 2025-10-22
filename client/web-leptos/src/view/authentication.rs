use entertainarr_client_core::authentication::{AuthenticationEvent, AuthenticationKind};
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

use crate::context::core::use_events;

stylance::import_style!(style, "authentication.module.scss");

#[component]
pub fn AuthenticationView(
    model: entertainarr_client_core::authentication::AuthenticationView,
) -> impl IntoView {
    let (_, on_change) = use_events();

    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let form = event_target::<web_sys::HtmlFormElement>(&ev);

        let mode_input = form
            .get_with_name("mode")
            .expect("mode input")
            .dyn_into::<web_sys::RadioNodeList>()
            .expect("radio element");
        let email_input = form
            .get_with_name("email")
            .expect("email input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .expect("input element");
        let password_input = form
            .get_with_name("password")
            .expect("password input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .expect("input element");

        let kind = match mode_input.value().as_str() {
            "login" => AuthenticationKind::Login,
            "signup" => AuthenticationKind::Signup,
            other => panic!("invalid mode {other:?}"),
        };

        let event = AuthenticationEvent::Execute {
            email: email_input.value(),
            password: password_input.value(),
            kind,
        };
        on_change.set(entertainarr_client_core::Event::Authentication(event));
    };

    view! {
        <div class="content">
            <div class={style::form_container}>
                <h1>{"Welcome to Entertainarr"}</h1>
                <form
                    novalidate
                    on:submit=handle_submit
                >
                    <h2 class={style::toggle_tabs}>
                        <label>
                            <input type="radio" name="mode" value="login" checked />
                            <span>{"Login"}</span>
                        </label>
                        <label>
                            <input type="radio" name="mode" value="signup" />
                            <span>{"Signup"}</span>
                        </label>
                    </h2>
                    <div class={style::form_group}>
                        <label for="email">{"Email"}</label>
                        <input id="email" type="email" name="email" required />
                    </div>
                    <div class={style::form_group}>
                        <label for="password">{"Password"}</label>
                        <input id="password" type="password" name="password" required />
                    </div>
                    {model.error.clone().map(|message| view! {
                        <div class={style::error_message}>
                            {message}
                        </div>
                    })}
                    <button type="submit">
                        {"Continue"}
                    </button>
                </form>
            </div>
        </div>
    }
}
