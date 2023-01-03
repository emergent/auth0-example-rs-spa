use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NavItemProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

const PROP_CLASS: &str = "nav-item";

#[function_component]
pub fn NavItem(props: &NavItemProps) -> Html {
    let NavItemProps { children, class } = props;
    html! {
        <li class={classes!(class.clone(), PROP_CLASS)}>
            { for children.iter() }
        </li>
    }
}
