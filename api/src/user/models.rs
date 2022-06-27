use anyhow::Result;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    email: String,
}

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    id: Uuid,
    email: UserInfo,
    enabled: bool,
    #[serde(with = "ts_seconds")]
    created_at: DateTime<Utc>
}

#[derive(Debug, FromRow, Serialize)]
pub struct AuthInfos {
    salt: String,
    hash: String,
    #[serde(with = "ts_seconds")]
    created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    updated_at: DateTime<Utc>
}



impl User {
    pub async fn register(
        info: &UserInfo, 
        db: &PgPool
    ) -> Result<()> {
        
        // Check CSRF

        // Check Email doesn't exist

        // Persist User

        
        Ok(())
    }
}