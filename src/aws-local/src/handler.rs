use axum::{
    body::{Body, Bytes},
    extract::{OriginalUri, Query},
    http::{HeaderMap, Method},
    response::{IntoResponse, Response},
    Extension,
};

use log::info;
use std::collections::HashMap;

use crate::lambda::LambdaFunction;
use crate::request::RequestInfo;
use crate::response::ResponseInfo;

impl IntoResponse for ResponseInfo {
    fn into_response(self) -> Response {
        let mut builder = Response::builder().status(self.status_code);

        for (key, header_value) in self.headers {
            if let Some(header_name) = key {
                builder = builder.header(header_name, header_value);
            }
        }

        builder.body(Body::from(self.body)).unwrap()
    }
}

pub async fn call_proxy(
    Extension(lambda): Extension<LambdaFunction>,
    method: Method,
    uri: OriginalUri,
    headers: HeaderMap,
    query: Query<HashMap<String, Vec<String>>>,
    body: Bytes,
) -> impl IntoResponse {
    let uri = uri.0;
    info!("Handling {} {}", method, uri);

    let info = RequestInfo::new(method.clone(), uri.clone().into(), headers, query.0, body);
    let request = info.into_http_request();

    let response = match lambda.call_http(request).await {
        Ok(response) => ResponseInfo::from_http_response(response),
        Err(error) => return ResponseInfo::internal_server_error(error.to_string()),
    };

    let response = match response {
        Ok(value) => value,
        Err(error) => return ResponseInfo::internal_server_error(error.to_string()),
    };

    info!("Handling {} {} completed", method, uri);
    response
}
