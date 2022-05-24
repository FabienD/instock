use dotenv::dotenv;
use isahc::{prelude::*, HttpClient};
use lapin::{options::*, types::FieldTable, BasicProperties};
use std::{env, time::Duration};

mod config;
mod models;

pub use config::*;
pub use models::*;

fn main() {
    dotenv().ok();

    let rmq_dns = env::var("RABBITMQ_DSN").expect("RABBITMQ_DSN is not set in .env file");
    let api_host = env::var("API_HOST").expect("API_HOST is not set in .env file");
    let api_port = env::var("API_PORT").expect("API_PORT is not set in .env file");
    let api_scheme = env::var("API_SCHEME").expect("API_SCHEME is not set in .env file");

    async_global_executor::block_on(async {
        // RabbitMQ
        let conn = init_rmq(RmqConfig { dsn: rmq_dns })
            .await
            .expect("RMQ connection");

        let channel = conn.create_channel().await.expect("Channel create");

        channel
            .queue_declare(
                "notifier",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("Queue declared");

        // Get last tracking, use our API with filter
        let client = HttpClient::builder()
            .timeout(Duration::from_secs(2))
            .build()
            .expect("Build Http client");

        let url =
            format!("{api_scheme}://{api_host}:{api_port}/api/tracking/last?only_positive=true");
        let mut response = client.get(url).expect("Process API request");

        if response.status().as_u16() <= 400 {
            let reponse_content = response.text().expect("Content");
            let last_tracked_products: Vec<TrackedProduct> =
                serde_json::from_str(reponse_content.as_str()).unwrap();

            for product in last_tracked_products {
                let payload = serde_json::to_string(&product).unwrap();
                channel
                    .basic_publish(
                        "",
                        "notifier",
                        BasicPublishOptions::default(),
                        str::as_bytes(&payload),
                        BasicProperties::default(),
                    )
                    .await
                    .expect("basic_publish");
            }
        }
    });
}
