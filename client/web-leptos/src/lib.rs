use leptos::prelude::*;

use crate::context::core::CoreContext;

mod component;
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
