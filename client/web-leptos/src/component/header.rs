use leptos::prelude::*;

stylance::import_style!(style, "header.module.scss");

#[component]
pub fn Header(on_toggle_sidebar: impl Fn() + 'static) -> impl IntoView {
    let profile_picture = "/placeholder-profile.jpg";
    view! {
        <header class={style::header}>
            <div class={style::burger} on:click={move |_| on_toggle_sidebar()}>
                <span></span>
                <span></span>
                <span></span>
            </div>
            <div class={style::profile}>
                <img class={style::profile_picture} src={profile_picture} alt={"Profile"} />
            </div>
        </header>
    }
}
