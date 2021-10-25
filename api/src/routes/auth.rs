use oauth2::{basic::BasicClient, CsrfToken};
use rocket::{response::Redirect, State};

use crate::{config::Config, utils::OAuth2Client};

#[get("/auth/login")]
pub async fn login(oauth: OAuth2Client) -> Redirect {
    let oauth = oauth.inner();
    let auth = oauth.authorize_url(CsrfToken::new_random);
    let (url, _) = auth.url();
    let url_str = url.as_str().to_owned();
    Redirect::to(url_str)
}

#[get("/auth/callback")]
pub async fn callback() -> Redirect {
    Redirect::to("/")
}
