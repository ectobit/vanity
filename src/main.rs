//! Crate vanity implements Go vanity imports HTTP server.
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use anyhow::{Context, Result};
use slog::{info, o, warn, Drain, Logger};
use slog_json::Json as JsonLogger;
use std::{io, process, sync::Mutex};
use vanity::{Config, Server};

#[tokio::main]
async fn main() {
    process::exit(match run().await {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}

async fn run() -> Result<()> {
    let log = Logger::root(
        Mutex::new(JsonLogger::default(io::stderr())).map(slog::Fuse),
        o!(),
    );

    let config_path =
        std::env::var("VANITY_CONFIG_PATH").context("env var VANITY_CONFIG_PATH not set")?;
    info!(log, "config path: {}", config_path);
    let config = config::Config::builder()
        .add_source(config::File::with_name(&config_path))
        .build()
        .context("failed building config")?
        .try_deserialize::<Config>()
        .context("failed deserializing config")?;

    config.packages.iter().for_each(
        |(k, v)| info!(log, "config"; "repository" => v, "package"=> format!("{}/{}", config.domain, k)),
    );

    let server = Server::new(3000, config, log);
    server.serve().await.context("failed running server")?;

    Ok(())
}
