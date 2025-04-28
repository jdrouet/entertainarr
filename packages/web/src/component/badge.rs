#![allow(unused)]

use yew::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeKind {
    Primary,
    Info,
    Warning,
    Danger,
    #[default]
    Default,
}

impl std::fmt::Display for BadgeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.classes())
    }
}

impl BadgeKind {
    const fn classes(&self) -> &'static str {
        match self {
            Self::Primary => "border-green-800 text-green-800 bg-green-100",
            Self::Info => "border-blue-800 text-blue-800 bg-blue-100",
            Self::Warning => "border-orange-800 text-orange-800 bg-orange-100",
            Self::Danger => "border-red-800 text-red-800 bg-red-100",
            Self::Default => "border-gray-800 text-gray-800 bg-gray-200",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeSize {
    #[default]
    Normal,
    Small,
}

impl BadgeSize {
    const fn classes(&self) -> &'static str {
        match self {
            Self::Normal => "px-3 py-1",
            Self::Small => "px-2 py-0.5",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub classes: String,
    #[prop_or_default]
    pub kind: BadgeKind,
    #[prop_or_default]
    pub size: BadgeSize,
    pub label: String,
}

#[function_component(Badge)]
pub fn badge(props: &Props) -> Html {
    let mut classes = format!(
        "border {} {} rounded-full",
        props.kind.classes(),
        props.size.classes()
    );
    if !props.classes.is_empty() {
        classes.push(' ');
        classes.push_str(&props.classes);
    }

    html! {
        <span class={classes}>
            {&props.label}
        </span>
    }
}
