use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

use super::*;
use crate::media::types::{MediaLaunchType, MediaType};
use crate::scanner::traits::MediaScanner;

#[tokio::test]
async fn test_steam_scanner_with_mock_library() {
    let temp_root = tempdir().unwrap();
    let root_path = temp_root.path();

    // 1. Create directory structure: <root>/steamapps/common/Portal 2
    let steamapps = root_path.join("steamapps");
    let common = steamapps.join("common");
    let portal2_dir = common.join("Portal 2");
    fs::create_dir_all(&portal2_dir).unwrap();

    // 2. Create appmanifest_620.acf
    let manifest_content = r#"
    "AppState"
    {
        "appid"         "620"
        "Universe"      "1"
        "name"          "Portal 2"
        "StateFlags"    "4"
        "installdir"    "Portal 2"
        "LastUpdated"   "1629828823"
        "SizeOnDisk"    "12000000000"
    }
    "#;
    let mut manifest_file = File::create(steamapps.join("appmanifest_620.acf")).unwrap();
    manifest_file
        .write_all(manifest_content.as_bytes())
        .unwrap();

    // 3. Create libraryfolders.vdf
    let vdf_content = format!(
        r#"
        "libraryfolders"
        {{
            "0"
            {{
                "path"      "{}"
                "apps"
                {{
                    "620"   "12000000000"
                }}
            }}
        }}
        "#,
        root_path.to_string_lossy().replace('\\', "\\\\")
    );
    let mut vdf_file = File::create(steamapps.join("libraryfolders.vdf")).unwrap();
    vdf_file.write_all(vdf_content.as_bytes()).unwrap();

    // 4. Instantiate SteamScanner with our mock path
    let scanner = SteamScanner::with_paths(vec![root_path.to_path_buf()]);
    let results = scanner.scan().await.unwrap();

    // 5. Assert results
    assert_eq!(results.len(), 1);
    let game = &results[0];

    assert_eq!(game.title, "Portal 2");
    assert_eq!(game.media_type, MediaType::Game);
    assert_eq!(game.external_id, Some("620".to_string()));
    assert_eq!(game.source, "steam");
    assert_eq!(
        game.working_directory,
        Some(portal2_dir.to_string_lossy().to_string())
    );

    let launch = game.launch.as_ref().unwrap();
    assert_eq!(launch.name, "Steam");
    assert_eq!(launch.launch_type, MediaLaunchType::Storefront);
    assert_eq!(launch.program, "steam");
    assert_eq!(launch.arguments, vec!["steam://run/620".to_string()]);
    assert_eq!(
        launch.working_directory,
        Some(portal2_dir.to_string_lossy().to_string())
    );
}

#[tokio::test]
async fn test_steam_scanner_ignores_downloading_games() {
    let temp_root = tempdir().unwrap();
    let root_path = temp_root.path();

    let steamapps = root_path.join("steamapps");
    fs::create_dir_all(&steamapps).unwrap();

    // StateFlags = 1026 means downloading / uninstalled
    let manifest_content = r#"
    "AppState"
    {
        "appid"         "730"
        "name"          "Counter-Strike 2"
        "StateFlags"    "1026"
        "installdir"    "Counter-Strike Global Offensive"
    }
    "#;
    let mut manifest_file = File::create(steamapps.join("appmanifest_730.acf")).unwrap();
    manifest_file
        .write_all(manifest_content.as_bytes())
        .unwrap();

    let scanner = SteamScanner::with_paths(vec![root_path.to_path_buf()]);
    let results = scanner.scan().await.unwrap();

    // Downloading games should be excluded
    assert_eq!(results.len(), 0);
}
