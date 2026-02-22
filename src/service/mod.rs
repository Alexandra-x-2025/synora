use crate::domain::{SoftwareItem, SourceRecommendation, UpdateItem, UpdatePlan};
use crate::integration::{IntegrationError, ParsePath, WingetClient};
use crate::repository::DatabaseRepository;
use crate::security::SecurityGuard;

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
            repo.upsert_software(
                &item.name,
                &item.version,
                &item.source,
                "",
                "unknown",
            )?;
            synced += 1;
        }

        Ok(synced)
    }
}

pub struct UpdateService;

impl UpdateService {
    pub fn plan_apply(&self, package_id: &str, confirmed: bool, dry_run: bool) -> Result<UpdatePlan, String> {
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
        let mode = if confirmed { "confirmed-plan" } else { "plan-only" };
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
}

#[derive(Default, Clone)]
pub struct SourceSuggestionService {
    repo: DatabaseRepository,
}

impl SourceSuggestionService {
    pub fn suggest_from_repository(&self) -> Result<Vec<SourceRecommendation>, crate::repository::RepositoryError> {
        let rows = self.repo.list_software()?;
        let mut output = Vec::with_capacity(rows.len());
        for row in rows {
            output.push(score_recommendation(&row.name, &row.source));
        }
        Ok(output)
    }
}

fn score_recommendation(name: &str, current_source: &str) -> SourceRecommendation {
    let n = name.to_ascii_lowercase();
    let src = current_source.to_ascii_lowercase();

    if src.contains("winget") {
        return SourceRecommendation {
            software_name: name.to_string(),
            current_source: current_source.to_string(),
            recommended_source: "winget".to_string(),
            score: 95,
            reasons: vec!["already_managed_by_winget".to_string(), "highest_trust_for_windows_cli".to_string()],
        };
    }

    if n.contains("python") || n.contains("git") || n.contains("node") {
        return SourceRecommendation {
            software_name: name.to_string(),
            current_source: current_source.to_string(),
            recommended_source: "winget".to_string(),
            score: 86,
            reasons: vec!["common_dev_tool".to_string(), "winget_metadata_expected".to_string()],
        };
    }

    if src.is_empty() || src == "unknown" {
        return SourceRecommendation {
            software_name: name.to_string(),
            current_source: current_source.to_string(),
            recommended_source: "winget".to_string(),
            score: 72,
            reasons: vec!["source_missing".to_string(), "prefer_safe_default".to_string()],
        };
    }

    SourceRecommendation {
        software_name: name.to_string(),
        current_source: current_source.to_string(),
        recommended_source: "winget".to_string(),
        score: 64,
        reasons: vec!["normalize_source_management".to_string()],
    }
}

#[cfg(test)]
mod tests {
    use super::score_recommendation;

    #[test]
    fn recommendation_prefers_existing_winget() {
        let r = score_recommendation("Git", "winget");
        assert_eq!(r.recommended_source, "winget");
        assert!(r.score >= 90);
    }

    #[test]
    fn recommendation_prefers_winget_for_common_dev_tools() {
        let r = score_recommendation("Python", "manual");
        assert_eq!(r.recommended_source, "winget");
        assert!(r.score >= 80);
    }
}
