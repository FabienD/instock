extern crate syslog;
#[macro_use]
extern crate log;


use syslog::{Facility, Formatter3164, BasicLogger};
use log::{LevelFilter};
use actix_web::{http::header, web, App, HttpServer};
use actix_cors::Cors;
use anyhow::Result;
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;


// import tracking module (routes and models)
mod tracking;
mod product;


#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // Log configuration
    let syslog_host = env::var("SYSLOG_HOST")
        .expect("SYSLOG_HOST is not set in .env file");

    let formatter = Formatter3164 {
        facility: Facility::LOG_LOCAL5,
        hostname: None,
        process: "instock_api".into(),
        pid: 0,
    };

    let logger = syslog::udp(formatter, "localhost:1234", &syslog_host)
        .expect("could not connect to syslog");

    if let Ok(()) = log::set_boxed_logger(Box::new(BasicLogger::new(logger))) {
        log::set_max_level(LevelFilter::Info);
    }
    
    // Database configuration
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set in .env file");

    let pool = PgPool::connect(&database_url).await?;
    
    // Let's start HTTP server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api/tracking").configure(tracking::init))
            .service(web::scope("/api/product").configure(product::init))
    })
    .bind("127.0.0.1:8080")?;

    server.run().await?;

    Ok(())
}
