use bytes::Bytes;
use http::{HeaderMap, Method, Uri};

use base64::{engine::general_purpose::STANDARD, Engine};
use std::collections::HashMap;

use aws_lambda_events::apigw::{
    ApiGatewayV2httpRequest, ApiGatewayV2httpRequestContext,
    ApiGatewayV2httpRequestContextHttpDescription,
};

pub struct RequestInfo {
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    query: HashMap<String, Vec<String>>,
    body: Bytes,
}

impl RequestInfo {
    pub fn new(
        method: Method,
        uri: Uri,
        headers: HeaderMap,
        query: HashMap<String, Vec<String>>,
        body: Bytes,
    ) -> Self {
        RequestInfo {
            method: method,
            uri: uri,
            headers: headers,
            query: query,
            body: body,
        }
    }

    pub fn into_http_request(self) -> ApiGatewayV2httpRequest {
        let user_agent = match self.headers.get(http::header::USER_AGENT) {
            None => None,
            Some(value) => Some(String::from_utf8_lossy(value.as_bytes()).to_string()),
        };

        let query_string = match self.uri.query() {
            None => None,
            Some(value) => Some(value.to_owned()),
        };

        let api = "7zw9uyk9kl";
        let domain = "7zw9uyk9kl";
        let account = "123456789012";
        let route = "$default";

        let context = ApiGatewayV2httpRequestContext {
            route_key: Some(route.to_owned()),
            account_id: Some(account.to_owned()),
            apiid: Some(api.to_owned()),
            domain_name: Some(format!("{api}.execute-api.eu-west-1.amazonaws.com")),
            domain_prefix: Some(domain.to_owned()),
            http: ApiGatewayV2httpRequestContextHttpDescription {
                method: self.method,
                path: Some(self.uri.path().to_owned()),
                protocol: Some("HTTP/1.1".to_owned()),
                source_ip: Some("127.0.0.1".to_owned()),
                user_agent: user_agent,
            },
            ..Default::default()
        };

        ApiGatewayV2httpRequest {
            version: Some("2.0".to_owned()),
            route_key: context.route_key.clone(),
            http_method: context.http.method.clone(),
            headers: self.headers,
            raw_path: context.http.path.clone(),
            raw_query_string: query_string,
            query_string_parameters: self.query.into(),
            is_base64_encoded: true,
            body: Some(STANDARD.encode(self.body)),
            request_context: context,
            ..Default::default()
        }
    }
}
