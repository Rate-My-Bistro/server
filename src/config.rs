use serde::Deserialize;

/// Application wide configuration that
/// allows the server to run on various
/// stages with different settings.
///
#[derive(Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub database_username: String,
    pub database_password: String,
    pub database_use_jwt: bool,
    pub database_name: String,
    pub database_menu_collection: String
}
