use cookie::Key;
use oauth2::{ClientId, ClientSecret};

#[derive(Clone)]
pub struct Config {
    pub key: Key,
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
}
