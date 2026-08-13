//! Structured logging setup for Kelpie (kelpie.md §115 Diagnostics, §121-122 Linux
//! Integration/Filesystem Layout).
//!
//! Installs a global `tracing` subscriber that writes structured, newline-delimited
//! log records to a daily-rotating file under `$XDG_STATE_HOME/kelpie/logs`, plus a
//! human-readable stderr layer active in debug builds (or when `KELPIE_LOG_STDERR` is
//! set), so developers see logs in the terminal without having to tail the log file.

use std::io;
use std::path::PathBuf;
use std::sync::OnceLock;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

use crate::paths;

/// Name prefix for rotated log files, e.g. `kelpie.log.2026-08-13`.
const LOG_FILE_PREFIX: &str = "kelpie.log";

/// Holds the non-blocking file writer's flush guard for the process lifetime.
/// Dropping it would stop the background writer thread and could lose buffered
/// log lines, so it must outlive every caller of `init`.
static LOG_GUARD: OnceLock<WorkerGuard> = OnceLock::new();

/// Initializes global structured logging and returns the directory logs are written
/// to (`$XDG_STATE_HOME/kelpie/logs`).
///
/// Writes daily-rotating, non-blocking file logs, and additionally mirrors log
/// records to stderr when running a debug build or when `KELPIE_LOG_STDERR` is set
/// in the environment. The minimum log level is controlled by the `RUST_LOG`
/// environment variable (see [`tracing_subscriber::EnvFilter`]), defaulting to
/// `info` when unset or invalid.
///
/// Safe to call more than once (e.g. from tests): only the first call installs the
/// global subscriber and stores the flush guard; later calls still ensure the log
/// directory exists and return its path, but are otherwise no-ops.
pub fn init() -> io::Result<PathBuf> {
    let log_dir = paths::state_dir()?.join("logs");
    std::fs::create_dir_all(&log_dir)?;

    let file_appender = tracing_appender::rolling::daily(&log_dir, LOG_FILE_PREFIX);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(true);

    let stderr_active =
        cfg!(debug_assertions) || std::env::var_os("KELPIE_LOG_STDERR").is_some();

    // Built once per call so `RUST_LOG` (or the `info` default) applies identically
    // to both layers.
    let env_filter = || EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = tracing_subscriber::registry()
        .with(env_filter())
        .with(file_layer);

    let install_result = if stderr_active {
        let stderr_layer = fmt::layer().with_writer(io::stderr).with_target(true);
        subscriber.with(stderr_layer).try_init()
    } else {
        subscriber.try_init()
    };

    // A second call (e.g. from another test in this process) finds the global
    // subscriber already installed; that's expected and not an error condition
    // for callers of `init`.
    if install_result.is_ok() {
        // Only the call that actually installed the subscriber owns a meaningful
        // guard; store it so the writer thread keeps running for the process.
        let _ = LOG_GUARD.set(guard);
        tracing::info!(log_dir = %log_dir.display(), "kelpie logging initialized");
    }

    Ok(log_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_creates_and_returns_the_log_dir() {
        let log_dir = init().expect("logging init should succeed");
        assert!(log_dir.is_dir());
        assert!(log_dir.ends_with("logs"));
    }

    #[test]
    fn init_is_idempotent() {
        let first = init().expect("first init should succeed");
        let second = init().expect("second init should succeed");
        assert_eq!(first, second);
    }
}
