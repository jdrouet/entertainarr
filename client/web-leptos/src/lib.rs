use leptos::prelude::*;

use crate::context::core::CoreContext;

mod context;
mod core;
mod service;
mod view;

#[component]
pub fn RootComponent() -> impl IntoView {
    view! {
        <CoreContext>
            <view::RouterView />
        </CoreContext>
    }
}
