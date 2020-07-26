extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};
use std::fmt::{self, Formatter, Display};
use crate::currency::Currency;

#[derive(Serialize, Deserialize, Debug)]
pub struct Price {
    pub value: f64,
    pub currency: Currency,
}


impl Display for Price {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{value}{currency}",
            currency = self.currency,
            value = self.value
        )
    }
}
