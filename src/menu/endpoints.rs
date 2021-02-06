use chrono::NaiveDate;
use tokio::runtime::Runtime;
use rocket::{Response, Request, response};
use rocket::http::{ContentType, RawStr, Status};
use crate::menu::repository::{query_all_menus, query_menus_by_range, query_menu_by_id};
use rocket::response::{Responder};
use rocket::request::{FromFormValue};
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;
use std::ops::Deref;

pub struct NaiveDateForm(NaiveDate);

impl<'v> FromFormValue<'v> for NaiveDateForm {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<NaiveDateForm, &'v RawStr> {
        let decoded = form_value.url_decode().map_err(|_| form_value)?;
        if let Ok(date) = NaiveDate::parse_from_str(&decoded, "%Y-%m-%d") {
            return Ok(NaiveDateForm(date));
        }
        Err(form_value)
    }
}

impl Deref for NaiveDateForm {
    type Target = NaiveDate;
    fn deref(&self) -> &NaiveDate {
        &self.0
    }
}

#[derive(Debug)]
pub struct ApiResponse {
    json: JsonValue,
    status: Status,
}

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[get("/")]
pub async fn get_all_menus() -> ApiResponse {
    let menu_result = query_all_menus().await;

    match menu_result {
        Ok(menus) => ApiResponse {
            json: json!(menus),
            status: Status::Ok,
        },
        Err(_) => ApiResponse {
            json: json!({"error": {"short": "Failed to query for menus", "long": "Database operation wasn't successful"}}),
            status: Status::InternalServerError,
        }
    }
}

#[get("/?<from>&<to>")]
pub fn get_all_menus_by_date_range(from: NaiveDateForm, to: NaiveDateForm) -> ApiResponse {
    let menu_result = Runtime::new().unwrap().block_on(query_menus_by_range(*from, *to));

    match menu_result {
        Ok(menus) => ApiResponse {
            json: json!(menus),
            status: Status::Ok,
        },
        Err(_) => ApiResponse {
            json: json!({"error": {"short": "Failed to query for menus", "long": "Given data range could not be used to query menus"}}),
            status: Status::InternalServerError,
        }
    }
}

#[get("/<menu_id>")]
pub fn get_menu_by_id(menu_id: &RawStr) -> ApiResponse {
    let menu_result = Runtime::new().unwrap().block_on(query_menu_by_id(menu_id));

    match menu_result {
        Ok(menu) => ApiResponse {
            json: json!(menu),
            status: Status::Ok,
        },
        Err(_) =>  ApiResponse {
            json: json!({"error": {"short": "No menu found for id", "long": "Given id is not related to any known menu"}}),
            status: Status::BadRequest,
        }
    }
}

