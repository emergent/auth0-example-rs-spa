#[derive(PartialEq, Properties)]
pub struct LoadingProps {}
use yew::{function_component, html, Html, Properties};

#[function_component(Loading)]
pub fn loading(props: &LoadingProps) -> Html {
    let LoadingProps {} = props;
    let loading = "../assets/loading.svg";

    html! {
        <div className="spinner">
            <img src={loading} alt="Loading" />
        </div>
    }
}
