use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NavLinkProps {}

const PROP_CLASS: &str = "nav-link";

#[function_component]
pub fn NavLink(props: &NavLinkProps) -> Html {
    let NavLinkProps {} = props;
    html! {
        <a class={classes!(PROP_CLASS)}></a>
    }
}
