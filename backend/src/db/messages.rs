use chrono::{DateTime, Local};
use sqlx::Error;
use sqlx::postgres::PgQueryResult;
use sqlx::types::ipnetwork::IpNetwork;

pub struct NewMessage<'a> {
    pub title: Option<&'a str>,
    pub author: Option<&'a str>,
    pub content: &'a str,
    pub ip: IpNetwork,
}

impl crate::db::NewMessage<'_> {
    pub fn new<'a>(
        title: Option<&'a str>,
        author: Option<&'a str>,
        content: &'a str,
        ip: IpNetwork,
    ) -> crate::db::NewMessage<'a> {
        crate::db::NewMessage {
            title,
            author,
            content,
            ip,
        }
    }
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

#[derive(sqlx::FromRow)]
pub struct Message {
    pub id: i32,
    pub content: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub created_at: DateTime<Local>,
}
pub async fn get_messages(pool: &sqlx::PgPool) -> Vec<Message> {
    sqlx::query_as!(
        Message,
        "SELECT id, content, title, author, created_at FROM messages;"
    )
    .fetch_all(pool)
    .await
    .expect("Error getting Stuff from the DB")
}
