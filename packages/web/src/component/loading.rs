use yew::prelude::*;

#[function_component(Loading)]
pub fn loading() -> Html {
    html! {
        <div class="flex justify-center items-center min-h-[200px]">
            <div class="animate-spin rounded-full h-10 w-10 border-t-2 border-b-2 border-gray-600"></div>
            <span class="ml-4 text-gray-700 font-medium">{"Loading..."}</span>
        </div>
    }
}
