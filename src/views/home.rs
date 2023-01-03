use crate::component::{content::Content, hero::Hero};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HomeProps {}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
    let HomeProps {} = props;
    html! {
        <div>
            <Hero />
            <hr />
            <Content />
        </div>
    }
}
