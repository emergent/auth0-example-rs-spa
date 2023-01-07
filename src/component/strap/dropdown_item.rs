use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct DropdownItemProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub header: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn DropdownItem(props: &DropdownItemProps) -> Html {
    let DropdownItemProps {
        children,
        class,
        id,
        header,
        onclick,
    } = props;

    if *header {
        html! {
            <h6 {id} tabindex="-1" class={classes!("dropdown-header", class.clone())}>{ for children.iter() }</h6>
        }
    } else {
        html! {
            <button {id} type="button" tabindex="0" role="menuitem" class={classes!("dropdown-item", class.clone())} onclick={onclick}>
                { for children.iter() }
            </button>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct DropdownItemLinkProps<T>
where
    T: Routable + 'static,
{
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    pub to: T,
}

#[function_component]
pub fn DropdownItemLink<T>(props: &DropdownItemLinkProps<T>) -> Html
where
    T: Routable + 'static,
{
    let DropdownItemLinkProps {
        children,
        class,
        to,
    } = props;

    html! {
        //<Link<T> {id} classes={classes!("dropdown-item", class.clone())} to={to} tabindex="0" role="menuitem">{ for children.iter() }</Link<T>>
         <Link<T> classes={classes!("dropdown-item", class.clone())} to={to.clone()}>{ for children.iter() }</Link<T>>
    }
}
