use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

// TODO
// pub enum Currency {
//     EUR,
//     USD,
// }
//
// pub struct Price {
//     pub value: f64,
//     pub currency: Currency,
// }

mod naive_date_format {
    use chrono::{NaiveDate};
    use serde::{self, Deserialize, Serializer, Deserializer};
    use serde::de::Error;

    const FORMAT: &'static str = "%Y-%m-%d";

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
        NaiveDate::parse_from_str(&s, FORMAT).map_err(Error::custom)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Supplement {
    pub name: String,
    pub price: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SupplementList(pub Vec<Supplement>);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Menu {
    // pub id: String,
    // #[serde(with = "naive_date_format")]
    // pub served_at: NaiveDate,
    pub name: String,
    // pub image: String,
    // pub optional_supplements: SupplementList,
    // pub mandatory_supplements: SupplementList,
    pub price: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuList(pub Vec<Menu>);
