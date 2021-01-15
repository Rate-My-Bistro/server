extern crate arangors;

use arangors::{Connection, Database};
use arangors::client::reqwest::ReqwestClient;
use crate::menu::entity::Menu;


async fn connect() -> Option<Connection> {
    let conn = Connection::establish_basic_auth("http://localhost:8529", "root", "iamroot")
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