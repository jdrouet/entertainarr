use yew::prelude::*;

use crate::component::text::{Text, TextSize};

use super::text::TextColor;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub classes: String,
    #[prop_or_default]
    pub color: TextColor,
    #[prop_or_default]
    pub bold: bool,
    #[prop_or_default]
    pub tag: Option<&'static str>,
    #[prop_or_default]
    pub full_width: bool,
    #[prop_or_default]
    pub size: TextSize,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub width: Option<&'static str>,
}

fn placeholder_height(size: TextSize) -> &'static str {
    match size {
        TextSize::Xxl => "h-[2rem]",
        TextSize::Xl | TextSize::Lg => "h-[1.75rem]",
        TextSize::Md => "h-[1.5rem]",
        _ => "h-[1rem]",
    }
}

fn placeholder_round(size: TextSize) -> &'static str {
    match size {
        TextSize::Xxl => "rounded-xl",
        TextSize::Xl => "rounded-lg",
        _ => "rounded-md",
    }
}

#[function_component(TextPlaceholder)]
pub fn text_placeholder(props: &Props) -> Html {
    if let Some(ref value) = props.value {
        html! {
            <Text classes={props.classes.clone()} color={props.color} bold={props.bold} tag={props.tag} size={props.size} value={value.clone()} />
        }
    } else {
        let width = if props.full_width {
            "w-full"
        } else {
            props.width.unwrap_or("w-[240px]")
        };
        let classes = format!(
            "{} {} {width} animate-pulse {} bg-gray-400",
            props.classes,
            placeholder_height(props.size),
            placeholder_round(props.size)
        );
        html! {
            <div class={classes} />
        }
    }
}
