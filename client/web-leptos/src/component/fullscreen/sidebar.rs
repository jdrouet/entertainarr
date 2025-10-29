use leptos::prelude::*;

stylance::import_style!(style, "sidebar.module.scss");

#[component]
pub fn Sidebar(visible: ReadSignal<bool>, on_close: impl Fn() + 'static) -> impl IntoView {
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
                    <a href="#/">{"Home"}</a>
                    <a href="#/podcasts">{"Podcasts"}</a>
                </section>
            </nav>
        </>
    }
}
