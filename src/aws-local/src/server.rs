use axum::{routing::any, Extension, Router};
use thiserror::Error;

use log::info;
use reqwest;
use std::net::SocketAddr;

use crate::config::CliConfig;
use crate::handler::call_proxy;
use crate::lambda::LambdaFunction;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Binding on {address} failed: {error}")]
    Binding {
        address: SocketAddr,
        error: std::io::Error,
    },

    #[error("Serving on {address} failed: {error}")]
    Serving {
        address: SocketAddr,
        error: std::io::Error,
    },

    #[error("Creating HTTP client failed: {error}")]
    Reqwest { error: reqwest::Error },
}

impl ServerError {
    fn raise_binding<T>(address: SocketAddr, error: std::io::Error) -> Result<T, Self> {
        Err(Self::Binding {
            address: address,
            error: error,
        })
    }

    fn raise_serving<T>(address: SocketAddr, error: std::io::Error) -> Result<T, Self> {
        Err(Self::Serving {
            address: address,
            error: error,
        })
    }

    fn raise_reqwest<T>(error: reqwest::Error) -> Result<T, Self> {
        Err(Self::Reqwest { error: error })
    }
}

pub struct Server {
    lambda: LambdaFunction,
}

impl Server {
    pub fn new(config: CliConfig) -> Result<Self, ServerError> {
        let lambda = match reqwest::Client::builder().http1_only().build() {
            Ok(gateway) => LambdaFunction::new(config.function_name, gateway),
            Err(error) => return ServerError::raise_reqwest(error),
        };

        Ok(Server { lambda })
    }

    pub async fn start(self) -> Result<(), ServerError> {
        let address = SocketAddr::from(([127, 0, 0, 1], 3000));
        info!("Binding server on {address}");

        let listener = match tokio::net::TcpListener::bind(address).await {
            Ok(listener) => listener,
            Err(error) => return ServerError::raise_binding(address, error),
        };

        let app = Router::new()
            .route("/", any(call_proxy))
            .route("/*path", any(call_proxy))
            .layer(Extension(self.lambda));

        match axum::serve(listener, app).await {
            Ok(()) => info!("Server started on {address}"),
            Err(error) => return ServerError::raise_serving(address, error),
        };

        Ok(())
    }
}
