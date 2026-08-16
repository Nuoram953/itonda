use async_trait::async_trait;
use std::path::Path;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};

use crate::tracker::{
    errors::TrackerError,
    models::{ProcessInfo, TrackTarget},
    traits::MediaTracker,
};

#[cfg(test)]
mod tests;

pub fn matches_directory(
    target_dir: &Path,
    proc_cwd: Option<&Path>,
    proc_exe: Option<&Path>,
) -> bool {
    let target = std::fs::canonicalize(target_dir).unwrap_or_else(|_| target_dir.to_path_buf());

    let cwd_matches = proc_cwd.is_some_and(|c| {
        std::fs::canonicalize(c)
            .unwrap_or_else(|_| c.to_path_buf())
            .starts_with(&target)
    });

    let exe_matches = proc_exe.is_some_and(|e| {
        std::fs::canonicalize(e)
            .unwrap_or_else(|_| e.to_path_buf())
            .starts_with(&target)
    });

    cwd_matches || exe_matches
}

#[derive(Debug, Default, Clone)]
pub struct DirectoryTracker;

impl DirectoryTracker {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl MediaTracker for DirectoryTracker {
    fn name(&self) -> &'static str {
        "Directory"
    }

    fn is_available(&self) -> bool {
        true
    }

    async fn find_processes(&self, target: &TrackTarget) -> Result<Vec<ProcessInfo>, TrackerError> {
        let Some(target_dir) = &target.working_directory else {
            return Err(TrackerError::InvalidDirectory(
                "Working directory must be specified for DirectoryTracker".into(),
            ));
        };

        let mut sys = System::new();
        sys.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing()
                .with_cwd(UpdateKind::Always)
                .with_exe(UpdateKind::Always),
        );

        let matched = sys
            .processes()
            .iter()
            .filter_map(|(pid, proc)| {
                let cwd = proc.cwd();
                let exe = proc.exe();

                if matches_directory(target_dir, cwd, exe) {
                    Some(ProcessInfo {
                        pid: pid.as_u32(),
                        name: proc.name().to_string_lossy().to_string(),
                        exe: exe.map(|p| p.to_path_buf()),
                        cwd: cwd.map(|p| p.to_path_buf()),
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(matched)
    }
}
