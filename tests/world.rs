use chrono::NaiveDate;
use cucumber_rust::{async_trait, World, WorldInit};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible};

use crate::config::{CucumberConfig, parse_config};

/// Simplified representation of a list of
/// bistro menus
///
#[derive(Serialize,Deserialize,PartialEq,Eq,Debug,Clone)]
pub struct PersistedMenus(pub Vec<PersistedMenu>);

/// Simplified representation of a
/// bistro menu
///
#[derive(Serialize,Deserialize,PartialEq,Eq,Debug,Clone)]
pub struct PersistedMenu {
    pub id: String,
    pub name: String,
    pub date: NaiveDate
}

/// Query Range for menus
///
#[derive(Serialize,Deserialize,PartialEq,Eq,Debug,Clone)]
pub struct DateRange {
    pub from: NaiveDate,
    pub to: NaiveDate
}

#[derive(Serialize,Deserialize,PartialEq,Eq,Debug,Clone)]
pub struct FailureResponse {
    pub status_code: u16
}

/// This World represents the testing context
/// that is passed into each gherkin step
///
#[derive(WorldInit)]
pub struct BistroWorld {
    pub config: CucumberConfig,
    pub expected_menus: Vec<PersistedMenu>,
    pub actual_menus: Vec<PersistedMenu>,
    pub served_range: Option<DateRange>,
    pub expected_failure: Option<FailureResponse>
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
            actual_menus: vec![],
            expected_failure: None,
            served_range: None
        })
    }
}
