use serde::{Deserialize};

#[derive(Deserialize)]
pub struct DateRange {
    pub from: String,
    pub to: String,
}
