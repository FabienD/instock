use dotenv::dotenv;
use futures_lite::StreamExt;
use lapin::{options::*, types::FieldTable};
use std::env;

mod config;
mod mailer;
mod models;
mod notifier;

pub use config::*;
pub use mailer::*;
pub use models::*;
pub use notifier::*;

fn main() {
    dotenv().ok();

    // Init rabbitMQ
    let rmq_dns = env::var("RABBITMQ_DSN").expect("RABBITMQ_DSN is not set in .env file");
    // Postgres
    let pg_dsn = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    // SMTP
    let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST is not set in .env file");
    let smtp_port = env::var("SMTP_PORT").unwrap_or("25".to_string());
    let smtp_user = env::var("SMTP_USER").unwrap_or("".to_string());
    let smtp_password = env::var("SMTP_PASSWORD").unwrap_or("".to_string());
    // Notification
    let notification_from =
        env::var("NOTIFICATION_FROM").expect("NOTIFICATION_FROM is not set in .env file");

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

        // SMTP
        let mailer = init_mailer(
            smtp_host,
            smtp_port.parse::<u16>().unwrap(),
            smtp_user,
            smtp_password,
        )
        .expect("Mailer");

        // Rabbitmq
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
                handle_message(&delivery, &pool, &mailer, &notification_from).await;

                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("basic_ack");
            }
        }
    });
}
