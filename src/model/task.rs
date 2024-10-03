use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Task {
    id: String,
    content: String,
}

impl Task {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn content(&self) -> &String {
        &self.content
    }
}
