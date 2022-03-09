use sqlx::postgres::PgPool;
use sqlx::FromRow;
use sqlx::types::Uuid;

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "merchant")]
pub enum Merchant {
    AmazonFr,
    FnacFr,
    CDiscountFr,
}

#[derive(Debug, FromRow)]
pub struct MerchantProduct {
    pub id: Uuid,
    pub merchant: Merchant,
    pub url: String,
    pub tracked: Option<bool>,
}

impl MerchantProduct {
    pub async fn list_tracked_products(
        pool: &PgPool
    ) -> Result<Vec<MerchantProduct>, Box<dyn std::error::Error>> {
        let products = sqlx::query!(
            r#"
                SELECT 
                    mp.id,
                    mp.merchant as "merchant!: Merchant",
                    mp.url,
                    mp.tracked
                FROM instock.merchant_product AS mp
                WHERE mp.tracked
            "#
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|rec| MerchantProduct {
            id: rec.id,
            merchant: rec.merchant,
            url: rec.url,
            tracked: rec.tracked
        })
        .collect();
        
        Ok(products)
    }
}