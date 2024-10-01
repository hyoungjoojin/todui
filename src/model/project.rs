use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Project {
    id: String,
    name: String,
    parent_id: Option<String>,

    #[serde(skip)]
    depth: usize,
}

impl Project {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn parent_id(&self) -> &Option<String> {
        &self.parent_id
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn set_depth(&mut self, depth: usize) {
        self.depth = depth;
    }
}
