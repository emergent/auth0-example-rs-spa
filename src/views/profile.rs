use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ProfileProps {}

#[function_component(Profile)]
pub fn profile(props: &ProfileProps) -> Html {
    let ProfileProps {} = props;
    html! {
        <div></div>
    }
}
