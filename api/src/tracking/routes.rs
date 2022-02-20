use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::tracking::Tracking;

// Configuration
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_last_products);
}

#[get("/last")]
async fn get_last_products(
    db_pool: web::Data<PgPool>
) -> impl Responder {

    let result = Tracking::get(db_pool.get_ref()).await;

    match result {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(err) => {
            error!("error fetching products: {}", err);
            HttpResponse::InternalServerError()
                .body("Error occured while getting products.")
        }
    }
}
