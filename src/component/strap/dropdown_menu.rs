use crate::component::strap::dropdown::DropdownContext;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct DropdownMenuProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(DropdownMenu)]
pub fn dropdown_menu(props: &DropdownMenuProps) -> Html {
    let DropdownMenuProps { children, class } = props;

    let ctx = use_context::<DropdownContext>();
    log::info!("DropdownMenu:{:?}", ctx);

    let is_open = ctx.map(|ctx| ctx.is_open).unwrap_or_default();
    let show = if is_open { "show" } else { "" };
    let classes = classes!(class.clone(), "dropdown-menu", show);

    html! {
        <div class={classes} tabindex="-1" role="menu" aria-hidden={(!is_open).to_string()}>
            { for children.iter() }
        </div>
    }
}
