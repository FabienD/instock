use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::tracking::Filter;
use crate::tracking::Tracking;

// Configuration
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_last_tracking);
}

#[get("/last")]
async fn get_last_tracking(
    filter: web::Query<Filter>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result = Tracking::get_last(&filter, db_pool.get_ref()).await;

    match result {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(err) => {
            error!("error fetching products: {}", err);
            HttpResponse::InternalServerError().body("Error occured while getting products.")
        }
    }
}
