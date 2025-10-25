use entertainarr_client_core::application::authentication::AuthenticationModel;
use entertainarr_client_core::application::authentication::{
    AuthenticationError, AuthenticationEvent, AuthenticationKind, AuthenticationRequest,
};
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

use crate::component::form::button::Button;
use crate::component::form::error_message::ErrorMessage;
use crate::component::form::form_group::FormGroup;
use crate::component::form::layout::FormLayout;
use crate::component::form::title::Title;
use crate::component::toggle_tabs::{ToggleTabOption, ToggleTabs};
use crate::context::core::use_events;

const TOGGLE_TAB_OPTIONS: [ToggleTabOption; 2] = [
    ToggleTabOption {
        label: "Login",
        value: "login",
    },
    ToggleTabOption {
        label: "Signup",
        value: "signup",
    },
];

fn error_message(err: AuthenticationError) -> &'static str {
    match err {
        AuthenticationError::EmailConflict => "Email address already used",
        AuthenticationError::EmailTooShort => "Email address too short",
        AuthenticationError::PasswordTooShort => "Password too short",
        AuthenticationError::InvalidCredentials => "Invalid credentials",
        AuthenticationError::Network => "Internal error",
    }
}

#[component]
pub fn View(model: AuthenticationModel) -> impl IntoView {
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

        let req = AuthenticationEvent::Submit(AuthenticationRequest {
            email: email_input.value(),
            password: password_input.value(),
            kind,
        });
        on_change.set(req.into());
    };

    view! {
        <FormLayout>
            <Title label={"Welcome to Entertainarr"} />
            <form
                novalidate
                on:submit=handle_submit
            >
                <ToggleTabs name="mode" options={&TOGGLE_TAB_OPTIONS} index={0} />
                <FormGroup>
                    <label for="email">{"Email"}</label>
                    <input id="email" type="email" name="email" required />
                </FormGroup>
                <FormGroup>
                    <label for="password">{"Password"}</label>
                    <input id="password" type="password" name="password" required />
                </FormGroup>
                {model.error.clone().map(|err| view! {
                    <ErrorMessage>
                        {error_message(err)}
                    </ErrorMessage>
                })}
                <Button disabled={model.loading} label="Continue" />
            </form>
        </FormLayout>
    }
}
