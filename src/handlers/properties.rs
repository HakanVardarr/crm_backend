use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use serde::Deserialize;

use crate::{AppState, models::PaginatedProperties};

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
}

pub async fn list_properties(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedProperties>, StatusCode> {
    let page = params.page.unwrap_or(1);

    let result = state
        .db
        .list_properties(page, 50)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
}
