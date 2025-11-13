use axum::extract::FromRef;
use pokefight_core::PokeService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub poke_service: Arc<PokeService>,
}

impl FromRef<AppState> for Arc<PokeService> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.poke_service.clone()
    }
}
