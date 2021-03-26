use serde::Deserialize;
use toml;
use std::fs;

/// A database configuration parsed from
/// a file called 'Cucumber.toml' and
/// located in the root of the project
///
/// This part of the configuration is
/// separated by the [database] tag
///
#[derive(Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub collection: String,
}

/// A testing configuration parsed from
/// a file called 'Cucumber.toml' and
/// located in the root of the project
///
/// It combines all sub configurations
/// like e.g. the data base configuration
///
#[derive(Deserialize, Clone)]
pub struct CucumberConfig {
    pub database: DatabaseConfig,
}

/// Creates a testing configuration
/// from a file called 'Cucumber.toml'
/// that is located in the root of this
/// project
///
pub fn parse_config() -> CucumberConfig {
    let config_string = fs::read_to_string("./Cucumber.toml")
        .expect("Something went wrong reading the file");

    toml::from_str(&*config_string)
        .expect("File cannot be parsed by into config struct")
}
