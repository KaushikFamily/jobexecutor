use std::sync::Arc;

use axum::{Router, extract::State, response::IntoResponse, routing::{get, post}};

use crate::executor::{AppState, Event};

async fn hello_world(
) -> &'static str 
{
    "hello world this is job scheduler"
}

async fn consume_event(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse
{
    let event = Event {
        event_type: "TEST".to_string(),
        payload: "hello".to_string(),
    };

    state
        .event_sender
        .send(event)
        .await
        .unwrap();

    "Event queued"
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