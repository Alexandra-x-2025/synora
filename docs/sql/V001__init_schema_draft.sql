-- Synora Draft DB Schema (SQLite)
-- Version: V001
-- Status: Draft / Temporary baseline

PRAGMA foreign_keys = ON;

BEGIN;

CREATE TABLE IF NOT EXISTS schema_meta (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL,
  updated_at INTEGER NOT NULL
);

INSERT OR REPLACE INTO schema_meta (key, value, updated_at)
VALUES ('schema_version', 'V001_draft', CAST(strftime('%s','now') AS INTEGER));

CREATE TABLE IF NOT EXISTS software_inventory (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  version TEXT,
  publisher TEXT,
  install_location TEXT,
  discovery_source TEXT NOT NULL CHECK (discovery_source IN ('registry', 'manual')),
  source_confidence INTEGER NOT NULL DEFAULT 50 CHECK (source_confidence BETWEEN 0 AND 100),
  first_seen_at INTEGER NOT NULL,
  last_seen_at INTEGER NOT NULL,
  is_active INTEGER NOT NULL DEFAULT 1 CHECK (is_active IN (0,1)),
  fingerprint TEXT
);

CREATE INDEX IF NOT EXISTS idx_software_name_publisher
  ON software_inventory (name, publisher);
CREATE INDEX IF NOT EXISTS idx_software_last_seen
  ON software_inventory (last_seen_at DESC);

CREATE TABLE IF NOT EXISTS source_candidate (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  software_id INTEGER NOT NULL,
  url TEXT NOT NULL,
  domain TEXT NOT NULL,
  channel TEXT,
  confidence INTEGER NOT NULL CHECK (confidence BETWEEN 0 AND 100),
  reason TEXT,
  status TEXT NOT NULL CHECK (status IN ('pending', 'approved', 'rejected')),
  created_at INTEGER NOT NULL,
  reviewed_at INTEGER,
  review_note TEXT,
  FOREIGN KEY (software_id) REFERENCES software_inventory(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_candidate_software_status
  ON source_candidate (software_id, status);
CREATE INDEX IF NOT EXISTS idx_candidate_confidence
  ON source_candidate (confidence DESC);

CREATE TABLE IF NOT EXISTS update_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  software_id INTEGER NOT NULL,
  operation TEXT NOT NULL CHECK (operation IN ('install', 'upgrade', 'uninstall')),
  old_version TEXT,
  new_version TEXT,
  requested_mode TEXT NOT NULL CHECK (requested_mode IN ('dry-run', 'confirm')),
  status TEXT NOT NULL,
  risk_level TEXT NOT NULL CHECK (risk_level IN ('low', 'medium', 'high')),
  message TEXT,
  created_at INTEGER NOT NULL,
  FOREIGN KEY (software_id) REFERENCES software_inventory(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_update_history_software_created
  ON update_history (software_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_update_history_status
  ON update_history (status);

CREATE TABLE IF NOT EXISTS cleanup_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  operation_id TEXT NOT NULL,
  package_id TEXT NOT NULL,
  requested_mode TEXT NOT NULL CHECK (requested_mode IN ('dry-run', 'confirm')),
  status TEXT NOT NULL CHECK (
    status IN (
      'quarantine_planned',
      'quarantine_confirmed',
      'quarantine_success',
      'quarantine_failed',
      'quarantine_rollback_success',
      'quarantine_rollback_failed'
    )
  ),
  mutation_boundary_reached INTEGER NOT NULL CHECK (mutation_boundary_reached IN (0,1)),
  rollback_attempted INTEGER NOT NULL CHECK (rollback_attempted IN (0,1)),
  rollback_status TEXT NOT NULL CHECK (rollback_status IN ('not_needed', 'success', 'failed')),
  risk_level TEXT NOT NULL CHECK (risk_level IN ('low', 'medium', 'high')),
  created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_cleanup_history_operation
  ON cleanup_history (operation_id);
CREATE INDEX IF NOT EXISTS idx_cleanup_history_created
  ON cleanup_history (created_at DESC);

CREATE TABLE IF NOT EXISTS gate_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  timestamp INTEGER NOT NULL,
  real_mutation_enabled INTEGER NOT NULL CHECK (real_mutation_enabled IN (0,1)),
  gate_version TEXT NOT NULL,
  approval_record_ref TEXT,
  approval_record_present INTEGER NOT NULL CHECK (approval_record_present IN (0,1)),
  reason TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_gate_history_timestamp
  ON gate_history (timestamp DESC);

CREATE TABLE IF NOT EXISTS audit_event (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  event_type TEXT NOT NULL,
  actor TEXT NOT NULL,
  target TEXT,
  result TEXT NOT NULL,
  severity TEXT NOT NULL CHECK (severity IN ('low', 'medium', 'high', 'critical')),
  payload_json TEXT,
  created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_audit_event_type_created
  ON audit_event (event_type, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_audit_event_severity_created
  ON audit_event (severity, created_at DESC);

CREATE TABLE IF NOT EXISTS plugin_registry (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  plugin_id TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  version TEXT NOT NULL,
  kind TEXT NOT NULL CHECK (kind IN ('source_provider', 'update_policy', 'ai_tool', 'system_tool')),
  entry TEXT NOT NULL,
  runtime TEXT NOT NULL CHECK (runtime IN ('native', 'wasm')),
  api_compat TEXT NOT NULL,
  permissions_json TEXT NOT NULL,
  actions_json TEXT NOT NULL,
  signature TEXT,
  trust_status TEXT NOT NULL CHECK (trust_status IN ('trusted', 'untrusted', 'blocked')),
  lifecycle_status TEXT NOT NULL DEFAULT 'installed' CHECK (
    lifecycle_status IN ('discovered', 'installed', 'enabled', 'running', 'disabled', 'blocked')
  ),
  enabled INTEGER NOT NULL DEFAULT 0 CHECK (enabled IN (0,1)),
  installed_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_plugin_registry_kind_enabled
  ON plugin_registry (kind, enabled);
CREATE INDEX IF NOT EXISTS idx_plugin_registry_trust
  ON plugin_registry (trust_status);

CREATE TABLE IF NOT EXISTS plugin_execution_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  plugin_id TEXT NOT NULL,
  action TEXT NOT NULL,
  requested_by TEXT NOT NULL,
  status TEXT NOT NULL CHECK (status IN ('planned', 'running', 'success', 'failed', 'blocked')),
  result_code INTEGER NOT NULL CHECK (result_code IN (0,2,3,4,10)),
  result_summary TEXT,
  error_message TEXT,
  started_at INTEGER NOT NULL,
  finished_at INTEGER
);

CREATE INDEX IF NOT EXISTS idx_plugin_exec_plugin_started
  ON plugin_execution_history (plugin_id, started_at DESC);
CREATE INDEX IF NOT EXISTS idx_plugin_exec_status
  ON plugin_execution_history (status);

CREATE TABLE IF NOT EXISTS ai_insight_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  insight_type TEXT NOT NULL CHECK (insight_type IN ('analyze', 'recommend', 'repair_plan')),
  input_summary TEXT NOT NULL,
  result_json TEXT NOT NULL,
  confidence INTEGER NOT NULL CHECK (confidence BETWEEN 0 AND 100),
  risk_level TEXT NOT NULL CHECK (risk_level IN ('low', 'medium', 'high')),
  created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_ai_insight_type_created
  ON ai_insight_history (insight_type, created_at DESC);

CREATE TABLE IF NOT EXISTS repair_plan_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  target_software TEXT NOT NULL,
  issue_summary TEXT NOT NULL,
  plan_steps_json TEXT NOT NULL,
  rollback_hint TEXT,
  status TEXT NOT NULL CHECK (
    status IN ('repair_planned', 'repair_reviewed', 'repair_applied', 'repair_rejected')
  ),
  requested_by TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  applied_at INTEGER
);

CREATE INDEX IF NOT EXISTS idx_repair_plan_status_created
  ON repair_plan_history (status, created_at DESC);

CREATE TABLE IF NOT EXISTS job_queue (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  job_type TEXT NOT NULL,
  payload_json TEXT NOT NULL,
  priority INTEGER NOT NULL DEFAULT 50,
  status TEXT NOT NULL CHECK (status IN ('queued', 'running', 'success', 'failed', 'retrying', 'deadletter')),
  attempt_count INTEGER NOT NULL DEFAULT 0,
  max_attempts INTEGER NOT NULL DEFAULT 3,
  scheduled_at INTEGER NOT NULL,
  started_at INTEGER,
  finished_at INTEGER,
  last_error TEXT,
  created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_job_queue_status_priority_scheduled
  ON job_queue (status, priority DESC, scheduled_at ASC);
CREATE INDEX IF NOT EXISTS idx_job_queue_type_created
  ON job_queue (job_type, created_at DESC);

CREATE TABLE IF NOT EXISTS scheduler_task (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  task_name TEXT NOT NULL UNIQUE,
  job_type TEXT NOT NULL,
  cron_expr TEXT NOT NULL,
  enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0,1)),
  last_enqueued_at INTEGER,
  next_run_at INTEGER,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_scheduler_enabled_next_run
  ON scheduler_task (enabled, next_run_at ASC);

CREATE TABLE IF NOT EXISTS download_task_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  job_id INTEGER,
  software_id INTEGER,
  source_url TEXT NOT NULL,
  target_path TEXT NOT NULL,
  status TEXT NOT NULL CHECK (
    status IN ('download_queued', 'download_running', 'download_success', 'download_failed')
  ),
  bytes_total INTEGER,
  bytes_downloaded INTEGER,
  checksum_expected TEXT,
  checksum_actual TEXT,
  verify_status TEXT NOT NULL CHECK (verify_status IN ('not_verified', 'verify_success', 'verify_failed')),
  started_at INTEGER,
  finished_at INTEGER,
  error_message TEXT,
  created_at INTEGER NOT NULL,
  FOREIGN KEY (software_id) REFERENCES software_inventory(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_download_task_status_created
  ON download_task_history (status, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_download_task_software_created
  ON download_task_history (software_id, created_at DESC);

CREATE TABLE IF NOT EXISTS download_artifact (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  software_id INTEGER,
  file_path TEXT NOT NULL,
  sha256 TEXT NOT NULL,
  size_bytes INTEGER NOT NULL,
  source_url TEXT NOT NULL,
  trust_level TEXT NOT NULL CHECK (trust_level IN ('trusted', 'untrusted', 'unknown')),
  created_at INTEGER NOT NULL,
  expires_at INTEGER,
  FOREIGN KEY (software_id) REFERENCES software_inventory(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_download_artifact_software_created
  ON download_artifact (software_id, created_at DESC);
CREATE UNIQUE INDEX IF NOT EXISTS idx_download_artifact_file_sha
  ON download_artifact (file_path, sha256);

CREATE TABLE IF NOT EXISTS repository_registry (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  repo_id TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  kind TEXT NOT NULL CHECK (kind IN ('public', 'personal', 'candidate')),
  source TEXT NOT NULL,
  trust_status TEXT NOT NULL CHECK (trust_status IN ('trusted', 'untrusted', 'blocked')),
  enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0,1)),
  last_sync_at INTEGER,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_repository_registry_kind_enabled
  ON repository_registry (kind, enabled);
CREATE INDEX IF NOT EXISTS idx_repository_registry_trust
  ON repository_registry (trust_status);

CREATE TABLE IF NOT EXISTS repository_package (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  package_id TEXT NOT NULL UNIQUE,
  repo_id TEXT NOT NULL,
  name TEXT NOT NULL,
  version TEXT NOT NULL,
  publisher TEXT,
  install_url TEXT NOT NULL,
  uninstall_command TEXT,
  check_update_provider TEXT NOT NULL,
  check_update_ref TEXT,
  sha256 TEXT,
  risk_level TEXT NOT NULL CHECK (risk_level IN ('low', 'medium', 'high')),
  source_confidence INTEGER NOT NULL DEFAULT 50 CHECK (source_confidence BETWEEN 0 AND 100),
  status TEXT NOT NULL CHECK (status IN ('active', 'deprecated', 'blocked')),
  manifest_yaml TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  FOREIGN KEY (repo_id) REFERENCES repository_registry(repo_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_repository_package_repo_name
  ON repository_package (repo_id, name);
CREATE INDEX IF NOT EXISTS idx_repository_package_status
  ON repository_package (status);
CREATE INDEX IF NOT EXISTS idx_repository_package_name_version
  ON repository_package (name, version DESC);

CREATE TABLE IF NOT EXISTS repository_submission (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  repo_id TEXT NOT NULL,
  package_id TEXT NOT NULL,
  submitted_by TEXT NOT NULL,
  reason TEXT NOT NULL,
  status TEXT NOT NULL CHECK (status IN ('submitted', 'approved', 'rejected', 'published')),
  review_note TEXT,
  created_at INTEGER NOT NULL,
  reviewed_at INTEGER,
  FOREIGN KEY (repo_id) REFERENCES repository_registry(repo_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_repository_submission_repo_created
  ON repository_submission (repo_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_repository_submission_status
  ON repository_submission (status);

-- Helpful read model for quick MVP dashboard summaries.
CREATE VIEW IF NOT EXISTS v_audit_summary AS
SELECT
  (SELECT COUNT(*) FROM update_history) +
  (SELECT COUNT(*) FROM cleanup_history) +
  (SELECT COUNT(*) FROM plugin_execution_history) +
  (SELECT COUNT(*) FROM ai_insight_history) +
  (SELECT COUNT(*) FROM download_task_history) +
  (SELECT COUNT(*) FROM job_queue) AS total_events;

COMMIT;
