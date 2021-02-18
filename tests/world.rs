use chrono::NaiveDate;
use cucumber_rust::{async_trait, World, WorldInit};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible};

use crate::config::{CucumberConfig, parse_config};

/// Simplified representation of a
/// bistro menu
///
#[derive(Serialize,Deserialize,PartialEq,Eq,Debug)]
pub struct PersistedMenu {
    pub id: String,
    pub name: String,
    pub date: NaiveDate
}

/// This World represents the testing context
/// that is passed into each gherkin step
///
#[derive(WorldInit)]
pub struct BistroWorld {
    pub config: CucumberConfig,
    pub expected_menus: Vec<PersistedMenu>,
    pub actual_menus: Vec<PersistedMenu>
}

/// Creates a fresh test context for the
/// Bistro App. This context is used to
/// manage the required state between
/// each gherkin step
///
#[async_trait(?Send)]
impl World for BistroWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            config: parse_config(),
            expected_menus: vec![],
            actual_menus: vec![]
        })
    }
}
