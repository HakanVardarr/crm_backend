use crate::AppState;
use crate::models::{Claims, CreateUser, LoginRequest, LoginResponse, User};
use axum::{Json, extract::State, http::StatusCode};
use axum::{extract::Request, http::header, middleware::Next, response::Response};
use jsonwebtoken::{DecodingKey, Validation, decode};
use jsonwebtoken::{EncodingKey, Header, encode};

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let user = state
        .db
        .find_user_by_email(&body.email)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?; // kullanıcı yoksa 401

    let valid = bcrypt::verify(&body.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET ayarlanmamış");

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email,
        is_admin: user.is_admin,
        exp: chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(7))
            .unwrap()
            .timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse { token }))
}

pub async fn auth_middleware(
    State(_state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET ayarlanmamış");
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?
    .claims;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
