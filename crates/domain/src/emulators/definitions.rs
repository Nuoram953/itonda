use std::{fs, path::Path};

use tracing::{info, warn};

use crate::emulators::models::EmulatorDefinition;

const BUNDLED_PRESETS: &[(&str, &str)] = &[
    ("pcsx2.toml", include_str!("presets/pcsx2.toml")),
    ("dolphin.toml", include_str!("presets/dolphin.toml")),
    ("duckstation.toml", include_str!("presets/duckstation.toml")),
    ("rpcs3.toml", include_str!("presets/rpcs3.toml")),
    ("retroarch.toml", include_str!("presets/retroarch.toml")),
];

/// Writes bundled emulator definition `.toml` files into `dir` if they do not already exist.
pub fn seed_emulator_definitions(dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;

    for (filename, contents) in BUNDLED_PRESETS {
        let target = dir.join(filename);
        if !target.exists() {
            fs::write(&target, contents)?;
            info!(path = ?target, "Seeded default emulator definition");
        }
    }

    Ok(())
}

/// Loads all `*.toml` emulator definition files from `dir`, sorted by name.
pub fn load_emulator_definitions(dir: &Path) -> Vec<EmulatorDefinition> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };

    let mut definitions = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }
        match fs::read_to_string(&path) {
            Ok(raw) => match toml::from_str::<EmulatorDefinition>(&raw) {
                Ok(def) => definitions.push(def),
                Err(err) => warn!(path = ?path, "Failed to parse emulator definition: {err}"),
            },
            Err(err) => warn!(path = ?path, "Failed to read emulator definition: {err}"),
        }
    }

    definitions.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    definitions
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn seeds_and_loads_bundled_emulator_definitions() {
        let dir = tempdir().unwrap();
        seed_emulator_definitions(dir.path()).unwrap();

        let defs = load_emulator_definitions(dir.path());
        assert_eq!(defs.len(), BUNDLED_PRESETS.len());
        assert!(defs.iter().any(|d| d.id == "pcsx2" && !d.options.is_empty()));
        assert!(defs.iter().any(|d| d.id == "dolphin"));
        assert!(defs.iter().any(|d| d.id == "retroarch"));
    }

    #[test]
    fn preserves_existing_user_modified_definition_file() {
        let dir = tempdir().unwrap();
        let custom_path = dir.path().join("pcsx2.toml");
        fs::write(
            &custom_path,
            r#"
id = "pcsx2"
name = "My Custom PCSX2"
default_program = "/opt/pcsx2"
"#,
        )
        .unwrap();

        seed_emulator_definitions(dir.path()).unwrap();
        let defs = load_emulator_definitions(dir.path());
        let pcsx2 = defs.iter().find(|d| d.id == "pcsx2").unwrap();
        assert_eq!(pcsx2.name, "My Custom PCSX2");
        assert_eq!(pcsx2.default_program, "/opt/pcsx2");
    }
}
