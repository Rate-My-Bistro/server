use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub database_username: String,
    pub database_password: String,
    pub database_use_jwt: bool,
    pub database_collection: String
}
