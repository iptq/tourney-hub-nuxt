#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub osu_client_id: u32,
    pub osu_client_secret: String,
    pub public_url: String,
}
