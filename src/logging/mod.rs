use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::paths::ensure_synora_home;

pub fn ensure_log_file() -> io::Result<PathBuf> {
    let home = ensure_synora_home()?;
    let logs_dir = home.join("logs");
    fs::create_dir_all(&logs_dir)?;

    let log_path = logs_dir.join("synora.log");
    if !log_path.exists() {
        fs::write(&log_path, b"")?;
    }
    Ok(log_path)
}

pub fn log_event(level: &str, message: &str) -> io::Result<()> {
    let log_path = ensure_log_file()?;
    let mut file = OpenOptions::new().create(true).append(true).open(log_path)?;
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    writeln!(file, "{ts} {level} {message}")?;
    Ok(())
}
