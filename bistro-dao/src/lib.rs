extern crate bistro_contract;
extern crate arangors;

use arangors::{Connection, Database};
use arangors::client::reqwest::ReqwestClient;
use bistro_contract::menu::Menu;

pub async fn connect() -> Option<Connection> {
    let conn = Connection::establish_basic_auth("http://localhost:8529", "bistrouser", "bistropassword")
        .await
        .unwrap();

    Some(conn)
}

pub async fn get_bistro_db() -> Option<Database<ReqwestClient>> {
    let conn = connect().await.unwrap();
    let db = conn.db("bistro").await.unwrap();

    Some(db)
}

pub async fn get_menus() -> Option<Vec<Menu>> {
    let db = get_bistro_db().await.unwrap();
    let menus: Vec<Menu> = db
        .aql_str("FOR menu IN menus RETURN menu")
        .await
        .unwrap();

    Some(menus)
}
