#[macro_use]
extern crate log;

use anyhow::Result;
use dotenv::dotenv;
use std::env;

// import tracking module (routes and models)
mod config;
mod default;
mod product;
mod server;
mod tracking;
mod user;

pub use crate::config::*;
pub use crate::server::run_server;

#[tokio::main]
#[allow(unused_must_use)]
async fn main() -> Result<()> {
    dotenv().ok();

    // Log configuration
    let syslog_dns = env::var("SYSLOG_DSN").expect("SYSLOG_DSN is not set in .env file");

    init_logger(LoggerConfig {
        host: syslog_dns,
        facility: "local5".to_string(),
        log_level: "Info".to_string(),
    });

    // PostgreSQL configuration
    let pg_dsn = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let pool = init_db(DbConfig {
        dsn: pg_dsn.to_string(),
    })
    .await?;

    // Run actix server
    run_server(pool)?.await;

    Ok(())
}
