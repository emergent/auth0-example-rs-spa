use crate::app::Route;
use crate::component::strap::{
    button::{Button, ButtonColor},
    collapse::Collapse,
    container::Container,
    nav::Nav,
    navbar::{Mode, Navbar},
    navbar_brand::NavbarBrand,
    navbar_toggler::NavbarToggler,
    navitem::NavItem,
};
use yew::prelude::*;
use yew_oauth2::openid::*;
use yew_oauth2::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NavBarProps {}

const PROP_CLASS: &str = "nav-container";

//TODO: NavbarToggler
#[function_component(NavBar)]
pub fn navbar(props: &NavBarProps) -> Html {
    let NavBarProps {} = props;
    let agent = use_auth_agent().expect("Requires OAuth2Context component in parent hierarchy");

    let login = {
        let agent = agent.clone();
        Callback::from(move |_: MouseEvent| {
            log::info!("click event: login");
            if let Err(err) = agent.start_login() {
                log::warn!("Failed to start login: {err}");
            }
        })
    };
    let logout = Callback::from(move |_: MouseEvent| {
        log::info!("click event: logout");
        if let Err(err) = agent.logout() {
            log::warn!("Failed to logout: {err}");
        }
    });

    html! {
        <div class={PROP_CLASS}>
            <Navbar color={"light"} mode={Mode::Light} expand={"md"}>
                <Container>
                    <NavbarBrand class={classes!("logo")} />
                    <NavbarToggler />
                    <Collapse is_open={false} navbar={true}>
                        <Nav class={classes!("mr-auto")} navbar={true}>
                            <NavItem><Link<Route> classes={classes!("nav-link", "router-link-exact-active")} to={Route::Home}>{"Home"}</Link<Route>></NavItem>
                            <Authenticated>
                                <NavItem><Link<Route> classes={classes!("nav-link")} to={Route::Profile}>{"Profile"}</Link<Route>></NavItem>
                                <NavItem><Link<Route> classes={classes!("nav-link")} to={Route::ExternalApi}>{"External API"}</
                                Link<Route>></NavItem>
                            </Authenticated>
                        </Nav>
                        <Nav class={classes!("d-none", "d-md-block")} navbar={true}>
                            <NotAuthenticated>
                                <NavItem>
                                    <Button id="qsLoginBtn" class={classes!("btn-margin")} color={ButtonColor::Primary}
                                    onclick={login}
                                    >
                                    {"Log in"}</Button>
                                </NavItem>
                            </NotAuthenticated>
                            <Authenticated>
                                <NavItem>
                                    <Button id="qsLoginBtn" class={classes!("btn-margin")} color={ButtonColor::Primary}
                                    onclick={logout}
                                    >
                                    {"Log out"}</Button>
                                </NavItem>
                        </Authenticated>
                        </Nav>
                    </Collapse>
                </Container>
            </Navbar>
        </div>
    }
}
