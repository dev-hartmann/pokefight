use axum::{extract::State, http::StatusCode, Json, Router};
use pokefight_core::MatchResult;
use serde::{Deserialize, Serialize};

use crate::{adapters::http::app_state::AppState, application::tournament::TournamentService};

#[derive(Debug, Serialize, Deserialize)]
pub struct TournamentRequest {
    names: Vec<String>,
    chore: String,
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TournamentResponse {
    name: String,
    chore: String,
    matches: Vec<MatchResult>,
    champion: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: String,
}

#[axum::debug_handler]
pub async fn tournament(
    State(app_state): State<AppState>,
    Json(req): Json<TournamentRequest>,
) -> Result<(StatusCode, Json<TournamentResponse>), (StatusCode, Json<ErrorResponse>)> {
    let poke_service = app_state.poke_service.clone();
    match TournamentService::run_tournament(req.names, req.title, req.chore, poke_service).await {
        Ok(result) => Ok((
            StatusCode::OK,
            Json(TournamentResponse {
                name: result.name,
                chore: result.chore,
                matches: result.matches,
                champion: result.champion,
            }),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", axum::routing::post(tournament))
}
