use leptos::prelude::*;

stylance::import_style!(style, "error_message.module.scss");

#[component]
pub fn ErrorMessage(children: Children) -> impl IntoView {
    view! {
        <div class={style::error_message}>
            {children()}
        </div>
    }
}
