mod project;

use project::Project;
use reqwest;
use std::error::Error;
use tokio;

pub struct Model {
    projects: Vec<Project>,
}

impl Model {
    pub fn new() -> Model {
        Model {
            projects: Vec::new(),
        }
    }

    pub fn projects(&self) -> &Vec<Project> {
        &self.projects
    }

    #[tokio::main]
    pub async fn update(&mut self, api_token: &String) -> Result<(), Box<dyn Error>> {
        let client = reqwest::Client::new();
        let projects = client
            .get("https://api.todoist.com/rest/v2/projects")
            .bearer_auth(api_token)
            .send()
            .await?
            .json::<Vec<Project>>()
            .await?;

        self.projects = projects
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

        Ok(())
    }
}
