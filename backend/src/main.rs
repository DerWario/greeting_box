mod print;
mod web_server;
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    web_server::run_webserver().await;
    println!("YES");
}
