use http::{header::AUTHORIZATION, HeaderMap, HeaderValue};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthorizationCode,
    CsrfToken, RefreshToken, TokenResponse,
};
use reqwest::Client;
use rocket::{
    http::{Cookie, CookieJar, SameSite},
    response::Redirect,
    serde::json::Json,
    State,
};
use rocket_db_pools::Connection;
use rosu_v2::prelude::User;

use crate::{
    config::Config,
    utils::{error::Result, oauth::OAuth2Client},
    Db,
};

#[get("/auth/login")]
pub async fn login(oauth: OAuth2Client) -> Redirect {
    let oauth = oauth.inner();
    let auth = oauth.authorize_url(CsrfToken::new_random);
    let (url, _) = auth.url();
    let url_str = url.as_str().to_owned();
    Redirect::to(url_str)
}

#[get("/auth/callback?<code>")]
pub async fn callback(
    oauth: OAuth2Client,
    code: &str,
    mut pool: Connection<Db>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect> {
    // exchange the code for an access token
    let oauth = oauth.inner();
    let code = AuthorizationCode::new(code.to_owned());
    let token = oauth
        .exchange_code(code)
        .request_async(async_http_client)
        .await?;
    let access_token = token.access_token().secret();
    let refresh_token = token.refresh_token().map(RefreshToken::secret);

    // get some info about the user
    let mut headers = HeaderMap::new();
    headers.append(
        AUTHORIZATION,
        HeaderValue::from_str(&format!(
            "Bearer {}",
            token.access_token().secret()
        ))
        .unwrap(),
    );
    let client = Client::builder().default_headers(headers).build()?;
    let res = client.get("https://osu.ppy.sh/api/v2/me").send().await?;
    let user: User = res.json().await?;
    let statistics = user.statistics.unwrap();

    // check if it's already in the database
    let db_user = match query!(
        r#"
        SELECT username FROM "users"
        WHERE osu_id = ?
    "#,
        user.user_id
    )
    .fetch_one(&mut *pool)
    .await
    {
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
        ).execute(&mut *pool).await?;
    }

    // put it back into the cookies
    cookies.add_private(
        Cookie::build("XSRF-TOKEN", "")
            .same_site(SameSite::Lax)
            .finish(),
    );

    Ok(Redirect::to("/"))
}

#[derive(Debug, Serialize)]
pub struct AuthStatus {
    is_logged_in: bool,
}

#[get("/auth/status")]
pub fn status(cookies: &CookieJar<'_>) -> Json<AuthStatus> {
    Json(AuthStatus { is_logged_in: true })
}
