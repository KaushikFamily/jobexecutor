// Implementation of Task and TaskMetadata

use std::{env, error::Error};

use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::header::ContentType, transport::smtp::authentication::Credentials};

use crate::executor::{EmailNotification, Task, models::{EmailData, TaskMetadata, TaskMetadataValueTypes}};

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

impl EmailNotification for Task {
    async fn send_email(&self) -> Result<String, Box<dyn Error>> {
        
        // Get email data
        let metadata = &self.metadata;
        let email_data = metadata.email_data().unwrap();

        // Implement SMTP Logic
        dotenvy::dotenv()?;

        let user_email = email_data.sender.clone();
        let user_password = env::var("SMTP_PASSWORD")?;
        let recipients: Vec<String> = email_data.recipients.clone();

        let mut builder = Message::builder()
            .from(user_email.parse()?)
            .subject("Feed Fishes 9:45 PM")
            .header(ContentType::TEXT_PLAIN)
        ;

        for recipient in recipients {
            builder = builder.to(recipient.parse()?);
        }

        let email = builder
            .body(String::from("Feed the Fishes"))?
        ;

        let credentials = Credentials::new(
            user_email,
            user_password
        );

        let mailer =
            AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")?
                .credentials(credentials)
                .build();

        mailer.send(email).await?;

        println!("Email sent!");

        Ok(String::from("HELLO"))
    }
}
 
