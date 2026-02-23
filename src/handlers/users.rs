use axum::{Json, extract::State, http::StatusCode};

use crate::{AppState, models::User};

pub async fn list_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    if let Ok(users) = state.db.list_users().await {
        Ok(Json(users))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
