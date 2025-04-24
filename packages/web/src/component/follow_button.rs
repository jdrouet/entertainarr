use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<MouseEvent>,
    pub following: bool,
    pub loading: bool,
}

#[function_component(FollowButton)]
pub fn follow_button(props: &Props) -> Html {
    let classes = if props.loading {
        "text-sm px-4 py-2 rounded bg-gray-300 text-gray-800"
    } else if props.following {
        "text-sm px-4 py-2 rounded bg-green-500 text-white hover:bg-green-600"
    } else {
        "text-sm px-4 py-2 rounded bg-gray-300 text-gray-800 hover:bg-gray-400"
    };
    let label = if props.loading {
        "Loading..."
    } else if props.following {
        "Following"
    } else {
        "Follow"
    };

    html! {
        <button
            class={classes}
            disabled={props.loading}
            onclick={props.onclick.clone()}
        >
            {label}
        </button>
    }
}
