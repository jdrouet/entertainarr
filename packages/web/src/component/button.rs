#![allow(unused)]

use yew::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonKind {
    Primary,
    Secondary,
    Info,
    #[default]
    Default,
}

impl std::fmt::Display for ButtonKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.classes())
    }
}

impl ButtonKind {
    const fn classes(&self) -> &'static str {
        match self {
            Self::Primary => "text-white bg-blue-500 hover:bg-blue-700",
            Self::Secondary => "text-white bg-emerald-500 hover:bg-emerald-700",
            Self::Info => "text-white bg-cyan-500 hover:bg-cyan-700",
            Self::Default => "bg-gray-300 hover:bg-gray-400",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    #[default]
    Normal,
    Small,
}

impl ButtonSize {
    const fn classes(&self) -> &'static str {
        match self {
            Self::Normal => "text-sm px-4 py-2",
            Self::Small => "text-xs px-3 py-1",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub alt: String,
    #[prop_or_default]
    pub classes: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub kind: ButtonKind,
    #[prop_or_default]
    pub size: ButtonSize,
    pub label: String,
    pub onclick: Callback<web_sys::MouseEvent>,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let mut classes = format!(
        "{} {} rounded transition",
        props.kind.classes(),
        props.size.classes()
    );
    if !props.classes.is_empty() {
        classes.push(' ');
        classes.push_str(&props.classes);
    }

    html! {
        <button
            alt={props.alt.clone()}
            class={classes}
            disabled={props.disabled}
            onclick={props.onclick.clone()}
        >
            {&props.label}
        </button>
    }
}
