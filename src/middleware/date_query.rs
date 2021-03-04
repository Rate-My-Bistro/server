use chrono::NaiveDate;
use rocket::request::{FromQuery, Query};

/// Query Parameter for a date range
///
/// This param is only parsed, if both a 'from' and a 'to'
/// query param is set. If one of these dates cannot be parsed
/// with the format 'YYYY-MM-DD', then no date range can be used.
///
#[derive(Copy, Clone, Debug)]
pub struct DateRangeQueryParam {
    pub from: NaiveDate,
    pub to: NaiveDate
}

impl<'v> FromQuery<'v> for DateRangeQueryParam {
    type Error = &'static str;

    fn from_query(query: Query<'v>) -> Result<Self, Self::Error> {
        let (from_param, to_param) = query
            .fold((None, None), |(from, to), curr| {
                match curr.key.as_str() {
                     "from" => (Some(curr), to),
                    "to" => (from, Some(curr)),
                    _ => (from, to)
                }
            });

        let from = from_param.ok_or("Query param 'from' is missing")?.value.url_decode().map_err(|_| "Query param 'from' is malformed")?;
        let to = to_param.ok_or("Query param 'to' is missing")?.value.url_decode().map_err(|_| "Query param 'to' is malformed")?;

        Ok(DateRangeQueryParam {
            from: NaiveDate::parse_from_str(&from, "%Y-%m-%d").map_err(|_| "Query param 'from' has to be in format YYYY-MM-DD")?,
            to: NaiveDate::parse_from_str(&to, "%Y-%m-%d").map_err(|_| "Query param 'from' has to be in format YYYY-MM-DD")?,
        })
    }
}
