use std::fs;
use std::path::{Component, Path, PathBuf};

#[derive(Debug)]
pub enum SecurityError {
    EmptyCommand,
    ProgramNotAllowlisted(String),
    OperationNotAllowlisted(String),
    PathTraversalDetected(String),
    TargetOutsideAllowlist(String),
    SymbolicLinkDetected(String),
    ConfirmationRequired(String),
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SecurityGuard;

impl SecurityGuard {
    pub fn validate_command(&self, command: &[String]) -> Result<(), SecurityError> {
        if command.is_empty() {
            return Err(SecurityError::EmptyCommand);
        }

        let program = &command[0];
        if program != "winget" {
            return Err(SecurityError::ProgramNotAllowlisted(program.clone()));
        }

        let operation = command
            .iter()
            .skip(1)
            .find(|part| !part.starts_with('-'))
            .cloned()
            .unwrap_or_default();

        if operation != "list" && operation != "upgrade" {
            return Err(SecurityError::OperationNotAllowlisted(operation));
        }

        Ok(())
    }

    pub fn validate_target_path(
        &self,
        target: &Path,
        allowed_root: &Path,
    ) -> Result<PathBuf, SecurityError> {
        for component in target.components() {
            if let Component::ParentDir = component {
                return Err(SecurityError::PathTraversalDetected(
                    target.display().to_string(),
                ));
            }
        }

        let normalized_root = normalize_path(allowed_root);
        let normalized_target = normalize_path(target);

        if !normalized_target.starts_with(&normalized_root) {
            return Err(SecurityError::TargetOutsideAllowlist(
                normalized_target.display().to_string(),
            ));
        }

        // Reject any existing symlink component between root and target parent.
        if let Some(parent) = normalized_target.parent() {
            let mut current = normalized_root.clone();
            for component in parent
                .components()
                .skip(normalized_root.components().count())
            {
                current.push(component.as_os_str());
                if let Ok(meta) = fs::symlink_metadata(&current) {
                    if meta.file_type().is_symlink() {
                        return Err(SecurityError::SymbolicLinkDetected(
                            current.display().to_string(),
                        ));
                    }
                }
            }
        }

        Ok(normalized_target)
    }

    pub fn validate_risk_confirmation(
        &self,
        risk_level: &str,
        confirmed: bool,
    ) -> Result<(), SecurityError> {
        let risk = risk_level.to_ascii_lowercase();
        if (risk == "high" || risk == "critical") && !confirmed {
            return Err(SecurityError::ConfirmationRequired(risk));
        }
        Ok(())
    }
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = normalized.pop();
            }
            Component::Normal(part) => normalized.push(part),
            Component::RootDir | Component::Prefix(_) => {
                normalized.push(component.as_os_str());
            }
        }
    }
    normalized
}

#[cfg(test)]
mod tests {
    use super::{SecurityError, SecurityGuard};
    use std::path::Path;

    #[test]
    fn validate_target_path_rejects_parent_traversal() {
        let guard = SecurityGuard;
        let root = Path::new("/safe/root");
        let target = Path::new("/safe/root/../evil/file.txt");
        let err = guard
            .validate_target_path(target, root)
            .expect_err("traversal should be rejected");
        assert!(matches!(err, SecurityError::PathTraversalDetected(_)));
    }

    #[test]
    fn validate_target_path_rejects_outside_allowlist() {
        let guard = SecurityGuard;
        let root = Path::new("/safe/root");
        let target = Path::new("/safe/other/file.txt");
        let err = guard
            .validate_target_path(target, root)
            .expect_err("outside target should be rejected");
        assert!(matches!(err, SecurityError::TargetOutsideAllowlist(_)));
    }

    #[test]
    fn validate_risk_confirmation_blocks_high_without_confirm() {
        let guard = SecurityGuard;
        let err = guard
            .validate_risk_confirmation("high", false)
            .expect_err("high risk should require confirmation");
        assert!(matches!(err, SecurityError::ConfirmationRequired(_)));
    }

    #[cfg(unix)]
    #[test]
    fn validate_target_path_rejects_symlink_component() {
        use std::os::unix::fs::symlink;
        use std::time::{SystemTime, UNIX_EPOCH};

        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let base = std::env::temp_dir().join(format!("synora-sec-symlink-{unique}"));
        let root = base.join("root");
        let _ = fs::create_dir_all(&root);
        let link = root.join("linked");
        let target_dir = base.join("target");
        let _ = fs::create_dir_all(&target_dir);
        symlink(&target_dir, &link).expect("symlink should be created");

        let guard = SecurityGuard;
        let candidate = link.join("file.txt");
        let err = guard
            .validate_target_path(&candidate, &root)
            .expect_err("symlink component should be rejected");
        assert!(matches!(err, SecurityError::SymbolicLinkDetected(_)));

        let _ = fs::remove_dir_all(base);
    }
}
