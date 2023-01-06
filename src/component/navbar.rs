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
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NavBarProps {}

const PROP_CLASS: &str = "nav-container";

//TODO: NavbarToggler
#[function_component(NavBar)]
pub fn navbar(props: &NavBarProps) -> Html {
    let NavBarProps {} = props;
    let is_authenticated = false;

    html! {
        <div class={PROP_CLASS}>
            <Navbar color={"light"} mode={Mode::Light} expand={"md"}>
                <Container>
                    <NavbarBrand class={classes!("logo")} />
                    <NavbarToggler />
                    <Collapse is_open={false} navbar={true}>
                        <Nav class={classes!("mr-auto")} navbar={true}>
                        <NavItem><Link<Route> classes={classes!("nav-link", "router-link-exact-active")} to={Route::Home}>{"Home"}</Link<Route>></NavItem>
                        if is_authenticated {
                            <NavItem><Link<Route> classes={classes!("nav-link")} to={Route::Profile}>{"Profile"}</Link<Route>></NavItem>
                            <NavItem><Link<Route> classes={classes!("nav-link")} to={Route::ExternalApi}>{"External API"}</
                            Link<Route>></NavItem>
                        }
                        </Nav>
                        <Nav class={classes!("d-none", "d-md-block")} navbar={true}>
                            if !is_authenticated {
                                <NavItem>
                                    <Button id="qsLoginBtn" class={classes!("btn-margin")} color={ButtonColor::Primary}>{"Log in"}</Button>
                                </NavItem>
                            }
                        </Nav>
                    </Collapse>
                </Container>
            </Navbar>
        </div>
    }
}
