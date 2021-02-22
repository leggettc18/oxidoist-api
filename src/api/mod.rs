use reqwest::header;
use reqwest::Client;
use url::Url;

use crate::{
    label::Label,
    project::Project,
    section::{Section, SectionParams},
    task::{Task, TaskParams},
};

const BASE_URL: &str = "https://api.todoist.com/rest/v1/";

#[derive(Debug)]
#[allow(dead_code)]
pub enum TodoistAPIError {
    Error(reqwest::Error),
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
    InvalidHeaderName(reqwest::header::InvalidHeaderName),
    UrlParseError(url::ParseError),
    ParamsBuilderError(String),
}

#[derive(Debug)]
pub struct TodoistAPI {
    pub base_url: Url,
    pub client: Client,
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

    pub async fn get_tasks(&self, data: TaskParams) -> Result<Vec<Task>, TodoistAPIError> {
        let mut url = self
            .base_url
            .join("tasks")
            .map_err(TodoistAPIError::UrlParseError)?;
        if let Some(project_id) = data.project_id {
            url.query_pairs_mut()
                .append_pair("project_id", &*project_id.to_string());
        }
        if let Some(label_id) = data.label_id {
            url.query_pairs_mut()
                .append_pair("label_id", &*label_id.to_string());
        }
        if let Some(filter) = data.filter {
            url.query_pairs_mut().append_pair("filter", &*filter);
        }
        if let Some(lang) = data.lang {
            url.query_pairs_mut().append_pair("lang", &*lang);
        }
        if let Some(ids) = data.ids {
            url.query_pairs_mut()
                .append_pair("ids", &*format!("{:?}", ids));
        }
        let tasks = self
            .client
            .get(url)
            .send()
            .await
            .map_err(TodoistAPIError::Error)?
            .json::<Vec<Task>>()
            .await
            .map_err(TodoistAPIError::Error)?;
        return Ok(tasks);
    }

    pub async fn get_sections(&self, data: SectionParams) -> Result<Vec<Section>, TodoistAPIError> {
        let mut url = self
            .base_url
            .join("sections")
            .map_err(TodoistAPIError::UrlParseError)?;
        if let Some(project_id) = data.project_id {
            url.query_pairs_mut()
                .append_pair("project_id", &*project_id.to_string());
        }
        let sections = self
            .client
            .get(url)
            .send()
            .await
            .map_err(TodoistAPIError::Error)?
            .json::<Vec<Section>>()
            .await
            .map_err(TodoistAPIError::Error)?;
        return Ok(sections);
    }

    pub async fn get_labels(&self) -> Result<Vec<Label>, TodoistAPIError> {
        let url = self
            .base_url
            .join("labels")
            .map_err(TodoistAPIError::UrlParseError)?;
        let labels = self
            .client
            .get(url)
            .send()
            .await
            .map_err(TodoistAPIError::Error)?
            .json::<Vec<Label>>()
            .await
            .map_err(TodoistAPIError::Error)?;
        return Ok(labels);
    }
}
