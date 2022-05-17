use dotenv::dotenv;
use futures_lite::StreamExt;
use lapin::{options::*, types::FieldTable};
use std::env;

mod config;
mod models;
mod notifier;

pub use config::*;
pub use models::*;
pub use notifier::*;

fn main() {
    dotenv().ok();

    // Init rabbitMQ
    let rmq_dns = env::var("RABBITMQ_DSN").expect("RABBITMQ_DSN is not set in .env file");
    // Postgres
    let pg_dsn = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

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

        channel
            .queue_declare(
                "notifier",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("Declare channel");

        let mut consumer = channel
            .basic_consume(
                "notifier",
                "notifier_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("basic_consume");

        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                // Notify user by product
                handle_message(&delivery, &pool).await;

                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("basic_ack");
            }
        }
    });
}
