use actix_web::{get,Responder,HttpResponse};
use bistro_contract::date_range::DateRange;
use actix_web::web;

use crate::menu::menu_service;

#[get("/")]
pub(crate) async fn index() -> impl Responder  {
    format!("Endpoints: /menus")
}

async fn list_menu_ids_by_date_range(from: String, to: String) -> HttpResponse {
    let result = menu_service::list_menu_ids_by_date_range(from, to).await;
    match result {
        Some(x) => HttpResponse::Ok().json(x),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

async fn list_menus() -> HttpResponse {
    let result = menu_service::list_menus().await;
    match result {
        Some(x) => HttpResponse::Ok().json(x),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[get("/menus")]
pub async fn list_menu_ids_by_date(web::Query(range): web::Query<DateRange>) -> HttpResponse {
    let from = range.from.as_ref();
    let to = range.to.as_ref();

    match (from, to) {
        (Some(x), Some(y)) => list_menu_ids_by_date_range(x.to_string(), y.to_string()).await,
        _ => list_menus().await
    }
}

