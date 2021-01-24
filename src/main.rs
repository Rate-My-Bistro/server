#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod menu;

fn main() {
    rocket::ignite()
        .mount("/", routes![test1])
        .mount("/menus", routes![
            menu::endpoints::get_all_menus,
            menu::endpoints::get_menu_by_id,
            menu::endpoints::get_all_menus_by_date_range
        ])
        .launch();
}

#[get("/")]
fn test1() -> &'static str {
    "It workz"
}
