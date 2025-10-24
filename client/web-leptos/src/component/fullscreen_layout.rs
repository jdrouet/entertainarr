use leptos::prelude::*;

stylance::import_style!(style, "fullscreen_layout.module.scss");

#[component]
pub fn FormLayout(children: Children, classname: &'static str) -> impl IntoView {
    let (sidebar_opened, sidebar_toggle) = signal(false);

    let on_toggle_sidebar = move || sidebar_toggle.update(|prev| *prev = !*prev);

    view! {
        <div class={format!("{} {classname}", style::container)}>
            <crate::component::header::Header on_toggle_sidebar />
            <crate::component::sidebar::Sidebar
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
