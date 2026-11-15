use crate::{
    emulators::{
        models::{EmulatorInsert, EmulatorUpdate},
        queries::{
            delete_emulator, find_all_emulators, find_emulator_by_id, insert_emulator,
            update_emulator,
        },
    },
    error::DatabaseError,
    test_utils::setup_db,
};

#[tokio::test]
async fn emulator_crud_lifecycle() {
    let pool = setup_db().await;

    let initial = find_all_emulators(&pool).await.unwrap();
    assert!(initial.is_empty());

    let created = insert_emulator(
        &pool,
        EmulatorInsert {
            name: "Dolphin".to_string(),
            program: "/usr/bin/dolphin-emu".to_string(),
            arguments: r#"["-b","-e","{game}"]"#.to_string(),
            working_directory: Some("/games/gc".to_string()),
        },
    )
    .await
    .unwrap();

    assert_eq!(created.name, "Dolphin");
    assert_eq!(created.program, "/usr/bin/dolphin-emu");
    assert_eq!(created.arguments, r#"["-b","-e","{game}"]"#);
    assert_eq!(created.working_directory.as_deref(), Some("/games/gc"));

    let fetched = find_emulator_by_id(&pool, &created.id).await.unwrap();
    assert_eq!(fetched.id, created.id);

    let updated = update_emulator(
        &pool,
        &created.id,
        EmulatorUpdate {
            name: "Dolphin Emu".to_string(),
            program: "/opt/dolphin/dolphin-emu".to_string(),
            arguments: r#"["-e","{rom}"]"#.to_string(),
            working_directory: None,
        },
    )
    .await
    .unwrap();

    assert_eq!(updated.name, "Dolphin Emu");
    assert_eq!(updated.program, "/opt/dolphin/dolphin-emu");
    assert_eq!(updated.working_directory, None);

    let all = find_all_emulators(&pool).await.unwrap();
    assert_eq!(all.len(), 1);

    let deleted = delete_emulator(&pool, &created.id).await.unwrap();
    assert_eq!(deleted, 1);

    let missing = find_emulator_by_id(&pool, &created.id).await;
    assert!(matches!(missing, Err(DatabaseError::NotFound)));
}
