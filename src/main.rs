use warp::Filter;

#[tokio::main]
async fn main() {
    json_env_logger::init();

    let live = warp::path("live").map(|| r#"{"status":"OK"}"#);

    let vanity = warp::path::end().map(|| {
        let package = "go.ectobit.com/clap";
        let repo = "https://github.com/ectobit/clap";

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

    let routes = warp::get().and(vanity.or(live));

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
