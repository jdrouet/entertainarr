use std::borrow::Cow;

use leptos::prelude::*;

stylance::import_style!(style, "layout.module.scss");

#[component]
pub fn FullscreenLayout(
    children: Children,
    #[prop(optional)] classname: Option<&'static str>,
) -> impl IntoView {
    let (sidebar_opened, sidebar_toggle) = signal(false);

    let on_toggle_sidebar = move || sidebar_toggle.update(|prev| *prev = !*prev);

    let cname = classname
        .map(|c| format!("{} {c}", style::container))
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed(style::container));

    view! {
        <div class={cname}>
            <super::header::Header on_toggle_sidebar />
            <super::sidebar::Sidebar
                visible={sidebar_opened}
                on_close={move || sidebar_toggle.set(false)}
            />
            // Main content
            <main class=style::main_content>
                {children()}
            </main>
        </div>
    }
}
