use std::sync::Arc;
use pokefight_core::PokeService;

use crate::adapters::http::app_state::AppState;

pub async fn init_app_state() -> AppState {
    let poke_service = Arc::new(PokeService::default());
    AppState { poke_service }
}
