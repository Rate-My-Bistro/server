#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

/// In case we want to switch to a contrib database, we have to enable this feature flag
///(see more on https://api.rocket.rs/master/rocket_contrib/databases/trait.Poolable.html):
// #[macro_use]
extern crate rocket_contrib;

mod menu;

#[launch]
fn rocket() -> _ {
    rocket::ignite()
        .mount("/", routes![index_handler])
        .mount("/menus", routes![
            menu::endpoints::get_all_menus,
            menu::endpoints::get_menu_by_id,
            menu::endpoints::get_all_menus_by_date_range
        ])
}

#[get("/")]
fn index_handler() -> &'static str {
    "It workz"
}
