use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NavbarTogglerProps {
    #[prop_or_default]
    pub class: Classes,
}

const PROP_CLASS: &str = "navbar-toggler";

#[function_component(NavbarToggler)]
pub fn navbar_toggler(props: &NavbarTogglerProps) -> Html {
    let NavbarTogglerProps { class } = props;

    html! {
        <div class={classes!(class.clone(), PROP_CLASS)}></div>
    }
}
