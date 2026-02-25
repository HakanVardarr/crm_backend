use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, sqlx::FromRow)]
pub struct Reminder {
    pub id: Uuid,
    pub title: String,
    pub reminder_date: chrono::DateTime<chrono::Utc>,
    pub created_by: Uuid,
    pub customer_id: Uuid,
}

#[derive(Deserialize)]
pub struct CreateReminder {
    pub title: String,
    pub reminder_date: chrono::DateTime<chrono::Utc>,
}
