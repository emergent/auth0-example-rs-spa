use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct FooterProps {}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let FooterProps {} = props;
    html! {
        <footer class={classes!("bg-light", "p-3", "text-center")}>
            <div class={classes!("logo")} />
            <p>
                { "Sample project provided by " }<a href={"https://auth0.com"}>{"Auth0"}</a>
            </p>
        </footer>
    }
}
