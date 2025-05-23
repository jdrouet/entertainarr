use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default] // min-h-[200px]
    pub classes: String,
}

#[function_component(Loading)]
pub fn loading(props: &Props) -> Html {
    html! {
        <div class={format!("flex justify-center items-center gap-2 {}", props.classes)}>
            <svg class="animate-spin h-8 w-8 text-indigo-500" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z"></path>
            </svg>
            <span class="text-gray-500 font-medium">{"Loading..."}</span>
        </div>
    }
}
