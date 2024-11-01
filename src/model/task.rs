use chrono::NaiveDate;
use ratatui::{
    text::Text,
    widgets::{Cell, Row},
};
use serde::Deserialize;

use crate::utils::date::naive_date_serializer;

#[derive(Deserialize, Debug, Clone)]
pub struct Task {
    id: String,
    content: String,
    description: String,
    project_id: String,
    priority: u8,
    due: Option<Due>,
}

impl<'a> From<&&Task> for Row<'a> {
    fn from(task: &&Task) -> Row<'a> {
        [
            task.project_id(),
            task.content(),
            &task.due_date(),
            &task.priority().to_string(),
        ]
        .into_iter()
        .map(|text| Cell::from(Text::from(text.clone())))
        .collect::<Row>()
    }
}

impl Task {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn project_id(&self) -> &String {
        &self.project_id
    }

    pub fn priority(&self) -> u8 {
        self.priority
    }

    pub fn due_date(&self) -> String {
        match self.due {
            Some(due) => due.date().to_string(),
            None => "".to_string(),
        }
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
