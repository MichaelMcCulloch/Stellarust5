use crate::empire::EmpireData;
use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ModelDataPoint {
    pub campaign_name: String,
    #[serde(with = "naive_date_serde")]
    pub date: NaiveDate,
    pub empires: Vec<EmpireData>,
}

mod naive_date_serde {
    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::date::Date;

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let x = Date::from(date);
        x.serialize(serializer)
    }

    pub fn deserialize<'de, D>(input: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        Date::deserialize(input)
            .map(|date| NaiveDate::from_ymd(date.0.into(), date.1.into(), date.2.into()))
    }
}
