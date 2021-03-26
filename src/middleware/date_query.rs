use rocket::form::{self, Error};
use time::{Date};


/// Query Parameter for a date range
///
/// This param is only parsed, if both a 'from' and a 'to'
/// query param is set. If one of these dates cannot be parsed
/// with the format 'YYYY-MM-DD', then no date range can be used.
///
#[derive(FromForm)]
pub struct DateRangeQueryParam {
    #[field(validate = is_date_range(&self.to))]
    pub from: Date,
    pub to: Date
}

fn is_date_range<'v>(from: &time::Date, to: &time::Date) -> form::Result<'v, ()> {
    if from.gt(to) {
        Err(Error::validation("'From' has to be earlier than 'to'"))?;
    }

    Ok(())
}
