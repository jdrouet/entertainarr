use entertainarr_client_core::application::router::Route;
use leptos::prelude::*;

use crate::context::core::use_events;

stylance::import_style!(style, "sidebar.module.scss");

#[component]
pub fn Sidebar(visible: ReadSignal<bool>, on_close: impl Fn() + 'static) -> impl IntoView {
    let (_, dispatch) = use_events();

    view! {
        <>
            <div
                class={style::overlay}
                data-visible={visible}
                on:click={move |_| on_close()}
            />
            <nav class={style::sidebar} data-visible={visible}>
                <header>
                    {"Entertainarr"}
                </header>
                <section>
                    <a href="#" on:click={move |_| dispatch.set(Route::Home.into())}>{"Home"}</a>
                    <a href="#" on:click={move |_| dispatch.set(Route::PodcastDashboard.into())}>{"Podcasts"}</a>
                </section>
            </nav>
        </>
    }
}
