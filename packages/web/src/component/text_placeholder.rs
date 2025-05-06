use yew::prelude::*;

#[function_component(TextPlaceholder)]
pub fn text_placeholder() -> Html {
    html! {
        <div class="h-[1.5rem] animate-pulse rounded-xl bg-gray-800"></div>
    }
}
