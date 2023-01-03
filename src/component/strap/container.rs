use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ContainerProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

const PROP_CLASS: &str = "container";

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    let ContainerProps { children, class } = props;

    html! {
        <div class={classes!(PROP_CLASS, class.clone())}>
            { for children.iter() }
        </div>
    }
}
