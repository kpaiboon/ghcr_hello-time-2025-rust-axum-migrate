mod errors;
mod person;
mod routes;

use std::sync::{Arc, RwLock};
use std::env;
use axum::Router;
use tower_http::trace::TraceLayer;
use routes::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let greeting_text = env::var("GREETING_TEXT").unwrap_or_else(|_| "Hi!".to_string());

    let shared_state = Arc::new(AppState {
        person_collection: RwLock::new(person::create_person_collection()),
        greeting_text,
    });

    let app = Router::new()
        .merge(routes::create_routes())
        .layer(TraceLayer::new_for_http())
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    
    tracing::info!("Server running on http://0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}
