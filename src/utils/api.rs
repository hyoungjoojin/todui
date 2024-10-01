use reqwest::{self, Response};
use std::{
    env::{self},
    io::{Error, ErrorKind::Other},
};

const API_TOKEN_NAME: &str = "TODOIST_API_TOKEN";
const BASE_PATH: &str = "https://api.todoist.com/rest/v2";

pub enum HttpMethod {
    GET,
}

pub struct RestClient {
    client: reqwest::Client,
    token: String,
}

impl RestClient {
    pub fn new() -> Option<RestClient> {
        let token = match env::var(API_TOKEN_NAME).map_err(|error| Error::new(Other, error)) {
            Ok(token) => token,
            Err(_) => {
                println!("Your Todoist API token could not be found by todui.\n");
                println!("Export or set an environment variable.");
                println!("  export TODOIST_API_TOKEN=\"YOUR_API_TOKEN\"");
                return None;
            }
        };

        Some(RestClient {
            client: reqwest::Client::new(),
            token,
        })
    }

    pub async fn send(&self, url: &str, method: HttpMethod) -> Result<Response, Error> {
        let response = match method {
            HttpMethod::GET => self
                .client
                .get(format!("{}{}", BASE_PATH, url))
                .bearer_auth(self.token.clone())
                .send(),
        }
        .await;

        match response {
            Ok(response) => {
                return Ok(response);
            }
            Err(_) => {
                return Err(Error::new(Other, "network request failed"));
            }
        }
    }
}
