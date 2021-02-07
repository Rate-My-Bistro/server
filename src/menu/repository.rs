extern crate arangors;

use self::arangors::client::surf::SurfClient;
use self::arangors::ClientError;
use crate::menu::entity::Menu;
use crate::middleware::arango_pool::ArangoPool;
use arangors::{AqlQuery, Database};
use chrono::NaiveDate;
use crate::config::AppConfig;

async fn get_connection(pool: ArangoPool, config: &AppConfig) -> Database<SurfClient> {
    let client = &*pool.get().await.unwrap();
    let db = client.db(&*config.database_collection).await.unwrap();

    db
}

pub async fn query_all_menus(pool: ArangoPool, config: &AppConfig) -> Result<Vec<Menu>, ClientError> {
    get_connection(pool, config)
        .await
        .aql_str("FOR menu IN menus RETURN menu")
        .await
}

pub async fn query_menus_by_range(
    from: NaiveDate,
    to: NaiveDate,
    pool: ArangoPool,
    config: &AppConfig
) -> Result<Vec<Menu>, ClientError> {
    let aql = AqlQuery::builder()
        .query(
            "FOR menu IN @@collection FILTER menu.date >= @from AND menu.date <= @to RETURN menu",
        )
        .bind_var("@collection", "menus")
        .bind_var("from", from.to_string())
        .bind_var("to", to.to_string())
        .build();

    get_connection(pool, config).await.aql_query(aql).await
}

pub async fn query_menu_by_id(id: &str, pool: ArangoPool, config: &AppConfig) -> Result<Menu, ClientError> {
    let db = get_connection(pool, config).await;
    let collection = db.collection("menus").await.unwrap();
    let response = collection.document(id).await;

    match response {
        Ok(menu) => Ok(menu.document),
        Err(e) => Err(e),
    }
}
