use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct DropdownProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub nav: bool,
    #[prop_or_default]
    pub is_open: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DropdownContext {
    pub is_open: bool,
}

impl DropdownContext {
    pub fn toggle(&self) -> Self {
        Self {
            is_open: !self.is_open,
        }
    }
}

#[function_component]
pub fn Dropdown(props: &DropdownProps) -> Html {
    let DropdownProps {
        children,
        class,
        nav,
        is_open,
    } = props;

    let ctx = use_state(|| DropdownContext { is_open: *is_open });
    let toggle = {
        let ctx = ctx.clone();
        Callback::from(move |_| ctx.set(ctx.toggle()))
    };

    let show = if ctx.is_open { "show" } else { "" };
    let nav_item = if *nav { "nav-item" } else { "" };
    let classes = classes!("dropdown", class.clone(), nav_item, show);

    html! {
        <ContextProvider<DropdownContext> context={(*ctx).clone()}>
            { if *nav {
                html! {
                    <li class={classes} onclick={toggle}>
                        { for children.iter() }
                    </li>
                }
            } else {
                html! {
                    <div class={classes} onclick={toggle}>
                        { for children.iter() }
                    </div>
                }
            }}
        </ContextProvider<DropdownContext>>
    }
}
