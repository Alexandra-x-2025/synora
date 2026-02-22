use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

use rusqlite::{params, Connection};

use crate::paths::ensure_synora_home;

#[derive(Debug)]
pub enum RepositoryError {
    Io(io::Error),
    Sql(rusqlite::Error),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::Io(err) => write!(f, "io error: {err}"),
            RepositoryError::Sql(err) => write!(f, "sqlite error: {err}"),
        }
    }
}

impl From<io::Error> for RepositoryError {
    fn from(value: io::Error) -> Self {
        RepositoryError::Io(value)
    }
}

impl From<rusqlite::Error> for RepositoryError {
    fn from(value: rusqlite::Error) -> Self {
        RepositoryError::Sql(value)
    }
}

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

#[derive(Default, Clone)]
pub struct DatabaseRepository {
    pub db_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct SoftwareRow {
    pub id: i64,
    pub name: String,
    pub version: String,
    pub source: String,
    pub install_path: String,
    pub risk_level: String,
}

impl DatabaseRepository {
    pub fn resolved_db_path(&self) -> Result<PathBuf, RepositoryError> {
        if let Some(path) = &self.db_path {
            return Ok(path.clone());
        }
        let root = ensure_synora_home()?;
        Ok(root.join("db").join("synora.db"))
    }

    pub fn init_schema(&self) -> Result<PathBuf, RepositoryError> {
        let db_path = self.resolved_db_path()?;
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&db_path)?;
        conn.execute_batch(
            "
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS software (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                version TEXT NOT NULL,
                source TEXT NOT NULL,
                install_path TEXT NOT NULL,
                risk_level TEXT NOT NULL,
                UNIQUE(name, source)
            );

            CREATE TABLE IF NOT EXISTS update_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                software_id INTEGER NOT NULL,
                old_version TEXT NOT NULL,
                new_version TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                status TEXT NOT NULL,
                FOREIGN KEY (software_id) REFERENCES software(id)
            );

            CREATE TABLE IF NOT EXISTS quarantine (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                software_id INTEGER NOT NULL,
                file_path TEXT NOT NULL,
                original_location TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                reason TEXT NOT NULL,
                FOREIGN KEY (software_id) REFERENCES software(id)
            );

            CREATE TABLE IF NOT EXISTS registry_backup (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                hive TEXT NOT NULL,
                key_path TEXT NOT NULL,
                backup_blob TEXT NOT NULL,
                timestamp INTEGER NOT NULL
            );
            ",
        )?;

        Ok(db_path)
    }

    pub fn upsert_software(
        &self,
        name: &str,
        version: &str,
        source: &str,
        install_path: &str,
        risk_level: &str,
    ) -> Result<i64, RepositoryError> {
        let db_path = self.init_schema()?;
        let conn = Connection::open(db_path)?;

        conn.execute(
            "
            INSERT INTO software (name, version, source, install_path, risk_level)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT(name, source) DO UPDATE SET
                version = excluded.version,
                install_path = excluded.install_path,
                risk_level = excluded.risk_level
            ",
            params![name, version, source, install_path, risk_level],
        )?;

        let id: i64 = conn.query_row(
            "SELECT id FROM software WHERE name = ?1 AND source = ?2",
            params![name, source],
            |row| row.get(0),
        )?;
        Ok(id)
    }

    pub fn list_software(&self) -> Result<Vec<SoftwareRow>, RepositoryError> {
        let db_path = self.init_schema()?;
        let conn = Connection::open(db_path)?;
        let mut stmt = conn.prepare(
            "SELECT id, name, version, source, install_path, risk_level FROM software ORDER BY name ASC",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(SoftwareRow {
                id: row.get(0)?,
                name: row.get(1)?,
                version: row.get(2)?,
                source: row.get(3)?,
                install_path: row.get(4)?,
                risk_level: row.get(5)?,
            })
        })?;

        let mut items = Vec::new();
        for row in rows {
            items.push(row?);
        }
        Ok(items)
    }

    pub fn log_update(
        &self,
        software_id: i64,
        old_version: &str,
        new_version: &str,
        timestamp: i64,
        status: &str,
    ) -> Result<(), RepositoryError> {
        let db_path = self.init_schema()?;
        let conn = Connection::open(db_path)?;

        conn.execute(
            "INSERT INTO update_history (software_id, old_version, new_version, timestamp, status)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![software_id, old_version, new_version, timestamp, status],
        )?;
        Ok(())
    }

    pub fn add_quarantine_entry(
        &self,
        software_id: i64,
        file_path: &str,
        original_location: &str,
        timestamp: i64,
        reason: &str,
    ) -> Result<(), RepositoryError> {
        let db_path = self.init_schema()?;
        let conn = Connection::open(db_path)?;

        conn.execute(
            "INSERT INTO quarantine (software_id, file_path, original_location, timestamp, reason)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![software_id, file_path, original_location, timestamp, reason],
        )?;
        Ok(())
    }

    pub fn add_registry_backup(
        &self,
        hive: &str,
        key_path: &str,
        backup_blob: &str,
        timestamp: i64,
    ) -> Result<(), RepositoryError> {
        let db_path = self.init_schema()?;
        let conn = Connection::open(db_path)?;

        conn.execute(
            "INSERT INTO registry_backup (hive, key_path, backup_blob, timestamp)
             VALUES (?1, ?2, ?3, ?4)",
            params![hive, key_path, backup_blob, timestamp],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{ConfigRepository, DatabaseRepository};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_dir(prefix: &str) -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        std::env::temp_dir().join(format!("{prefix}-{unique}"))
    }

    #[test]
    fn init_default_creates_config_with_quarantine_dir() {
        let root = unique_dir("synora-config-test");
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

    #[test]
    fn init_schema_creates_db_file() {
        let root = unique_dir("synora-db-init-test");
        let db_path = root.join("db").join("synora.db");
        let repo = DatabaseRepository {
            db_path: Some(db_path.clone()),
        };

        let created = repo.init_schema().expect("schema init should succeed");
        assert_eq!(created, db_path);
        assert!(created.exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn software_upsert_and_update_history_roundtrip() {
        let root = unique_dir("synora-db-roundtrip-test");
        let db_path = root.join("db").join("synora.db");
        let repo = DatabaseRepository {
            db_path: Some(db_path.clone()),
        };

        let software_id = repo
            .upsert_software("Git", "2.44.0", "winget", "C:/Program Files/Git", "low")
            .expect("upsert should succeed");

        repo.log_update(software_id, "2.44.0", "2.45.0", 1_700_000_000, "success")
            .expect("update history insert should succeed");

        repo.add_quarantine_entry(
            software_id,
            "C:/Program Files/Git/tmp.old",
            "C:/Program Files/Git/tmp",
            1_700_000_100,
            "cleanup",
        )
        .expect("quarantine insert should succeed");

        repo.add_registry_backup(
            "HKCU",
            "Software\\Synora\\Git",
            "{\"key\":\"value\"}",
            1_700_000_200,
        )
        .expect("registry backup insert should succeed");

        let software = repo.list_software().expect("list software should succeed");
        assert_eq!(software.len(), 1);
        assert_eq!(software[0].name, "Git");
        assert_eq!(software[0].version, "2.44.0");

        let _ = fs::remove_dir_all(root);
    }
}
