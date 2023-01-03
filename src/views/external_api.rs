use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ExternalApiProps {}

#[function_component(ExternalApi)]
pub fn external_api(props: &ExternalApiProps) -> Html {
    let ExternalApiProps {} = props;
    html! {
        <div></div>
    }
}
