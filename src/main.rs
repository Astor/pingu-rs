use warp::{http::Response, Filter};

#[tokio::main]
async fn main() {

    // Environment port check, defaults to 5001
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "5001".to_string())
        .parse()
        .expect("PORT must be a number");

    // Simple PINGU /ping route
    let ping = warp::path("ping")
        .map(|| Response::builder().body("pong!"));

    // Enable CORS for any origin
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET"]);

        // Combine routes with defined CORS policy
    let routes = ping.with(cors);

    // Start PINGU on port
    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}
