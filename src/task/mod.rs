use crate::api::{TodoistAPI, TodoistAPIError};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

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

impl Task {
    pub async fn get_all(client: &TodoistAPI) -> Result<Vec<Task>, TodoistAPIError> {
        let url = client
            .base_url
            .join("tasks")
            .map_err(TodoistAPIError::UrlParseError)?;
        let task = client
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
}

#[derive(Default, Builder)]
#[builder(build_fn(private))]
pub struct TaskParams {
    #[builder(setter(strip_option), default)]
    pub project_id: Option<u64>,
    #[builder(setter(strip_option), default)]
    pub label_id: Option<u64>,
    #[builder(setter(strip_option), default)]
    pub filter: Option<String>,
    #[builder(setter(strip_option), default)]
    pub lang: Option<String>,
    #[builder(setter(strip_option), default)]
    pub ids: Option<Vec<u64>>,
}

impl TaskParamsBuilder {
    pub async fn call(&self, client: &TodoistAPI) -> Result<Vec<Task>, TodoistAPIError> {
        let data = self.build().map_err(TodoistAPIError::ParamsBuilderError)?;
        client.get_tasks(data).await
    }
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
