use axum::{Router, routing::get};
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};

use crate::{collector::collect_sysinfo, ws::ws::ws_handler};

pub mod collector;
pub mod models;
pub mod ws;

#[derive(Clone)]
pub struct AppState {
    pub tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel::<String>(16);

    collect_sysinfo::run(tx.clone()).await;

    let state = AppState { tx };

    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_origin(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .layer(cors)
        .with_state(state);

    let addr = "0:0:0:0:3000";

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
