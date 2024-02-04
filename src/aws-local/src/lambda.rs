use aws_lambda_events::apigw::{ApiGatewayV2httpRequest, ApiGatewayV2httpResponse};

use log::warn;
use reqwest::Client;

#[derive(Clone)]
pub struct LambdaFunction {
    endpoint: String,
    gateway: Client,
}

impl LambdaFunction {
    pub fn new(name: impl AsRef<str>, gateway: Client) -> Self {
        let endpoint = format!(
            "http://127.0.0.1:9000/2015-03-31/functions/{}/invocations",
            name.as_ref()
        );

        Self { endpoint, gateway }
    }

    pub async fn call_http(
        &self,
        request: ApiGatewayV2httpRequest,
    ) -> Result<ApiGatewayV2httpResponse, reqwest::Error> {
        let response = self
            .gateway
            .post(&self.endpoint)
            .json(&request)
            .send()
            .await;

        let response = match response {
            Ok(response) => response,
            Err(error) => {
                warn!("Calling lambda failed: {}", error.to_string());
                return Err(error);
            }
        };

        let json = match response.json::<ApiGatewayV2httpResponse>().await {
            Ok(data) => data,
            Err(error) => {
                warn!("Parsing lambda response failed: {}", error.to_string());
                return Err(error);
            }
        };

        Ok(json)
    }
}
