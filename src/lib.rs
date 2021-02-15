use serde::{Serialize, Deserialize};
use reqwest::header;

const BASE_URL: &str = "https://api.todoist.com/rest/v1/";

#[derive(Debug)]
#[allow(dead_code)]
enum TodoistAPIError {
    Error(reqwest::Error),
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue)
}

#[derive(Debug)]
struct TodoistAPI{
    client: reqwest::Client
}

impl TodoistAPI {
    #[allow(dead_code)]
    pub fn new(token: &str) -> Result<TodoistAPI, TodoistAPIError> {
        let mut headers = header::HeaderMap::new();
        let header_token_value = header::HeaderValue::from_str(token).map_err(TodoistAPIError::InvalidHeaderValue)?;
        headers.insert(header::AUTHORIZATION, header_token_value);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build().map_err(TodoistAPIError::Error)?;
        return Ok(TodoistAPI{ client })
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Project {
    id: u32,
    name: String,
    color: u32,
    parent_id: u32,
    order: u32,
    comment_count: u32,
    shared: bool,
    favorite: bool,
    inbox_project: bool,
    team_inbox: bool,
    sync_id: u32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
