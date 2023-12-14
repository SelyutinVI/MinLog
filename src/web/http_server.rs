use std::sync::Arc;

use crate::usecase::Logger;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Router,
};
use tokio::net::TcpListener;

use crate::web::get_route_log;

pub(crate) async fn start<T>(logger: Arc<T>)
where
    T: Logger + Send + Sync + 'static,
{
    let app = Router::new()
        .merge(get_route_log(logger.clone()))
        .fallback(fallback);
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn fallback() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Html("<h1>This page does not exist</h1>"),
    )
}
