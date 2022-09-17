use chrono::{Datelike, NaiveDate};

use serde::{ser::SerializeTupleStruct, Serialize};

#[cfg(feature = "test")]
use serde_derive::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "test", derive(Deserialize))]
pub struct Date(i16, u8, u8);

impl From<NaiveDate> for Date {
    fn from(_d: NaiveDate) -> Self {
        Date(_d.year() as i16, _d.month() as u8, _d.day() as u8)
    }
}

impl Into<NaiveDate> for &Date {
    fn into(self) -> NaiveDate {
        NaiveDate::from_ymd(self.0.into(), self.1.into(), self.2.into())
    }
}

#[cfg(feature = "test")]
impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_tuple_struct("", 3)?;
        s.serialize_field(&self.0)?;
        s.serialize_field(&self.1)?;
        s.serialize_field(&self.2)?;
        s.end()
    }
}
