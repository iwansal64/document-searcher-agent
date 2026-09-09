use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct AccessToken {
        pub id: i32,
        pub token: String,
        pub created_at: DateTime<Utc>,
}
