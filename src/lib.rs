use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use maud::{html, DOCTYPE};
use serde::Deserialize;
use serde_json::{json, Value};
use slog::{info, Logger};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use thiserror::Error;

pub struct Server {
    port: u16,
    config: SharedConfig,
    log: Logger,
}

impl Server {
    pub fn new(port: u16, config: Config, log: Logger) -> Self {
        Self {
            port,
            config: Arc::new(config),
            log,
        }
    }

    pub async fn serve(self) -> Result<(), VanityError> {
        let app = Router::new()
            .route("/:package", get(vanity))
            .route("/", get(health))
            .layer(Extension(self.config));

        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        info!(self.log, "listening"; "address" => addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await?;

        Ok(())
    }
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

type SharedConfig = Arc<Config>;

async fn vanity(
    Path(package): Path<String>,
    query: Option<Query<GoGetQuery>>,
    Extension(config): Extension<SharedConfig>,
) -> Result<Html<String>, VanityError> {
    let config = Arc::clone(&config);
    let domain = &config.domain;
    let repository = config
        .packages
        .get(&package)
        .ok_or_else(|| VanityError::NotFound(format!("{domain}/{package}")))?;

    match query.is_some() && query.unwrap().go_get.is_some() {
        true => Ok(response(domain, &package, repository)),
        false => Ok(human_response(domain, &package, repository)),
    }
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum VanityError {
    #[error("hyper error")]
    NotFound(String),
    #[error("hyper error")]
    Hyper(#[from] hyper::Error),
}

impl IntoResponse for VanityError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            VanityError::NotFound(package) => (
                StatusCode::NOT_FOUND,
                format!("Package {package} not found"),
            ),
            VanityError::Hyper(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        (status, error_response(&error_message)).into_response()
    }
}

fn response(domain: &str, package: &str, repository: &str) -> Html<String> {
    let markup = html! {
        (DOCTYPE)
        html {
            head {
                meta name="go-import" content={ (domain) "/" (package) " git " (repository) };
            }
            body {
                "Nothing to see here."
            }
        }
    };

    Html(markup.into_string())
}

fn error_response(message: &str) -> Html<String> {
    let markup = html! {
        (DOCTYPE)
        html {
            head {
                title { "vanity 404" }
            }
            body {
                (message)
            }
        }
    };

    Html(markup.into_string())
}

fn human_response(domain: &str, package: &str, repository: &str) -> Html<String> {
    let markup = html! {
        (DOCTYPE)
        html {
            head {
                title { (domain) "/" (package) }
            }
            body {
                h2 { (domain) "/" (package) }
                code { "go get " (domain) "/" (package) }
                br;
                code { "import \"" (domain) "/" (package) "\"" }
                br;
                a href={ (repository) } { "Source" } " | "
                a href={ "https://pkg.go.dev/" (domain) "/" (package) } { "Docs" } " | "
                a href={ "https://goreportcard.com/report/" (domain) "/" (package) } { "Go Report Card" }
            }
        }
    };

    Html(markup.into_string())
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub domain: String,
    pub packages: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct GoGetQuery {
    #[serde(rename = "go-get")]
    go_get: Option<u8>,
}
