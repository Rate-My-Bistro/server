use chrono::NaiveDate;
use rocket::http::RawStr;
use rocket::request::FromFormValue;
use std::ops::Deref;

// TODO Create a DateRange Extractor
pub struct DateQueryParam(NaiveDate);

impl<'v> FromFormValue<'v> for DateQueryParam {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<DateQueryParam, &'v RawStr> {
        let decoded = form_value.url_decode().map_err(|_| form_value)?;
        if let Ok(date) = NaiveDate::parse_from_str(&decoded, "%Y-%m-%d") {
            return Ok(DateQueryParam(date));
        }
        Err(form_value)
    }
}

impl Deref for DateQueryParam {
    type Target = NaiveDate;
    fn deref(&self) -> &NaiveDate {
        &self.0
    }
}
