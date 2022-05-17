use anyhow::Result;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use sqlx::types::Uuid;
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow, sqlx::Type)]
pub struct TrackedProductLinks {
    pub merchant_product_url: String,
    pub merchant: String,
    pub is_in_stock: bool,
    #[serde(with = "ts_seconds")]
    pub tracked_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TrackedProduct {
    pub product_id: Uuid,
    pub product_name: String,
    pub links: Vec<TrackedProductLinks>,
}

impl TrackedProduct {
    pub async fn get(product_id: &str, pool: &PgPool) -> Result<TrackedProduct> {
        let product_with_positive_tracked_links = sqlx::query_as!(
            TrackedProduct,
            r#"
            SELECT 
                p.id as product_id, 
                p.name as product_name, 
                array_agg((
                    mp.url,
                    m.name,
                    lt.is_in_stock,
                    lt.tracked_at
                )) as "links!: Vec<TrackedProductLinks>"
            FROM (
                SELECT
                    *,
                    RANK () OVER (
                        PARTITION BY t.merchant_product_id
                        ORDER BY t.tracked_at DESC
                    ) rank_number
                FROM instock.tracking AS t
                WHERE t.is_in_stock
            ) as lt
            JOIN instock.merchant_product AS mp ON mp.id = lt.merchant_product_id
            JOIN instock.merchant AS m ON m.id = mp.merchant_id
            JOIN instock.product AS p ON p.id = mp.product_id
            WHERE lt.rank_number = 1 AND p.id = $1
            GROUP BY p.id, p.name
            "#,
            Uuid::parse_str(product_id)?
        )
        .fetch_one(pool)
        .await?;

        Ok(product_with_positive_tracked_links)
    }
}
