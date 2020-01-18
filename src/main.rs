#![deny(warnings)]
use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = warp::any().map(|| "Hello, Gate8!");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}