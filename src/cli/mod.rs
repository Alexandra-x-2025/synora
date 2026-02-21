use crate::integration::IntegrationError;
use crate::repository::ConfigRepository;
use crate::security::SecurityError;
use crate::service::{SoftwareService, UpdateService};

pub const EXIT_OK: i32 = 0;
pub const EXIT_USAGE: i32 = 2;
pub const EXIT_SECURITY: i32 = 3;
pub const EXIT_INTEGRATION: i32 = 4;
pub const EXIT_INTERNAL: i32 = 10;

pub fn run() -> i32 {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_help();
        return EXIT_USAGE;
    }

    match dispatch(&args) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("Unexpected error: {err}");
            EXIT_INTERNAL
        }
    }
}

fn dispatch(args: &[String]) -> Result<i32, String> {
    match args[0].as_str() {
        "software" => handle_software(&args[1..]),
        "update" => handle_update(&args[1..]),
        "config" => handle_config(&args[1..]),
        "-h" | "--help" => {
            print_help();
            Ok(EXIT_OK)
        }
        _ => {
            print_help();
            Ok(EXIT_USAGE)
        }
    }
}

fn handle_software(args: &[String]) -> Result<i32, String> {
    if args.is_empty() || args[0] != "list" {
        print_software_help();
        return Ok(EXIT_USAGE);
    }

    let as_json = args.iter().any(|v| v == "--json");
    let service = SoftwareService::default();
    match service.list_software() {
        Ok(items) => {
            if as_json {
                print_software_json(&items);
            } else {
                print_software_table(&items);
            }
            Ok(EXIT_OK)
        }
        Err(err) => Ok(map_integration_error(err)),
    }
}

fn handle_update(args: &[String]) -> Result<i32, String> {
    if args.is_empty() {
        print_update_help();
        return Ok(EXIT_USAGE);
    }

    match args[0].as_str() {
        "check" => {
            let as_json = args.iter().any(|v| v == "--json");
            let service = SoftwareService::default();
            match service.check_updates() {
                Ok(items) => {
                    if as_json {
                        print_software_json(&items);
                    } else {
                        print_software_table(&items);
                    }
                    Ok(EXIT_OK)
                }
                Err(err) => Ok(map_integration_error(err)),
            }
        }
        "apply" => {
            let mut package_id: Option<String> = None;
            let mut dry_run = false;
            let mut confirmed = false;
            let mut as_json = false;

            let mut idx = 1usize;
            while idx < args.len() {
                match args[idx].as_str() {
                    "--id" => {
                        if idx + 1 >= args.len() {
                            eprintln!("Validation error: --id requires a value");
                            return Ok(EXIT_USAGE);
                        }
                        package_id = Some(args[idx + 1].clone());
                        idx += 2;
                    }
                    "--dry-run" => {
                        if confirmed {
                            eprintln!("Validation error: --dry-run cannot be used with --confirm/--yes");
                            return Ok(EXIT_USAGE);
                        }
                        dry_run = true;
                        idx += 1;
                    }
                    "--confirm" | "--yes" => {
                        if dry_run {
                            eprintln!("Validation error: --confirm/--yes cannot be used with --dry-run");
                            return Ok(EXIT_USAGE);
                        }
                        confirmed = true;
                        idx += 1;
                    }
                    "--json" => {
                        as_json = true;
                        idx += 1;
                    }
                    _ => {
                        eprintln!("Validation error: unknown option '{}'", args[idx]);
                        return Ok(EXIT_USAGE);
                    }
                }
            }

            let Some(package_id) = package_id else {
                eprintln!("Validation error: --id is required");
                return Ok(EXIT_USAGE);
            };

            let updater = UpdateService;
            let plan = match updater.plan_apply(&package_id, confirmed, dry_run) {
                Ok(plan) => plan,
                Err(msg) => {
                    eprintln!("Validation error: {msg}");
                    return Ok(EXIT_USAGE);
                }
            };

            if as_json {
                println!(
                    "{{\n  \"package_id\": \"{}\",\n  \"risk\": \"{}\",\n  \"confirmed\": {},\n  \"dry_run\": {},\n  \"requested_mode\": \"{}\",\n  \"mode\": \"{}\",\n  \"message\": \"{}\"\n}}",
                    escape_json(&plan.package_id),
                    escape_json(&plan.risk),
                    plan.confirmed,
                    plan.dry_run,
                    escape_json(&plan.requested_mode),
                    escape_json(&plan.mode),
                    escape_json(&plan.message),
                );
            } else {
                println!("Package: {}", plan.package_id);
                println!("Risk: {}", plan.risk);
                println!("Requested Mode: {}", plan.requested_mode);
                println!("Mode: {}", plan.mode);
                println!("Note: {}", plan.message);
            }
            Ok(EXIT_OK)
        }
        _ => {
            print_update_help();
            Ok(EXIT_USAGE)
        }
    }
}

fn handle_config(args: &[String]) -> Result<i32, String> {
    if args.len() != 1 || args[0] != "init" {
        print_config_help();
        return Ok(EXIT_USAGE);
    }

    let repo = ConfigRepository::default();
    match repo.init_default() {
        Ok(path) => {
            println!("Config initialized: {}", path.display());
            Ok(EXIT_OK)
        }
        Err(err) => {
            eprintln!("Integration failure: {err}");
            Ok(EXIT_INTEGRATION)
        }
    }
}

fn map_integration_error(err: IntegrationError) -> i32 {
    match err {
        IntegrationError::Security(se) => {
            eprintln!("Security blocked: {}", format_security_error(se));
            EXIT_SECURITY
        }
        IntegrationError::CommandFailed(msg) => {
            eprintln!("Integration failure: {msg}");
            EXIT_INTEGRATION
        }
    }
}

fn format_security_error(err: SecurityError) -> String {
    match err {
        SecurityError::EmptyCommand => "empty command is not allowed".to_string(),
        SecurityError::ProgramNotAllowlisted(p) => format!("program '{p}' is not allowlisted"),
        SecurityError::OperationNotAllowlisted(op) => {
            format!("operation '{op}' is not allowlisted for winget")
        }
    }
}

fn print_software_json(items: &[crate::domain::SoftwareItem]) {
    println!("[");
    for (idx, item) in items.iter().enumerate() {
        let comma = if idx + 1 == items.len() { "" } else { "," };
        println!(
            "  {{\"name\":\"{}\",\"package_id\":\"{}\",\"version\":\"{}\",\"source\":\"{}\"}}{}",
            escape_json(&item.name),
            escape_json(&item.package_id),
            escape_json(&item.version),
            escape_json(&item.source),
            comma
        );
    }
    println!("]");
}

fn print_software_table(items: &[crate::domain::SoftwareItem]) {
    if items.is_empty() {
        println!("No entries found.");
        return;
    }

    println!("name  package_id  version  source");
    println!("----  ----------  -------  ------");
    for item in items {
        println!(
            "{}  {}  {}  {}",
            item.name, item.package_id, item.version, item.source
        );
    }
}

fn print_help() {
    println!("Synora CLI v0.1");
    println!("Usage: synora <software|update|config> <subcommand> [options]");
    println!("Try: synora software list --json");
}

fn print_software_help() {
    println!("Usage: synora software list [--json]");
}

fn print_update_help() {
    println!("Usage: synora update check [--json]");
    println!("   or: synora update apply --id <package_id> [--dry-run|--confirm|--yes] [--json]");
}

fn print_config_help() {
    println!("Usage: synora config init");
}

fn escape_json(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::{dispatch, map_integration_error, EXIT_INTEGRATION, EXIT_OK, EXIT_SECURITY, EXIT_USAGE};
    use crate::integration::IntegrationError;
    use crate::security::SecurityError;

    fn args(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|v| (*v).to_string()).collect()
    }

    #[test]
    fn update_apply_missing_id_returns_usage() {
        let code = dispatch(&args(&["update", "apply"])).expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn update_apply_conflicting_flags_returns_usage() {
        let code = dispatch(&args(&["update", "apply", "--id", "Git.Git", "--dry-run", "--confirm"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn update_apply_yes_alias_returns_ok() {
        let code = dispatch(&args(&["update", "apply", "--id", "Git.Git", "--yes", "--json"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_OK);
    }

    #[test]
    fn map_integration_failure_to_exit_4() {
        let code = map_integration_error(IntegrationError::CommandFailed("boom".to_string()));
        assert_eq!(code, EXIT_INTEGRATION);
    }

    #[test]
    fn map_security_failure_to_exit_3() {
        let code = map_integration_error(IntegrationError::Security(SecurityError::ProgramNotAllowlisted(
            "powershell".to_string(),
        )));
        assert_eq!(code, EXIT_SECURITY);
    }
}
