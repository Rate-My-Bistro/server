#![feature(iterator_fold_self)]

mod config;
mod world;
mod menu_steps;

mod example_steps {
    use cucumber::{Steps, t};
    use isahc::prelude::*;

    pub fn steps() -> Steps<crate::world::MyWorld> {
        let mut builder: Steps<crate::world::MyWorld> = Steps::new();

        builder
            .when_regex_async(
                "something goes (.*)",
                t!(|world, _matches, _step| world),
            )
            .given_async(
                "I fetch the index route",
                t!(|mut world: crate::world::MyWorld, _step| {
                    let mut response = isahc::get("http://localhost:8001").unwrap();
                    assert_eq!(response.status(), 200);
                    assert_eq!(response.text().unwrap(), "It workz");

                    world
                }),
            )
            .when("I consider what I am doing", |mut world, _step| {
                world
            })
            .then("I am interested in ATDD", |world, _step| {
                assert!(world.menus.len() > 0, "My first assert.");
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
    cucumber::Cucumber::<world::MyWorld>::new()
        .features(&["./tests/features"])
        .steps(example_steps::steps())
        .steps(menu_steps::steps())
        .cli()
        .run_and_exit()
        .await
}
