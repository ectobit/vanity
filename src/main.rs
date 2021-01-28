//! Crate vanity implements Go vanity imports HTTP server.
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use log::info;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::{http::Response, Filter};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
struct Config {
    domain: String,
    packages: HashMap<String, String>,
}

#[tokio::main]
async fn main() {
    json_env_logger::init();

    let config_path = std::env::var("VANITY_CONFIG_PATH").unwrap();
    info!("config path: {}", config_path);
    let config: Config = confy::load_path(config_path).unwrap();
    info!("Config: {:#?}", config);

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
                    r#"<!DOCTYPE html>
<html>
    <head>
        <meta name="go-import" content="{} git {}">
    </head>
    <body>
        Nothing to see here.
    </body>
</html>"#,
                    package_name, repository_name
                )),

                None => Response::builder()
                    .status(404)
                    .body(format!("Package {} not found", package_name)),
            }
        });

    let routes = warp::get().and(live.or(vanity));

    warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;
}
