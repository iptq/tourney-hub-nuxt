use std::{collections::HashMap, str::FromStr};

use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthorizationCode,
    CsrfToken,
};
use sqlx::SqlitePool;
use warp::{http::Uri, Filter, Rejection};

use crate::utils::with_pool;

pub fn login(oauth2_client: BasicClient) -> Resp!() {
    path!("login").map(move || {
        let auth = oauth2_client.authorize_url(CsrfToken::new_random);
        let (url, _) = auth.url();
        let uri = Uri::from_str(url.as_str()).unwrap();
        warp::redirect::temporary(uri)
    })
}

pub fn callback(oauth2_client: BasicClient, pool: SqlitePool) -> Resp!() {
    warp::query().and(with_pool(pool)).and_then(
        move |query: HashMap<String, String>, pool| {
            let client = oauth2_client.clone();
            let code = query.get("code").unwrap().to_string();
            async move {
                let code = AuthorizationCode::new(code);
                let state = query.get("state");
                let access_token = client
                    .exchange_code(code)
                    .request_async(async_http_client)
                    .await;
                println!("access token: {:?}", access_token);
                Ok::<_, Rejection>("hellosu")
            }
        },
    )
}
