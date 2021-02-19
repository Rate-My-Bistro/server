#![feature(proc_macro_hygiene, decl_macro)]
#![feature(iterator_fold_self)]
#![warn(missing_docs)]

//!
//! This API server represents the entrypoint for the rate my bistro
//! application. It serves menus which were imported by the
//! [crawler project](https://github.com/Rate-My-Bistro/crawler),
//! so that the app users can interact with these menus, e.g. provide
//! more information (image, rating, comment).
//! In addition, this server is responsible for any user authentication,
//! authorization and profile management.
//!
//! In a further step, this server might get extended to also
//! serve as an API server for the [control center](https://github.com/Rate-My-Bistro/control).

#[macro_use]
extern crate rocket;

/// Todo for later rocket releases:
///     In case we want to switch to a contrib database, we have to enable this feature flag
///     (see more on https://api.rocket.rs/master/rocket_contrib/databases/trait.Poolable.html)
extern crate rocket_contrib;

use config::AppConfig;
use middleware::arango_pool::provide_fairing_pool;
use middleware::api_response::ApiResponse;
use menu::endpoints::{get_all_menus, get_menu_by_id, get_all_menus_by_date_range};
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket_contrib::json;
use rocket::Request;
use std::collections::HashMap;

mod menu;
mod middleware;
mod config;

/// Assembles the rocket server with all relevant middlewares
/// 1. Creates a config and injects it into all handlers that need it
/// 2. Creates a connection pool for arango and injects a connection into all handlers that need it
/// 3. Maps all route handlers to specific routes
/// 4. Registers catchers for unexpected requests
///
#[launch]
fn launch_server() -> rocket::Rocket {
    rocket::ignite()
        .register(catchers![route_not_found])
        .mount("/", routes![handle_index_route])
        .mount(
            "/menus",
            routes![
                get_all_menus,
                get_menu_by_id,
                get_all_menus_by_date_range
            ],
        )
        .attach(AdHoc::config::<AppConfig>())
        .attach(AdHoc::on_attach("arango", move |rocket| provide_fairing_pool(rocket)))
}

/// Catches unknown route requests and sends back
/// a response with server information like all
/// available (public) routes
///
#[catch(404)]
fn route_not_found(req: &Request) -> ApiResponse {
    ApiResponse {
        json: json!({
            "message": "Please find a route that matches your needs. Just don't use this one.",
            "requestedRoute": req.uri().to_string(),
            "routes": get_available_routes()

        }),
        status: Status::NotFound
    }
}

/// Index Route handler that provides information about
/// the API server.
///
/// Each handler will respond with the ApiResponse, that
/// is building a Rest response in the background. The
/// status code of the API response is mapped into an
/// http status code und the JSON is used as the
/// response payload.
///
#[get("/")]
fn handle_index_route() -> ApiResponse {
    ApiResponse {
        json: json!({
            "message": "Welcome to the Bistro API Server",
            "routes": get_available_routes()
        }),
        status: Status::Ok,
    }
}

/// Creates a dictionary of all available routes
/// including their route and query parameters:
///
/// ```
/// RouteRegister { [RouteGroup]: Subroute[] }
/// ```
///
fn get_available_routes() -> HashMap<&'static str, Vec<&'static str>> {
    let mut route_register = HashMap::new();

    route_register.insert("Index", vec!["/"]);
    route_register.insert("Menu", vec!["/menus?from=1991-01-19&to=1991-01-19", "/menus/<id>"]);

    route_register
}
