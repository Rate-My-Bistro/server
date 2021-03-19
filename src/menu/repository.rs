extern crate arangors;

use self::arangors::client::surf::SurfClient;
use self::arangors::ClientError;
use crate::menu::entity::Menu;
use crate::middleware::arango_pool::{ArangoConfig};
use arangors::{AqlQuery, Database};
use time::Date;

/// Queries all menus that exist inside the menu collection
///
pub async fn query_all_menus(db: Database<SurfClient>, config: ArangoConfig) -> Result<Vec<Menu>, ClientError> {
    let aql = AqlQuery::builder()
        .query("FOR menu IN @@collection RETURN menu")
        .bind_var("@collection", config.menu_collection)
        .build();

    db.aql_query(aql).await
}

/// Queries menus that are served within the given time range
/// Returns an empty list if no menu is found
///
pub async fn query_menus_by_range(
    from: Date,
    to: Date,
    db: Database<SurfClient>,
    config: ArangoConfig
) -> Result<Vec<Menu>, ClientError> {
    let aql = AqlQuery::builder()
        .query("FOR menu IN @@collection FILTER menu.date >= @from AND menu.date <= @to RETURN menu")
        .bind_var("@collection", config.menu_collection)
        .bind_var("from", from.to_string())
        .bind_var("to", to.to_string())
        .build();

    db.aql_query(aql).await
}

/// Queries a single menu by its id
/// Forwards a client error, if no menu is found
///
pub async fn query_menu_by_id(id: &str, db: Database<SurfClient>, config: ArangoConfig) -> Result<Menu, ClientError> {
    let collection = db.collection(&config.menu_collection).await.unwrap();
    let response = collection.document(id).await;

    match response {
        Ok(menu) => Ok(menu.document),
        Err(e) => Err(e),
    }
}
