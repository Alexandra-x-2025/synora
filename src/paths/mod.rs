use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn resolve_synora_home_path(
    synora_home: Option<&str>,
    userprofile: Option<&str>,
    home: Option<&str>,
    cwd: &Path,
) -> PathBuf {
    if let Some(explicit) = synora_home {
        if !explicit.trim().is_empty() {
            return PathBuf::from(explicit);
        }
    }

    if let Some(up) = userprofile {
        if !up.trim().is_empty() {
            return PathBuf::from(up).join(".synora");
        }
    }

    if let Some(h) = home {
        if !h.trim().is_empty() {
            return PathBuf::from(h).join(".synora");
        }
    }

    cwd.join(".synora")
}

pub fn ensure_synora_home() -> io::Result<PathBuf> {
    let synora_home = env::var("SYNORA_HOME").ok();
    let userprofile = env::var("USERPROFILE").ok();
    let home = env::var("HOME").ok();
    let cwd = env::current_dir()?;

    let preferred = resolve_synora_home_path(
        synora_home.as_deref(),
        userprofile.as_deref(),
        home.as_deref(),
        &cwd,
    );

    // If user explicitly sets SYNORA_HOME, respect it strictly.
    if synora_home.as_deref().is_some_and(|v| !v.trim().is_empty()) {
        fs::create_dir_all(&preferred)?;
        return Ok(preferred);
    }

    match fs::create_dir_all(&preferred) {
        Ok(()) => Ok(preferred),
        Err(_) => {
            let fallback = cwd.join(".synora");
            fs::create_dir_all(&fallback)?;
            Ok(fallback)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::resolve_synora_home_path;
    use std::path::{Path, PathBuf};

    #[test]
    fn resolve_prefers_synora_home_env() {
        let cwd = Path::new("C:/repo/synora");
        let p = resolve_synora_home_path(
            Some("C:/custom/synora-home"),
            Some("C:/Users/tester"),
            Some("/home/tester"),
            cwd,
        );
        assert_eq!(p, PathBuf::from("C:/custom/synora-home"));
    }

    #[test]
    fn resolve_falls_back_to_userprofile() {
        let cwd = Path::new("C:/repo/synora");
        let p = resolve_synora_home_path(None, Some("C:/Users/tester"), None, cwd);
        assert_eq!(p, PathBuf::from("C:/Users/tester").join(".synora"));
    }

    #[test]
    fn resolve_falls_back_to_cwd_when_no_env() {
        let cwd = Path::new("/tmp/synora");
        let p = resolve_synora_home_path(None, None, None, cwd);
        assert_eq!(p, PathBuf::from("/tmp/synora").join(".synora"));
    }
}
