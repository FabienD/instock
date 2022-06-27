use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::user::User;
use crate::user::UserInfo;

// Configuration
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user);
}


#[post("/register")]
async fn register_user(
    payload: web::Form<UserInfo>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {

    let result = User::register(
        &payload,
        db_pool.get_ref()
    ).await;

    match result {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(err) => {
            error!("error while registered user: {}", err);
            HttpResponse::InternalServerError().body("Error occured while registered user.")
        }
    }
}
