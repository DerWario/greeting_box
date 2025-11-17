mod db;
mod print;
mod web_server;
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let pool = db::get_connection_pool().await;

    tokio::join!(web_server::run_webserver(pool.clone()));
    println!("YES");
}
