#[macro_use] extern crate log;

extern crate env_logger;
extern crate bistro_contract;
extern crate actix_web;

mod service;
mod endpoints;

use endpoints::*;
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    info!("Rust Actix Server running... http://localhost:8080/");

    HttpServer::new(|| App::new()
        .service(index)
        .service(list_menus)
        .service(list_menu_ids_by_date)
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
