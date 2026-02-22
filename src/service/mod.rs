use crate::domain::{CleanupPlan, SoftwareItem, SourceRecommendation, UpdateItem, UpdatePlan};
use crate::integration::{IntegrationError, ParsePath, WingetClient};
use crate::repository::DatabaseRepository;
use crate::security::{SecurityError, SecurityGuard};
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Default, Clone, Copy)]
pub struct SoftwareService {
    winget: WingetClient,
    guard: SecurityGuard,
}

impl SoftwareService {
    pub fn list_software(&self) -> Result<(Vec<SoftwareItem>, ParsePath), IntegrationError> {
        self.winget.list_installed(&self.guard)
    }

    pub fn check_updates(&self) -> Result<(Vec<UpdateItem>, ParsePath), IntegrationError> {
        self.winget.list_upgrades(&self.guard)
    }

    pub fn sync_software_snapshot(
        &self,
        items: &[SoftwareItem],
    ) -> Result<usize, crate::repository::RepositoryError> {
        let repo = DatabaseRepository::default();
        let mut synced = 0usize;

        for item in items {
            repo.upsert_software(&item.name, &item.version, &item.source, "", "unknown")?;
            synced += 1;
        }

        Ok(synced)
    }
}

#[derive(Default, Clone)]
pub struct UpdateService {
    repo: DatabaseRepository,
}

impl UpdateService {
    pub fn plan_apply(
        &self,
        package_id: &str,
        confirmed: bool,
        dry_run: bool,
    ) -> Result<UpdatePlan, String> {
        if package_id.trim().is_empty() {
            return Err("package_id is required".to_string());
        }

        let is_dry_run = !confirmed;
        let requested_mode = if confirmed {
            "confirm"
        } else if dry_run {
            "dry-run"
        } else {
            "dry-run"
        };
        let mode = if confirmed {
            "confirmed-plan"
        } else {
            "plan-only"
        };
        let risk = if confirmed { "low" } else { "medium" };

        Ok(UpdatePlan {
            package_id: package_id.to_string(),
            confirmed,
            dry_run: is_dry_run,
            requested_mode: requested_mode.to_string(),
            mode: mode.to_string(),
            risk: risk.to_string(),
            message: "v0.1 does not execute real updates yet".to_string(),
        })
    }

    pub fn persist_planned_update(
        &self,
        plan: &UpdatePlan,
    ) -> Result<(), crate::repository::RepositoryError> {
        // v0.1 stores package_id as a synthetic software name under `plan` source.
        let software_id =
            self.repo
                .upsert_software(&plan.package_id, "unknown", "plan", "", "unknown")?;
        let status = if plan.confirmed {
            "planned_confirmed"
        } else {
            "planned_dry_run"
        };
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|v| v.as_secs() as i64)
            .unwrap_or(0);

        self.repo
            .log_update(software_id, "unknown", "unknown", timestamp, status)?;

        if plan.confirmed {
            // v0.1 keeps these as audit placeholders; no real system mutation occurs.
            self.repo.add_registry_backup(
                "HKLM",
                &format!("Software\\Synora\\Plan\\{}", plan.package_id),
                "{\"mode\":\"planned_confirmed_placeholder\"}",
                timestamp,
            )?;
            self.repo.add_quarantine_entry(
                software_id,
                &format!("planned://{}", plan.package_id),
                "N/A",
                timestamp,
                "planned_confirmed_safety_placeholder",
            )?;
        }

        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct CleanupService {
    repo: DatabaseRepository,
    guard: SecurityGuard,
}

#[derive(Debug)]
pub enum CleanupError {
    Security(SecurityError),
    Repository(crate::repository::RepositoryError),
}

impl From<crate::repository::RepositoryError> for CleanupError {
    fn from(value: crate::repository::RepositoryError) -> Self {
        CleanupError::Repository(value)
    }
}

impl CleanupService {
    pub fn plan_quarantine(
        &self,
        package_id: &str,
        confirmed: bool,
        dry_run: bool,
    ) -> Result<CleanupPlan, String> {
        if package_id.trim().is_empty() {
            return Err("package_id is required".to_string());
        }

        let requested_mode = if confirmed {
            "confirm"
        } else if dry_run {
            "dry-run"
        } else {
            "dry-run"
        };
        let mode = if confirmed {
            "confirmed-execution"
        } else {
            "plan-only"
        };
        let status = if confirmed {
            "quarantine_confirmed"
        } else {
            "quarantine_planned"
        };
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|v| v.as_secs())
            .unwrap_or(0);
        let op_id = format!(
            "cleanup-{}-{}",
            ts,
            package_id
                .chars()
                .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                .collect::<String>()
        );

        Ok(CleanupPlan {
            operation_id: op_id,
            package_id: package_id.to_string(),
            requested_mode: requested_mode.to_string(),
            mode: mode.to_string(),
            status: status.to_string(),
            mutation_boundary_reached: false,
            rollback_attempted: false,
            rollback_status: "not_needed".to_string(),
            message: if confirmed {
                "phase3 m2: confirmed precheck only, mutation not executed".to_string()
            } else {
                "phase3 m1: dry-run audit only, no real mutation".to_string()
            },
        })
    }

    pub fn execute_cleanup_plan(
        &self,
        mut plan: CleanupPlan,
        simulate_failure: bool,
        simulate_rollback_failure: bool,
    ) -> Result<(CleanupPlan, usize), CleanupError> {
        let quarantine_root = crate::paths::ensure_synora_home()
            .map_err(|err| CleanupError::Repository(crate::repository::RepositoryError::Io(err)))?
            .join("quarantine");
        let target = quarantine_root.join(format!("{}.pending", plan.package_id));
        let _normalized_target = self
            .guard
            .validate_target_path(&target, &quarantine_root)
            .map_err(CleanupError::Security)?;

        let software_id = self.repo.upsert_software(
            &plan.package_id,
            "unknown",
            "cleanup_plan",
            "",
            "unknown",
        )?;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|v| v.as_secs() as i64)
            .unwrap_or(0);
        self.repo
            .log_update(software_id, "unknown", "unknown", timestamp, &plan.status)?;
        let mut written = 1usize;

        if plan.requested_mode == "confirm" {
            // M3 still uses simulated execution, but crosses mutation boundary explicitly.
            self.repo.add_registry_backup(
                "HKLM",
                &format!("Software\\Synora\\Cleanup\\{}", plan.package_id),
                "{\"mode\":\"cleanup_confirm_simulated_execution\"}",
                timestamp,
            )?;
            self.repo.add_quarantine_entry(
                software_id,
                &format!("simulate://{}", plan.package_id),
                "N/A",
                timestamp,
                "cleanup_confirm_simulated_execution",
            )?;
            written += 2;

            plan.mutation_boundary_reached = true;
            if simulate_failure {
                plan.status = "quarantine_failed".to_string();
                plan.message = "phase3 m4: simulated mutation failed".to_string();
                self.repo
                    .log_update(software_id, "unknown", "unknown", timestamp, &plan.status)?;
                written += 1;

                plan.rollback_attempted = true;
                if simulate_rollback_failure {
                    plan.rollback_status = "failed".to_string();
                    self.repo.log_update(
                        software_id,
                        "unknown",
                        "unknown",
                        timestamp,
                        "quarantine_rollback_failed",
                    )?;
                    written += 1;
                } else {
                    plan.rollback_status = "success".to_string();
                    self.repo.log_update(
                        software_id,
                        "unknown",
                        "unknown",
                        timestamp,
                        "quarantine_rollback_success",
                    )?;
                    written += 1;
                }
            } else {
                plan.status = "quarantine_success".to_string();
                plan.message = "phase3 m3: simulated mutation succeeded".to_string();
                self.repo
                    .log_update(software_id, "unknown", "unknown", timestamp, &plan.status)?;
                written += 1;
            }
        }

        Ok((plan, written))
    }
}

#[derive(Default, Clone)]
pub struct SourceSuggestionService {
    repo: DatabaseRepository,
    software: SoftwareService,
}

impl SourceSuggestionService {
    pub fn suggest_from_signals(
        &self,
    ) -> Result<Vec<SourceRecommendation>, crate::repository::RepositoryError> {
        let rows = self.repo.list_software()?;
        let update_names = self
            .software
            .check_updates()
            .map(|(updates, _)| {
                updates
                    .into_iter()
                    .map(|u| u.name.to_ascii_lowercase())
                    .collect::<HashSet<String>>()
            })
            .unwrap_or_default();

        let mut output = Vec::with_capacity(rows.len());
        for row in rows {
            let has_update = update_names.contains(&row.name.to_ascii_lowercase());
            output.push(score_recommendation(&row.name, &row.source, has_update));
        }
        Ok(output)
    }
}

fn score_recommendation(
    name: &str,
    current_source: &str,
    has_update_signal: bool,
) -> SourceRecommendation {
    let n = name.to_ascii_lowercase();
    let src = current_source.to_ascii_lowercase();

    if src.contains("winget") {
        let mut reasons = vec![
            "already_managed_by_winget".to_string(),
            "highest_trust_for_windows_cli".to_string(),
        ];
        if has_update_signal {
            reasons.push("update_detected_via_winget".to_string());
        }
        return SourceRecommendation {
            software_name: name.to_string(),
            current_source: current_source.to_string(),
            recommended_source: "winget".to_string(),
            score: 95,
            reasons,
        };
    }

    if n.contains("python") || n.contains("git") || n.contains("node") {
        let mut score = 86;
        let mut reasons = vec![
            "common_dev_tool".to_string(),
            "winget_metadata_expected".to_string(),
        ];
        if has_update_signal {
            score = 91;
            reasons.push("update_detected_prefer_managed_channel".to_string());
        }
        return SourceRecommendation {
            software_name: name.to_string(),
            current_source: current_source.to_string(),
            recommended_source: "winget".to_string(),
            score,
            reasons,
        };
    }

    if src.is_empty() || src == "unknown" {
        let mut score = 72;
        let mut reasons = vec![
            "source_missing".to_string(),
            "prefer_safe_default".to_string(),
        ];
        if has_update_signal {
            score = 78;
            reasons.push("update_detected_requires_traceable_source".to_string());
        }
        return SourceRecommendation {
            software_name: name.to_string(),
            current_source: current_source.to_string(),
            recommended_source: "winget".to_string(),
            score,
            reasons,
        };
    }

    let mut score = 64;
    let mut reasons = vec!["normalize_source_management".to_string()];
    if has_update_signal {
        score = 70;
        reasons.push("update_detected_improve_upgrade_stability".to_string());
    }
    SourceRecommendation {
        software_name: name.to_string(),
        current_source: current_source.to_string(),
        recommended_source: "winget".to_string(),
        score,
        reasons,
    }
}

#[cfg(test)]
mod tests {
    use super::{score_recommendation, CleanupService, UpdateService};
    use crate::repository::DatabaseRepository;
    use rusqlite::Connection;
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
    fn recommendation_prefers_existing_winget() {
        let r = score_recommendation("Git", "winget", false);
        assert_eq!(r.recommended_source, "winget");
        assert!(r.score >= 90);
    }

    #[test]
    fn recommendation_prefers_winget_for_common_dev_tools() {
        let r = score_recommendation("Python", "manual", false);
        assert_eq!(r.recommended_source, "winget");
        assert!(r.score >= 80);
    }

    #[test]
    fn recommendation_boosts_score_when_update_signal_present() {
        let baseline = score_recommendation("Python", "manual", false);
        let boosted = score_recommendation("Python", "manual", true);
        assert!(boosted.score > baseline.score);
        assert!(boosted
            .reasons
            .iter()
            .any(|r| r.contains("update_detected")));
    }

    #[test]
    fn persist_planned_update_writes_update_history() {
        let root = unique_dir("synora-update-plan-test");
        let db_path = root.join("db").join("synora.db");
        let service = UpdateService {
            repo: DatabaseRepository {
                db_path: Some(db_path.clone()),
            },
        };

        let plan = service
            .plan_apply("Git.Git", true, false)
            .expect("plan_apply should succeed");
        service
            .persist_planned_update(&plan)
            .expect("persist_planned_update should succeed");

        let conn = Connection::open(db_path).expect("db should open");
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM update_history WHERE status = 'planned_confirmed'",
                [],
                |row| row.get(0),
            )
            .expect("count query should succeed");
        assert_eq!(count, 1);

        let backup_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM registry_backup", [], |row| row.get(0))
            .expect("backup count query should succeed");
        assert_eq!(backup_count, 1);

        let quarantine_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM quarantine", [], |row| row.get(0))
            .expect("quarantine count query should succeed");
        assert_eq!(quarantine_count, 1);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn persist_cleanup_dry_run_writes_quarantine_planned_status() {
        let root = unique_dir("synora-cleanup-plan-test");
        let db_path = root.join("db").join("synora.db");
        let service = CleanupService {
            repo: DatabaseRepository {
                db_path: Some(db_path.clone()),
            },
            guard: crate::security::SecurityGuard,
        };

        let plan = service
            .plan_quarantine("Git.Git", false, true)
            .expect("plan_quarantine should succeed");
        let (result, written) = service
            .execute_cleanup_plan(plan, false, false)
            .expect("execute_cleanup_plan should succeed");
        assert_eq!(written, 1);
        assert!(!result.mutation_boundary_reached);
        assert_eq!(result.status, "quarantine_planned");

        let conn = Connection::open(db_path).expect("db should open");
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM update_history WHERE status = 'quarantine_planned'",
                [],
                |row| row.get(0),
            )
            .expect("count query should succeed");
        assert_eq!(count, 1);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn persist_cleanup_confirm_writes_precheck_artifacts() {
        let root = unique_dir("synora-cleanup-confirm-test");
        let db_path = root.join("db").join("synora.db");
        let service = CleanupService {
            repo: DatabaseRepository {
                db_path: Some(db_path.clone()),
            },
            guard: crate::security::SecurityGuard,
        };

        let plan = service
            .plan_quarantine("Git.Git", true, false)
            .expect("plan_quarantine should succeed");
        let (result, written) = service
            .execute_cleanup_plan(plan, false, false)
            .expect("execute_cleanup_plan should succeed");
        assert_eq!(written, 4);
        assert!(result.mutation_boundary_reached);
        assert_eq!(result.status, "quarantine_success");

        let conn = Connection::open(db_path).expect("db should open");
        let confirmed_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM update_history WHERE status = 'quarantine_confirmed'",
                [],
                |row| row.get(0),
            )
            .expect("status count query should succeed");
        assert_eq!(confirmed_count, 1);

        let success_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM update_history WHERE status = 'quarantine_success'",
                [],
                |row| row.get(0),
            )
            .expect("success count query should succeed");
        assert_eq!(success_count, 1);

        let backup_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM registry_backup", [], |row| row.get(0))
            .expect("backup count query should succeed");
        assert_eq!(backup_count, 1);

        let quarantine_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM quarantine", [], |row| row.get(0))
            .expect("quarantine count query should succeed");
        assert_eq!(quarantine_count, 1);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn persist_cleanup_confirm_simulated_failure_rolls_back_successfully() {
        let root = unique_dir("synora-cleanup-failure-test");
        let db_path = root.join("db").join("synora.db");
        let service = CleanupService {
            repo: DatabaseRepository {
                db_path: Some(db_path.clone()),
            },
            guard: crate::security::SecurityGuard,
        };

        let plan = service
            .plan_quarantine("Git.Git", true, false)
            .expect("plan_quarantine should succeed");
        let (result, written) = service
            .execute_cleanup_plan(plan, true, false)
            .expect("execute_cleanup_plan should succeed");
        assert_eq!(written, 5);
        assert_eq!(result.status, "quarantine_failed");
        assert!(result.rollback_attempted);
        assert_eq!(result.rollback_status, "success");

        let conn = Connection::open(db_path).expect("db should open");
        let rollback_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM update_history WHERE status = 'quarantine_rollback_success'",
                [],
                |row| row.get(0),
            )
            .expect("rollback count query should succeed");
        assert_eq!(rollback_count, 1);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn persist_cleanup_confirm_simulated_rollback_failure_is_logged() {
        let root = unique_dir("synora-cleanup-rollback-failure-test");
        let db_path = root.join("db").join("synora.db");
        let service = CleanupService {
            repo: DatabaseRepository {
                db_path: Some(db_path.clone()),
            },
            guard: crate::security::SecurityGuard,
        };

        let plan = service
            .plan_quarantine("Git.Git", true, false)
            .expect("plan_quarantine should succeed");
        let (result, written) = service
            .execute_cleanup_plan(plan, true, true)
            .expect("execute_cleanup_plan should succeed");
        assert_eq!(written, 5);
        assert_eq!(result.status, "quarantine_failed");
        assert!(result.rollback_attempted);
        assert_eq!(result.rollback_status, "failed");

        let conn = Connection::open(db_path).expect("db should open");
        let rollback_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM update_history WHERE status = 'quarantine_rollback_failed'",
                [],
                |row| row.get(0),
            )
            .expect("rollback failed count query should succeed");
        assert_eq!(rollback_count, 1);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn cleanup_execute_rejects_traversal_target() {
        let root = unique_dir("synora-cleanup-security-test");
        let db_path = root.join("db").join("synora.db");
        let service = CleanupService {
            repo: DatabaseRepository {
                db_path: Some(db_path),
            },
            guard: crate::security::SecurityGuard,
        };

        let plan = service
            .plan_quarantine("../evil", true, false)
            .expect("plan should build");
        let err = service
            .execute_cleanup_plan(plan, false, false)
            .expect_err("should reject traversal");

        assert!(matches!(err, super::CleanupError::Security(_)));
    }
}
