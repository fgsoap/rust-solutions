use chrono::Utc;
use warp::Filter;
use warp_reverse_proxy::reverse_proxy_filter;

#[tokio::main]
async fn main() {
    let log = warp::log::custom(|info| {
        // Use a log macro, or slog, or println, or whatever!
        eprintln!(
            "{} {} {} {} {} {} {} {:?}",
            Utc::now(),
            info.host().unwrap(),
            info.remote_addr().unwrap(),
            info.user_agent().unwrap(),
            info.method(),
            info.path(),
            info.status(),
            info.request_headers(),
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
