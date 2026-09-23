use std::{println, sync::Arc};

use tokio::{net::TcpListener, sync::mpsc};

use crate::{executor::{AppState, Event}, routes::create_routes};

pub mod executor;
pub mod routes;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Event>(100);

    let state = Arc::new(AppState {
        event_sender: tx
    });

    let app = create_routes(state);

    let listener = TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap(); 

    // PROCESSOR THREAD
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await{
            println!("HELLO I'M RECIEVING THE EVENT");
        }
    });

    axum::serve(listener, app)
        .await
        .unwrap()
    ;
}