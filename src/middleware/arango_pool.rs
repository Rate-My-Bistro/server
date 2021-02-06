use mobc::Pool;
use mobc_arangors::ArangoDBConnectionManager;
use rocket::request::FromRequest;
use rocket::{request, Request};
use std::ops::Deref;

pub struct ArangoPool(Pool<ArangoDBConnectionManager>);

impl Deref for ArangoPool {
    type Target = Pool<ArangoDBConnectionManager>;
    fn deref(&self) -> &Pool<ArangoDBConnectionManager> {
        &self.0
    }
}

// TODO Move this into global state of request by using a Fairing
#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for ArangoPool {
    type Error = ();

    async fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.local_cache(|| create_arango_pool());

        request::Outcome::Success(ArangoPool(pool.clone()))
    }
}

fn create_arango_pool() -> Pool<ArangoDBConnectionManager> {
    warn!("Creating a new manager...");
    let manager = ArangoDBConnectionManager::new(
        "http://localhost:8529",
        "bistrouser",
        "bistropassword",
        true,
        true,
    );
    let pool = Pool::builder().max_open(20).build(manager);

    pool
}
