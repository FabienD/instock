use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::PgPool;
use sqlx::types::Uuid;
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;


#[derive(Debug, FromRow, Serialize)]
pub struct Tracking {
    product_id: Uuid,
    product_name: String,
    product_url: String,
    product_merchant: String,
    is_in_stock: bool,
    #[serde(with = "ts_seconds")]
    tracked_at: DateTime<Utc>,
}

impl Responder for Tracking {
    type Body = BoxBody;
    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        // create response and set content type
        HttpResponse::Ok().json(&self)
    }
}

impl Tracking {
    pub async fn get(
        pool: &PgPool
    ) -> Result<Vec<Tracking>> {
        let products = sqlx::query!(
            r#"
            SELECT 
                DISTINCT ON (t.merchant_product_id) t.merchant_product_id, 
                p.name as product_name,
                mp.url as product_url,
                m.name as product_merchant,
                t.is_in_stock, 
                t.tracked_at 
            FROM instock.tracking AS t
                JOIN instock.merchant_product AS mp ON mp.id = t.merchant_product_id
                    JOIN instock.product AS p  ON p.id = mp.product_id
                    JOIN instock.merchant AS m ON m.id = mp.merchant_id
            ORDER BY t.merchant_product_id, t.tracked_at DESC
            "#
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|rec| Tracking {
            product_id: rec.merchant_product_id,
            product_name: rec.product_name,
            product_url: rec.product_url,
            product_merchant: rec.product_merchant,
            is_in_stock: rec.is_in_stock,
            tracked_at: rec.tracked_at,
        })
        .collect();

        Ok(products)
    }
}
