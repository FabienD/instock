use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::PgPool;
use sqlx::types::Uuid;
use sqlx::types::Json;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ScrapingElements {
    pub title: String,
    pub cart: String
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Merchant {
    pub id: Option<Uuid>,
    pub scraping_elements: Option<Json<ScrapingElements>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct MerchantProduct {
    pub id: Option<Uuid>,
    pub url: Option<String>,
    pub merchant: Merchant,
}

impl MerchantProduct {
    pub async fn get(
        pool: &PgPool
    ) -> Result<Vec<MerchantProduct>> {
        let merchant_products = sqlx::query!(
            r#"
            SELECT
                mp.id,
                mp.url,
                m.id as merchant_id,
                m.scraping_elements as "scraping_elements: Json<ScrapingElements>"
            FROM instock.merchant_product AS mp
                JOIN instock.merchant AS m ON m.id = mp.merchant_id
            WHERE mp.tracked
            "#
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|rec| MerchantProduct {
            id: rec.id,
            url: rec.url,
            merchant: Merchant {
                id: rec.merchant_id,
                scraping_elements: rec.scraping_elements,
            }
        })
        .collect();

        Ok(merchant_products)
    }
}