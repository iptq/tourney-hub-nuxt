use anyhow::{Error, Result};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl,
};
use rocket::{
    http::Status, request::FromRequest, request::Outcome, Request, State,
};

use crate::config::Config;

pub struct OAuth2Client(BasicClient);

impl OAuth2Client {
    pub fn inner(self) -> BasicClient { self.0 }
}

const AUTH_URL: &str = "https://osu.ppy.sh/oauth/authorize";
const TOKEN_URL: &str = "https://osu.ppy.sh/oauth/token";

fn construct_client(config: &Config) -> Result<BasicClient> {
    let auth_url = AuthUrl::new(AUTH_URL.to_owned())?;
    let token_url = TokenUrl::new(TOKEN_URL.to_owned())?;
    let redirect_url =
        RedirectUrl::new(format!("{}/api/auth/callback", config.public_url))?;

    let client_id = ClientId::new(config.osu_client_id.to_string());
    let client_secret = ClientSecret::new(config.osu_client_secret.clone());
    let client = BasicClient::new(
        client_id,
        Some(client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(redirect_url);

    Ok(client)
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for OAuth2Client {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let config: &State<Config> = match <_>::from_request(req).await {
            Outcome::Success(c) => c,
            Outcome::Failure(_) => {
                return Outcome::Failure((
                    Status::InternalServerError,
                    anyhow!("no config found"),
                ))
            }
            Outcome::Forward(f) => return Outcome::Forward(f),
        };

        match construct_client(config.inner()) {
            Ok(c) => Outcome::Success(OAuth2Client(c)),
            Err(e) => Outcome::Failure((Status::InternalServerError, e)),
        }
    }
}
