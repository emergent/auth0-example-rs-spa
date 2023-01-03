use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NavProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub navbar: bool,
}

#[function_component(Nav)]
pub fn nav(props: &NavProps) -> Html {
    let NavProps {
        children,
        class,
        navbar,
    } = props;
    html! {
        <ul class={classes!(
            class.clone(),
            if *navbar {"navbar-nav"} else {"nav"})
        }>
            { for children.iter() }
        </ul>
    }
}
