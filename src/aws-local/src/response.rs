use aws_lambda_events::apigw::ApiGatewayV2httpResponse;
use aws_lambda_events::encodings::Body;

use bytes::Bytes;
use http::{HeaderMap, StatusCode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResponseError {
    #[error("Value '{value}' is invalid status code: {error}")]
    InvalidStatusCode { value: i64, error: String },
}

impl ResponseError {
    fn raise_invalid_status_code<T>(value: i64, error: String) -> Result<T, Self> {
        Err(Self::InvalidStatusCode {
            value: value,
            error: error,
        })
    }
}

pub struct ResponseInfo {
    pub status_code: StatusCode,
    pub headers: HeaderMap,
    pub body: Bytes,
}

impl ResponseInfo {
    pub fn internal_server_error(error: String) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            headers: HeaderMap::new(),
            body: Bytes::from(error),
        }
    }

    pub fn from_http_response(response: ApiGatewayV2httpResponse) -> Result<Self, ResponseError> {
        let status_code: u16 = match response.status_code.try_into() {
            Ok(value) => value,
            Err(error) => {
                return ResponseError::raise_invalid_status_code(
                    response.status_code,
                    error.to_string(),
                )
            }
        };

        let status_code = match StatusCode::from_u16(status_code) {
            Ok(value) => value,
            Err(error) => {
                return ResponseError::raise_invalid_status_code(
                    response.status_code,
                    error.to_string(),
                )
            }
        };

        let body = match response.body {
            None | Some(Body::Empty) => Bytes::default(),
            Some(Body::Binary(data)) => Bytes::from(data),
            Some(Body::Text(data)) => Bytes::from(data),
        };

        Ok(Self {
            status_code: status_code,
            headers: response.headers,
            body: body,
        })
    }
}
