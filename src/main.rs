#![deny(warnings)]
use warp::*;
use warp::http::Response;

#[tokio::main]
async fn main() {
    let main = warp::path("home").map(|| "Hello from Gate8!");
    let create_tasks = warp::path("tasks")
        .and(warp::post())
        .map(|| {
            Response::builder().body("{The task is saved!}")
        });
    let routes = main.or(create_tasks);
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}