use mobc::Pool;
use mobc_arangors::ArangoDBConnectionManager;
use rocket::request::FromRequest;
use rocket::{request, Request};
use std::ops::Deref;
use serde::Deserialize;

pub struct ArangoPool(Pool<ArangoDBConnectionManager>);

#[derive(Deserialize)]
struct DatabaseConfig {
    database_url: String,
    database_username: String,
    database_password: String,
    database_use_jwt: bool
}

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

    let figment = rocket::Config::figment();
    let config: DatabaseConfig = figment.extract().expect("Expected a database configuration");

    let manager = ArangoDBConnectionManager::new(
        &config.database_url,
        &config.database_username,
        &config.database_password,
        config.database_use_jwt,
        true,
    );
    let pool = Pool::builder().max_open(20).build(manager);

    pool
}
