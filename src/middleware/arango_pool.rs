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

/// TODO Catch any error inside here
///
async fn retrieve_db(pool: &Pool<ArangoDBConnectionManager>, database_name: &String, menu_collection_name: &String) -> ArangoDb {
    let client = &*pool.get().await.unwrap();
    let db = client.db(&database_name).await.unwrap();

    ArangoDb { db, config: ArangoConfig { menu_collection: menu_collection_name.clone() } }
}

#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for ArangoDb {
    type Error = ();

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let maybe_config = request.managed_state::<AppConfig>();
        let maybe_pool = request.managed_state::<Pool<ArangoDBConnectionManager>>();

        match (maybe_config, maybe_pool) {
            (Some(config), Some(pool)) => Outcome::Success(
                retrieve_db(pool, &config.database_name, &config.database_menu_collection).await
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
