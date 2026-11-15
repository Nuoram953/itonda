use crate::common::{app::test_app, fixtures::media::MediaFixture, response::json};
use axum::body::Body;
use axum::http::{Request, StatusCode};
use itonda_domain::{
    emulators::models::Emulator, media::models::Launch, protocol::ServerToAgentMessage,
};
use itonda_server::api::emulators::schemas::GetEmulatorsResponse;
use serde_json::json as json_body;
use tower::ServiceExt;

#[tokio::test]
async fn emulator_and_media_launch_crud_and_execution() {
    let mut app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;

    // 1. Create emulator
    let create_emu_res = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/emulators")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(
                    json_body!({
                        "name": "Dolphin",
                        "program": "/usr/bin/dolphin-emu",
                        "arguments": ["-b", "-e", "{game}"],
                        "working_directory": "/games/gc"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_emu_res.status(), StatusCode::CREATED);
    let emu: Emulator = json(create_emu_res).await;
    assert_eq!(emu.name, "Dolphin");
    assert_eq!(emu.arguments, vec!["-b", "-e", "{game}"]);

    // 2. List emulators
    let list_res = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/emulators")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(list_res.status(), StatusCode::OK);
    let list: GetEmulatorsResponse = json(list_res).await;
    assert_eq!(list.items.len(), 1);

    // 3. Create emulator launch profile on media with override
    let create_launch_res = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/launches", fixture.media.id))
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(
                    json_body!({
                        "name": "Dolphin",
                        "launch_type": "emulator",
                        "emulator_id": emu.id,
                        "game_location": "/roms/wind_waker.iso",
                        "arguments": ["--fullscreen"],
                        "argument_mode": "append",
                        "is_default": true
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_launch_res.status(), StatusCode::CREATED);
    let launch: Launch = json(create_launch_res).await;
    assert_eq!(launch.emulator_id.as_deref(), Some(emu.id.as_str()));
    assert_eq!(launch.program, "/usr/bin/dolphin-emu");
    assert_eq!(
        launch.arguments,
        vec!["-b", "-e", "/roms/wind_waker.iso", "--fullscreen"]
    );
    assert_eq!(launch.working_directory.as_deref(), Some("/games/gc"));
    assert_eq!(
        launch.game_location.as_deref(),
        Some("/roms/wind_waker.iso")
    );
    assert!(launch.is_default);

    // 4. Launch media and verify resolved command sent to agent
    let launch_res = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/media/launch/{}", launch.id))
                .method("POST")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(launch_res.status(), StatusCode::ACCEPTED);

    let msg = app
        .agent_messages
        .recv()
        .await
        .expect("agent did not receive launch command");
    match msg {
        ServerToAgentMessage::Launch(cmd) => {
            assert_eq!(cmd.program, "/usr/bin/dolphin-emu");
            assert_eq!(
                cmd.args,
                vec!["-b", "-e", "/roms/wind_waker.iso", "--fullscreen"]
            );
            assert_eq!(cmd.working_directory.as_deref(), Some("/games/gc"));
        }
        other => panic!("expected Launch message, got {other:?}"),
    }

    // 5. Update launch profile
    let patch_launch_res = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/launches/{}", fixture.media.id, launch.id))
                .method("PATCH")
                .header("content-type", "application/json")
                .body(Body::from(
                    json_body!({
                        "name": "Dolphin 4K",
                        "argument_mode": "replace",
                        "arguments": ["--batch"]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(patch_launch_res.status(), StatusCode::OK);
    let updated_launch: Launch = json(patch_launch_res).await;
    assert_eq!(updated_launch.name, "Dolphin 4K");
    assert_eq!(updated_launch.argument_mode, "replace");

    // 6. Delete launch profile and emulator
    let del_launch_res = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/launches/{}", fixture.media.id, launch.id))
                .method("DELETE")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(del_launch_res.status(), StatusCode::NO_CONTENT);

    let del_emu_res = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/emulators/{}", emu.id))
                .method("DELETE")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(del_emu_res.status(), StatusCode::NO_CONTENT);
}
