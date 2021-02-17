use serde::{Serialize, Deserialize};
use reqwest::header;
use reqwest::Client;
use url::Url;

const BASE_URL: &str = "https://api.todoist.com/rest/v1/";

#[derive(Debug)]
#[allow(dead_code)]
pub enum TodoistAPIError {
    Error(reqwest::Error),
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
    InvalidHeaderName(reqwest::header::InvalidHeaderName),
    UrlParseError(url::ParseError)
}

#[derive(Debug)]
pub struct TodoistAPI{
    base_url: Url,
    client: Client,
}

impl TodoistAPI {
    #[allow(dead_code)]
    pub fn new(token: &str) -> Result<TodoistAPI, TodoistAPIError> {
        let base_url = Url::parse(BASE_URL).map_err(TodoistAPIError::UrlParseError)?;
        let mut headers = header::HeaderMap::new();
        let mut bearer_token: String = "Bearer ".to_string();
        bearer_token.push_str(&token);
        let header_token_value = header::HeaderValue::from_str(&bearer_token).map_err(TodoistAPIError::InvalidHeaderValue)?;
        headers.insert(header::HeaderName::from_bytes(b"Authorization").map_err(TodoistAPIError::InvalidHeaderName)?, header_token_value);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build().map_err(TodoistAPIError::Error)?;
        return Ok(TodoistAPI{ base_url, client })
    }

    #[allow(dead_code)]
    pub async fn get_projects(&self) -> Result<Vec<Project>, TodoistAPIError> {
        let url = self.base_url.join("projects").map_err(TodoistAPIError::UrlParseError)?;
        let projects = self.client.get(url)
            .send()
            .await.map_err(TodoistAPIError::Error)?
            .json::<Vec<Project>>()
            .await.map_err(TodoistAPIError::Error)?;
        return Ok(projects);
    }

    #[allow(dead_code)]
    pub async fn get_project(&self, id: u32) -> Result<Project, TodoistAPIError> {
        let url = self.base_url.join("projects/").map_err(TodoistAPIError::UrlParseError)?.join(format!("{}", id).as_str()).map_err(TodoistAPIError::UrlParseError)?;
        let project = self.client.get(url)
            .send()
            .await.map_err(TodoistAPIError::Error)?
            .json::<Project>()
            .await.map_err(TodoistAPIError::Error)?;
        return Ok(project)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    id: u32,
    name: String,
    color: u32,
    #[serde(default)]
    parent_id: u32,
    #[serde(default)]
    order: u32,
    comment_count: u32,
    shared: bool,
    favorite: bool,
    #[serde(default)]
    inbox_project: bool,
    #[serde(default)]
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
