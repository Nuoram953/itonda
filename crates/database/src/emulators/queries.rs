use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    emulators::models::{EmulatorInsert, EmulatorRow, EmulatorUpdate},
    error::DatabaseError,
};

pub async fn find_all_emulators(pool: &SqlitePool) -> Result<Vec<EmulatorRow>, DatabaseError> {
    sqlx::query_as::<sqlx::Sqlite, EmulatorRow>(
        r#"
        SELECT id, name, program, arguments, working_directory, created_at, updated_at
        FROM emulators
        ORDER BY name COLLATE NOCASE ASC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn find_emulator_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<EmulatorRow, DatabaseError> {
    sqlx::query_as::<sqlx::Sqlite, EmulatorRow>(
        r#"
        SELECT id, name, program, arguments, working_directory, created_at, updated_at
        FROM emulators
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn insert_emulator(
    pool: &SqlitePool,
    emulator: EmulatorInsert,
) -> Result<EmulatorRow, DatabaseError> {
    let id = Uuid::new_v4().to_string();

    sqlx::query_as::<sqlx::Sqlite, EmulatorRow>(
        r#"
        INSERT INTO emulators (id, name, program, arguments, working_directory)
        VALUES (?, ?, ?, ?, ?)
        RETURNING id, name, program, arguments, working_directory, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(emulator.name)
    .bind(emulator.program)
    .bind(emulator.arguments)
    .bind(emulator.working_directory)
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn update_emulator(
    pool: &SqlitePool,
    id: &str,
    emulator: EmulatorUpdate,
) -> Result<EmulatorRow, DatabaseError> {
    sqlx::query_as::<sqlx::Sqlite, EmulatorRow>(
        r#"
        UPDATE emulators
        SET
            name = ?,
            program = ?,
            arguments = ?,
            working_directory = ?,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        RETURNING id, name, program, arguments, working_directory, created_at, updated_at
        "#,
    )
    .bind(emulator.name)
    .bind(emulator.program)
    .bind(emulator.arguments)
    .bind(emulator.working_directory)
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn delete_emulator(pool: &SqlitePool, id: &str) -> Result<u64, DatabaseError> {
    let result = sqlx::query("DELETE FROM emulators WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(DatabaseError::from)?;

    Ok(result.rows_affected())
}
