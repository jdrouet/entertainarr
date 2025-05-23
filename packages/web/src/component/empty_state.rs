use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default] // min-h-[200px]
    pub classes: String,
    pub title: String,
    pub subtitle: String,
}

#[function_component(EmptyState)]
pub fn empty_state(props: &Props) -> Html {
    html! {
        <div class={format!("flex flex-col items-center justify-center text-center text-gray-500 py-4 {}", props.classes)}>
            <svg class="w-16 h-16 mb-4 text-gray-300" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" d="M4 16h16M4 12h8m-8-4h16" />
            </svg>
            <p class="text-lg font-medium">{&props.title}</p>
            <p class="text-sm mt-1">{&props.subtitle}</p>
        </div>
    }
}
