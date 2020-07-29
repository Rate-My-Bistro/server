
extern crate serde;
extern crate serde_json;

mod supplements;
mod price;
mod currency;
mod menu;


#[cfg(test)]
mod tests {
    use super::*;
    use crate::supplements::{Supplement, SupplementList};
    use crate::menu::Menu;
    use crate::currency::Currency;
    use crate::price::Price;
    use chrono::NaiveDate;

    #[test]
    fn test_display() {
        let menu = Menu {
            id: String::from("1"),
            served_at: NaiveDate::from_ymd(2020, 10, 10),
            name: String::from("Chili Noodles"),
            image: String::from("http://some-image.com/image.png"),
            optional_supplements: SupplementList(vec![Supplement { name: String::from("Noodles"), price: Price { value: 2.88, currency: Currency::EUR } }]),
            mandatory_supplements: SupplementList(vec![Supplement { name: String::from("Noodles"), price: Price { value: 2.88, currency: Currency::EUR } }]),
            price: Price { value: 0.0, currency: Currency::EUR }
        };
        println!("{}", menu);
    }
}
