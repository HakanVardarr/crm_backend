use axum::{
    Router,
    http::{Method, header},
    middleware,
    routing::{delete, get, post},
};
use tower_http::cors::{Any, CorsLayer};

use crate::models::CreateUser;

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
            "create-superuser" => {
                let name = args.get(2).expect("İsim gerekli");
                let last_name = args.get(3).expect("Soyisim gerekli");
                let email = args.get(4).expect("Email gerekli");
                let password = args.get(5).expect("Şifre gerekli");

                let body = CreateUser {
                    name: name.clone(),
                    last_name: last_name.clone(),
                    email: email.clone(),
                    password: password.clone(),
                    is_admin: true,
                };

                match db.create_user(&body).await {
                    Ok(user) => println!("Superuser oluşturuldu: {} {}", user.name, user.last_name),
                    Err(e) => eprintln!("Hata: {}", e),
                }
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
        .route(
            "/users",
            get(handlers::users::list_users).post(handlers::users::create_user),
        )
        .route("/users/me", get(handlers::users::me))
        .route("/users/:id", delete(handlers::users::delete_user))
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
        .route(
            "/customers/:id/notes/:note_id",
            delete(handlers::customers::delete_customer_note),
        )
        .route("/properties", get(handlers::properties::list_properties))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            handlers::auth::auth_middleware,
        ));

    let public = Router::new().route("/auth/login", post(handlers::auth::login));

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
