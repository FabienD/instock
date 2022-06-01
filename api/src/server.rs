use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{http::header, guard, web, App, HttpServer};
use dotenv::dotenv;
use log::info;
use sqlx::PgPool;
use std::env;

use crate::default::health_check;
use crate::product;
use crate::tracking;

pub fn run_server(pool: PgPool) -> Result<Server, std::io::Error> {
    dotenv().ok();

    let allowed_cors = env::var("ALLOWED_CORS_ORIGIN")
        .expect("ALLOWED_CORS_ORIGIN address is not set in .env file");

    let api_host = env::var("API_HOST").expect("API_HOST is not set in .env file");
    let api_port = env::var("API_PORT").expect("API_PORT is not set in .env file");
    let server_addr = format!("{api_host}:{api_port}");

    // Let's start HTTP server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(allowed_cors.as_str())
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api/tracking").configure(tracking::init))
            .service(web::scope("/api/product").configure(product::init))
            .route("/health_check", 
                web::route()
                        .guard(guard::Any(guard::Get()).or(guard::Head()))
                        .to(health_check),
            )
    })
    .bind(server_addr.as_str())?
    .run();

    info!("Actix server started.");

    Ok(server)
}
