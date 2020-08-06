use actix_web::{get,Responder,HttpResponse};

use crate::service;

#[get("/")]
pub(crate) async fn index() -> impl Responder  {
    format!("Endpoints: /menus")
}

#[get("/menus")]
pub async fn list_menus() -> HttpResponse {
    let menus = service::list_menus().await;
    HttpResponse::Ok().json(menus)
}

