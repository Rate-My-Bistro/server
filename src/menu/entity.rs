use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

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
    pub date: NaiveDate,
    pub name: String,
    pub image: Option<String>,
    pub low_kcal: bool,
    pub optional_supplements: Option<Vec<Supplement>>,
    pub mandatory_supplements: Option<Vec<Supplement>>,
    pub price: f64,
}
