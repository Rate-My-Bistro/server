#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;

mod menu;

fn main() {
    rocket::ignite()
        .mount("/", routes![test1, test2])
        .mount("/menus", routes![
            menu::get_all_menus,
            menu::get_menu_by_id,
            menu::get_all_menus_by_date_range
        ])
        .launch();
}

#[get("/test1")]
fn test1() -> &'static str {
    "test1"
}

#[get("/test2?<name>")]
fn test2(name: &RawStr) -> String {
    return format!("Hello, {}!", name.as_str());
}
