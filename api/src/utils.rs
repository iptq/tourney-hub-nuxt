use std::convert::Infallible;

use sqlx::SqlitePool;
use warp::Filter;

pub fn with_pool(
    db: SqlitePool,
) -> impl Filter<Extract = (SqlitePool,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
