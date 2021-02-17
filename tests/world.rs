use cucumber::async_trait;
use std::{convert::Infallible, cell::RefCell};
use chrono::NaiveDate;
use crate::config::{CucumberConfig, parse_config};

pub struct PersistedMenu {
    pub id: String,
    pub name: String,
    pub date: NaiveDate
}

pub struct MyWorld {
    pub config: CucumberConfig,
    pub bar: usize,
    pub some_value: RefCell<u8>,
    pub menus: Vec<PersistedMenu>,
}

// impl MyWorld {
//     async fn test_async_fn(&mut self) {
//         *self.some_value.borrow_mut() = 123u8;
//         self.bar = 123;
//         self.menus = vec![];
//     }
// }

#[async_trait(?Send)]
impl cucumber::World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            bar: 0,
            config: parse_config(),
            some_value: RefCell::new(0),
            menus: vec![]
        })
    }
}
