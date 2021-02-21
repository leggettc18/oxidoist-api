use crate::{
    api::{TodoistAPI, TodoistAPIError},
    task::{Task, TaskParamsBuilder},
};
use serde::{Deserialize, Serialize};

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

impl Project {
    pub async fn get_all(client: &TodoistAPI) -> Result<Vec<Project>, TodoistAPIError> {
        let url = client
            .base_url
            .join("projects")
            .map_err(TodoistAPIError::UrlParseError)?;
        let projects = client
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
    pub async fn get(id: u64, client: &TodoistAPI) -> Result<Project, TodoistAPIError> {
        let url = client
            .base_url
            .join("projects/")
            .map_err(TodoistAPIError::UrlParseError)?
            .join(format!("{}", id).as_str())
            .map_err(TodoistAPIError::UrlParseError)?;
        let project = client
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
    pub async fn get_tasks(&self, client: &TodoistAPI) -> Result<Vec<Task>, TodoistAPIError> {
        let tasks = TaskParamsBuilder::default()
            .project_id(self.id)
            .call(&client)
            .await?;
        return Ok(tasks);
    }
}
