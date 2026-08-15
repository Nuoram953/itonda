use std::collections::HashMap;
use std::path::PathBuf;

pub use crate::scanner::errors::VdfParseError;

/// Represents a discovered Steam library storage folder from `libraryfolders.vdf`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SteamLibraryFolder {
    pub path: PathBuf,
    pub app_ids: Vec<String>,
}

/// Represents installed game metadata parsed from `appmanifest_<appid>.acf`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SteamAppManifest {
    pub app_id: String,
    pub name: String,
    pub install_dir: String,
    pub state_flags: u32,
    pub is_fully_installed: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Token {
    String(String),
    OpenBrace,
    CloseBrace,
}

fn tokenize_vdf(input: &str) -> Result<Vec<Token>, VdfParseError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Whitespace
            ' ' | '\t' | '\r' | '\n' => {
                chars.next();
            }
            // Comments (// ...)
            '/' => {
                chars.next();
                if chars.peek() == Some(&'/') {
                    while let Some(&c) = chars.peek() {
                        chars.next();
                        if c == '\n' {
                            break;
                        }
                    }
                }
            }
            // Braces
            '{' => {
                chars.next();
                tokens.push(Token::OpenBrace);
            }
            '}' => {
                chars.next();
                tokens.push(Token::CloseBrace);
            }
            // Quoted strings
            '"' => {
                chars.next(); // Consume opening quote
                let mut string_val = String::new();
                let mut closed = false;

                while let Some(c) = chars.next() {
                    if c == '\\' {
                        if let Some(next_c) = chars.next() {
                            string_val.push(next_c);
                        }
                    } else if c == '"' {
                        closed = true;
                        break;
                    } else {
                        string_val.push(c);
                    }
                }

                if !closed {
                    return Err(VdfParseError::UnterminatedString);
                }

                tokens.push(Token::String(string_val));
            }
            // Unquoted words
            _ => {
                let mut word = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_whitespace() || c == '{' || c == '}' || c == '"' {
                        break;
                    }
                    word.push(c);
                    chars.next();
                }
                if !word.is_empty() {
                    tokens.push(Token::String(word));
                }
            }
        }
    }

    Ok(tokens)
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyValue {
    Str(String),
    Section(HashMap<String, KeyValue>),
}

fn parse_key_values(
    tokens: &[Token],
    pos: &mut usize,
) -> Result<HashMap<String, KeyValue>, VdfParseError> {
    let mut map = HashMap::new();

    while *pos < tokens.len() {
        let key = match &tokens[*pos] {
            Token::CloseBrace => {
                *pos += 1;
                return Ok(map);
            }
            Token::String(k) => {
                *pos += 1;
                k.clone()
            }
            Token::OpenBrace => {
                return Err(VdfParseError::UnexpectedToken {
                    expected: "Key String".into(),
                    found: "{".into(),
                });
            }
        };

        if *pos >= tokens.len() {
            return Err(VdfParseError::UnexpectedEof);
        }

        match &tokens[*pos] {
            Token::OpenBrace => {
                *pos += 1;
                let child_section = parse_key_values(tokens, pos)?;
                map.insert(key, KeyValue::Section(child_section));
            }
            Token::String(val) => {
                let val_str = val.clone();
                *pos += 1;
                map.insert(key, KeyValue::Str(val_str));
            }
            Token::CloseBrace => {
                return Err(VdfParseError::UnexpectedToken {
                    expected: "Value String or {".into(),
                    found: "}".into(),
                });
            }
        }
    }

    Ok(map)
}

/// Parses the contents of `libraryfolders.vdf` into a list of `SteamLibraryFolder`.
pub fn parse_library_folders(content: &str) -> Result<Vec<SteamLibraryFolder>, VdfParseError> {
    let tokens = tokenize_vdf(content)?;
    let mut pos = 0;
    let root = parse_key_values(&tokens, &mut pos)?;

    let library_root = root
        .get("libraryfolders")
        .or_else(|| root.get("LibraryFolders"))
        .or_else(|| root.get("library_folders"));

    let root_map = match library_root {
        Some(KeyValue::Section(map)) => map,
        _ => &root,
    };

    let mut folders = Vec::new();

    for (key, val) in root_map {
        // Match numbered section keys ("0", "1", "2", ...)
        if key.chars().all(|c| c.is_ascii_digit()) {
            match val {
                // Modern Format: "0" { "path" "/path" "apps" { "220" "..." } }
                KeyValue::Section(folder_data) => {
                    if let Some(KeyValue::Str(path_str)) = folder_data.get("path") {
                        let mut app_ids = Vec::new();

                        if let Some(KeyValue::Section(apps_map)) = folder_data.get("apps") {
                            for app_id in apps_map.keys() {
                                app_ids.push(app_id.clone());
                            }
                        }

                        folders.push(SteamLibraryFolder {
                            path: PathBuf::from(path_str),
                            app_ids,
                        });
                    }
                }
                // Legacy Format: "1" "/mnt/games/SteamLibrary"
                KeyValue::Str(path_str) => {
                    folders.push(SteamLibraryFolder {
                        path: PathBuf::from(path_str),
                        app_ids: Vec::new(),
                    });
                }
            }
        }
    }

    folders.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(folders)
}

/// Parses the contents of an `appmanifest_<appid>.acf` file into a `SteamAppManifest`.
pub fn parse_app_manifest(content: &str) -> Result<SteamAppManifest, VdfParseError> {
    let tokens = tokenize_vdf(content)?;
    let mut pos = 0;
    let root = parse_key_values(&tokens, &mut pos)?;

    let app_root = root
        .get("AppState")
        .or_else(|| root.get("appstate"))
        .or_else(|| root.get("AppState_"));

    let app_map = match app_root {
        Some(KeyValue::Section(map)) => map,
        _ => &root,
    };

    let app_id = match app_map.get("appid") {
        Some(KeyValue::Str(s)) => s.clone(),
        _ => return Err(VdfParseError::MissingField("appid".into())),
    };

    let name = match app_map.get("name") {
        Some(KeyValue::Str(s)) => s.clone(),
        _ => match app_map.get("name_") {
            Some(KeyValue::Str(s)) => s.clone(),
            _ => return Err(VdfParseError::MissingField("name".into())),
        },
    };

    let install_dir = match app_map.get("installdir") {
        Some(KeyValue::Str(s)) => s.clone(),
        _ => name.clone(), // Fallback to game name if installdir is omitted
    };

    let state_flags = match app_map.get("StateFlags") {
        Some(KeyValue::Str(s)) => s.parse::<u32>().unwrap_or(0),
        _ => 0,
    };

    // StateFlags bit 4 (0x4) indicates STATE_FULLY_INSTALLED
    let is_fully_installed = (state_flags & 4) != 0;

    Ok(SteamAppManifest {
        app_id,
        name,
        install_dir,
        state_flags,
        is_fully_installed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_modern_libraryfolders() {
        let vdf = r#"
        "libraryfolders"
        {
            "0"
            {
                "path"      "/home/deck/.local/share/Steam"
                "label"     ""
                "apps"
                {
                    "220"   "10492810"
                    "620"   "12984012"
                }
            }
            "1"
            {
                "path"      "/run/media/mmcblk0p1"
                "label"     "SD Card"
                "apps"
                {
                    "730"   "34019283"
                }
            }
        }
        "#;

        let folders = parse_library_folders(vdf).unwrap();
        assert_eq!(folders.len(), 2);
        assert_eq!(
            folders[0].path,
            PathBuf::from("/home/deck/.local/share/Steam")
        );
        assert!(folders[0].app_ids.contains(&"620".to_string()));
        assert_eq!(folders[1].path, PathBuf::from("/run/media/mmcblk0p1"));
        assert!(folders[1].app_ids.contains(&"730".to_string()));
    }

    #[test]
    fn test_parse_legacy_libraryfolders() {
        let vdf = r#"
        "LibraryFolders"
        {
            "TimeNextStatsReport"   "12345678"
            "1"                     "/mnt/games/SteamLibrary"
            "2"                     "D:\\SteamLibrary"
        }
        "#;

        let folders = parse_library_folders(vdf).unwrap();
        assert_eq!(folders.len(), 2);
        assert_eq!(folders[0].path, PathBuf::from("/mnt/games/SteamLibrary"));
        assert_eq!(folders[1].path, PathBuf::from("D:\\SteamLibrary"));
    }

    #[test]
    fn test_parse_valid_app_manifest() {
        let acf = r#"
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

        let manifest = parse_app_manifest(acf).unwrap();
        assert_eq!(manifest.app_id, "620");
        assert_eq!(manifest.name, "Portal 2");
        assert_eq!(manifest.install_dir, "Portal 2");
        assert_eq!(manifest.state_flags, 4);
        assert!(manifest.is_fully_installed);
    }

    #[test]
    fn test_parse_downloading_app_manifest() {
        let acf = r#"
        "AppState"
        {
            "appid"         "730"
            "name"          "Counter-Strike 2"
            "StateFlags"    "1026"
            "installdir"    "Counter-Strike Global Offensive"
        }
        "#;

        let manifest = parse_app_manifest(acf).unwrap();
        assert_eq!(manifest.app_id, "730");
        assert_eq!(manifest.state_flags, 1026);
        assert!(!manifest.is_fully_installed);
    }
}
