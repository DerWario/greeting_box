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

async fn print_me_some(
    State(pool): State<Pool<Postgres>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) {
    dbg!(addr);
    let printer_data = print::PrintData {
        print_meta_data: print::PrintMetaData {
            global_message_count: 10,
            message_number: 1,
            global_print_count: 5,
            message_print_count: 1,
            written_at: "16.11.2025 23:55".to_string(),
        },
        title: Some("Test Titel Nr. 1 Wird sehr sehr lang, was passiert dann wohl!?".to_string()),
        message: "Diese Nachricht ist \n sehr wichtig!\n\n Jajaja12324566788\n!@#$%^&*(){}-=\\|"
            .to_string(),
        author: Some("Marlon".to_string()),
    };

    print::print(printer_data).expect("Printer fucked");
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
