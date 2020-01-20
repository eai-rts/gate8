#![deny(warnings)]
use warp::*;
use warp::http::Response;
use std::fmt::Write;
use std::time::Duration;
use kafka::producer::{Producer, Record, RequiredAcks};

#[tokio::main]
async fn main() {
    

    let main = warp::path("home").map(|| "Hello from Gate8!");
    let create_tasks = warp::path("tasks")
        .and(warp::post())
        .map(|| {
            let mut producer = Producer::from_hosts(vec!("localhost:9092".to_owned()))
                .with_ack_timeout(Duration::from_secs(1))
                .with_required_acks(RequiredAcks::One)
                .create()
                .unwrap();
            let mut buf = String::with_capacity(2);
            for i in 0..10 {
            let _ = write!(&mut buf, "{}", i);
            producer.send(&Record::from_value("my-topic", buf.as_bytes())).unwrap();
            buf.clear();
        }
            Response::builder().body("{The task is saved!}")
        });
    let routes = main.or(create_tasks);
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}