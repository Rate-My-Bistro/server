use serde::{Serialize, Deserialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
pub enum Currency {
    EUR,
    USD,
}

impl Display for Currency {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            Currency::EUR => f.write_str("€"),
            Currency::USD => f.write_str("$"),
        }
    }
}

