use crate::config::AppConfig;
use crate::menu::repository::{query_all_menus, query_menu_by_id, query_menus_by_range};
use crate::middleware::api_response::ApiResponse;
use crate::middleware::arango_pool::ArangoPool;
use crate::middleware::date_query::{DateRangeQueryParam};

use rocket::http::{RawStr, Status};
use rocket_contrib::json;
use rocket::State;

/// Fetches all menus that exist in database - this handler is used for convenience
///
/// TechDebt: Remove this handler, when server hits production (see issue #TODO)
///
#[get("/?debug=all")]
pub async fn get_all_menus(pool: ArangoPool, config: State<'_, AppConfig>) -> ApiResponse {
    let menu_result = query_all_menus(pool, config.inner()).await;

    match menu_result {
        Ok(menus) => ApiResponse {
            json: json!(menus),
            status: Status::Ok,
        },
        Err(e) => ApiResponse {
            json: json!({"error": {"short": "Failed to query for menus", "long": format!("Database operation wasn't successful: {:?}", e)}}),
            status: Status::InternalServerError,
        },
    }
}

/// Fetches Menus using an inclusive date range
///
/// If no menus are found, then an empty list will be sent back.
/// The handler always expects both a from AND a to param to be set.
///
/// Todo from and to should be combined --> max range == 7 days as restriction
///
#[get("/?<range..>")]
pub async fn get_all_menus_by_date_range(
    range: DateRangeQueryParam,
    pool: ArangoPool,
    config: State<'_, AppConfig>
) -> ApiResponse {
    let menu_result = query_menus_by_range(range.from, range.to, pool, config.inner()).await;

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

/// Fetches a menu by its id
///
/// This identifier represents the _key of the menu set in database
///
#[get("/<menu_id>")]
pub async fn get_menu_by_id(menu_id: &RawStr, pool: ArangoPool, config: State<'_, AppConfig>) -> ApiResponse {
    let menu_result = query_menu_by_id(menu_id, pool, config.inner()).await;

    match menu_result {
        Ok(menu) => ApiResponse {
            json: json!(menu),
            status: Status::Ok,
        },
        Err(_) => ApiResponse {
            json: json!({"error": {"short": format!("No menu found for id '{}'", menu_id), "long": "Given id is not related to any known menu"}}),
            status: Status::NotFound,
        },
    }
}
