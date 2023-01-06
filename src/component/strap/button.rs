use yew::prelude::*;

#[derive(Default, PartialEq)]
pub enum ButtonColor {
    Primary,
    #[default]
    Secondary,
}

impl ToString for ButtonColor {
    fn to_string(&self) -> String {
        match self {
            Self::Primary => "btn-primary",
            Self::Secondary => "btn-secondary",
        }
        .into()
    }
}

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    pub id: AttrValue,
    #[prop_or_default]
    pub color: ButtonColor,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let ButtonProps {
        children,
        class,
        id,
        color,
        onclick,
    } = props;

    html! {
        <button
            id={id}
            class={classes!(class.clone(), "btn", color.to_string())}
            onclick={onclick}
        >
            { for children.iter() }
        </button>
    }
}
