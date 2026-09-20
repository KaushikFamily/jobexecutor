// Implementation of Task and TaskMetadata

use std::error::Error;

use crate::executor::{Task, models::{EmailData, TaskMetadata, TaskMetadataValueTypes}, task, traits::EmailNotificationTask};

// -------------- Implement Structs -------------- //

impl Task {
}

impl TaskMetadata {
    pub fn email_data(&self) -> Option<&EmailData> {
        match self.values.get("emailData") {
            Some(TaskMetadataValueTypes::Email(data)) => Some(data),
            _ => None
        }
    }
}

// -------------- Implement Traits -------------- //

impl EmailNotificationTask for Task {
    fn send_email(&self) -> Result<String, Box<dyn Error>> {
        // Get email data
        let metadata = &self.metadata
        let email_data = metadata.email_data().unwrap();

        // Implement SMTP Logic

        Ok(String::from("HELLO"))
    }
}
 
