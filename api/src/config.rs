use std::{str::FromStr};

use syslog::{Facility, Formatter3164, BasicLogger};
use sqlx::{PgPool, Pool, Postgres};
use log::LevelFilter;

pub struct LoggerConfig {
    pub host: String,
    pub facility: String,
    pub log_level: String,
}

pub struct DbConfig {
    pub dsn: String,
}

pub struct RmqConfig {
    pub dsn: String,
}

pub fn init_logger(config: LoggerConfig) {
    
    let formatter = Formatter3164 {
        facility: Facility::from_str(config.facility.as_str()).unwrap(),
        hostname: None,
        process: "instock".into(),
        pid: 0,
    };

    let logger = syslog::udp(formatter, "localhost:1234", &config.host)
        .expect("Could not connect to syslog");
    
    if let Ok(()) = log::set_boxed_logger(Box::new(BasicLogger::new(logger))) {
        log::set_max_level(LevelFilter::from_str(config.log_level.as_str()).unwrap());
    }
}

pub async fn init_db(config: DbConfig) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPool::connect(
        config.dsn.as_str()
    ).await
}