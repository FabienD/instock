use actix_web::body::BoxBody;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow, PgPool};

#[derive(Debug, Deserialize)]
pub struct Filter {
    pub only_positive: Option<String>,
    pub since_hours: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Tracking {
    product_id: Uuid,
    product_name: String,
    links: Vec<TrackingLink>,
}

#[derive(Debug, Serialize, sqlx::Type, FromRow)]
pub struct TrackingLink {
    merchant_product_url: String,
    merchant: String,
    price: String,
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
    pub async fn get_last(filter: &web::Query<Filter>, pool: &PgPool) -> Result<Vec<Tracking>> {
        let mut sql_instock_filter = vec![true, false];
        let mut sql_tracked_at_since_hour_filter:i64 = 24;

        match filter.only_positive.to_owned() {
            Some(f) => {
                if matches!(f.as_str(), "true" | "t" | "1") {
                    sql_instock_filter = vec![true];
                }
            }
            _ => {}
        }
        
        match filter.since_hours.to_owned() {
            Some(f) => {
                sql_tracked_at_since_hour_filter = f.parse::<i64>().unwrap_or(sql_tracked_at_since_hour_filter);
            }
            _ => {}
        }

        let products = sqlx::query_as!(
            Tracking,
            r#"WITH last_tracking AS (
                SELECT 
                    DISTINCT ON (t.merchant_product_id) t.merchant_product_id,
                    t.price,
                    t.is_in_stock, 
                    t.tracked_at
                FROM instock.tracking AS t
                WHERE t.tracked_at > $2
                ORDER BY t.merchant_product_id, t.tracked_at DESC
            ), tracked_products AS (
                SELECT
                    p.id as product_id,
                    p.name as product_name,
                    m.name as merchant,
                    COALESCE(mp.affiliate_url, mp.url) as product_merchant_url,
                    lt.price,
                    lt.is_in_stock, 
                    lt.tracked_at
                FROM last_tracking AS lt
                    JOIN instock.merchant_product AS mp ON mp.id = lt.merchant_product_id
                    JOIN instock.product AS p ON p.id = mp.product_id
                    JOIN instock.merchant AS m ON m.id = mp.merchant_id
                WHERE lt.is_in_stock = ANY($1) 
                ORDER BY p.id, m.name
            )
            SELECT 
                tp.product_id,
                tp.product_name,
                array_agg((
                    tp.product_merchant_url,
                    tp.merchant,
                    tp.price,
                    tp.is_in_stock,
                    tp.tracked_at
                )) as "links!: Vec<TrackingLink>"
            FROM tracked_products AS tp 
            GROUP BY tp.product_id, tp.product_name
            "#,
            &sql_instock_filter, 
            Utc::now() - Duration::hours(sql_tracked_at_since_hour_filter)
        )
        .fetch_all(pool)
        .await?;

        Ok(products)
    }
}
