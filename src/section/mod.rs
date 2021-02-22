use crate::{
    api::{TodoistAPI, TodoistAPIError},
    task::{Task, TaskParamsBuilder},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Section {
    id: u64,
    project_id: u64,
    order: u64,
    name: String,
}

impl Section {
    pub async fn get_all(client: &TodoistAPI) -> Result<Vec<Section>, TodoistAPIError> {
        let sections = SectionParamsBuilder::default().call(client).await?;
        return Ok(sections);
    }
}

#[derive(Default, Builder)]
#[builder(build_fn(private))]
pub struct SectionParams {
    #[builder(setter(strip_option), default)]
    pub project_id: Option<u64>,
}

impl SectionParamsBuilder {
    pub async fn call(&self, client: &TodoistAPI) -> Result<Vec<Section>, TodoistAPIError> {
        let data = self.build().map_err(TodoistAPIError::ParamsBuilderError)?;
        client.get_sections(data).await
    }
}
