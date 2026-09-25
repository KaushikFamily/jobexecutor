use std::fs::Metadata;

use reqwest::{Client, StatusCode};

use crate::executor::{Event, Task, models::{FeedFishesV1, TaskMetadata}};

pub async fn process_event(
    event: Event
) -> Result<String, StatusCode>
{
    let res = match event.job_name.as_str() {
        "feed_fishes_v1" => {
            let metadata_resp = get_metadata(event.job_id.clone()).await;

            let metadata = match metadata_resp {
                Ok(data) =>
                {
                    data.1
                }
                Err(err) =>
                {
                    println!("ERROR RETRIEVING METADATA: {}", err);
                    None
                }
            };

            let feed_fishes_v1 = match metadata {
                Some(data) => {
                    let task = Task {
                        job_name : event.job_name.clone(),
                        metadata : data
                    };

                    Some(FeedFishesV1 {
                        job_id: event.job_id.clone(),
                        task : task
                    })
                }
                None => None,
            };

            match feed_fishes_v1 {
                Some(job) => {
                    let _ = job.execute();
                },
                None => todo!(),
            }
            
            "FEED_FISHES"
        }
        _ => "NO JOB FOUND"
    };

    Ok(res.to_string())
}

pub async fn get_metadata(
    job_id: String
) -> Result<(StatusCode, Option<TaskMetadata>), StatusCode> 
{
    let client = Client::new();
    
    let url = format!("http://localhost:8000/jobs/metadata/{}", job_id);

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if (response.status().is_success())
    {
        let metadata_response: TaskMetadata = response.json()
            .await
            .map_err(|e| {
                println!("DESERIALIZATION ERROR: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR

            })?;

        Ok((StatusCode::OK, Some(metadata_response)))
        
    } else 
    {
        Err(StatusCode::from_u16(response.status().as_u16())
        .unwrap_or(StatusCode::BAD_REQUEST))
    }
} 
