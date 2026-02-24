use std::str::FromStr;

use crate::{
    AppState,
    models::{
        Claims, CreateCustomer, CreateCustomerNote, Customer, CustomerDetail, CustomerNote,
        CustomerWithProperties,
    },
};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;

pub async fn list_customers(
    State(state): State<AppState>,
) -> Result<Json<Vec<CustomerWithProperties>>, StatusCode> {
    let result = state
        .db
        .list_customers()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
}

pub async fn customer_detail(
    State(state): State<AppState>,
    Path(customer_id): Path<Uuid>,
) -> Result<Json<CustomerDetail>, StatusCode> {
    let customer = state
        .db
        .customer_detail(customer_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(customer))
}

pub async fn create_customer(
    State(state): State<AppState>,
    Json(body): Json<CreateCustomer>,
) -> Result<(StatusCode, Json<Customer>), StatusCode> {
    let customer = state
        .db
        .create_customer(&body)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(customer)))
}

#[derive(Deserialize)]
pub struct AssignDanisan {
    pub danisan_id: Uuid,
}

pub async fn assign_consultant(
    State(state): State<AppState>,
    Path(customer_id): Path<Uuid>,
    Json(body): Json<AssignDanisan>,
) -> Result<Json<Customer>, StatusCode> {
    let customer = state
        .db
        .assign_consultant(customer_id, body.danisan_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(customer))
}

pub async fn create_customer_note(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(customer_id): Path<Uuid>,
    Json(body): Json<CreateCustomerNote>,
) -> Result<(StatusCode, Json<CustomerNote>), StatusCode> {
    let note = state
        .db
        .create_customer_note(customer_id, Uuid::from_str(&claims.sub).unwrap(), &body)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(note)))
}

pub async fn delete_customer_note(
    State(state): State<AppState>,
    Path((_customer_id, note_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, StatusCode> {
    state
        .db
        .delete_customer_note(note_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(StatusCode::OK)
}
