use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    AppState,
    models::{Claims, CreateUser, User},
};

pub async fn list_users(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<User>>, StatusCode> {
    if !claims.is_admin {
        return Err(StatusCode::UNAUTHORIZED);
    }

    if let Ok(users) = state.db.list_users().await {
        Ok(Json(users))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn create_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    if !claims.is_admin {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let user = state
        .db
        .create_user(&body)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    if !claims.is_admin {
        return Err(StatusCode::UNAUTHORIZED);
    }

    if claims.sub == user_id.to_string() {
        return Err(StatusCode::BAD_REQUEST);
    }

    state.db.delete_user(user_id).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(StatusCode::OK)
}

pub async fn me(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<User>, StatusCode> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user = state
        .db
        .get_user_by_id(user_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(user))
}
