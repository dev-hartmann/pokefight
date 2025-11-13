use adapters::http::router::init_router;
use infra::app_state::init_app_state;

mod adapters;
mod application;
mod infra;

#[tokio::main]
async fn main() {
    println!("PokeFight Web Service starting...");

    // Initialize AppState
    let app_state = init_app_state().await;

    // Initialize router with state
    let router = init_router().with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("Server listening on http://127.0.0.1:3001");
    axum::serve(listener, router).await.unwrap();
}
