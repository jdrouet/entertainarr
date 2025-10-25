use leptos::prelude::*;

stylance::import_style!(style, "title.module.scss");

#[component]
pub fn Title(label: &'static str) -> impl IntoView {
    view! {
        <h1 class={style::title}>
            {label}
        </h1>
    }
}
