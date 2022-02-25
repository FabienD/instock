use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::product::Product;

// Configuration
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_products);
}

#[get("/all")]
async fn get_all_products(
    db_pool: web::Data<PgPool>
) -> impl Responder {

    let result = Product::get(db_pool.get_ref()).await;

    match result {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(err) => {
            error!("error fetching products: {}", err);
            HttpResponse::InternalServerError()
                .body("Error occured while getting products.")
        }
    }
}
