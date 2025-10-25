use std::borrow::Cow;

use leptos::prelude::*;

stylance::import_style!(style, "layout.module.scss");

#[component]
pub fn FormLayout(
    children: Children,
    #[prop(optional)] classname: Option<&'static str>,
) -> impl IntoView {
    let cname = classname
        .map(|c| format!("{} {c}", style::container))
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed(style::container));
    view! {
        <div class={cname}>
            {children()}
        </div>
    }
}
