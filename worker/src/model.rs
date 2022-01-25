use serde::{Serialize, Deserialize};
use sqlx::postgres::PgPool;
use sqlx::FromRow;

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "merchant")]
pub enum Merchant {
    AmazonFr,
    FnacFr,
    CDiscountFr,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ProductUrl {
    pub id: i32,
    pub merchant: Merchant,
    pub url: String,
    pub name: String,
    pub description: Option<String>,
    pub upc: Option<String>,
    pub tracked: Option<bool>,
}

impl ProductUrl {
    pub async fn list_tracked_products(
        pool: &PgPool
    ) -> Result<Vec<ProductUrl>, Box<dyn std::error::Error>> {
        let product_urls = sqlx::query!(
            r#"
                SELECT 
                    p.id,
                    p.merchant as "merchant!: Merchant",
                    p.url,
                    p.name,
                    p.description,
                    p.upc,
                    p.tracked
                FROM product AS p
                WHERE p.tracked
            "#
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|rec| ProductUrl {
            id: rec.id,
            merchant: rec.merchant,
            url: rec.url,
            name: rec.name,
            description: rec.description,
            upc: rec.upc,
            tracked: rec.tracked
        })
        .collect();
        
        Ok(product_urls)
    }
}