pub mod project;
pub mod task;

use project::Project;
use std::error::Error;
use task::Task;

use crate::utils::api::{HttpMethod, RestClient};

#[derive(Clone)]
pub struct Model {
    projects: Vec<Project>,
    tasks: Vec<Task>,
}

impl Model {
    pub fn new() -> Model {
        Model {
            projects: Vec::new(),
            tasks: Vec::new(),
        }
    }

    pub fn projects(&self) -> &Vec<Project> {
        &self.projects
    }

    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub async fn update(&mut self, client: &RestClient) -> Result<(), Box<dyn Error>> {
        self.projects = client
            .send("/projects", HttpMethod::GET)
            .await?
            .json::<Vec<Project>>()
            .await?
            .iter()
            .map(|project| {
                let mut project = project.clone();
                project.set_depth(match project.parent_id() {
                    Some(_) => 1,
                    None => 0,
                });

                project
            })
            .collect();

        self.tasks = client
            .send("/tasks", HttpMethod::GET)
            .await?
            .json::<Vec<Task>>()
            .await?
            .iter()
            .map(|task| task.clone())
            .collect();

        Ok(())
    }
}
