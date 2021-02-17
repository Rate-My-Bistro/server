use serde::Deserialize;
use toml;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct Database {
    pub url: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub collection: String,
}

#[derive(Deserialize, Clone)]
pub struct CucumberConfig {
    pub database: Database,
}

pub fn parse_config() -> CucumberConfig {
    let config_string = fs::read_to_string("./Cucumber.toml")
        .expect("Something went wrong reading the file");

    toml::from_str(&*config_string)
        .expect("File cannot be parsed by into config struct")
}
