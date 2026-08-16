use std::path::Path;

use super::*;

#[test]
fn test_matches_directory_exact_cwd() {
    let target = Path::new("/games/portal2");
    let cwd = Path::new("/games/portal2");
    assert!(matches_directory(target, Some(cwd), None));
}

#[test]
fn test_matches_directory_nested_cwd() {
    let target = Path::new("/games/portal2");
    let cwd = Path::new("/games/portal2/bin/x64");
    assert!(matches_directory(target, Some(cwd), None));
}

#[test]
fn test_matches_directory_nested_exe() {
    let target = Path::new("/games/portal2");
    let exe = Path::new("/games/portal2/bin/portal2_linux");
    assert!(matches_directory(target, None, Some(exe)));
}

#[test]
fn test_matches_directory_rejects_different_directory() {
    let target = Path::new("/games/portal2");
    let cwd = Path::new("/games/halflife");
    let exe = Path::new("/usr/bin/bash");
    assert!(!matches_directory(target, Some(cwd), Some(exe)));
}

#[test]
fn test_matches_directory_handles_none() {
    let target = Path::new("/games/portal2");
    assert!(!matches_directory(target, None, None));
}

#[tokio::test]
async fn test_directory_tracker_errors_on_missing_working_directory() {
    let tracker = DirectoryTracker::new();
    let target = TrackTarget {
        media_id: "media-123".into(),
        working_directory: None,
        process_name: None,
        program: None,
    };

    let result = tracker.find_processes(&target).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        TrackerError::InvalidDirectory(_) => {}
        err => panic!("Expected InvalidDirectory error, got {:?}", err),
    }
}

#[tokio::test]
async fn test_directory_tracker_finds_real_spawned_process() {
    let temp_dir = tempfile::tempdir().unwrap();

    let mut child = std::process::Command::new("sleep")
        .arg("5")
        .current_dir(temp_dir.path())
        .spawn()
        .expect("Failed to spawn sleep process for test");

    let tracker = DirectoryTracker::new();
    let target = TrackTarget::from_directory("test-media", temp_dir.path());

    let found = tracker.find_processes(&target).await.unwrap();
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].pid, child.id());

    let is_running = tracker.is_running(&target).await.unwrap();
    assert!(is_running);

    let _ = child.kill();
    let _ = child.wait();
}

#[tokio::test]
async fn test_directory_tracker_wait_for_startup_buffer() {
    let temp_dir = tempfile::tempdir().unwrap();
    let tracker = DirectoryTracker::new();
    let target = TrackTarget::from_directory("test-media", temp_dir.path());

    // Spawn process slightly delayed in background to test the startup buffer
    let temp_path = temp_dir.path().to_path_buf();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;
        let mut child = std::process::Command::new("sleep")
            .arg("5")
            .current_dir(&temp_path)
            .spawn()
            .expect("Failed to spawn delayed process");
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        let _ = child.kill();
        let _ = child.wait();
    });

    let found = tracker
        .wait_for_startup(
            &target,
            std::time::Duration::from_secs(5),
            std::time::Duration::from_millis(50),
        )
        .await
        .unwrap();

    assert!(!found.is_empty());
}

#[tokio::test]
async fn test_directory_tracker_wait_until_stopped_debounces() {
    let temp_dir = tempfile::tempdir().unwrap();
    let tracker = DirectoryTracker::new();
    let target = TrackTarget::from_directory("test-media", temp_dir.path());

    let start = std::time::Instant::now();
    let mut child = std::process::Command::new("sleep")
        .arg("0.6")
        .current_dir(temp_dir.path())
        .spawn()
        .expect("Failed to spawn process");

    let is_running = tracker.is_running(&target).await.unwrap();
    assert!(is_running);

    let _ = tracker
        .wait_until_stopped(&target, std::time::Duration::from_millis(100))
        .await;

    let elapsed = start.elapsed();
    assert!(
        elapsed >= std::time::Duration::from_millis(400),
        "Elapsed: {:?}",
        elapsed
    );

    let _ = child.wait();
}
