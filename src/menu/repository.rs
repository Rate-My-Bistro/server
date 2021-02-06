extern crate arangors;

use self::arangors::client::surf::SurfClient;
use self::arangors::ClientError;
use crate::menu::entity::Menu;
use crate::middleware::arango_pool::ArangoPool;
use arangors::{AqlQuery, Database};
use chrono::NaiveDate;

async fn get_connection(pool: ArangoPool) -> Database<SurfClient> {
    let client = &*pool.get().await.unwrap();
    let db = client.db("bistro").await.unwrap();

    db
}

pub async fn query_all_menus(pool: ArangoPool) -> Result<Vec<Menu>, ClientError> {
    get_connection(pool)
        .await
        .aql_str("FOR menu IN menus RETURN menu")
        .await
}

pub async fn query_menus_by_range(
    pool: ArangoPool,
    from: NaiveDate,
    to: NaiveDate,
) -> Result<Vec<Menu>, ClientError> {
    let aql = AqlQuery::builder()
        .query(
            "FOR menu IN @@collection FILTER menu.date >= @from AND menu.date <= @to RETURN menu",
        )
        .bind_var("@collection", "menus")
        .bind_var("from", from.to_string())
        .bind_var("to", to.to_string())
        .build();

    get_connection(pool).await.aql_query(aql).await
}

pub async fn query_menu_by_id(pool: ArangoPool, id: &str) -> Result<Menu, ClientError> {
    let db = get_connection(pool).await;
    let collection = db.collection("menus").await.unwrap();
    let response = collection.document(id).await;

    match response {
        Ok(menu) => Ok(menu.document),
        Err(e) => Err(e),
    }
}
