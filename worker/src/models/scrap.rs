use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use sqlx::types::Json;
use sqlx::types::Uuid;
use sqlx::FromRow;
use sqlx::PgPool;
use std::fmt;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
pub enum ScrapingMethod {
    Library,
    Browser,
}

impl fmt::Display for ScrapingMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScrapingMethod::Library => write!(f, "library"),
            ScrapingMethod::Browser => write!(f, "browser"),
        }
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ScrapingElements {
    pub title: String,
    pub cart: String,
    pub price: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Merchant {
    pub id: Uuid,
    pub scraping_elements: Json<ScrapingElements>,
    pub scraping_method: ScrapingMethod,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MerchantProduct {
    pub id: Uuid,
    pub url: String,
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
                m.scraping_elements as "scraping_elements: Json<ScrapingElements>",
                m.scraping_method as "scraping_method: ScrapingMethod"
            FROM instock.merchant_product AS mp
                JOIN instock.merchant AS m ON m.id = mp.merchant_id
            WHERE mp.tracked IS TRUE
            ORDER BY md5(random()::text)
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
                scraping_method: rec.scraping_method,
            },
        })
        .collect();

        Ok(merchant_products)
    }
}

#[derive(Debug)]
pub struct Tracking {
    pub product_id: Uuid,
    pub price: String,
    pub is_in_stock: bool,
}

impl Tracking {
    pub async fn save(&self, pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let tracking = sqlx::query(
            "
            INSERT INTO instock.tracking (merchant_product_id, price, is_in_stock, tracked_at) VALUES (
                $1, $2, $3, now()
            )
            RETURNING merchant_product_id
            ",
        )
        .bind(self.product_id)
        .bind(self.price.to_owned())
        .bind(self.is_in_stock)
        .execute(pool)
        .await;

        if let Err(e) = tracking {
            eprintln!("An error occurred while inserting tracking results : {}", e)
        }

        Ok(())
    }
}
