use dotenv::dotenv;
use lapin::{options::*, types::FieldTable, BasicProperties};
use isahc::{prelude::*, HttpClient};
use std::{
    env,
    time::Duration,
};

mod config;
mod models;

pub use config::*;
pub use models::*;

fn main() {
    dotenv().ok();

    let rmq_dns = env::var("RABBITMQ_DSN").expect("RABBITMQ_DSN is not set in .env file");
    let api_dsn = env::var("API_SERVER_DSN").expect("API_DSN is not set in .env file");


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

        // Get last tracking, use our API.
        let client = HttpClient::builder()
            .timeout(Duration::from_secs(2))
            .build()
            .expect("Build Http client");
        
        let url = format!("http://{api_dsn}/api/tracking/last");
        let mut response = client.get(url).expect("Process request");
        
        if response.status().as_u16() <= 400 {
            let reponse_content = response.text().expect("Content");
            let last_tracked_products: Vec<TrackedProduct> = serde_json::from_str(reponse_content.as_str()).unwrap();

            for product in last_tracked_products {
                for link in product.links {
                    // Push product_id in notifier queue when available
                    if link.is_in_stock {
                        let payload = serde_json::to_string(&product.product_id).unwrap();
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
                        break;
                    }
                }
            }
        }
    });
}
