use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NavbarBrandProps {
    #[prop_or_default]
    pub class: Classes,
}

const PROP_CLASS: &str = "navbar-brand";

#[function_component(NavbarBrand)]
pub fn navbar_brand(props: &NavbarBrandProps) -> Html {
    let NavbarBrandProps { class } = props;

    html! {
         <a class={classes!(class.clone(), PROP_CLASS)}></a>
    }
}
