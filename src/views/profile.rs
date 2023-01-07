use crate::component::strap::container::Container;
use yew::prelude::*;
use yew_oauth2::prelude::OAuth2Context;

#[derive(PartialEq, Properties)]
pub struct ProfileProps {}

#[function_component(Profile)]
pub fn profile(props: &ProfileProps) -> Html {
    let ProfileProps {} = props;
    let auth = use_context::<OAuth2Context>();
    let (picture_url, user_name, email) = match auth {
        Some(ref auth) => (picture_url(&auth), user_name(&auth), email(&auth)),
        _ => (None, None, None),
    };

    html! {
        if let Some(auth) = auth {
            <Container class={classes!("mb-5")}>
                <div class={classes!("align-items-center", "profile-header", "mb-5", "text-center", "text-md-left", "row")}> // Row
                    <div class={classes!("col-md-2")}> // Col
                        <img
                            src={picture_url.unwrap_or_default()}
                            alt="Profile"
                            class={classes!("rounded-circle", "img-fluid", "profile-picure", "mb-3", "mb-md-0")} />
                    </div>
                    <div class={classes!("col-md")}> // Col
                        <h2>{user_name}</h2>
                        <p class={classes!("lead", "text-muted")}>{email}</p>
                    </div>
                </div>
                <div>
                    <pre className="rounded">
                        <code class="hlrs">
                          {format!("{:#?}", auth)}
                        </code>
                    </pre>
                </div>
            </Container>
        } else {
            { "Authenticated context not found" }
        }
    }
}

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

fn email(auth: &OAuth2Context) -> Option<String> {
    if let Some(claims) = auth.claims() {
        return claims.email().map(|x| x.to_owned().to_string());
    }
    None
}
