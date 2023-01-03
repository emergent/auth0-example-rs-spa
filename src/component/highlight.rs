use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct HighlightProps {}

#[function_component(Highlight)]
pub fn highlight(props: &HighlightProps) -> Html {
    let HighlightProps {} = props;
    html! {
        <div></div>
    }
}
