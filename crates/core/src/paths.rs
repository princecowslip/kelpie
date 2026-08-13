//! XDG base directory resolution for Kelpie (kelpie.md §121 Linux Integration, §122 Filesystem Layout).
//!
//! Each function returns the Kelpie-specific subdirectory for that XDG base directory,
//! creating it if it does not already exist.

use std::io;
use std::path::PathBuf;

use directories::ProjectDirs;

fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("dev", "kelpie", "kelpie")
        .expect("no valid home directory could be found for the current user")
}

fn ensure_dir(path: PathBuf) -> io::Result<PathBuf> {
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

/// `$XDG_CONFIG_HOME/kelpie` — settings, provider manifests the user has installed.
pub fn config_dir() -> io::Result<PathBuf> {
    ensure_dir(project_dirs().config_dir().to_path_buf())
}

/// `$XDG_DATA_HOME/kelpie` — the SQLite database, downloaded media, provider packages.
pub fn data_dir() -> io::Result<PathBuf> {
    ensure_dir(project_dirs().data_dir().to_path_buf())
}

/// `$XDG_CACHE_HOME/kelpie` — thumbnails, feed cache, other regenerable data.
pub fn cache_dir() -> io::Result<PathBuf> {
    let dir = project_dirs()
        .cache_dir()
        .to_path_buf();
    ensure_dir(dir)
}

/// `$XDG_STATE_HOME/kelpie` — logs and other state that isn't safe to delete but isn't
/// critical user data either.
pub fn state_dir() -> io::Result<PathBuf> {
    let dir = project_dirs()
        .state_dir()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| project_dirs().data_local_dir().join("state"));
    ensure_dir(dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_dirs_resolve_and_are_created() {
        assert!(config_dir().unwrap().is_dir());
        assert!(data_dir().unwrap().is_dir());
        assert!(cache_dir().unwrap().is_dir());
        assert!(state_dir().unwrap().is_dir());
    }
}
