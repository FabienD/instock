use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::PgPool;


#[derive(Debug, FromRow, Serialize)]
pub struct Brand {
    id: i32,
    name: String,
    description: Option<String>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Product {
    id: i32,
    name: String,
    description: Option<String>,
    brand: Brand,
    url: Option<String>,
    upc: Option<String>,
}

impl Responder for Product {
    type Body = BoxBody;
    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        // create response and set content type
        HttpResponse::Ok().json(&self)
    }
}

impl Product {
    pub async fn get(
        pool: &PgPool
    ) -> Result<Vec<Product>> {
        let products = sqlx::query!(
            r#"
            SELECT
                p.id,
                p.name,
                p.description,
                p.url,
                p.upc,
                p.brand_id,
                b.name as brand_name,
                b.description as brand_description
            FROM instock.product AS p
                JOIN instock.brand AS b ON b.id = p.brand_id
            ORDER BY p.id
            "#
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|rec| Product {
            id: rec.id,
            name: rec.name,
            description: rec.description,
            url: rec.url,
            upc: rec.upc,
            brand: Brand {
                id: rec.brand_id,
                name: rec.brand_name,
                description: rec.brand_description
            }
        })
        .collect();

        Ok(products)
    }
}
