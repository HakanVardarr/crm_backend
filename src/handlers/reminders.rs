use crate::{
    AppState,
    models::{Claims, CreateReminder, Reminder},
};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use std::str::FromStr;
use uuid::Uuid;

pub async fn create_reminder(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(customer_id): Path<Uuid>,
    Json(body): Json<CreateReminder>,
) -> Result<(StatusCode, Json<Reminder>), StatusCode> {
    let user_id = Uuid::from_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let reminder = state
        .db
        .create_reminder(customer_id, user_id, &body)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(reminder)))
}
