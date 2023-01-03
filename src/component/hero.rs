use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HeroProps {}

#[function_component(Hero)]
pub fn hero(props: &HeroProps) -> Html {
    let HeroProps {} = props;
    let logo = "/assets/logo.svg";

    html! {
        <div class={classes!("text-center", "hero", "my-5")}>
        <img class={classes!("mb-3", "app-logo")} src={logo} alt={"React logo"} width={120} />
        <h1 class={classes!("mb-4")}>{ "React.js Sample Project" }</h1>

        <p class={classes!("lead")}>
          { "This is a sample application that demonstrates an authentication flow for
          an SPA, using "}<a href={"https://reactjs.org"}>{"React.js"}</a>
        </p>
      </div>
    }
}
