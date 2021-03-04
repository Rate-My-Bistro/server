use mobc::Pool;
use mobc_arangors::ArangoDBConnectionManager;
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, Rocket};
use crate::config::AppConfig;
use rocket::http::Status;
use arangors::client::surf::SurfClient;
use arangors::Database;

/// Pooled arango db connection including
/// an additional config used for different
/// collections and further settings
///
/// A connection can be retrieved in every
/// handler that adds this interface to its
/// method params
///
pub struct ArangoDb {
    pub db: Database<SurfClient>,
    pub config: ArangoConfig
}

#[derive(Clone, Debug)]
pub struct ArangoConfig {
    pub menu_collection: String
}

/// Retrieves a new database connection from pool and select the given database.
/// It aggregates the result with all relevant database configurations.
///
async fn retrieve_db(pool: &Pool<ArangoDBConnectionManager>, database_name: &String, menu_collection_name: &String) -> Result<ArangoDb, &'static str> {
    let client = &*pool.get().await.map_err(|_| "Could not retrieve an arango connection")?;
    let db = client.db(&database_name).await.map_err(|_| "Could not select arango database from connection")?;

    Ok(ArangoDb { db, config: ArangoConfig { menu_collection: menu_collection_name.clone() } })
}

#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for ArangoDb {
    type Error = ();

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let maybe_config = request.managed_state::<AppConfig>();
        let maybe_pool = request.managed_state::<Pool<ArangoDBConnectionManager>>();

        match (maybe_config, maybe_pool) {
            (Some(config), Some(pool)) =>
                retrieve_db(pool, &config.database_name, &config.database_menu_collection)
                .await
                .map_or(
                    Outcome::Failure((Status::InternalServerError, ())),
                    |db| Outcome::Success(db)
                ),

            _ => Outcome::Failure((Status::InternalServerError, ()))
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
