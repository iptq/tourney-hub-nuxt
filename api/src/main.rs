#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate sqlx;

mod config;
mod routes;
mod utils;

use rocket::{
    fairing::AdHoc,
    figment::{
        providers::{Env, Format, Toml},
        Figment,
    },
    State,
};
use rocket_db_pools::Database;
use sqlx::migrate::Migrator;

use crate::config::Config;

static MIGRATOR: Migrator = migrate!();

#[derive(Database)]
#[database("db")]
struct Db(sqlx::SqlitePool);

// TODO: DEBUG ONLY
#[get("/")]
fn hello(config: &State<Config>) -> String { format!("{:?}", config) }

#[launch]
fn rocket() -> _ {
    dotenv::from_filename("../.env").ok();

    let figment = Figment::from(rocket::Config::default())
        .merge(Toml::file("Rocket.toml").nested())
        .merge(Env::prefixed("THUB_").global());

    rocket::custom(figment)
        .attach(AdHoc::config::<Config>())
        .attach(Db::init())
        .mount("/api", {
            use routes::*;
            routes![hello, auth::login, auth::callback]
        })
}
