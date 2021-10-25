use cookie::{Cookie, CookieJar, Key};
use warp::{Filter, Rejection};

pub fn with_secure_cookie(
    secret_key: Key,
    cookie_name: &str,
) -> impl Filter<Extract = ((),), Error = Rejection> + Clone {
    warp::header::optional("cookie").map(|value: Option<String>| {})
}
