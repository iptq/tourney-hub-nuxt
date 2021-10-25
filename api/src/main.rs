#[macro_use]
extern crate sqlx;
#[macro_use]
extern crate warp;

#[macro_use]
mod macros;

mod auth;
mod utils;

use std::env;

use anyhow::Result;
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl,
};
use sqlx::{migrate::Migrator, sqlite::SqlitePoolOptions};
use warp::Filter;

static MIGRATOR: Migrator = migrate!();

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::from_filename("../.env").ok();
    stderrlog::new()
        .module("api")
        .module("warp")
        .verbosity(5)
        .init()
        .unwrap();

    let pool = SqlitePoolOptions::new().connect("sqlite://test.db").await?;
    MIGRATOR.run(&pool).await?;

    let client_id = ClientId::new(env::var("THUB_OSU_CLIENT_ID").unwrap());
    let client_secret =
        ClientSecret::new(env::var("THUB_OSU_CLIENT_SECRET").unwrap());
    let redirect_uri = RedirectUrl::new(
        env::var("THUB_PUBLIC_URL").unwrap() + "/auth/callback",
    )?;

    let oauth2_client = BasicClient::new(
        client_id,
        Some(client_secret),
        AuthUrl::new("https://osu.ppy.sh/oauth/authorize".to_string())?,
        Some(TokenUrl::new("https://osu.ppy.sh/oauth/token".to_string())?),
    )
    .set_redirect_uri(redirect_uri);

    let auth = warp::path("auth").and(route_any! {
        GET ("login") => auth::login(oauth2_client.clone()),
        GET ("callback") => auth::callback(oauth2_client.clone(), pool.clone()),
    });

    warp::serve(auth.with(warp::log("api")))
        .run(([127, 0, 0, 1], 3002))
        .await;

    Ok(())
}
