// I want it, I get it!
#![feature(iterator_fold_self)]

use cucumber_rust::{WorldInit};

mod world;
mod config;
mod menu_steps;

/// Entrypoint for all cucumber feature tests.
///
#[tokio::main]
async fn main() {
    crate::world::BistroWorld::init(&["./tests/features"])
        .run_and_exit()
        .await;
}
