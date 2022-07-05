use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use validator::Validate;

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
    
    match payload.validate() {
        Ok(()) => (),
        Err(_err) => {
            return HttpResponse::BadRequest().body("Bad payload");
        }
    }
    
    let result = User::register(
        &payload,
        db_pool.get_ref()
    ).await;

    match result {
        Ok(result) => HttpResponse::Ok().json(""),
        Err(err) => {
            error!("error while registered user: {}", err);
            HttpResponse::InternalServerError().body("Error occured while registered user.")
        }
    }
}
