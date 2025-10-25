use entertainarr_client_core::application::authenticated::podcast::subscribe::PodcastSubscribeModel;
// use js_sys::wasm_bindgen::JsCast;
use leptos::prelude::*;
use web_sys::SubmitEvent;

use crate::component::form::button::Button;
use crate::component::form::form_group::FormGroup;
use crate::component::form::layout::FormLayout;
use crate::component::form::title::Title;
// use crate::context::core::use_events;

#[component]
pub fn View(model: PodcastSubscribeModel) -> impl IntoView {
    // let (_, on_change) = use_events();

    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        // let form = event_target::<web_sys::HtmlFormElement>(&ev);

        // let url_input = form
        //     .get_with_name("url")
        //     .expect("url input")
        //     .dyn_into::<web_sys::HtmlInputElement>()
        //     .expect("input element");

        // on_change.set(req.into());
    };

    view! {
        <FormLayout>
            <Title label={"Subscribe to Podcast"} />
            <form
                novalidate
                on:submit=handle_submit
            >
                <FormGroup>
                    <label for="url">{"Podcast URL"}</label>
                    <input id="url" type="url" name="url" required />
                </FormGroup>
                <Button disabled={model.loading} label="Continue" />
            </form>
        </FormLayout>
    }
}
