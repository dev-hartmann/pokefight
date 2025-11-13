use axum::{routing::get, Router};

use crate::adapters::http::app_state::AppState;

pub async fn health() -> &'static str {
    "Serivce is running"
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(health))
}
