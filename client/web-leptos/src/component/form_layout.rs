use leptos::prelude::*;

stylance::import_style!(style, "form_layout.module.scss");

#[component]
pub fn FormLayout(children: Children, classname: &'static str) -> impl IntoView {
    view! {
        <div class={format!("{} {classname}", style::container)}>
            {children()}
        </div>
    }
}
