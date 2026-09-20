use std::iter::Map;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Task {
    pub task_name: String,
    pub task_id: String,
    pub metadata: TaskMetadata
}

#[derive(Debug, Deserialize)]
pub struct TaskMetadata {
    #[serde(flatten)]
    pub values: Map<String, TaskMetadataValueTypes>
}

#[derive(Debug, Deserialize)]
pub enum TaskMetadataValueTypes {

}