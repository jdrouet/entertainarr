use yew::prelude::*;

use crate::component::button::{Button, ButtonKind};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<MouseEvent>,
    pub completed: bool,
    pub loading: bool,
}

impl Props {
    fn kind(&self) -> ButtonKind {
        ButtonKind::Default
    }

    fn alt(&self) -> &'static str {
        if self.loading {
            "Loading..."
        } else if self.completed {
            "Mark as unwatched"
        } else {
            "Mark as watched"
        }
    }

    fn label(&self) -> &'static str {
        if self.loading {
            "Loading..."
        } else if self.completed {
            "Mark unwatched"
        } else {
            "Mark watched"
        }
    }
}

#[function_component(WatchButton)]
pub fn watch_button(props: &Props) -> Html {
    html! {
        <Button
            alt={props.alt()}
            disabled={props.loading}
            kind={props.kind()}
            onclick={props.onclick.clone()}
            label={props.label()}
        />
    }
}
