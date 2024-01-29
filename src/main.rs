use error::*;
use std::sync::Arc;
use storage::LocalStorage;
use tokio::sync::oneshot;
use usecase::{Logger, LoggerBuilder};

mod config;
mod domain;
mod error;
mod storage;
mod usecase;
mod web;

#[tokio::main]
async fn main() {
    let logger = LoggerBuilder::new()
        .use_min_level(domain::Level::Debug)
        .use_storage(LocalStorage::new())
        .build_common_logger();
    let logger = Arc::new(logger);
    let (sender, reciever) = oneshot::channel();

    // start HTTP server
    web::start(logger.clone(), reciever);
    // start cleanup
    start_cleanup(logger.clone());
    // start cli
    start_cli(sender);
}

fn start_cleanup<T: Logger + Send + Sync + 'static>(logger: Arc<T>) {
    let logger = logger.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            logger.cleanup();
        }
    });
}

fn start_cli(sender: oneshot::Sender<()>) {
    loop {
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap();

        match command.trim().to_lowercase().as_str() {
            "exit" => {
                sender.send(()).unwrap();
                break;
            }
            _ => {
                println!("Unknown command {}", command);
            }
        }
    }
}
