use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// API representation of a single
/// supplement (additional dish served
/// alongside a menu)
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Supplement {
    pub name: String,
    pub price: f64,
}

/// API representation of a bistro menu
/// This model connects the database
/// entity with the outside domain model
///
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Menu {
    #[serde(alias = "_key")]
    pub id: String,
    pub name: String,
    pub price: f64,
    pub date: NaiveDate,
    pub image: Option<String>,
    pub low_kcal: bool,
    pub optional_supplements: Option<Vec<Supplement>>,
    pub mandatory_supplements: Option<Vec<Supplement>>,
}
