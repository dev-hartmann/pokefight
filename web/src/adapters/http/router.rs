use super::{
    app_state::AppState,
    routes::{health, tournament},
};
use axum::Router;

pub fn init_router() -> Router<AppState> {
    let api_routes = Router::new()
        .nest("/healthz", health::router())
        .nest("/tournament", tournament::router());

    Router::new().nest("/api", api_routes)
}
