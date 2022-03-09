use dotenv::dotenv;
use std::env;
use lapin::{options::*, types::FieldTable, BasicProperties};

mod models;
mod config;

pub use crate::config::*;
pub use crate::models::*;

fn main() {
    dotenv().ok();

    let rmq_dns = env::var("RABBITMQ_DSN")
        .expect("RABBITMQ_DSN is not set in .env file");

    let pg_dsn = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set in .env file");


    async_global_executor::block_on(async {
        // RabbitMQ
        let conn = init_rmq(RmqConfig {
            dsn: rmq_dns,
        })
        .await.expect("RMQ connection");

        let channel = conn.create_channel()
        .await.expect("Channel create");
        
        channel
            .queue_declare(
                "instock",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
        .await.expect("Queue declared");
        
        // PostgreSQL
        let pool = init_db(DbConfig {
            dsn: pg_dsn.to_string(),
        })
        .await.expect("DB pool create");
        
        // Get products for scraping
        let merchant_products = models::MerchantProduct::get(&pool)
        .await.expect("Get merchant products");

        // Publish messages
        for message in merchant_products {

            let payload = serde_json::to_string(&message).unwrap();

            channel.basic_publish(
                "",
                "instock",
                BasicPublishOptions::default(),
                str::as_bytes(&payload),
                BasicProperties::default(),
            )
            .await
            .expect("basic_publish");
        }
    });
}