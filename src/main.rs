//! Crate vanity implements Go vanity imports HTTP server.
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use anyhow::Result;
use config::Config;
use serde::Deserialize;
use slog::{info, o, Drain};
use slog_json::Json;
use std::{collections::HashMap, io, process, sync::Mutex};
use warp::{http::Response, Filter};

#[derive(Debug, Deserialize)]
struct Cfg {
    domain: String,
    packages: HashMap<String, String>,
}

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
        Mutex::new(Json::default(io::stderr())).map(slog::Fuse),
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

    let live = warp::path::end()
        .and(warp::get())
        .map(|| r#"{"status":"OK"}"#);

    let vanity = warp::path::param::<String>()
        .and(warp::get())
        .and(warp::path::end())
        .map(move |p| {
            let package_name = format!("{}/{}", &config.domain, &p);
            let repository_name = config.packages.get(&p);
            match repository_name {
                Some(repository_name) => Response::builder().body(format!(
                    r#"<!DOCTYPE html><html><head><meta name="go-import" content="{} git {}"></head><body>Nothing to see here.</body></html>"#,
                    package_name, repository_name
                )),

                None => Response::builder()
                    .status(404)
                    .body(format!("Package {} not found", package_name)),
            }
        });

    let routes = warp::get().and(live.or(vanity));

    warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;

    Ok(())
}
