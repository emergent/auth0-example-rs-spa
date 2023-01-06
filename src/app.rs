use crate::component::{footer::Footer, navbar::NavBar, strap::container::Container};
use crate::views::{external_api::ExternalApi, home::Home, profile::Profile};
use yew::prelude::*;
use yew_oauth2::openid::*;
use yew_oauth2::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/profile")]
    Profile,
    #[at("/external-api")]
    ExternalApi,
}

#[function_component(AppInner)]
pub fn app_inner() -> Html {
    html! {
        <BrowserRouter>
            <Failure>
                <ul>
                    <li><FailureMessage/></li>
                </ul>
            </Failure>

            <div id="app" class={classes!("d-flex","flex-column", "h-100")}>
                <NavBar />
                <Container class={classes!("flex-grow-1", "mt-5")}>
                    <Switch<Route> render={switch} />
                </Container>
                <Footer />
            </div>
        </BrowserRouter>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let config = {
        let client_id = std::option_env!("AUTH0_CLIENT_ID").unwrap_or("a");
        let domain = std::option_env!("AUTH0_DOMAIN").unwrap_or("b");
        let issuer_url = format!("https://{}/", domain);

        log::info!("CLIENT_ID: {}, DOMAIN: {}", client_id, domain);

        Config {
            client_id: client_id.into(),
            issuer_url: issuer_url.clone(),
            additional: Additional {
                end_session_url: Some(format!("{}v2/logout?client_id={}", &issuer_url, client_id)),
                after_logout_url: Some("/".into()),
                post_logout_redirect_name: Some("returnTo".into()),
                ..Default::default()
            },
        }
    };
    let scopes: Vec<String> = vec!["openid".into(), "profile".into()];

    html! {
        <OAuth2 {config} scopes={scopes}>
            <AppInner />
        </OAuth2>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Profile => {
            html! { <Profile /> }
        }
        Route::ExternalApi => {
            html! { <ExternalApi /> }
        }
    }
}
