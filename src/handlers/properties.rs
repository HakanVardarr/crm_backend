use crate::{AppState, models::PropertyWithCustomer};
use axum::{Json, extract::State, http::StatusCode};

pub async fn list_properties(
    State(state): State<AppState>,
) -> Result<Json<Vec<PropertyWithCustomer>>, StatusCode> {
    let result = state
        .db
        .list_properties()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
}
