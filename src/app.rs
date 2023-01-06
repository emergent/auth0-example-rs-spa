use crate::component::{footer::Footer, navbar::NavBar, strap::container::Container};
use crate::views::{external_api::ExternalApi, home::Home, profile::Profile};
use yew::prelude::*;
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

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
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
