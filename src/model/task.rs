use chrono::NaiveDate;
use serde::Deserialize;

use crate::utils::date::naive_date_serializer;

#[derive(Deserialize, Debug, Clone)]
pub struct Task {
    id: String,
    content: String,
    due: Option<Due>,
}

impl Task {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn due(&self) -> &Option<Due> {
        &self.due
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Due {
    #[serde(with = "naive_date_serializer")]
    date: NaiveDate,
}

impl Due {
    pub fn date(&self) -> &NaiveDate {
        &self.date
    }
}
