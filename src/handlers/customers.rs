use crate::{
    AppState,
    models::{CreateCustomer, CreateCustomerNote, Customer, CustomerNote},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;

pub async fn list_customers(
    State(state): State<AppState>,
) -> Result<Json<Vec<Customer>>, StatusCode> {
    let customers = state
        .db
        .list_customers()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(customers))
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
    Path(customer_id): Path<Uuid>,
    Json(body): Json<CreateCustomerNote>,
) -> Result<(StatusCode, Json<CustomerNote>), StatusCode> {
    let note = state
        .db
        .create_customer_note(customer_id, &body)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(note)))
}
