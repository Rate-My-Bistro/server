use cucumber::async_trait;
use std::{convert::Infallible, cell::RefCell};
use chrono::NaiveDate;

pub struct PersistedMenu {
    pub id: String,
    pub name: String,
    pub date: NaiveDate
}

pub struct MyWorld {
    pub foo: String,
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
            foo: "wat".into(),
            bar: 0,
            some_value: RefCell::new(0),
            menus: vec![]
        })
    }
}
