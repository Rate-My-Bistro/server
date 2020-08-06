extern crate bistro_contract;
extern crate bistro_dao;

use bistro_contract::menu::Menu;

pub async fn list_menus() -> Option<Vec<Menu>> {
    // let menus = vec![Menu {
    //     id: String::from("1"),
    //     served_at: NaiveDate::from_ymd(2020, 10, 10),
    //     name: String::from("Chili Noodles"),
    //     image: String::from("http://some-image.com/image.png"),
    //     optional_supplements: SupplementList(vec![Supplement { name: String::from("Noodles"), price: Price { value: 2.88, currency: Currency::EUR } }]),
    //     mandatory_supplements: SupplementList(vec![Supplement { name: String::from("Noodles"), price: Price { value: 2.88, currency: Currency::EUR } }]),
    //     price: Price { value: 11.0, currency: Currency::EUR }
    // }, Menu {
    //     id: String::from("2"),
    //     served_at: NaiveDate::from_ymd(2020, 10, 10),
    //     name: String::from("Green Noodles"),
    //     image: String::from("http://some-image.com/imageGreen.png"),
    //     optional_supplements: SupplementList(vec![Supplement { name: String::from("Noodles"), price: Price { value: 2.88, currency: Currency::EUR } }]),
    //     mandatory_supplements: SupplementList(vec![Supplement { name: String::from("Noodles"), price: Price { value: 2.88, currency: Currency::EUR } }]),
    //     price: Price { value: 10.0, currency: Currency::EUR }
    // }];
    //
    // return Some(menus);

    return bistro_dao::get_menus().await;
}

