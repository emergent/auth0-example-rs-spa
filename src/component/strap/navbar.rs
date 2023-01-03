use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NavbarProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    pub color: AttrValue,
    pub mode: Mode,
    #[prop_or_default]
    pub expand: Option<AttrValue>,
}

const PROP_CLASS: &str = "navbar";

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let NavbarProps {
        children,
        class,
        color,
        mode,
        expand,
    } = props;

    let color = format!("bg-{}", color);
    let expand = get_expand_class(expand.as_ref());

    html! {
        <nav class={classes!(class.clone(), mode.to_string(), color, expand, PROP_CLASS)}>
            { for children.iter() }
        </nav>
    }
}

#[derive(PartialEq)]
pub enum Mode {
    Light,
    Dark,
}

impl ToString for Mode {
    fn to_string(&self) -> String {
        match self {
            Mode::Light => "navbar-light",
            Mode::Dark => "navbar-dark",
        }
        .into()
    }
}

fn get_expand_class(expand: Option<&AttrValue>) -> Option<String> {
    match expand {
        Some(a) if a == "xs" => Some("navbar-expand".into()),
        Some(a) => Some(format!("navbar-expand-{}", a)),
        _ => None,
    }
}
