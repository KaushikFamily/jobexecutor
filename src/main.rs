use std::{println, sync::Arc};

use axum::extract::State;
use tokio::{net::TcpListener, sync::mpsc};

use crate::{executor::{AppState, Event, process_event, process_event_v2}, routes::create_routes};

pub mod executor;
pub mod routes;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Event>(100);

    let state = Arc::new(AppState {
        event_sender: tx
    });

    let app = create_routes(State(state));

    let listener = TcpListener::bind("0.0.0.0:3001")
        .await
        .unwrap();

    println!("SERVER IS RUNNING ON HTTP://0.0.0.0:3001");

    // PROCESSOR THREAD
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await{
            println!("HELLO I'M RECIEVING THE EVENT");
            let resp = process_event_v2(event).await;
            match resp {
                Ok(data) => println!("{}", data),
                Err(err) => println!("{}", err)
            }
        }
    });

    axum::serve(listener, app)
        .await
        .unwrap();
}