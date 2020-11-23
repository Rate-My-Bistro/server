use serde::{Deserialize};
use chrono::NaiveDate;

mod naive_date_format {
    use chrono::{NaiveDate};
    use serde::{self, Deserialize, Serializer, Deserializer};
    use serde::de::Error;

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(
        optional_date: &Option<NaiveDate>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match optional_date {
            Some(date) => {
                let s = format!("{}", date.format(FORMAT));
                serializer.serialize_str(&s)
            },
            None => None
        }
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<NaiveDate>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(Error::custom)
    }
}

#[derive(Deserialize,Debug)]
pub struct DateRange {
    #[serde(skip_serializing_if = "Option::is_none", with = "naive_date_format")]
    pub from: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none", with = "naive_date_format")]
    pub to: Option<NaiveDate>,
}
