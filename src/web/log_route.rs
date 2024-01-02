use std::collections::HashMap;
use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde_json::{json, Value};

use crate::domain::Log;
use crate::usecase::Logger;

pub fn get_route_log<T>(logger: Arc<T>) -> Router
where
    T: Logger + Send + Sync + 'static,
{
    Router::new()
        .route("/log", post(post_log::<T>).get(get_all_log::<T>))
        .with_state(logger.clone())
}

pub async fn post_log<T>(logger: State<Arc<T>>, Json(log): Json<LogDTO>) -> impl IntoResponse
where
    T: Logger,
{
    if log.level.is_none()
        || log.msg.is_none()
        || log.lifetime.is_none()
        || (log.body.is_some() && !log.body.as_ref().unwrap().is_object())
    {
        return (StatusCode::BAD_REQUEST, "Invalid log".to_string());
    }

    let (msg, level, lifetime, mut body) = (
        log.msg.unwrap(),
        log.level.unwrap(),
        log.lifetime.unwrap(),
        None,
    );

    if let Some(Value::Object(b)) = log.body {
        let mut values = HashMap::new();
        for (k, v) in b {
            values.insert(k.to_string(), v.to_string());
        }
        body = Some(values);
    }

    match logger.log(msg, &level, &lifetime, body) {
        Ok(_) => (StatusCode::OK, "OK".to_string()),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()),
    }
}

pub async fn get_all_log<T>(logger: State<Arc<T>>) -> impl IntoResponse
where
    T: Logger,
{
    let predicate = |_l: &Log| {
        return true;
    };

    let result = logger.find_logs(predicate);
    (StatusCode::OK, json!(result).to_string())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LogDTO {
    pub msg: Option<String>,
    pub level: Option<String>,
    pub lifetime: Option<String>,
    pub body: Option<Value>,
}
