use dotenv::dotenv;
use futures_lite::StreamExt;
use lapin::{options::*, types::FieldTable};
use rand::Rng;
use std::{env, thread, time::Duration};

mod config;
mod models;
mod scrap;

pub use config::*;
pub use models::*;
pub use scrap::*;

fn main() {
    dotenv().ok();

    // Init rabbitMQ
    let rmq_dns = env::var("RABBITMQ_DSN").expect("RABBITMQ_DSN is not set in .env file");
    // Postgres
    let pg_dsn = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    // Scraping pause
    let scrap_min_pause_sec = env::var("SCRAP_MIN_PAUSE_SEC").unwrap_or("2".to_string());
    let scrap_max_pause_sec = env::var("SCRAP_MAX_PAUSE_SEC").unwrap_or("10".to_string());

    async_global_executor::block_on(async {
        let conn = init_rmq(RmqConfig { dsn: rmq_dns })
            .await
            .expect("connection error");

        // PostgreSQL
        let pool = init_db(DbConfig {
            dsn: pg_dsn.to_string(),
        })
        .await
        .expect("DB pool create");

        let channel = conn.create_channel().await.expect("queue_declare");
        
        // Headless browser manager
        let playwright = init_browser().await.expect("Playright init failed");

        channel
            .queue_declare(
                "scrap",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("Declare channel");

        let mut consumer = channel
            .basic_consume(
                "scrap",
                "instock_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("basic_consume");

        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                // Scraping product
                let tracking = handle_message(&delivery, &playwright).await;
                // Save tracking result
                match tracking {
                    Ok(tracking) => tracking.save(&pool).await.expect("Save tracking result"),
                    Err(e) => println!("{:?}", e),
                };

                // Wait a little before next message
                let wait_ms = rand::thread_rng().gen_range((1000*scrap_min_pause_sec.parse::<u64>().unwrap())..(1000*scrap_max_pause_sec.parse::<u64>().unwrap()));
                thread::sleep(Duration::from_millis(wait_ms));

                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("basic_ack");
            }
        }
    });
}
