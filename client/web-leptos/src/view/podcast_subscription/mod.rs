use js_sys::wasm_bindgen::JsCast;
use leptos::prelude::*;
use web_sys::SubmitEvent;

use crate::context::core::use_events;

stylance::import_style!(style, "style.scss");

#[component]
pub fn View(model: entertainarr_client_core::domain::podcast_subscription::View) -> impl IntoView {
    let (_, on_change) = use_events();

    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let form = event_target::<web_sys::HtmlFormElement>(&ev);

        let url_input = form
            .get_with_name("url")
            .expect("url input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .expect("input element");

        // on_change.set(req.into());
    };

    view! {
        <crate::component::form_layout::FormLayout classname={style::podcast_subscription}>
            <h1>{"Subscribe to Podcast"}</h1>
            <form
                novalidate
                on:submit=handle_submit
            >
                <div class={style::form_group}>
                    <label for="url">{"Podcast URL"}</label>
                    <input id="url" type="url" name="url" required />
                </div>
                <button type="submit">
                    {"Continue"}
                </button>
            </form>
        </crate::component::form_layout::FormLayout>
    }
}
