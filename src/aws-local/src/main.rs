mod config;
mod handler;
mod lambda;
mod logging;
mod request;
mod response;
mod server;

use clap::Parser;

use crate::config::CliConfig;
use crate::logging::initialize_logging;
use crate::server::Server;

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    let server = match CliConfig::try_parse() {
        Err(error) => return eprintln!("Parsing command line failed: {}", error),
        Ok(config) => {
            if let Err(error) = initialize_logging(config.verbosity) {
                return eprintln!("{}", error);
            }

            match Server::new(config) {
                Ok(server) => server,
                Err(error) => return eprintln!("Creating server failed: {}", error),
            }
        }
    };

    if let Err(error) = server.start().await {
        eprintln!("Starting server failed: {}", error)
    }
}
