use std::sync::Arc;

use crate::usecase::Logger;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Router,
};
use tokio::{net::TcpListener, sync::oneshot};

use crate::web::get_route_log;

pub fn start<T>(logger: Arc<T>, signal: oneshot::Receiver<()>)
where
    T: Logger + Clone + Send + Sync + 'static,
{
    tokio::spawn(async move {
        let app = Router::new()
            .merge(get_route_log(logger))
            .fallback(fallback);
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
        let listener = TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal(signal))
            .await
            .unwrap();
    });
}

async fn fallback() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Html("<h1>This page does not exist</h1>"),
    )
}

async fn shutdown_signal(signal: oneshot::Receiver<()>) {
    match signal.await {
        Ok(_) => println!("Shutdown"),
        Err(_) => panic!("Panic from signal receiver"),
    }
}
