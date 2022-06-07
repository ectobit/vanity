//! Crate vanity implements Go vanity imports HTTP server.
#![warn(missing_debug_implementations, rust_2018_idioms)]
// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use anyhow::Result;
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use config::Config;
use serde::{Deserialize, Serialize};
use slog::{info, o, warn, Drain, Logger};
use slog_json::Json as JsonLogger;
use std::{
    collections::HashMap,
    io,
    net::SocketAddr,
    process,
    sync::{Arc, Mutex, RwLock},
};

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
        .layer(Extension(Arc::new(RwLock::new(config))))
        .layer(Extension(log.clone()));

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
    query: Option<Query<HashMap<String, String>>>,
    Extension(state): Extension<SharedState>,
    Extension(log): Extension<Logger>,
) -> Result<Html<String>, VanityError> {
    let Query(query) = query.unwrap_or_default();
    if query.get("go-get").is_none() {
        return Ok(Html(
            "<!DOCTYPE html><html><body>Show some human readable stuff here.</body></html>"
                .to_owned(),
        )); // TODO: Show some human readable stuff here
    }

    let s = &state.read().map_err(|err| {
        warn!(log, "error: {}", err);
        VanityError::Poisoned
    })?;

    let repository = s.packages.get(&package);
    match repository {
        Some(repository) => Ok(Html(format!(
            "<!DOCTYPE html><html><head><meta name=\"go-import\" content=\"{}/{} git {}\"></head><body>Nothing to see here.</body></html>",
            s.domain, package, repository,
        ))),
        None => Err(VanityError::NotFound(package)),
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Cfg {
    domain: String,
    packages: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
struct Status<'a> {
    status: &'a str,
}

#[derive(Debug)]
pub enum VanityError {
    Poisoned,
    NotFound(String),
}

impl IntoResponse for VanityError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            VanityError::NotFound(package) => (
                StatusCode::NOT_FOUND,
                format!("Package {} not found", package),
            ),
            VanityError::Poisoned => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_owned(),
            ),
        };

        let body = Html(format!(
            "<!DOCTYPE html><html><body>{}.</body></html>",
            error_message
        ));

        (status, body).into_response()
    }
}
