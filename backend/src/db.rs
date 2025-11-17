use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;

mod messages;

pub use messages::Message;
pub use messages::NewMessage;
pub use messages::get_messages;
pub use messages::insert_message;

mod prints;
pub use prints::insert_print;

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
