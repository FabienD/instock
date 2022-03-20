use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use sqlx::types::Json;
use sqlx::types::Uuid;
use sqlx::FromRow;
use sqlx::PgPool;


#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ScrapingElements {
    pub title: String,
    pub cart: String,
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
    pub async fn get(pool: &PgPool) -> Result<Vec<MerchantProduct>> {
        let merchant_products = sqlx::query!(
            r#"
            SELECT
                mp.id,
                mp.url,
                m.id as merchant_id,
                m.scraping_elements as "scraping_elements: Json<ScrapingElements>"
            FROM instock.merchant_product AS mp
                JOIN instock.merchant AS m ON m.id = mp.merchant_id
            WHERE mp.tracked IS TRUE
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
            },
        })
        .collect();

        Ok(merchant_products)
    }
}

pub struct Tracking {
    pub product_id: Uuid,
    pub is_in_stock: bool,
}

impl Tracking {
    pub async fn save(
        &self,
        pool: &PgPool
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tracking = sqlx::query(
            "
            INSERT INTO instock.tracking (merchant_product_id, is_in_stock, tracked_at) VALUES (
                $1, $2, now()
            )
            RETURNING merchant_product_id
            "
        )
        .bind(self.product_id)
        .bind(self.is_in_stock)
        .execute(pool)
        .await;
        
        match tracking {
            Err(e) => eprintln!("An error occurred while inserting tracking results : {}", e),
            Ok(_) => ()
        }

        Ok(())
    }
}