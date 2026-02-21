use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Default, Clone)]
pub struct ConfigRepository {
    pub base_dir: Option<PathBuf>,
}

impl ConfigRepository {
    pub fn init_default(&self) -> io::Result<PathBuf> {
        let root = self
            .base_dir
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
            .join(".synora");

        fs::create_dir_all(&root)?;
        let config = root.join("config.json");
        if !config.exists() {
            let payload = "{\n  \"log_level\": \"INFO\",\n  \"allow_apply_updates\": false\n}\n";
            fs::write(&config, payload)?;
        }

        Ok(config)
    }
}
