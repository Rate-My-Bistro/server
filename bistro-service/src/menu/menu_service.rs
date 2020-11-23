extern crate bistro_contract;
extern crate bistro_dao;

use bistro_contract::menu::Menu;

pub async fn list_menu_ids_by_date_range(from: String, to: String) -> Option<Vec<String>> {
    bistro_dao::get_menu_ids_by_date_range(from, to).await
}

pub async fn list_menus() -> Option<Vec<Menu>> {
    bistro_dao::get_all_menus().await
}
