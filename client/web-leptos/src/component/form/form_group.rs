use leptos::prelude::*;

stylance::import_style!(style, "form_group.module.scss");

#[component]
pub fn FormGroup(children: Children) -> impl IntoView {
    view! {
        <div class={style::form_group}>
            {children()}
        </div>
    }
}
