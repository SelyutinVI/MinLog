use config::Config;
use error::*;
use std::sync::Arc;
use usecase::LoggerBuilder;

mod config;
mod domain;
mod error;
mod storage;
mod usecase;
mod web;

#[tokio::main]
async fn main() {
    let _config = Config::new();
    let logger = LoggerBuilder::new().use_local_storage().build_just_logger();
    let _ = tokio::spawn(web::start(Arc::new(logger)));

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
