use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub async fn connect(_url: &str) -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:///app/data/itonda.db?mode=rwc")
        .await
        .unwrap()
}

pub async fn migrate(pool: &SqlitePool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await.unwrap();

    Ok(())
}
