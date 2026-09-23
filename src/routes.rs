use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::{get, post}};

use crate::executor::{AppState, Event};
use reqwest::StatusCode;

async fn hello_world(
) -> &'static str 
{
    "hello world this is job executor"
}

// #[axum::debug_handler]
async fn consume_event(
    State(state): State<Arc<AppState>>,
    Json(event): Json<Event>
) -> Result<(StatusCode, Json<Option<String>>), StatusCode>
{
    let job_name = event.job_name.clone();
    
    state
        .event_sender
        .send(event)
        .await
        .unwrap();

    Ok((StatusCode::CREATED, Json(Some(job_name))))
}

pub fn create_routes(
    State(state): State<Arc<AppState>>,
) -> Router 
{
    Router::new()
        .route("/", get(hello_world))
        .route("/consume", post(consume_event))
        .with_state(state)
}