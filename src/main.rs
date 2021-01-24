use warp::Filter;

#[tokio::main]
async fn main() {
    json_env_logger::init();

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
