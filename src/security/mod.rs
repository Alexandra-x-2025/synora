use std::path::{Component, Path, PathBuf};

#[derive(Debug)]
pub enum SecurityError {
    EmptyCommand,
    ProgramNotAllowlisted(String),
    OperationNotAllowlisted(String),
    PathTraversalDetected(String),
    TargetOutsideAllowlist(String),
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

        Ok(normalized_target)
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
}
