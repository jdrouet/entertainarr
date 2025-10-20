use entertainarr_client_core::{
    Event,
    authentication::{AuthenticationEvent, AuthenticationKind},
};
use leptos::{html::Input, prelude::*};

#[component]
pub fn AuthenticationView(on_change: WriteSignal<Event>) -> impl IntoView {
    let email_ref = NodeRef::<Input>::new();
    let password_ref = NodeRef::<Input>::new();

    let authenticate_action = move |kind: AuthenticationKind| {
        let email = email_ref.get().expect("email input to exist");
        let password = password_ref.get().expect("password input to exist");

        let email = email.value();
        let password = password.value();

        tracing::info!("AuthenticationEvent::Execute");
        on_change.set(Event::Authentication(AuthenticationEvent::Execute {
            email,
            password,
            kind,
        }));
    };

    view! {
        <form
            on:submit=move |ev| {
                ev.prevent_default();
                authenticate_action(AuthenticationKind::Login);
            }
        >
            <h1>{"Authentication"}</h1>
            <div>
                <input node_ref=email_ref type="email" name="email" required />
                <input node_ref=password_ref type="password" name="password" required />
            </div>
            <div>
                <button
                    type="button"
                    on:click=move |_| { authenticate_action(AuthenticationKind::Login); }
                >
                    {"Login"}
                </button>
                <button
                    type="button"
                    on:click=move |_| { authenticate_action(AuthenticationKind::Signup); }
                >
                    {"Signup"}
                </button>
            </div>
        </form>
    }
}
