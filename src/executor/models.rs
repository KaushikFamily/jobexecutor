use std::{collections::HashMap, iter::Map};
use tokio::sync::mpsc;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Task {
    pub task_name: String,
    pub task_id: String,
    pub metadata: TaskMetadata
}

#[derive(Debug, Deserialize)]
pub struct FeedFishesV1 {
    pub job_id: String,
    pub task: Task
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub event_type: String,
    pub task_name: String
}

#[derive(Debug)]
pub struct AppState {
    pub event_sender: mpsc::Sender<Event>, 
}

#[derive(Debug, Deserialize)]
pub struct TaskMetadata {
    #[serde(flatten)]
    pub values: HashMap<String, TaskMetadataValueTypes>
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum TaskMetadataValueTypes {
    #[serde(rename = "email")]
    Email(EmailData)
}

#[derive(Debug, Deserialize)]
pub struct EmailData {
    pub recipients: Vec<String>,
    pub body: String,
    pub subject: String,
    pub sender: String
}