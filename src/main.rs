use axum::{routing::get, Router};
use std::env;
mod rest;
mod schemas;
use rest::{get_post_by_slug, home, publish_post};

#[tokio::main]
async fn main() {
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@je compile");
    basics();

    // Router is used to set up which paths points to which pages
    // In the future each file should handle routes and schemas
    let app = Router::new()
        .route("/", get(home))
        .route("/publish", get(publish_post))
        .route("/post/:slug", get(get_post_by_slug)); // The :slug allows us to pass the name of the post to the handler

    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn connect() -> redis::Connection {
    let redis_host_name = env::var("REDIS_HOST").unwrap_or("localhost".to_string());
    let redis_port = env::var("REDIS_PORT").unwrap_or("6379".to_string());

    let redis_conn_url = format!("redis://{}:{}", redis_host_name, redis_port);
    println!("{}", redis_conn_url);

    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

fn basics() {
    let mut conn = connect();
    println!("******* Running SET, GET, INCR commands *******");

    let _: () = redis::cmd("SET")
        .arg("verySpecialKey")
        .arg("bar")
        .query(&mut conn)
        .expect("failed to execute SET for 'verySpecialKey'");

    let bar: String = redis::cmd("GET")
        .arg("verySpecialKey")
        .query(&mut conn)
        .expect("failed to execute GET for 'verySpecialKey'");
    println!("value for tu compiles ? 'verySpecialKey' = {}", bar);
}

