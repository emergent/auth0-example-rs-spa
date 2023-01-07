use crate::app::Route;
use crate::component::strap::{
    button::{Button, ButtonColor},
    collapse::Collapse,
    container::Container,
    dropdown::Dropdown,
    dropdown_item::{DropdownItem, DropdownItemLink},
    dropdown_menu::DropdownMenu,
    dropdown_toggle::DropdownToggle,
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

fn picture_url(auth: &OAuth2Context) -> Option<String> {
    if let Some(claims) = auth.claims() {
        return claims
            .picture()
            .and_then(|p| p.get(None))
            .map(|x| x.to_owned().to_string());
    }
    None
}

fn user_name(auth: &OAuth2Context) -> Option<String> {
    if let Some(claims) = auth.claims() {
        return claims
            .name()
            .and_then(|p| p.get(None))
            .map(|x| x.to_owned().to_string());
    }
    None
}

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

    let auth = use_context::<OAuth2Context>();
    let (picture_url, user_name) = match auth {
        Some(auth) => (picture_url(&auth), user_name(&auth)),
        _ => (None, None),
    };

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
                                <Dropdown nav={true} is_open={false}>
                                    <DropdownToggle nav={true} caret={true} id="profileDropDown">
                                        { if let Some(picture) = picture_url {
                                            html! {
                                                <img
                                                    src={picture}
                                                    alt="Profile"
                                                    class={classes!("nav-user-profile", "rounded-circle")}
                                                    width="50"
                                                />
                                            }
                                        } else { html! { "no pics" }}}
                                    </DropdownToggle>
                                    <DropdownMenu>
                                        <DropdownItem header={true}>
                                            { user_name.unwrap_or_default() }
                                        </DropdownItem>
                                        <DropdownItemLink<Route> class={classes!("dropdown-profile")} to={Route::Profile}>
                                            { "Profile" }
                                        </DropdownItemLink<Route>>
                                        <DropdownItem
                                            id="qsLogoutBtn"
                                            onclick={logout}
                                        >
                                            { "Log out" }
                                        </DropdownItem>
                                    </DropdownMenu>
                                </Dropdown>
                            </Authenticated>
                        </Nav>
                    </Collapse>
                </Container>
            </Navbar>
        </div>
    }
}

//<FontAwesomeIcon icon="user" className="mr-3" />
//<FontAwesomeIcon icon="power-off" className="mr-3" />
