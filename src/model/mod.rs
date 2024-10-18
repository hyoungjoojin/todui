pub mod project;
pub mod task;

use project::Project;
use std::{error::Error, process::exit};
use task::Task;

use crate::utils::api::{HttpMethod, RestClient};

#[derive(Clone)]
pub struct Model {
    client: RestClient,
    projects: Vec<Project>,
    tasks: Vec<Task>,
}

impl Model {
    pub fn new() -> Model {
        Model {
            client: match RestClient::new() {
                Some(client) => client,
                None => exit(-1),
            },
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

    pub async fn update(&mut self) -> Result<(), Box<dyn Error>> {
        self.projects = self.client
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

        self.tasks = self.client
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
