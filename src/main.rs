//! Crate vanity implements Go vanity imports HTTP server.
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use anyhow::Result;
use axum::{
    extract::{Extension, Path},
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use config::Config;
use serde::{Deserialize, Serialize};
use slog::{info, o, Drain};
use slog_json::Json as JsonLogger;
use std::{
    collections::HashMap,
    io,
    net::SocketAddr,
    process,
    sync::{Arc, Mutex, RwLock},
};
use tower::ServiceBuilder;

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
    let log = slog::Logger::root(
        Mutex::new(JsonLogger::default(io::stderr())).map(slog::Fuse),
        o!(),
    );

    let config_path = std::env::var("VANITY_CONFIG_PATH")?;
    info!(log, "config path: {}", config_path);
    let config = Config::builder()
        .add_source(config::File::with_name(&config_path))
        .build()?
        .try_deserialize::<Cfg>()?;

    config.packages.iter().for_each(
        |(k, v)| info!(log, "config"; "repository" => v, "package"=> format!("{}/{}", config.domain, k)),
    );

    let app = Router::new()
        .route("/:package", get(vanity))
        .route("/", get(health))
        .layer(ServiceBuilder::new().layer(Extension(Arc::new(RwLock::new(config)))));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!(log, "listening"; "address" => addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn health() -> impl IntoResponse {
    Json(Status { status: "Ok" })
}

type SharedState = Arc<RwLock<Cfg>>;

async fn vanity(
    Path(package): Path<String>,
    Extension(state): Extension<SharedState>,
) -> impl IntoResponse {
    let s = &state.read().unwrap();
    Html(format!(
        r#"<!DOCTYPE html><html><head><meta name="go-import" content="{}/{} git {}"></head><body>Nothing to see here.</body></html>"#,
        s.domain,
        package,
        s.packages.get(&package).unwrap(),
    ))
}

#[derive(Debug, Deserialize, Clone)]
struct Cfg {
    domain: String,
    packages: HashMap<String, String>,
}

#[derive(Serialize)]
struct Status<'a> {
    status: &'a str,
}
