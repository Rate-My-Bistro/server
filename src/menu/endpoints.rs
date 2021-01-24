use std::io::Cursor;

use chrono::NaiveDate;
use tokio::runtime::Runtime;
use rocket::{Response, Request, response};
use rocket::http::{ContentType, RawStr, Status};
use crate::menu::repository::query_all_menus;
use rocket::response::{Responder};
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;


#[derive(Debug)]
pub struct ApiResponse {
    json: JsonValue,
    status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[get("/")]
pub fn get_all_menus() -> ApiResponse {
    let menus = Runtime::new().unwrap().block_on(query_all_menus());

    match menus {
        Some(_) => ApiResponse {
            json: json!(menus),
            status: Status::Ok,
        },
        _ => ApiResponse {
            json: json!({"error": {"short": "Cannot find any menu", "long": "There has to be a menu"}}),
            status: Status::BadRequest,
        }
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

// fn validate_date_string(date_string: &RawStr) -> Result<NaiveDate, ValidationError> {
//     NaiveDate::parse_from_str(date_string.as_str(), "%Y-%m-%d").map_err(ValidationError)
// }

// return true if the date string has the following format: 2020-09-30
fn is_date_string(date_string: &RawStr) -> bool {
    NaiveDate::parse_from_str(date_string.as_str(), "%Y-%m-%d").is_ok()
}
