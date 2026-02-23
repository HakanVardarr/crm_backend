use axum::{
    Router, middleware,
    routing::{get, post},
};

mod database;
mod handlers;
mod models;

#[derive(Clone)]
struct AppState {
    db: database::Database,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL ayarlanmamış");
    let db = database::Database::new(&database_url).await.unwrap();

    let state = AppState { db };

    let protected = Router::new()
        .route("/users", get(handlers::users::list_users))
        .route("/customers", post(handlers::customers::create_customer))
        .route(
            "/customers/:id/consultant",
            post(handlers::customers::assign_consultant),
        )
        .route(
            "/customers/:id/notes",
            post(handlers::customers::create_customer_note),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            handlers::auth::auth_middleware,
        ));

    let public = Router::new()
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login));

    let app = Router::new()
        .merge(public)
        .merge(protected)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Sunucu çalışıyor: http://localhost:8000");
    axum::serve(listener, app).await.unwrap();
}
