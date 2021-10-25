use rocket::{
    http::Status,
    response::{self, Responder},
    Request, Response,
};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("oauth error: {0}")]
    OAuth(
        #[from]
        oauth2::RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            oauth2::StandardErrorResponse<
                oauth2::basic::BasicErrorResponseType,
            >,
        >,
    ),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'o> {
        let status = Status::InternalServerError;
        Ok(Response::build().status(status).finalize())
    }
}
