use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Failed to connect to database.")]
    ConnectionFailed,
    #[error("Failed to migrate.")]
    MigrationFailed,
}

mod customer;
mod property;
mod reminder;
mod user;

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, DatabaseError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .map_err(|_| DatabaseError::ConnectionFailed)?;

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|_| DatabaseError::MigrationFailed)?;

        Ok(Self { pool })
    }
}
