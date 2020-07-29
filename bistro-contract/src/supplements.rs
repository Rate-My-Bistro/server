use serde::{Serialize, Deserialize};
use std::fmt::{self, Formatter, Display};
use crate::price::Price;

#[derive(Serialize, Deserialize, Debug)]
pub struct SupplementList(pub Vec<Supplement>);

impl Display for SupplementList{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            self
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Supplement {
    pub name: String,
    pub price: Price,
}

impl Display for Supplement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{{ name: {name}, price: {price} }}",
            name = self.name,
            price = self.price
        )
    }
}
