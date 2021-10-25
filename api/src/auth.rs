use std::{collections::HashMap, convert::Infallible, str::FromStr};

use anyhow::Error;
use futures::future::TryFutureExt;
use http::{HeaderMap, HeaderValue};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthorizationCode,
    CsrfToken, RefreshToken, TokenResponse,
};
use reqwest::{header::AUTHORIZATION, Client};
use rosu_v2::prelude::User;
use sqlx::SqlitePool;
use warp::{http::Uri, Filter};

use crate::utils::{with_pool, RejectError};

pub fn login(oauth2_client: BasicClient) -> Resp!(E = Infallible) {
    warp::any().map(move || {
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
                // exchange the code for an access token
                let code = AuthorizationCode::new(code);
                let token = client
                    .exchange_code(code)
                    .request_async(async_http_client)
                    .await?;
                let access_token = token.access_token().secret();
                let refresh_token = token.refresh_token().map(RefreshToken::secret);

                // get some basic info about the user
                let mut headers = HeaderMap::new();
                headers.append(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!(
                        "Bearer {}",
                        token.access_token().secret()
                    ))
                    .unwrap(),
                );
                let client =
                    Client::builder().default_headers(headers).build()?;
                let res =
                    client.get("https://osu.ppy.sh/api/v2/me").send().await?;
                let user: User = res.json().await?;
                let statistics = user.statistics.unwrap();

                // check if it's already in the database
                let db_user = match query!(r#"
                    SELECT username FROM "users"
                    WHERE osu_id = ?
                "#, user.user_id).fetch_one(&pool).await {
                    Ok(v) => Ok(Some(v)),
                    Err(sqlx::Error::RowNotFound) => Ok(None),
                    Err(err) => Err(err),
                }?;

                if db_user.is_none() {
                    // store it in the database
                    query!(r#"
                        INSERT INTO "users" (osu_id, username, country_code, rank, pp, access_token, refresh_token)
                        VALUES (?, ?, ?, ?, ?, ?, ?)
                    "#,
                        user.user_id,
                        user.username,
                        user.country_code,
                        statistics.global_rank,
                        statistics.pp,
                        access_token,
                        refresh_token,
                    ).execute(&pool).await?;
                }

                Ok::<_, Error>(warp::redirect::temporary(Uri::from_static("/")))
            }
            .map_err(Error::from)
            .map_err(RejectError::from)
            .map_err(warp::reject::custom)
        },
    )
}
