use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CollapseProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    pub is_open: bool,
    pub navbar: bool,
}

const PROP_CLASS: &str = "navbar-collapse";

#[function_component]
pub fn Collapse(props: &CollapseProps) -> Html {
    let CollapseProps {
        children,
        class,
        is_open: _,
        navbar,
    } = props;
    html! {
        <div class={classes!(class.clone(),navbar.then(||PROP_CLASS))}>
            { for children.iter() }
        </div>
    }
}
