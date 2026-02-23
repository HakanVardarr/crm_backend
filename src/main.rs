use axum::{
    Router,
    http::{Method, header},
    middleware,
    routing::{get, post},
};
use tower_http::cors::{Any, CorsLayer};

mod commands;
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

    // CLI komutları
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "import-properties" => {
                let filepath = args
                    .get(2)
                    .expect("Dosya yolu gerekli: cargo run -- import-properties dosya.xlsx");
                commands::import_properties::run(&db, filepath).await;
                return;
            }
            "import-customers" => {
                let filepath = args
                    .get(2)
                    .expect("Dosya yolu gerekli: cargo run -- import-customers dosya.xlsx");
                commands::import_customers::run(&db, filepath).await;
                return;
            }
            cmd => {
                eprintln!("Bilinmeyen komut: {}", cmd);
                eprintln!("Kullanım: cargo run -- import-properties <dosya.xlsx>");
                return;
            }
        }
    }

    let state = AppState { db };

    let protected = Router::new()
        .route("/users", get(handlers::users::list_users))
        .route("/users/me", get(handlers::users::me))
        .route(
            "/customers",
            get(handlers::customers::list_customers).post(handlers::customers::create_customer),
        )
        .route("/customers/:id", get(handlers::customers::customer_detail))
        .route(
            "/customers/:id/consultant",
            post(handlers::customers::assign_consultant),
        )
        .route(
            "/customers/:id/notes",
            post(handlers::customers::create_customer_note),
        )
        .route("/properties", get(handlers::properties::list_properties))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            handlers::auth::auth_middleware,
        ));

    let public = Router::new()
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);

    let app = Router::new()
        .merge(public)
        .merge(protected)
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Sunucu çalışıyor: http://localhost:8000");
    axum::serve(listener, app).await.unwrap();
}
