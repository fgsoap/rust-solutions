use chrono::Utc;
use serde_json;
use warp::{http::HeaderValue, hyper::HeaderMap, Filter};
use warp_reverse_proxy::reverse_proxy_filter;

#[tokio::main]
async fn main() {
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
        reverse_proxy_filter("".to_string(), "https://reqbin.com/".to_string())
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
