// IMPLEMENTATION OF FEED_FISHES TASKS

// -------------- Implement Structs -------------- //

use crate::executor::{EmailNotification, models::FeedFishesV1, traits::EmailNotificationTaskType};

impl FeedFishesV1 {
    
}


// -------------- Implement Structs -------------- //
impl EmailNotificationTaskType for FeedFishesV1 {
    async fn notify(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self.task.send_email().await {
            Ok(resp) => {
                Ok(String::from("SENT EMAIL"))
            }
            Err(err) => {
                println!("{}", err);
                Err(err)
            }

        }
    }
}
// -------------- Implement Functions -------------- //
