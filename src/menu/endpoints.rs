use std::io::Cursor;

use chrono::NaiveDate;
use tokio::runtime::Runtime;
use rocket::{Response};
use rocket::http::{ContentType, RawStr, Status};
use crate::menu::repository::query_all_menus;
use crate::menu::entity::Menu;
use rocket::response::Debug;

#[get("/")]
pub fn get_all_menus<'r>() -> Response<'r> {
    let menus = Runtime::new().unwrap().block_on(query_all_menus());

    match menus {
        Some(_) => Response::build()
            .header(ContentType::Plain)
            .sized_body(Cursor::new("It works"))
            .status(Status::Ok)
            .finalize(),
        _ =>  Response::build()
            .status(Status::BadRequest)
            .sized_body(Cursor::new("It fails"))
            .finalize()
    }
}

#[get("/?<from>&<to>")]
pub fn get_all_menus_by_date_range<'r>(from: &RawStr, to: &RawStr) -> Response<'r> {
    if !is_date_string(from) || !is_date_string(to) {
        Response::build()
            .status(Status::BadRequest)
            .sized_body(Cursor::new("'from' or 'to' is not a date string"))
            .finalize()
    } else {
        Response::build()
            .header(ContentType::Plain)
            .sized_body(Cursor::new(format!("{}:{}", from, to)))
            .status(Status::Ok)
            .finalize()
    }
}


#[get("/<menu_id>")]
pub fn get_menu_by_id(menu_id: &RawStr) -> String {
    format!("menu id {}", menu_id.as_str())
}

// return true if the date string has the following format: 2020-09-30
fn is_date_string(date_string: &RawStr) -> bool {
    NaiveDate::parse_from_str(date_string.as_str(), "%Y-%m-%d").is_ok()
}
