use chrono::{Datelike, NaiveDate};

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Date(pub i16, pub u8, pub u8);

impl From<NaiveDate> for Date {
    fn from(date: NaiveDate) -> Self {
        Date::from(&date)
    }
}
impl From<&NaiveDate> for Date {
    fn from(date: &NaiveDate) -> Self {
        Date(date.year() as i16, date.month() as u8, date.day() as u8)
    }
}

impl Into<NaiveDate> for &Date {
    fn into(self) -> NaiveDate {
        NaiveDate::from_ymd(self.0.into(), self.1.into(), self.2.into())
    }
}
