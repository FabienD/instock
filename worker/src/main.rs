use sqlx::postgres::PgPool;
use std::env;
use dotenv::dotenv;

mod scrap;
mod model;
pub use scrap::*;
pub use model::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use .env file to create env var.
    dotenv().ok();
    // Pg pool
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set in .env file");

    let pool = PgPool::connect(&database_url).await?;
    // Get then parse all urls results
    process_request(&pool).await;
        
    Ok(())
}
