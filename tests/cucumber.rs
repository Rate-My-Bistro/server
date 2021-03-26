//!
//! In order to prove the proper working of all requested features,
//! these behavior driven tests cases provide a set of scenarios.
//! These scenarios are using the Gherkin syntax for readability.
//!
//! As the [cucumber project](https://github.com/bbqsrc/cucumber-rust)
//! not only supports the Gherkin syntax, but also comes along with
//! additional convenience methods, it is used as testing framework.
//!
//! Run it by executing ```cargo test --test cucumber```

use cucumber_rust::{WorldInit};

mod world;
mod config;
mod steps;

/// Entrypoint for all cucumber feature tests.
///
#[tokio::main]
async fn main() {
    crate::world::BistroWorld::init(&["./tests/features"])
        .run_and_exit()
        .await;
}
