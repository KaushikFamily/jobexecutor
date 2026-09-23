use std::{error::Error, fs::Metadata};

use crate::executor::models::TaskMetadata;

pub trait NotificationTask {
    fn notify(&self, metadata: TaskMetadata);
}

pub trait EmailNotificationTask {
    async fn send_email(&self) -> Result<String, Box<dyn Error>>;
    
}