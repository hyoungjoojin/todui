use std::{
    env::{self},
    io::{Error, ErrorKind::Other, Result},
};

pub fn get_api_token() -> Result<String> {
    let key = "TODOIST_API_TOKEN";

    env::var(key).map_err(|error| Error::new(Other, error))
}
