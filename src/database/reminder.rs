use uuid::Uuid;

use crate::{
    database::Database,
    models::{CreateReminder, Reminder},
};

impl Database {
    pub async fn create_reminder(
        &self,
        customer_id: Uuid,
        created_by: Uuid,
        body: &CreateReminder,
    ) -> Result<Reminder, sqlx::Error> {
        sqlx::query_as::<_, Reminder>(
            "INSERT INTO reminders (title, reminder_date, created_by, customer_id)
         VALUES ($1, $2, $3, $4)
         RETURNING *",
        )
        .bind(&body.title)
        .bind(body.reminder_date)
        .bind(created_by)
        .bind(customer_id)
        .fetch_one(&self.pool)
        .await
    }
}
