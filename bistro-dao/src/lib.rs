extern crate bistro_contract;
extern crate arangors;

use arangors::{Connection, Database, AqlQuery};
use arangors::client::reqwest::ReqwestClient;
use bistro_contract::menu::Menu;

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

// TODO This is a helper and will be deleted:
pub async fn get_all_menus() -> Option<Vec<Menu>> {
    let db = get_bistro_db().await.unwrap();
    let menus: Vec<Menu> = db
        .aql_str("FOR menu IN menus RETURN menu")
        .await
        .unwrap();

    Some(menus)
}

pub async fn get_menu_ids_by_date_range(from: String, to: String) -> Option<Vec<String>> {
    let db = get_bistro_db().await.unwrap();
    let aql = AqlQuery::builder()
        .query("FOR menu IN @@collection FILTER menu.servedAt >= @from AND menu.servedAt <= @to RETURN menu.id")
        .bind_var("@collection", "menus")
        .bind_var("from", from)
        .bind_var("to", to)
        .build();

    let menus: Vec<String> = db.aql_query(aql).await.unwrap();
    Some(menus)
}
