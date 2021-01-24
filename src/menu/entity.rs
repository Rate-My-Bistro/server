use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

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
    pub price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Menu {
    #[serde(alias = "_key")]
    pub id: String,
    #[serde(with = "naive_date_format")]
    pub date: NaiveDate,
    pub name: String,
    pub image: Option<String>,
    pub low_kcal: bool,
    pub optional_supplements: Option<Vec<Supplement>>,
    pub mandatory_supplements: Option<Vec<Supplement>>,
    pub price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuList(pub Vec<Menu>);
