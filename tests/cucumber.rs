use cucumber::async_trait;
use std::{convert::Infallible, cell::RefCell};

pub struct MyWorld {
    // You can use this struct for mutable context in scenarios.
    foo: String,
    bar: usize,
    some_value: RefCell<u8>,
}

impl MyWorld {
    async fn test_async_fn(&mut self) {
        *self.some_value.borrow_mut() = 123u8;
        self.bar = 123;
    }
}

#[async_trait(?Send)]
impl cucumber::World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            foo: "wat".into(),
            bar: 0,
            some_value: RefCell::new(0),
        })
    }
}

mod example_steps {
    use cucumber::{Steps, t};
    use isahc::prelude::*;

    pub fn steps() -> Steps<crate::MyWorld> {
        let mut builder: Steps<crate::MyWorld> = Steps::new();

        builder
            .given_async(
                "a thing",
                t!(|mut world, _step| {
                    world.foo = "elho".into();
                    world.test_async_fn().await;
                    world
                })
            )
            .when_regex_async(
                "something goes (.*)",
                t!(|world, _matches, _step| world),
            )
            .given_async(
                "I fetch the index route",
                t!(|mut world: crate::MyWorld, _step| {
                    let mut response = isahc::get("http://localhost:8001").unwrap();
                    assert_eq!(response.status(), 200);
                    assert_eq!(response.text().unwrap(), "It workz");

                    world.foo = "Some string".to_string();
                    world
                }),
            )
            .when("I consider what I am doing", |mut world, _step| {
                let new_string = format!("{}.", &world.foo);
                world.foo = new_string;
                world
            })
            .then("I am interested in ATDD", |world, _step| {
                assert_eq!(world.foo, "Some string.");
                world
            })
            .then_regex(
                r"^we can (.*) rules with regex$",
                |world, matches, _step| {
                    // And access them as an array
                    assert_eq!(matches[1], "implement");
                    world
                },
            );

        builder
    }
}

#[tokio::main]
async fn main() {
    cucumber::Cucumber::<MyWorld>::new()
        .features(&["./tests/features"])
        .steps(example_steps::steps())
        .cli()
        .run_and_exit()
        .await
}
