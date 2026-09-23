use std::{error::Error};

use crate::executor::models::TaskMetadata;

pub trait NotificationTask 
{
    fn notify(&self, metadata: TaskMetadata);
}

pub trait EmailNotification
{
    async fn send_email(&self) -> Result<String, Box<dyn Error>>;
} 

pub trait EmailNotificationTaskType
{
    async fn notify(&self) -> Result<String, Box<dyn Error>>;
}