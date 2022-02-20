use sqlx::postgres::PgPool;
use sqlx::FromRow;

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "merchant")]
pub enum Merchant {
    AmazonFr,
    FnacFr,
    CDiscountFr,
}

#[derive(Debug, FromRow)]
pub struct Product {
    pub id: i32,
    pub merchant: Merchant,
    pub url: String,
    pub name: String,
    pub description: Option<String>,
    pub upc: Option<String>,
    pub tracked: Option<bool>,
}

impl Product {
    pub async fn list_tracked_products(
        pool: &PgPool
    ) -> Result<Vec<Product>, Box<dyn std::error::Error>> {
        let products = sqlx::query!(
            r#"
                SELECT 
                    p.id,
                    p.merchant as "merchant!: Merchant",
                    p.url,
                    p.name,
                    p.description,
                    p.upc,
                    p.tracked
                FROM instock.product AS p
                WHERE p.tracked
            "#
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|rec| Product {
            id: rec.id,
            merchant: rec.merchant,
            url: rec.url,
            name: rec.name,
            description: rec.description,
            upc: rec.upc,
            tracked: rec.tracked
        })
        .collect();
        
        Ok(products)
    }
}