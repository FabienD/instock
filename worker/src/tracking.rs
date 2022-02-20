use sqlx::postgres::PgPool;
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use crate::ScrapProductInfo;


#[derive(Debug, FromRow)]
pub struct Tracking {
    product_id: i32,
    is_in_stock: bool,
    tracked_at: DateTime<Utc>,
}

impl Tracking {
    pub async fn insert(
        info: &ScrapProductInfo,
        url: &str,
        pool: &PgPool
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tracking = sqlx::query(
            "
            WITH product_id AS (
                SELECT id FROM instock.product
                WHERE url = $1
            )
            INSERT INTO instock.tracking (product_id, is_in_stock, tracked_at) VALUES (
                (SELECT id FROM product_id), $2, now()
            )
            RETURNING product_id
            "
        )
        .bind(url)
        .bind(info.in_stock)
        .execute(pool)
        .await;
        
        match tracking {
            Err(e) => eprintln!("An error occurred while inserting tracking results : {}", e),
            Ok(_) => ()
        }

        Ok(())
    }
}
