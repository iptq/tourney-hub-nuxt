pub mod cookies;

use std::convert::Infallible;

use anyhow::Error;
use sqlx::SqlitePool;
use warp::{reject::Reject, Filter};

#[derive(Debug)]
pub struct RejectError(Error);

impl From<Error> for RejectError {
    fn from(err: Error) -> Self { RejectError(err) }
}

impl Reject for RejectError {}

pub fn with_pool(
    db: SqlitePool,
) -> impl Filter<Extract = (SqlitePool,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
