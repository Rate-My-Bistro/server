use chrono::NaiveDate;
use cucumber_rust::{async_trait, World, WorldInit};
use std::{cell::RefCell, convert::Infallible};

use crate::config::{CucumberConfig, parse_config};

/// Simplified representation of a
/// bistro menu
///
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
    pub foo: String,
    pub bar: usize,
    pub some_value: RefCell<u8>,
    pub menus: Vec<PersistedMenu>,
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
            bar: 0,
            foo: "wat".into(),
            config: parse_config(),
            some_value: RefCell::new(0),
            menus: vec![]
        })
    }
}
