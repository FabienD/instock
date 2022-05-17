use lapin::message::Delivery;
use sqlx::PgPool;

pub use crate::models::*;

pub async fn handle_message(delivery: &Delivery, pool: &PgPool) {
    let product_id = std::str::from_utf8(&delivery.data).unwrap();
    let product = TrackedProduct::get(product_id, &pool).await;

    println!("{:?}", product);
}
