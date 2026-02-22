use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

use rusqlite::{params, Connection};
use serde_json::{json, Value};

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

#[derive(Debug, Clone, Default)]
pub struct ExecutionGateConfig {
    pub real_mutation_enabled: bool,
    pub gate_version: String,
    pub approval_record_ref: String,
}

impl ConfigRepository {
    fn resolve_base_dir(&self) -> io::Result<PathBuf> {
        match &self.base_dir {
            Some(base) => Ok(base.clone()),
            None => ensure_synora_home(),
        }
    }

    pub fn init_default(&self) -> io::Result<PathBuf> {
        let root = self.resolve_base_dir()?;

        fs::create_dir_all(&root)?;
        let config = root.join("config.json");
        if !config.exists() {
            let payload = serde_json::to_string_pretty(&json!({
                "log_level": "INFO",
                "quarantine_dir": root.join("quarantine").display().to_string(),
                "allow_apply_updates": false,
                "execution": {
                    "real_mutation_enabled": false,
                    "gate_version": "phase3-draft-v1",
                    "approval_record_ref": ""
                }
            }))
            .map(|v| format!("{v}\n"))
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?;
            fs::write(&config, payload)?;
        }

        Ok(config)
    }

    fn parse_gate_from_text_fallback(content: &str) -> ExecutionGateConfig {
        let real_mutation_enabled = if let Some(i) = content.find("\"real_mutation_enabled\"") {
            let tail = &content[i..];
            if let Some(c) = tail.find(':') {
                tail[c + 1..].trim_start().starts_with("true")
            } else {
                false
            }
        } else {
            false
        };

        let gate_version = if let Some(i) = content.find("\"gate_version\"") {
            let tail = &content[i..];
            if let Some(c) = tail.find(':') {
                let mut chars = tail[c + 1..].trim_start().chars();
                if chars.next() == Some('"') {
                    let rem: String = chars.collect();
                    rem.split('"').next().unwrap_or("phase3-draft-v1").to_string()
                } else {
                    "phase3-draft-v1".to_string()
                }
            } else {
                "phase3-draft-v1".to_string()
            }
        } else {
            "phase3-draft-v1".to_string()
        };

        let approval_record_ref = if let Some(i) = content.find("\"approval_record_ref\"") {
            let tail = &content[i..];
            if let Some(c) = tail.find(':') {
                let mut chars = tail[c + 1..].trim_start().chars();
                if chars.next() == Some('"') {
                    let rem: String = chars.collect();
                    rem.split('"').next().unwrap_or("").to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        ExecutionGateConfig {
            real_mutation_enabled,
            gate_version,
            approval_record_ref,
        }
    }

    pub fn load_execution_gate(&self) -> io::Result<ExecutionGateConfig> {
        let root = self.resolve_base_dir()?;
        let config_path = root.join("config.json");
        if !config_path.exists() {
            return Ok(ExecutionGateConfig {
                real_mutation_enabled: false,
                gate_version: "phase3-draft-v1".to_string(),
                approval_record_ref: String::new(),
            });
        }

        let content = fs::read_to_string(config_path)?;
        let parsed: Value = match serde_json::from_str(&content) {
            Ok(v) => v,
            Err(_) => return Ok(Self::parse_gate_from_text_fallback(&content)),
        };
        let execution = parsed
            .get("execution")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();

        let real_mutation_enabled = execution
            .get("real_mutation_enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let gate_version = execution
            .get("gate_version")
            .and_then(|v| v.as_str())
            .unwrap_or("phase3-draft-v1")
            .to_string();
        let approval_record_ref = execution
            .get("approval_record_ref")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        Ok(ExecutionGateConfig {
            real_mutation_enabled,
            gate_version,
            approval_record_ref,
        })
    }

    pub fn set_execution_gate(
        &self,
        real_mutation_enabled: bool,
        gate_version: &str,
        approval_record_ref: &str,
    ) -> io::Result<PathBuf> {
        let config_path = self.init_default()?;
        let root = self.resolve_base_dir()?;
        let raw = fs::read_to_string(&config_path).unwrap_or_else(|_| "{}".to_string());
        let mut parsed: Value = serde_json::from_str(&raw).unwrap_or_else(|_| json!({}));
        if !parsed.is_object() {
            parsed = json!({});
        }

        let obj = parsed
            .as_object_mut()
            .expect("parsed must be object after normalization");
        if !obj.contains_key("log_level") {
            obj.insert("log_level".to_string(), json!("INFO"));
        }
        if !obj.contains_key("quarantine_dir") {
            obj.insert(
                "quarantine_dir".to_string(),
                json!(root.join("quarantine").display().to_string()),
            );
        }
        if !obj.contains_key("allow_apply_updates") {
            obj.insert("allow_apply_updates".to_string(), json!(false));
        }

        let normalized_gate_version = if gate_version.trim().is_empty() {
            "phase3-draft-v1"
        } else {
            gate_version.trim()
        };
        obj.insert(
            "execution".to_string(),
            json!({
                "real_mutation_enabled": real_mutation_enabled,
                "gate_version": normalized_gate_version,
                "approval_record_ref": approval_record_ref
            }),
        );

        let payload = serde_json::to_string_pretty(&parsed)
            .map(|v| format!("{v}\n"))
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?;
        fs::write(&config_path, payload)?;
        Ok(config_path)
    }

    pub fn resolved_config_path(&self) -> io::Result<PathBuf> {
        let root = self.resolve_base_dir()?;
        Ok(root.join("config.json"))
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

#[derive(Debug, Clone)]
pub struct UpdateHistoryRow {
    pub id: i64,
    pub software_id: i64,
    pub old_version: String,
    pub new_version: String,
    pub timestamp: i64,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct UpdateAuditSummary {
    pub total: i64,
    pub planned_confirmed: i64,
    pub planned_dry_run: i64,
    pub latest_timestamp: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct GateHistoryRow {
    pub id: i64,
    pub timestamp: i64,
    pub real_mutation_enabled: bool,
    pub gate_version: String,
    pub approval_record_ref: String,
    pub reason: String,
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

            CREATE TABLE IF NOT EXISTS gate_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                real_mutation_enabled INTEGER NOT NULL,
                gate_version TEXT NOT NULL,
                approval_record_ref TEXT NOT NULL,
                reason TEXT NOT NULL
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

    pub fn list_update_history(&self) -> Result<Vec<UpdateHistoryRow>, RepositoryError> {
        let db_path = self.init_schema()?;
        let conn = Connection::open(db_path)?;
        let mut stmt = conn.prepare(
            "SELECT id, software_id, old_version, new_version, timestamp, status
             FROM update_history
             ORDER BY timestamp DESC, id DESC",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(UpdateHistoryRow {
                id: row.get(0)?,
                software_id: row.get(1)?,
                old_version: row.get(2)?,
                new_version: row.get(3)?,
                timestamp: row.get(4)?,
                status: row.get(5)?,
            })
        })?;

        let mut items = Vec::new();
        for row in rows {
            items.push(row?);
        }
        Ok(items)
    }

    pub fn get_update_audit_summary(&self) -> Result<UpdateAuditSummary, RepositoryError> {
        let db_path = self.init_schema()?;
        let conn = Connection::open(db_path)?;

        let summary = conn.query_row(
            "
            SELECT
                COUNT(*) AS total,
                COALESCE(SUM(CASE WHEN status = 'planned_confirmed' THEN 1 ELSE 0 END), 0) AS planned_confirmed,
                COALESCE(SUM(CASE WHEN status = 'planned_dry_run' THEN 1 ELSE 0 END), 0) AS planned_dry_run,
                MAX(timestamp) AS latest_timestamp
            FROM update_history
            ",
            [],
            |row| {
                Ok(UpdateAuditSummary {
                    total: row.get(0)?,
                    planned_confirmed: row.get(1)?,
                    planned_dry_run: row.get(2)?,
                    latest_timestamp: row.get(3)?,
                })
            },
        )?;

        Ok(summary)
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

    pub fn log_gate_change(
        &self,
        timestamp: i64,
        real_mutation_enabled: bool,
        gate_version: &str,
        approval_record_ref: &str,
        reason: &str,
    ) -> Result<(), RepositoryError> {
        let db_path = self.init_schema()?;
        let conn = Connection::open(db_path)?;
        conn.execute(
            "INSERT INTO gate_history (timestamp, real_mutation_enabled, gate_version, approval_record_ref, reason)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                timestamp,
                if real_mutation_enabled { 1 } else { 0 },
                gate_version,
                approval_record_ref,
                reason
            ],
        )?;
        Ok(())
    }

    pub fn list_gate_history(&self) -> Result<Vec<GateHistoryRow>, RepositoryError> {
        let db_path = self.init_schema()?;
        let conn = Connection::open(db_path)?;
        let mut stmt = conn.prepare(
            "SELECT id, timestamp, real_mutation_enabled, gate_version, approval_record_ref, reason
             FROM gate_history
             ORDER BY timestamp DESC, id DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            let enabled: i64 = row.get(2)?;
            Ok(GateHistoryRow {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                real_mutation_enabled: enabled != 0,
                gate_version: row.get(3)?,
                approval_record_ref: row.get(4)?,
                reason: row.get(5)?,
            })
        })?;

        let mut items = Vec::new();
        for row in rows {
            items.push(row?);
        }
        Ok(items)
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
        assert!(content.contains("real_mutation_enabled"));
        assert!(content.contains("approval_record_ref"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn load_execution_gate_reads_defaults_when_file_missing() {
        let root = unique_dir("synora-gate-default-test");
        let repo = ConfigRepository {
            base_dir: Some(root.clone()),
        };

        let gate = repo
            .load_execution_gate()
            .expect("load_execution_gate should succeed");
        assert!(!gate.real_mutation_enabled);
        assert_eq!(gate.gate_version, "phase3-draft-v1");
        assert!(gate.approval_record_ref.is_empty());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn load_execution_gate_reads_config_values() {
        let root = unique_dir("synora-gate-config-test");
        fs::create_dir_all(&root).expect("test dir should be created");
        fs::write(
            root.join("config.json"),
            "{\n  \"execution\": {\n    \"real_mutation_enabled\": true,\n    \"gate_version\": \"phase3-custom\",\n    \"approval_record_ref\": \"docs/security/approval.md\"\n  }\n}\n",
        )
        .expect("config should be written");

        let repo = ConfigRepository {
            base_dir: Some(root.clone()),
        };

        let gate = repo
            .load_execution_gate()
            .expect("load_execution_gate should succeed");
        assert!(gate.real_mutation_enabled);
        assert_eq!(gate.gate_version, "phase3-custom");
        assert_eq!(gate.approval_record_ref, "docs/security/approval.md");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn load_execution_gate_fallback_parses_legacy_malformed_json() {
        let root = unique_dir("synora-gate-legacy-config-test");
        fs::create_dir_all(&root).expect("test dir should be created");
        fs::write(
            root.join("config.json"),
            "{\n  \"quarantine_dir\": \"C:\\dev\\synora\\.synora_custom\\quarantine\",\n  \"execution\": {\n    \"real_mutation_enabled\": true,\n    \"approval_record_ref\": \"docs/security/approval.md\"\n  }\n}\n",
        )
        .expect("config should be written");

        let repo = ConfigRepository {
            base_dir: Some(root.clone()),
        };

        let gate = repo
            .load_execution_gate()
            .expect("load_execution_gate should succeed");
        assert!(gate.real_mutation_enabled);
        assert_eq!(gate.gate_version, "phase3-draft-v1");
        assert_eq!(gate.approval_record_ref, "docs/security/approval.md");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn set_execution_gate_persists_values_and_roundtrips() {
        let root = unique_dir("synora-gate-set-roundtrip-test");
        let repo = ConfigRepository {
            base_dir: Some(root.clone()),
        };

        let path = repo
            .set_execution_gate(true, "phase3-go-live-v1", "docs/security/approved.md")
            .expect("set_execution_gate should succeed");
        assert!(path.exists());

        let gate = repo
            .load_execution_gate()
            .expect("load_execution_gate should succeed");
        assert!(gate.real_mutation_enabled);
        assert_eq!(gate.gate_version, "phase3-go-live-v1");
        assert_eq!(gate.approval_record_ref, "docs/security/approved.md");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn gate_history_roundtrip() {
        let root = unique_dir("synora-gate-history-roundtrip-test");
        let db_path = root.join("db").join("synora.db");
        let repo = DatabaseRepository {
            db_path: Some(db_path),
        };

        repo.log_gate_change(
            1_700_123_456,
            true,
            "phase3-go-live-v1",
            "docs/security/approved.md",
            "enable for pilot",
        )
        .expect("gate log should succeed");

        let rows = repo.list_gate_history().expect("gate history should read");
        assert_eq!(rows.len(), 1);
        assert!(rows[0].real_mutation_enabled);
        assert_eq!(rows[0].gate_version, "phase3-go-live-v1");
        assert_eq!(rows[0].reason, "enable for pilot");

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

        let history = repo
            .list_update_history()
            .expect("list update history should succeed");
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].status, "success");
        assert_eq!(history[0].old_version, "2.44.0");
        assert_eq!(history[0].new_version, "2.45.0");

        let summary = repo
            .get_update_audit_summary()
            .expect("summary query should succeed");
        assert_eq!(summary.total, 1);
        assert_eq!(summary.planned_confirmed, 0);
        assert_eq!(summary.planned_dry_run, 0);
        assert_eq!(summary.latest_timestamp, Some(1_700_000_000));

        let _ = fs::remove_dir_all(root);
    }
}
