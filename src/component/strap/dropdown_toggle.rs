use super::dropdown::DropdownContext;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct DropdownToggleProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub nav: bool,
    #[prop_or_default]
    pub caret: bool,
    #[prop_or_default]
    pub id: AttrValue,
}

#[function_component]
pub fn DropdownToggle(props: &DropdownToggleProps) -> Html {
    let DropdownToggleProps {
        children,
        class,
        nav,
        caret,
        id,
    } = props;

    let ctx = use_context::<DropdownContext>();
    log::info!("DropdownToggle:{:?}", ctx);
    let is_open = ctx.map(|ctx| ctx.is_open).unwrap_or_default();
    let dropdown_toggle = if *caret { "dropdown-toggle" } else { "" };
    let nav_link = if *nav { "nav-link" } else { "" };
    let classes = classes!(class.clone(), dropdown_toggle, nav_link);

    html! {
        <a id={id} href="#" class={classes} aria-haspopup={"true"} aria-expanded={is_open.to_string()}>
            { for children.iter() }
        </a>
    }
}
