use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NavLinkProps<T>
where
    T: Routable + 'static,
{
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    pub to: T,
    #[prop_or_default]
    pub active_class_name: AttrValue,
}

const PROP_CLASS: &str = "nav-link";

#[function_component(NavLink)]
pub fn nav_link<T>(props: &NavLinkProps<T>) -> Html
where
    T: Routable + 'static,
{
    let NavLinkProps {
        children,
        class,
        to,
        active_class_name,
    } = props;

    //TODO: dynamic setting for active_class_name

    html! {
        <Link<T> classes={classes!(PROP_CLASS, class.clone(), active_class_name.to_string())} to={to.clone()}>
            { for children.iter() }
        </Link<T>>
    }
}
