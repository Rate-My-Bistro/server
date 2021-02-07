#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

/// In case we want to switch to a contrib database, we have to enable this feature flag
///(see more on https://api.rocket.rs/master/rocket_contrib/databases/trait.Poolable.html):
// #[macro_use]
extern crate rocket_contrib;

use rocket::fairing::AdHoc;
use crate::config::AppConfig;
use crate::middleware::arango_pool::provide_fairing_pool;

mod menu;
mod middleware;
mod config;

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index_handler])
        .mount(
            "/menus",
            routes![
                menu::endpoints::get_all_menus,
                menu::endpoints::get_menu_by_id,
                menu::endpoints::get_all_menus_by_date_range
            ],
        )
        .attach(AdHoc::config::<AppConfig>())
        .attach(AdHoc::on_attach("arango", move |rocket| provide_fairing_pool(rocket)))
}

#[get("/")]
fn index_handler() -> &'static str {
    "It workz"
}
