//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use log::{error, info};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::Filter;

#[derive(Default, Debug, Serialize, Deserialize)]
struct Config {
    map: HashMap<String, String>,
}

#[tokio::main]
async fn main() {
    json_env_logger::init();

    let config_path = std::env::var("VANITY_CONFIG_PATH").unwrap();
    info!("config path: {}", config_path);
    let cfg: Config = confy::load_path(config_path).unwrap();
    info!("Config: {:#?}", cfg);

    let live = warp::path::end()
        .and(warp::get())
        .map(|| r#"{"status":"OK"}"#);

    let vanity = warp::path::param::<String>()
        .and(warp::get())
        .and(warp::path::end())
        .map(|p| {
            let package = format!("go.ectobit.com/{}", p);
            let repo = format!("https://github.com/ectobit/{}", p);

            format!(
                r#"<!DOCTYPE html>
<html>
    <head>
        <meta name="go-import" content="{} git {}">
    </head>
    <body>
        Nothing to see here.
    </body>
</html>"#,
                package, repo
            )
        });

    let routes = warp::get().and(live.or(vanity));

    warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;
}
