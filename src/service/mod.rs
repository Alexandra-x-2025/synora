use crate::domain::{SoftwareItem, UpdateItem, UpdatePlan};
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
