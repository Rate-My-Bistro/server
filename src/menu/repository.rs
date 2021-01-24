extern crate arangors;

use arangors::{Connection, Database, AqlQuery};
use arangors::client::reqwest::ReqwestClient;
use crate::menu::entity::{Menu};
use chrono::NaiveDate;

async fn connect() -> Option<Connection> {
    let conn = Connection::establish_basic_auth("http://localhost:8529", "bistrouser", "bistropassword")
        .await
        .unwrap();

    Some(conn)
}

async fn get_bistro_db() -> Option<Database<ReqwestClient>> {
    let conn = connect().await.unwrap();
    let db = conn.db("bistro").await.unwrap();

    Some(db)
}

pub async fn query_all_menus() -> Option<Vec<Menu>> {
    let db = get_bistro_db().await.unwrap();
    let menus: Vec<Menu> = db
        .aql_str("FOR menu IN menus RETURN menu")
        .await
        .unwrap();

    Some(menus)
}

pub async fn query_menus_by_range(from: NaiveDate, to: NaiveDate) -> Option<Vec<Menu>> {
    let db = get_bistro_db().await.unwrap();

    let aql = AqlQuery::builder()
        .query("FOR menu IN @@collection FILTER menu.date >= @from AND menu.date <= @to RETURN menu")
        .bind_var("@collection", "menus")
        .bind_var("from", from.to_string())
        .bind_var("to", to.to_string())
        .build();

    let menus: Vec<Menu> = db.aql_query(aql).await.unwrap();
    Some(menus)
}
