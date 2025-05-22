use yew::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum TextColor {
    #[default]
    Black,
    Gray,
}

impl TextColor {
    pub const fn as_text_class(&self) -> &'static str {
        match self {
            Self::Black => "text-black",
            Self::Gray => "text-gray-600",
        }
    }
}

#[allow(unused)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum TextSize {
    Xxl,
    Xl,
    Lg,
    #[default]
    Md,
    Sm,
    Xs,
}

impl TextSize {
    pub const fn as_text_class(&self) -> &'static str {
        match self {
            Self::Xxl => "text-2xl",
            Self::Xl => "text-xl",
            Self::Lg => "text-lg",
            Self::Md => "text-md",
            Self::Sm => "text-sm",
            Self::Xs => "text-xs",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub tag: Option<&'static str>,
    pub value: String,
    #[prop_or_default]
    pub bold: bool,
    #[prop_or_default]
    pub color: TextColor,
    #[prop_or_default]
    pub size: TextSize,
    #[prop_or_default]
    pub classes: String,
}

#[function_component(Text)]
pub fn text(props: &Props) -> Html {
    let tag = props.tag.unwrap_or("div");
    let mut classes = props.classes.clone();
    if !classes.is_empty() {
        classes.push(' ');
    }
    classes.push_str(props.size.as_text_class());
    if props.bold {
        classes.push_str(" font-bold");
    }
    classes.push(' ');
    classes.push_str(props.color.as_text_class());
    html! {
        <@{tag} class={classes}>
            {&props.value}
        </@>
    }
}
