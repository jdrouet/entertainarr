use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub error: String,
}

#[function_component(ErrorMessage)]
pub fn error_message(props: &Props) -> Html {
    html! {
        <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
            <strong class="font-bold">{"Error: "}</strong>
            <span class="block sm:inline">{ &props.error }</span>
        </div>
    }
}
