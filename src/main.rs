use config::Config;
use error::*;
use storage::LocalStorage;
use std::sync::Arc;
use usecase::{LoggerBuilder, Logger};

mod config;
mod domain;
mod error;
mod storage;
mod usecase;
mod web;

#[tokio::main]
async fn main() {
    let _config = Config::default();
    let logger = LoggerBuilder::new()
        .use_min_level(domain::Level::Debug)
        .use_storage(LocalStorage::new())
        .build_common_logger();
    let logger = Arc::new(logger);
    
    // start HTTP server
    let _ = tokio::spawn(web::start(logger.clone()));

    // start cleanup
    let _ = tokio::spawn( async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            logger.cleanup();
        }
    });

    loop {
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap();

        match command.trim().to_lowercase().as_str() {
            "exit" => break,
            _ => {
                println!("Unknown command {}", command);
            }
        }
    }
}
