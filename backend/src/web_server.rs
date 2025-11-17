use crate::db::insert_print;
use crate::{db, print};
use axum::extract::{ConnectInfo, State};
use axum::routing::post;
use axum::{Router, extract, routing::get};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;

pub async fn run_webserver(pool: Pool<Postgres>) {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(print_me_some))
        .route("/api/v1/addmessage", post(add_message))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Webserver crashed :( sorry.");
}

async fn print_me_some(State(pool): State<Pool<Postgres>>) {
    let messages = db::get_messages(&pool).await;

    let message = match messages.last() {
        Some(msg) => msg,
        _ => {
            eprintln!("No message found!");
            return;
        }
    };

    let printer_data = print::PrintData {
        print_meta_data: print::PrintMetaData {
            global_message_count: 10,
            message_number: 1,
            global_print_count: 5,
            message_print_count: 1,
            written_at: message.created_at.format("%d.%m.%Y %H:%M").to_string(),
        },
        title: message.title.clone(),
        message: message.content.clone(),
        author: message.author.clone(),
    };

    let print_result = print::print(printer_data);

    if print_result.is_ok() {
        insert_print(&pool, message.id).await;
    }
}

#[derive(Deserialize, Debug)]
struct MessageData {
    content: String,
    author: Option<String>,
    title: Option<String>,
}
async fn add_message(
    State(pool): State<Pool<Postgres>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    extract::Json(payload): extract::Json<MessageData>,
) {
    // TODO check for ISO8859_15 Encoding
    let message = db::NewMessage::new(
        payload.title.as_deref(),
        payload.author.as_deref(),
        payload.content.as_str(),
        addr.ip().into(),
    );

    db::insert_message(&pool, message).await.expect("Woopsie");
}
