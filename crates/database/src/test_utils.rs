use sqlx::SqlitePool;

pub async fn setup_db() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:").await.unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    pool
}
