#[macro_use]
extern crate log;

use actix_web::{http::header, web, App, HttpServer};
use actix_cors::Cors;
use anyhow::Result;
use dotenv::dotenv;
use std::env;

// import tracking module (routes and models)
mod tracking;
mod product;
mod config;

pub use crate::config::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // Log configuration
    let syslog_dns = env::var("SYSLOG_DSN")
        .expect("SYSLOG_DSN is not set in .env file");
    
    init_logger(LoggerConfig {
        host: syslog_dns,
        facility: "local5".to_string(),
        log_level: "Info".to_string(),
    });
    
    // PostgreSQL configuration
    let pg_dsn = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set in .env file");
    
    let pool = init_db(DbConfig {
        dsn: pg_dsn.to_string(),
    }).await?;
    
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