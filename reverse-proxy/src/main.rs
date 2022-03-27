use std::collections::HashMap;

use chrono::Utc;
use config::Config;
use serde_json;
use warp::{http::HeaderValue, hyper::HeaderMap, Filter};
use warp_reverse_proxy::reverse_proxy_filter;

#[tokio::main]
async fn main() {
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("Settings.toml").required(true))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let s = settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();
    let upstream = s.get("upstream").unwrap();

    let log = warp::log::custom(|info| {
        // Use a log macro, or slog, or println, or whatever!
        eprintln!(
            "{} {} {} {} {} {} {:?} {} {:?} {}",
            Utc::now(),
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
    let app = warp::path!("echo" / ..).and(
        reverse_proxy_filter("".to_string(), upstream.to_string())
            // .and_then(log_response)
            .with(log),
    );

    // spawn proxy server
    warp::serve(app).run(([0, 0, 0, 0], 3030)).await;
}

// async fn log_response(response: http::Response<Bytes>) -> Result<impl Reply, Rejection> {
//     println!("{:?}", response);
//     Ok(response)
// }

fn convert(headers: &HeaderMap<HeaderValue>) -> serde_json::Value {
    format!("{:?}", headers).into()
}
