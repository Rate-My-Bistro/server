extern crate serde;
extern crate serde_json;

mod supplement;
mod price;
mod currency;

use serde::{Serialize, Deserialize};
use std::fmt::{self, Formatter, Display};
use chrono::{NaiveDate};
use crate::price::Price;
use crate::supplement::Supplement;
use crate::currency::Currency;

mod naive_date_format {
    use chrono::{NaiveDate};
    use serde::{self, Deserialize, Serializer, Deserializer};
    use serde::de::Error;

    const FORMAT: &'static str = "%YYYY-%mm-%dd";

    pub fn serialize<S>(
        date: &NaiveDate,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDate, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Menu {
    pub id: String,
    #[serde(with = "naive_date_format")]
    pub served_at: NaiveDate,
    pub name: String,
    pub image: String,
    // Todo - Get Vectors running
    pub supplements: Vec<Supplement>,
    pub price: Price,
}

impl Display for Vec<Supplement> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{{ name: {name}, price: {price} }}",
            name = self.name,
            price = self.price
        )
    }
}

impl Display for Menu {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "
              {{
                id: {id},
                name: {name},
                served_at: {served_at},
                image: {image},
                price: {price},
                supplements: {supplements}
              }}
            ",
            id = self.id,
            served_at = self.served_at,
            name = self.name,
            image = self.image,
            price = self.price,
            supplements = self.supplements
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let menu = Menu {
            id: String::from("1"),
            served_at: NaiveDate::from_ymd(2020, 10, 10),
            name: String::from("Chili Noodles"),
            image: String::from("http://some-image.com/image.png"),
            supplements: vec![],
            price: Price { value: 0.0, currency: Currency::EUR }
        };
        println!("{}", menu);
    }
}
