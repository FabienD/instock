use actix_web::body::BoxBody;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde::Deserialize;
use sqlx::types::Uuid;
use sqlx::FromRow;
use sqlx::PgPool;


#[derive(Debug, Deserialize)]
pub struct Filter {
    pub only_positive: Option<String>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Tracking {
    product_id: Uuid,
    product_name: String,
    links: Vec<TrackingLink>,
}

#[derive(Debug, FromRow, Serialize, sqlx::Type)]
pub struct TrackingLink {
    merchant_product_url: String,
    merchant: String,
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
    pub async fn get_last(filter: &web::Query<Filter>, pool: &PgPool) -> Result<()> {
        
        let mut sql_filter = "";
        let mut only_positive: bool = false;
        
        match filter.only_positive.to_owned() {
            Some(f) => {
                if matches!(f.as_str(), "true"|"t"|"1") {
                    sql_filter = "t.is_in_stock";
                }
            },
            _ => {},
        }
        
        let query = r#"WITH last_tracking AS (
            SELECT 
                DISTINCT ON (t.merchant_product_id) t.merchant_product_id, 
                t.is_in_stock, 
                t.tracked_at
            FROM instock.tracking AS t
            WHERE {sql_filter}
            ORDER BY t.merchant_product_id, t.tracked_at DESC
        ), tracked_products AS (
            SELECT
                p.id as product_id,
                p.name as product_name,
                m.name as merchant,
                mp.url as product_merchant_url,
                lt.is_in_stock, 
                lt.tracked_at
            FROM last_tracking AS lt
                JOIN instock.merchant_product AS mp ON mp.id = lt.merchant_product_id
                JOIN instock.product AS p ON p.id = mp.product_id
                JOIN instock.merchant AS m ON m.id = mp.merchant_id
        )
        SELECT 
            tp.product_id,
            tp.product_name,
            array_agg((
                tp.product_merchant_url,
                tp.merchant,
                tp.is_in_stock,
                tp.tracked_at
            )) as "links!: Vec<TrackingLink>" 
        FROM tracked_products AS tp 
        GROUP BY tp.product_id, tp.product_name
        "#;
        
        

        Ok(())
    }
}
