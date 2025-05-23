use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default] // min-h-[200px]
    pub classes: String,
    pub message: String,
}

#[function_component(ErrorMessage)]
pub fn error_message(props: &Props) -> Html {
    html! {
        <div class={format!("flex flex-col items-center justify-center py-4 space-y-3 {}", props.classes)}>
            <h3 class="text-lg text-red-500 font-semibold">{"Oops! Something went wrong."}</h3>
            <p class="text-sm text-red-400 max-w-md">{&props.message}</p>
        </div>
    }
}
