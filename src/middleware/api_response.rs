use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{response, Request, Response};
use rocket_contrib::json::JsonValue;

/// Generic data model for all API responses
/// It is split into the JSON body and a HTTP
/// status code
///
#[derive(Debug)]
pub struct ApiResponse {
    pub json: JsonValue,
    pub status: Status,
}

/// Implementation of the responder trait for
/// bistro API responses. It always expects
/// an HTTP status code and a JSON body.
///
impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
