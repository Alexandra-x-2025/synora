#[derive(Debug)]
pub enum SecurityError {
    EmptyCommand,
    ProgramNotAllowlisted(String),
    OperationNotAllowlisted(String),
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
}
