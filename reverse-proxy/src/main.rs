use slog::info as log_info;
use sloggers::file::FileLoggerBuilder;
use sloggers::types::Severity;
use sloggers::Build;
use std::collections::HashMap;

use config::Config;
use warp::{http::HeaderValue, hyper::HeaderMap, Filter};
use warp_reverse_proxy::reverse_proxy_filter;

#[tokio::main]
async fn main() {
    let mut builder = FileLoggerBuilder::new("reverse-proxy.log");
    builder.level(Severity::Debug);

    let logger = builder.build().unwrap();

    let settings = Config::builder()
        .add_source(config::File::with_name("Settings.toml").required(true))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let s = settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();
    let upstream_echo = s.get("upstream_echo").unwrap();
    let upstream_hello = s.get("upstream_hello").unwrap();

    let log = warp::log::custom(move |info| {
        log_info!(
            logger,
            "{} {} {} {} {} {:?} {} {:?} {}",
            info.host().unwrap(),
            info.remote_addr().unwrap(),
            info.user_agent().unwrap(),
            info.method(),
            info.path(),
            info.version(),
            info.status(),
            info.request_headers(),
            format!("{}", convert(info.request_headers()))
                .into_bytes()
                .len()
        );
    });

    // Forward request to localhost in other port
    let echo = warp::path!("echo" / ..).and(reverse_proxy_filter(
        "".to_string(),
        upstream_echo.to_string(),
    ));

    let hello = warp::path!("hello" / ..).and(reverse_proxy_filter(
        "".to_string(),
        upstream_hello.to_string(),
    ));

    let echo_get_route = warp::get().and(echo.clone());
    // .and_then(log_response)
    let echo_post_route = warp::post().and(echo);
    let hello_get_route = warp::get().and(hello);
    let routes = echo_get_route
        .or(echo_post_route)
        .or(hello_get_route)
        .with(log);

    // spawn proxy server
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}

// async fn log_response(response: http::Response<Bytes>) -> Result<impl Reply, Rejection> {
//     println!("{:?}", response);
//     Ok(response)
// }

fn convert(headers: &HeaderMap<HeaderValue>) -> serde_json::Value {
    format!("{:?}", headers).into()
}
