#[macro_use]
extern crate sqlx;
#[macro_use]
extern crate warp;

#[macro_use]
mod macros;

mod config;
mod routes;
mod utils;

use std::env;

use anyhow::Result;
use cookie::Key;
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl,
};
use sqlx::{migrate::Migrator, sqlite::SqlitePoolOptions, SqlitePool};
use warp::Filter;
use warp_sessions::{
    request::with_session as request_with_session, CookieOptions, MemoryStore,
};

use crate::config::Config;

static MIGRATOR: Migrator = migrate!();

#[derive(Clone)]
pub struct State {
    pool: SqlitePool,
    oauth2_client: BasicClient,
    config: Config,
    session_store: MemoryStore,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::from_filename("../.env").ok();
    stderrlog::new()
        .module("api")
        .module("warp")
        .verbosity(5)
        .init()
        .unwrap();

    let key = hex::decode(env::var("THUB_SECRET_KEY").unwrap())?;
    let config = Config {
        key: Key::derive_from(&key),
        client_id: ClientId::new(env::var("THUB_OSU_CLIENT_ID").unwrap()),
        client_secret: ClientSecret::new(
            env::var("THUB_OSU_CLIENT_SECRET").unwrap(),
        ),
    };

    let pool = SqlitePoolOptions::new().connect("sqlite://test.db").await?;
    MIGRATOR.run(&pool).await?;

    let redirect_uri = RedirectUrl::new(
        env::var("THUB_PUBLIC_URL").unwrap() + "/api/auth/callback",
    )?;
    let oauth2_client = BasicClient::new(
        config.client_id.clone(),
        Some(config.client_secret.clone()),
        AuthUrl::new("https://osu.ppy.sh/oauth/authorize".to_string())?,
        Some(TokenUrl::new("https://osu.ppy.sh/oauth/token".to_string())?),
    )
    .set_redirect_uri(redirect_uri);

    let session_store = MemoryStore::new();

    let state = State {
        config,
        pool,
        oauth2_client,
        session_store,
    };

    let auth = {
        use routes::auth::*;
        warp::path("auth").and(route_any! {
            GET ("login") => login(state.clone()),
            GET ("callback") => callback(state.clone()),
        })
    };

    let api = auth;

    let session_store = MemoryStore::new();
    let session = request_with_session(session_store, Some(CookieOptions {
        cookie_name: "session",
        ..Default::default()
    }));

    warp::serve(
        warp::path("api")
            .and(api.clone())
            .or(api)
            .with(warp::log("api")),
    )
    .run(([127, 0, 0, 1], 3002))
    .await;

    Ok(())
}
