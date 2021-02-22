use crate::api::{TodoistAPI, TodoistAPIError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Label {
    id: u64,
    name: String,
    color: u64,
    order: u64,
    favorite: bool,
}

impl Label {
    pub async fn get_all(client: &TodoistAPI) -> Result<Vec<Label>, TodoistAPIError> {
        let labels = client.get_labels().await?;
        Ok(labels)
    }
}
