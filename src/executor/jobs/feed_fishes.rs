// IMPLEMENTATION OF FEED_FISHES TASKS

// -------------- Implement Structs -------------- //

use crate::executor::{EmailNotification, models::FeedFishesV1, traits::EmailNotificationTaskType};

impl FeedFishesV1 {

    pub async fn execute(
        &self
    ) -> Result<String, Box<dyn std::error::Error>>
    {
        match self.notify().await {
            Ok(resp) => {
                Ok(String::from("ABLE TO SEND EMAIL FOR FEEDING FISHES"))
            },
            Err(err) => {
                println!("UNABLE TO SEND NOTIFICATION FOR FEEDING FISHES {}", err);
                Err(err)
            },
        }
    }

}

// -------------- Implement Traits  -------------- //
impl EmailNotificationTaskType for FeedFishesV1 {

    async fn notify(
        &self
    ) -> Result<String, Box<dyn std::error::Error>> 
    {
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
