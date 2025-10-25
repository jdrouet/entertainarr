use leptos::prelude::*;

stylance::import_style!(style, "toggle_tabs.module.scss");

#[derive(Debug)]
pub struct ToggleTabOption {
    pub label: &'static str,
    pub value: &'static str,
}

#[component]
pub fn ToggleTabs(
    name: &'static str,
    options: &'static [ToggleTabOption],
    #[prop(optional)] index: usize,
) -> impl IntoView {
    view! {
        <h2 class={style::toggle_tabs}>
            {options.into_iter().enumerate().map(|(idx, option)| view! {
                <label>
                    <input type="radio" name={name} value={option.value} checked={idx == index} />
                    <span>{option.label}</span>
                </label>
            }).collect_view()}
        </h2>
    }
}
