use reqwest::StatusCode;

use crate::executor::Event;

pub fn process_event(
    event: Event
) -> Result<String, StatusCode>
{
    let res = match event.job_name.as_str() {
        "feed_fishes_v1" => {
            "FEED_FISHES"
        }
        _ => "NO JOB FOUND"
    };

    Ok(res.to_string())
}