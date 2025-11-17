use chrono::{DateTime, Local};
use dotenv::dotenv;
use sqlx::postgres::{PgPoolOptions, PgQueryResult};
use sqlx::types::ipnetwork::IpNetwork;
use sqlx::{Error, Pool, Postgres};
use std::env;

pub struct Message {
    id: i64,
    content: String,
    title: Option<String>,
    author: Option<String>,
    created_at: DateTime<Local>,
    ip: IpNetwork,
}

pub struct NewMessage<'a> {
    pub title: Option<&'a str>,
    pub author: Option<&'a str>,
    pub content: &'a str,
    pub ip: IpNetwork,
}

impl NewMessage<'_> {
    pub fn new<'a>(
        title: Option<&'a str>,
        author: Option<&'a str>,
        content: &'a str,
        ip: IpNetwork,
    ) -> NewMessage<'a> {
        NewMessage {
            title,
            author,
            content,
            ip,
        }
    }
}

pub async fn get_connection_pool() -> Pool<Postgres> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("env variable DATABASE_URL must be set.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Could not connect to database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("no migration no cookies.");

    pool
}

pub async fn insert_message(
    pool: &sqlx::PgPool,
    msg: NewMessage<'_>,
) -> Result<PgQueryResult, Error> {
    sqlx::query!(
        "INSERT INTO messages (title, author, content, ip) VALUES ($1, $2, $3, $4);",
        msg.title,
        msg.author,
        msg.content,
        msg.ip,
    )
    .execute(pool)
    .await
}
