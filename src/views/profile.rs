use yew::prelude::*;
use yew_oauth2::oauth2::*;
use yew_oauth2::prelude::OAuth2Context;

#[derive(PartialEq, Properties)]
pub struct ProfileProps {}

#[function_component(Profile)]
pub fn profile(props: &ProfileProps) -> Html {
    let ProfileProps {} = props;
    let auth = use_context::<OAuth2Context>();

    html! {
        if let Some(auth) = auth {
            <h2>{ "Authenticated Context"}</h2>
            { format!("{:#?}", auth) }
        } else {
            { "OAuth2 context not found" }
        }
    }
}
