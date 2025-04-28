use yew::prelude::*;

use crate::component::button::{Button, ButtonKind};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<MouseEvent>,
    pub following: bool,
    pub loading: bool,
}

impl Props {
    fn kind(&self) -> ButtonKind {
        if self.loading || self.following {
            ButtonKind::Default
        } else {
            ButtonKind::Primary
        }
    }

    fn label(&self) -> &'static str {
        if self.loading {
            "Loading..."
        } else if self.following {
            "Following"
        } else {
            "Follow"
        }
    }
}

#[function_component(FollowButton)]
pub fn follow_button(props: &Props) -> Html {
    html! {
        <Button
            alt={props.label()}
            disabled={props.loading}
            kind={props.kind()}
            onclick={props.onclick.clone()}
            label={props.label()}
        />
    }
}
