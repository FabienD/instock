use serde::{Deserialize, Serialize};

enum Merchant {
    amazon_fr,
}

#[derive(Deserialize, Serialize)]
struct ProductUrl {
    url: String,
    merchant: Merchant,
    tracked: Bool,
}

async fn list_tracked_products(pool: &PgPool) -> anyhow::Result<()> {
    let rows = sqlx::query_as!(
        Row,
        r#"
            SELECT 
                p.url, 
                p.merchant
            FROM product AS p
            WHERE p.tracked
        "#
    )
    .fetch_all(pool)
    .await?;



    Ok(())
}