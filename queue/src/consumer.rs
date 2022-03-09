use futures_lite::StreamExt;
use dotenv::dotenv;
use std::{env, thread, time::Duration};
use lapin::{options::*, types::FieldTable};
use rand::Rng;

mod models;
mod config;
mod scrap;

pub use crate::config::*;
pub use crate::models::*;
pub use crate::scrap::*;

fn main() {
    dotenv().ok();

    // Init rabbitMQ
    let rmq_dns = env::var("RABBITMQ_DSN")
        .expect("RABBITMQ_DSN is not set in .env file");

    async_global_executor::block_on(async {

        let conn = init_rmq(RmqConfig{
            dsn: rmq_dns,
        }).await.expect("connection error");

        let channel = conn.create_channel().await.expect("queue_declare");
        
        channel
            .queue_declare(
                "instock",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
        .await
        .expect("Declare channel");
        
        let mut consumer = channel
            .basic_consume(
                "instock",
                "instock_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("basic_consume");        
        
        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                // Scraping product
                handle_message(&delivery)
                    .await
                    .expect("Process message");
                
                // Wait a little before next message
                let wait_ms = rand::thread_rng().gen_range(1500..3000);
                thread::sleep(Duration::from_millis(wait_ms));

                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("basic_ack");
            }
        }
    });
}
