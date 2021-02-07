use mobc::Pool;
use mobc_arangors::ArangoDBConnectionManager;
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, Rocket};
use std::ops::Deref;
use crate::config::AppConfig;
use rocket::http::Status;

pub struct ArangoPool(Pool<ArangoDBConnectionManager>);

impl Deref for ArangoPool {
    type Target = Pool<ArangoDBConnectionManager>;
    fn deref(&self) -> &Pool<ArangoDBConnectionManager> {
        &self.0
    }
}

#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for ArangoPool {
    type Error = ();

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match request.managed_state::<Pool<ArangoDBConnectionManager>>() {
            Some(pool) => Outcome::Success(ArangoPool(pool.clone())),
            None => {
                error_!("Missing database fairing for arango");
                Outcome::Failure((Status::InternalServerError, ()))
            }
        }
    }
}

pub async fn provide_fairing_pool(rocket: Rocket) -> Result<Rocket, Rocket> {
    info!("📀 Creating arango connection pool");

    let figment = rocket::Config::figment();
    let config: AppConfig = figment.extract().expect("Expected a database configuration");

    let manager = ArangoDBConnectionManager::new(
        &config.database_url,
        &config.database_username,
        &config.database_password,
        config.database_use_jwt,
        true,
    );
    let pool = Pool::builder().max_open(20).build(manager);

    Ok(rocket.manage( pool))
}
