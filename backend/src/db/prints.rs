use sqlx::PgPool;

pub async fn insert_print(pool: &PgPool, message_id: i32) {
    sqlx::query!("INSERT INTO prints (message) VALUES ($1)", message_id)
        .execute(pool)
        .await
        .expect("woopsie");
}
