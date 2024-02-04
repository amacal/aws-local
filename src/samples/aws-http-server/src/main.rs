use axum::http::HeaderMap;
use axum::{routing, Json, Router};

use lambda_http::{self, Error};
use serde_json::{json, Value};
use std::collections::HashMap;

fn collect_headers(headers: HeaderMap) -> HashMap<String, String> {
    headers
        .iter()
        .map(|(key, value)| {
            (
                key.to_string(),
                value.to_str().unwrap_or_default().to_string(),
            )
        })
        .collect()
}

async fn echo(headers: HeaderMap, Json(body): Json<Value>) -> Json<Value> {
    Json(json!({ "body": body, "headers": collect_headers(headers) }))
}

pub fn build_app() -> Router {
    Router::new().route("/echo", routing::post(echo))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    lambda_http::run(build_app()).await
}
