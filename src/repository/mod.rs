use std::fs;
use std::io;
use std::path::PathBuf;

use crate::paths::ensure_synora_home;

#[derive(Default, Clone)]
pub struct ConfigRepository {
    pub base_dir: Option<PathBuf>,
}

impl ConfigRepository {
    pub fn init_default(&self) -> io::Result<PathBuf> {
        let root = match &self.base_dir {
            Some(base) => base.clone(),
            None => ensure_synora_home()?,
        };

        fs::create_dir_all(&root)?;
        let config = root.join("config.json");
        if !config.exists() {
            let payload = format!(
                "{{\n  \"log_level\": \"INFO\",\n  \"quarantine_dir\": \"{}\",\n  \"allow_apply_updates\": false\n}}\n",
                root.join("quarantine").display()
            );
            fs::write(&config, payload)?;
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::ConfigRepository;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn init_default_creates_config_with_quarantine_dir() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let root = std::env::temp_dir().join(format!("synora-config-test-{unique}"));
        let repo = ConfigRepository {
            base_dir: Some(root.clone()),
        };

        let config_path = repo.init_default().expect("config init should succeed");
        let content = fs::read_to_string(&config_path).expect("config should be readable");

        assert!(config_path.exists());
        assert!(content.contains("\"quarantine_dir\""));
        assert!(content.contains("allow_apply_updates"));

        let _ = fs::remove_dir_all(root);
    }
}
