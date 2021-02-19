use cucumber_rust::{given, when, then};
use restson::{Error, RestClient, RestPath};
use serde::{Deserialize, Serialize};

use crate::world::{BistroWorld, FailureResponse};
use std::collections::HashMap;

#[derive(Serialize,Deserialize,PartialEq,Eq,Debug,Clone)]
struct ApiResponse {
    pub message: String,
    pub routes: HashMap<String, Vec<String>>
}

/// RestClient implementation of the GET /menu/<id> route
///
impl RestPath<&'static str> for ApiResponse {
    fn get_path(param: &'static str) -> Result<String, Error> { Ok(format!("/{}", param)) }
}

#[given("is an up and running api server")]
fn server_running(_: &mut BistroWorld) {
    /* Stubbed step for convenience */
}

#[when("I request an unknown route")]
fn request_unknown_route(world: &mut BistroWorld) {
    let mut client = RestClient::new("http://localhost:8001").unwrap();
    let menu_result: Result<ApiResponse, Error> = client.get("i_am_not_known");

    match menu_result {
        Err(Error::HttpError(status_code, body)) => {
            world.expected_failure = Some(FailureResponse { status_code });
            let api_response: ApiResponse = serde_json::from_str(&body).unwrap();
            world.expected_routes = Some(api_response.routes);
        },
        _ => {}
    }
}

#[when("I request the index route")]
fn request_index_route(world: &mut BistroWorld) {
    let mut client = RestClient::new("http://localhost:8001").unwrap();
    let menu_result: Result<ApiResponse, Error> = client.get("");

    match menu_result {
        Ok(res) => {
            world.expected_routes = Some(res.routes);
        },
        _ => {}
    }
}

#[then("I expect to receive a list of possible routes")]
fn list_of_routes_exists(world: &mut BistroWorld) {
    assert!(world.expected_routes.is_some(), "No routes returned");

    let expected_routes = world.expected_routes.as_ref().unwrap();

    assert!(expected_routes.capacity() > 0, "List of possible routes is empty");
    assert!(expected_routes.keys().find(|key| key.to_string() == String::from("Index")).is_some(), "Index Group is missing");
    assert!(expected_routes.keys().find(|key| key.to_string() == String::from("Menu")).is_some(), "Menu Group is missing");
}
