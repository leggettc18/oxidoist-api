use chrono::{DateTime, NaiveDate, Utc};
use reqwest::header;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

const BASE_URL: &str = "https://api.todoist.com/rest/v1/";

#[derive(Debug)]
#[allow(dead_code)]
pub enum TodoistAPIError {
    Error(reqwest::Error),
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
    InvalidHeaderName(reqwest::header::InvalidHeaderName),
    UrlParseError(url::ParseError),
}

#[derive(Debug)]
pub struct TodoistAPI {
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
        let header_token_value = header::HeaderValue::from_str(&bearer_token)
            .map_err(TodoistAPIError::InvalidHeaderValue)?;
        headers.insert(
            header::HeaderName::from_bytes(b"Authorization")
                .map_err(TodoistAPIError::InvalidHeaderName)?,
            header_token_value,
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(TodoistAPIError::Error)?;
        return Ok(TodoistAPI { base_url, client });
    }

    #[allow(dead_code)]
    pub async fn get_projects(&self) -> Result<Vec<Project>, TodoistAPIError> {
        let url = self
            .base_url
            .join("projects")
            .map_err(TodoistAPIError::UrlParseError)?;
        let projects = self
            .client
            .get(url)
            .send()
            .await
            .map_err(TodoistAPIError::Error)?
            .json::<Vec<Project>>()
            .await
            .map_err(TodoistAPIError::Error)?;
        return Ok(projects);
    }

    #[allow(dead_code)]
    pub async fn get_project(&self, id: u64) -> Result<Project, TodoistAPIError> {
        let url = self
            .base_url
            .join("projects/")
            .map_err(TodoistAPIError::UrlParseError)?
            .join(format!("{}", id).as_str())
            .map_err(TodoistAPIError::UrlParseError)?;
        let project = self
            .client
            .get(url)
            .send()
            .await
            .map_err(TodoistAPIError::Error)?
            .json::<Project>()
            .await
            .map_err(TodoistAPIError::Error)?;
        return Ok(project);
    }

    pub async fn get_tasks(&self) -> Result<Vec<Task>, TodoistAPIError> {
        let url = self
            .base_url
            .join("tasks")
            .map_err(TodoistAPIError::UrlParseError)?;
        let task = self
            .client
            .get(url)
            .send()
            .await
            .map_err(TodoistAPIError::Error)?
            .json::<Vec<Task>>()
            .await
            .map_err(TodoistAPIError::Error)?;
        return Ok(task);
    }

    pub async fn get_project_tasks(&self, id: u64) -> Result<Vec<Task>, TodoistAPIError> {
        let tasks = self.get_tasks().await?;
        let project_tasks: Vec<Task> = tasks
            .into_iter()
            .filter(|task| task.project_id == id)
            .collect();
        return Ok(project_tasks);
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    id: u64,
    name: String,
    color: u64,
    #[serde(default)]
    parent_id: u64,
    #[serde(default)]
    order: u64,
    comment_count: u64,
    shared: bool,
    favorite: bool,
    #[serde(default)]
    inbox_project: bool,
    #[serde(default)]
    team_inbox: bool,
    sync_id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    id: u64,
    assignee: Option<u64>,
    comment_count: u64,
    completed: bool,
    content: String,
    due: Option<Due>,
    label_ids: Vec<u64>,
    order: u64,
    priority: u64,
    project_id: u64,
    section_id: u64,
    parent_id: Option<u64>,
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Due {
    #[serde(with = "todoist_date_format")]
    date: NaiveDate,
    recurring: bool,
    datetime: Option<DateTime<Utc>>,
    string: String,
    timezone: Option<String>,
}

mod todoist_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
