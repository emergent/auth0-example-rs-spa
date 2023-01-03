use crate::util::content_data::CONTENT_DATA;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ContentProps {}

//TODO: Row, Col
#[function_component]
pub fn Content(props: &ContentProps) -> Html {
    let ContentProps {} = props;
    html! {
        <div class={classes!("next-steps", "my-5")}>
            <h2 className={classes!("my-5", "text-center")}>{ "What can I do next?" }</h2>
            <div class={classes!("d-flex", "justify-content-between", "row")}>
            {CONTENT_DATA.iter().enumerate().map(|(i, data)| {
                html! {
                    <div key={i} class={classes!("col-md-5", "mb-4")}>
                    <h6 class={classes!("mb-3")}>
                        <a href={data.link}>{data.title}</a>
                    </h6>
                    <p>{data.description}</p>
                    </div>
                }
            }).collect::<Html>()}
            </div>
        </div>
    }
}
