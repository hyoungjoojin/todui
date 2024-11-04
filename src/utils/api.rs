use reqwest::{self, Response};
use std::{
    collections::HashMap,
    env::{self},
    io::{Error, ErrorKind::Other},
};

const API_TOKEN_NAME: &str = "TODOIST_API_TOKEN";
const BASE_PATH: &str = "https://api.todoist.com/rest/v2";

pub enum HttpMethod {
    Get,
    Post,
    Delete,
}

#[derive(Clone)]
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

    pub async fn send(
        &self,
        url: &str,
        method: HttpMethod,
        body: Option<HashMap<&str, &str>>,
    ) -> Result<Response, Error> {
        let url = format!("{}{}", BASE_PATH, url);
        let token = self.token.clone();

        tracing::info!("Sending network request to {url}.");
        let response = match method {
            HttpMethod::Get => self.client.get(&url).bearer_auth(token).send(),
            HttpMethod::Post => match body {
                Some(body) => self.client.post(&url).bearer_auth(token).json(&body).send(),
                None => self.client.post(&url).bearer_auth(token).send(),
            },
            HttpMethod::Delete => self.client.delete(&url).bearer_auth(token).send(),
        }
        .await;

        let response = match response {
            Ok(response) => response,
            Err(error) => {
                tracing::error!(
                    "Network request sent to {url} failed due to {error}.",
                    url = url,
                    error = error
                );
                return Err(Error::new(Other, "network request failed"));
            }
        };

        match response.error_for_status() {
            Ok(response) => Ok(response),
            Err(error) => {
                tracing::error!(
                    "Network request sent to {url} sent back an error status: {error}.",
                    url = url,
                    error = error
                );
                Err(Error::new(Other, "network request failed"))
            }
        }
    }
}
