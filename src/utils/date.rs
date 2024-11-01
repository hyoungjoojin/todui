use chrono::{Local, NaiveDate};

pub mod naive_date_serializer {
    use chrono::NaiveDate;
    use serde::{de::Error, Deserialize, Deserializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn deserialize<'d, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'d>,
    {
        let s = String::deserialize(deserializer)?;
        let date = NaiveDate::parse_from_str(&s, FORMAT).map_err(Error::custom)?;
        Ok(date)
    }
}

pub fn get_current_date() -> NaiveDate {
    NaiveDate::from(Local::now().naive_local())
}
