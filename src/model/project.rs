use std::collections::HashMap;

use serde::Deserialize;

#[derive(Clone)]
pub struct ProjectManager {
    projects: Vec<Project>,
    id_to_project: HashMap<String, Project>,
}

impl ProjectManager {
    pub fn new() -> ProjectManager {
        ProjectManager {
            projects: Vec::new(),
            id_to_project: HashMap::new(),
        }
    }

    pub fn get_project(&self, id: &String) -> Option<&Project> {
        self.id_to_project.get(id)
    }

    pub fn extend<I>(&mut self, iter: I)
    where
        I: Iterator<Item = Project>,
    {
        for project in iter {
            self.id_to_project
                .insert(project.id().clone(), project.clone());
            self.projects.push(project);
        }
    }

    pub fn clear(&mut self) {
        self.projects.clear();
        self.id_to_project.clear();
    }

    pub fn projects(&self) -> &Vec<Project> {
        &self.projects
    }
}

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
