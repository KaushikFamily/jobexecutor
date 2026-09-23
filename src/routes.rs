use std::sync::Arc;

use axum::{Router, routing::get};

use crate::executor::AppState;

async fn hello_world(
) -> &'static str 
{
    "hello world this is job scheduler"
}

pub fn create_routes(
    state: Arc<AppState>,
) -> Router 
{
    Router::new()
        .route("/", get(hello_world))
        .with_state(state)
}