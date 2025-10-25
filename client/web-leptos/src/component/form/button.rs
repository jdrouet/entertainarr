use leptos::prelude::*;

stylance::import_style!(style, "button.module.scss");

#[component]
pub fn Button(label: &'static str, disabled: bool) -> impl IntoView {
    view! {
        <button class={style::button} disabled={disabled} type="submit">
            {label}
        </button>
    }
}
