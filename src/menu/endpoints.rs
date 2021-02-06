use crate::menu::repository::{query_all_menus, query_menu_by_id, query_menus_by_range};
use crate::middleware::api_response::ApiResponse;
use crate::middleware::arango_pool::ArangoPool;
use crate::middleware::date_query::DateQueryParam;
use rocket::http::{RawStr, Status};
use rocket_contrib::json;

#[get("/")]
pub async fn get_all_menus(pool: ArangoPool) -> ApiResponse {
    let menu_result = query_all_menus(pool).await;

    match menu_result {
        Ok(menus) => ApiResponse {
            json: json!(menus),
            status: Status::Ok,
        },
        Err(_) => ApiResponse {
            json: json!({"error": {"short": "Failed to query for menus", "long": "Database operation wasn't successful"}}),
            status: Status::InternalServerError,
        },
    }
}

#[get("/?<from>&<to>")]
pub async fn get_all_menus_by_date_range(
    from: DateQueryParam,
    to: DateQueryParam,
    pool: ArangoPool,
) -> ApiResponse {
    let menu_result = query_menus_by_range(pool, *from, *to).await;

    match menu_result {
        Ok(menus) => ApiResponse {
            json: json!(menus),
            status: Status::Ok,
        },
        Err(_) => ApiResponse {
            json: json!({"error": {"short": "Failed to query for menus", "long": "Given data range could not be used to query menus"}}),
            status: Status::InternalServerError,
        },
    }
}

#[get("/<menu_id>")]
pub async fn get_menu_by_id(menu_id: &RawStr, pool: ArangoPool) -> ApiResponse {
    let menu_result = query_menu_by_id(pool, menu_id).await;

    match menu_result {
        Ok(menu) => ApiResponse {
            json: json!(menu),
            status: Status::Ok,
        },
        Err(_) => ApiResponse {
            json: json!({"error": {"short": "No menu found for id", "long": "Given id is not related to any known menu"}}),
            status: Status::BadRequest,
        },
    }
}
