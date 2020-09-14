use actix_web::{get,Responder,HttpResponse};

use crate::service;
use bistro_contract::date_range::DateRange;
use actix_web::web;

#[get("/")]
pub(crate) async fn index() -> impl Responder  {
    format!("Endpoints: /menus")
}

#[get("/menus")]
pub async fn list_menus() -> HttpResponse {
    let menus = service::list_menus().await;
    HttpResponse::Ok().json(menus)
}

#[get("/menus")]
pub async fn list_menu_ids_by_date(range: web::Query<DateRange>) -> HttpResponse {
    let menu_ids = service::list_menu_ids_by_date_range(format!("{}", range.from), format!("{}", range.to)).await;
    println!("Yep, this wqas the cdorrect obne");
    HttpResponse::Ok().json(menu_ids)
}

