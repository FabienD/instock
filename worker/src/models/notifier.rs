use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackedProductLinks {
    pub merchant_product_url: String,
    pub merchant: String,
    pub price: String,
    pub is_in_stock: bool,
    #[serde(with = "ts_seconds")]
    pub tracked_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackedProduct {
    pub product_id: Uuid,
    pub product_name: String,
    pub links: Vec<TrackedProductLinks>,
}

#[derive(Debug)]
pub struct Subscriber {
    pub id: Uuid,
    pub email: String,
}
