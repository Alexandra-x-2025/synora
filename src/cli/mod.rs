use crate::integration::{IntegrationError, ParsePath};
use crate::logging::log_event;
use crate::repository::{ConfigRepository, DatabaseRepository};
use crate::security::SecurityError;
use crate::service::{CleanupService, SoftwareService, SourceSuggestionService, UpdateService};

pub const EXIT_OK: i32 = 0;
pub const EXIT_USAGE: i32 = 2;
pub const EXIT_SECURITY: i32 = 3;
pub const EXIT_INTEGRATION: i32 = 4;
pub const EXIT_INTERNAL: i32 = 10;

pub fn run() -> i32 {
    if let Err(err) = log_event("INFO", "cli_start") {
        eprintln!("Logging unavailable: {err}");
    }

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
        "cleanup" => handle_cleanup(&args[1..]),
        "config" => handle_config(&args[1..]),
        "source" => handle_source(&args[1..]),
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
    let verbose = args.iter().any(|v| v == "--verbose");
    let service = SoftwareService::default();
    match service.list_software() {
        Ok((items, parse_path)) => {
            let synced = match service.sync_software_snapshot(&items) {
                Ok(count) => count,
                Err(err) => {
                    eprintln!("Integration failure: failed to persist software snapshot: {err}");
                    return Ok(EXIT_INTEGRATION);
                }
            };

            if as_json {
                print_software_json(&items);
            } else {
                print_software_table(&items);
                println!("db_sync_count: {synced}");
                if verbose {
                    println!("parse_path: {}", format_parse_path(parse_path));
                }
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
            let verbose = args.iter().any(|v| v == "--verbose");
            let service = SoftwareService::default();
            match service.check_updates() {
                Ok((items, parse_path)) => {
                    if as_json {
                        print_update_json(&items);
                    } else {
                        print_update_table(&items);
                        println!("has_updates: {}", !items.is_empty());
                        if verbose {
                            println!("parse_path: {}", format_parse_path(parse_path));
                        }
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
                            eprintln!(
                                "Validation error: --dry-run cannot be used with --confirm/--yes"
                            );
                            return Ok(EXIT_USAGE);
                        }
                        dry_run = true;
                        idx += 1;
                    }
                    "--confirm" | "--yes" => {
                        if dry_run {
                            eprintln!(
                                "Validation error: --confirm/--yes cannot be used with --dry-run"
                            );
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

            let updater = UpdateService::default();
            let plan = match updater.plan_apply(&package_id, confirmed, dry_run) {
                Ok(plan) => plan,
                Err(msg) => {
                    eprintln!("Validation error: {msg}");
                    return Ok(EXIT_USAGE);
                }
            };
            if let Err(err) = updater.persist_planned_update(&plan) {
                eprintln!("Integration failure: failed to persist update plan: {err}");
                return Ok(EXIT_INTEGRATION);
            }

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

fn handle_cleanup(args: &[String]) -> Result<i32, String> {
    if args.is_empty() || args[0] != "quarantine" {
        print_cleanup_help();
        return Ok(EXIT_USAGE);
    }

    let mut package_id: Option<String> = None;
    let mut dry_run = false;
    let mut confirmed = false;
    let mut as_json = false;
    let mut verbose = false;

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
                    eprintln!("Validation error: --dry-run cannot be used with --confirm");
                    return Ok(EXIT_USAGE);
                }
                dry_run = true;
                idx += 1;
            }
            "--confirm" => {
                if dry_run {
                    eprintln!("Validation error: --confirm cannot be used with --dry-run");
                    return Ok(EXIT_USAGE);
                }
                confirmed = true;
                idx += 1;
            }
            "--json" => {
                as_json = true;
                idx += 1;
            }
            "--verbose" => {
                verbose = true;
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

    let service = CleanupService::default();
    let plan = match service.plan_quarantine(&package_id, confirmed, dry_run) {
        Ok(plan) => plan,
        Err(msg) => {
            eprintln!("Validation error: {msg}");
            return Ok(EXIT_USAGE);
        }
    };
    if let Err(err) = service.persist_cleanup_plan(&plan) {
        eprintln!("Integration failure: failed to persist cleanup plan: {err}");
        return Ok(EXIT_INTEGRATION);
    }

    if as_json {
        println!(
            "{{\n  \"operation_id\": \"{}\",\n  \"package_id\": \"{}\",\n  \"requested_mode\": \"{}\",\n  \"mode\": \"{}\",\n  \"status\": \"{}\",\n  \"rollback_attempted\": {},\n  \"rollback_status\": \"{}\",\n  \"message\": \"{}\"\n}}",
            escape_json(&plan.operation_id),
            escape_json(&plan.package_id),
            escape_json(&plan.requested_mode),
            escape_json(&plan.mode),
            escape_json(&plan.status),
            plan.rollback_attempted,
            escape_json(&plan.rollback_status),
            escape_json(&plan.message),
        );
    } else {
        println!("operation_id: {}", plan.operation_id);
        println!("package_id: {}", plan.package_id);
        println!("status: {}", plan.status);
        println!("rollback_status: {}", plan.rollback_status);
        println!("message: {}", plan.message);
        if verbose {
            println!("mutation_boundary_reached: false");
            println!("audit_rows_written: 1");
        }
    }

    Ok(EXIT_OK)
}

fn handle_config(args: &[String]) -> Result<i32, String> {
    if args.is_empty() {
        print_config_help();
        return Ok(EXIT_USAGE);
    }

    match args[0].as_str() {
        "init" => {
            if args.len() != 1 {
                print_config_help();
                return Ok(EXIT_USAGE);
            }

            let repo = ConfigRepository::default();
            match repo.init_default() {
                Ok(path) => {
                    let db_repo = DatabaseRepository::default();
                    match db_repo.init_schema() {
                        Ok(db_path) => {
                            println!("Config initialized: {}", path.display());
                            println!("Database initialized: {}", db_path.display());
                            Ok(EXIT_OK)
                        }
                        Err(err) => {
                            eprintln!("Integration failure: {err}");
                            Ok(EXIT_INTEGRATION)
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Integration failure: {err}");
                    Ok(EXIT_INTEGRATION)
                }
            }
        }
        "db-list" => {
            let as_json = args.iter().any(|v| v == "--json");
            if args.len() > 2 || (args.len() == 2 && !as_json) {
                print_config_help();
                return Ok(EXIT_USAGE);
            }

            let db_repo = DatabaseRepository::default();
            match db_repo.list_software() {
                Ok(rows) => {
                    if as_json {
                        print_db_software_json(&rows);
                    } else {
                        print_db_software_table(&rows);
                    }
                    Ok(EXIT_OK)
                }
                Err(err) => {
                    eprintln!("Integration failure: {err}");
                    Ok(EXIT_INTEGRATION)
                }
            }
        }
        "history-list" => {
            let as_json = args.iter().any(|v| v == "--json");
            if args.len() > 2 || (args.len() == 2 && !as_json) {
                print_config_help();
                return Ok(EXIT_USAGE);
            }

            let db_repo = DatabaseRepository::default();
            match db_repo.list_update_history() {
                Ok(rows) => {
                    if as_json {
                        print_db_update_history_json(&rows);
                    } else {
                        print_db_update_history_table(&rows);
                    }
                    Ok(EXIT_OK)
                }
                Err(err) => {
                    eprintln!("Integration failure: {err}");
                    Ok(EXIT_INTEGRATION)
                }
            }
        }
        "audit-summary" => {
            let as_json = args.iter().any(|v| v == "--json");
            if args.len() > 2 || (args.len() == 2 && !as_json) {
                print_config_help();
                return Ok(EXIT_USAGE);
            }

            let db_repo = DatabaseRepository::default();
            match db_repo.get_update_audit_summary() {
                Ok(summary) => {
                    if as_json {
                        print_db_update_audit_summary_json(&summary);
                    } else {
                        print_db_update_audit_summary_table(&summary);
                    }
                    Ok(EXIT_OK)
                }
                Err(err) => {
                    eprintln!("Integration failure: {err}");
                    Ok(EXIT_INTEGRATION)
                }
            }
        }
        _ => {
            print_config_help();
            Ok(EXIT_USAGE)
        }
    }
}

fn handle_source(args: &[String]) -> Result<i32, String> {
    if args.is_empty() || args[0] != "suggest" {
        print_source_help();
        return Ok(EXIT_USAGE);
    }

    let mut as_json = false;
    let mut verbose = false;
    for opt in args.iter().skip(1) {
        match opt.as_str() {
            "--json" => as_json = true,
            "--verbose" => verbose = true,
            _ => {
                print_source_help();
                return Ok(EXIT_USAGE);
            }
        }
    }

    let service = SourceSuggestionService::default();
    match service.suggest_from_signals() {
        Ok(items) => {
            if as_json {
                print_source_suggestions_json(&items);
            } else {
                print_source_suggestions_table(&items);
                if verbose {
                    print_source_suggestions_verbose_summary(&items);
                }
            }
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

fn format_parse_path(path: ParsePath) -> &'static str {
    match path {
        ParsePath::Json => "json",
        ParsePath::TextFallback => "text_fallback",
        ParsePath::UnsupportedPlatform => "unsupported_platform",
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

fn print_update_json(items: &[crate::domain::UpdateItem]) {
    println!("[");
    for (idx, item) in items.iter().enumerate() {
        let comma = if idx + 1 == items.len() { "" } else { "," };
        println!(
            "  {{\"name\":\"{}\",\"package_id\":\"{}\",\"installed_version\":\"{}\",\"available_version\":\"{}\",\"source\":\"{}\"}}{}",
            escape_json(&item.name),
            escape_json(&item.package_id),
            escape_json(&item.installed_version),
            escape_json(&item.available_version),
            escape_json(&item.source),
            comma
        );
    }
    println!("]");
}

fn print_update_table(items: &[crate::domain::UpdateItem]) {
    if items.is_empty() {
        println!("No entries found.");
        return;
    }

    println!("name  package_id  installed_version  available_version  source");
    println!("----  ----------  -----------------  -----------------  ------");
    for item in items {
        println!(
            "{}  {}  {}  {}  {}",
            item.name, item.package_id, item.installed_version, item.available_version, item.source
        );
    }
}

fn print_help() {
    println!("Synora CLI v0.1");
    println!("Usage: synora <software|update|cleanup|config|source> <subcommand> [options]");
    println!("Try: synora software list --json");
}

fn print_software_help() {
    println!("Usage: synora software list [--json] [--verbose]");
}

fn print_update_help() {
    println!("Usage: synora update check [--json] [--verbose]");
    println!("   or: synora update apply --id <package_id> [--dry-run|--confirm|--yes] [--json]");
}

fn print_cleanup_help() {
    println!(
        "Usage: synora cleanup quarantine --id <package_id> [--dry-run|--confirm] [--json] [--verbose]"
    );
}

fn print_config_help() {
    println!("Usage: synora config init");
    println!("   or: synora config db-list [--json]");
    println!("   or: synora config history-list [--json]");
    println!("   or: synora config audit-summary [--json]");
}

fn print_source_help() {
    println!("Usage: synora source suggest [--json] [--verbose]");
}

fn escape_json(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

fn print_db_software_json(items: &[crate::repository::SoftwareRow]) {
    println!("[");
    for (idx, item) in items.iter().enumerate() {
        let comma = if idx + 1 == items.len() { "" } else { "," };
        println!(
            "  {{\"id\":{},\"name\":\"{}\",\"version\":\"{}\",\"source\":\"{}\",\"install_path\":\"{}\",\"risk_level\":\"{}\"}}{}",
            item.id,
            escape_json(&item.name),
            escape_json(&item.version),
            escape_json(&item.source),
            escape_json(&item.install_path),
            escape_json(&item.risk_level),
            comma
        );
    }
    println!("]");
}

fn print_db_software_table(items: &[crate::repository::SoftwareRow]) {
    if items.is_empty() {
        println!("No database software entries.");
        return;
    }

    println!("id  name  version  source  install_path  risk_level");
    println!("--  ----  -------  ------  ------------  ----------");
    for item in items {
        println!(
            "{}  {}  {}  {}  {}  {}",
            item.id, item.name, item.version, item.source, item.install_path, item.risk_level
        );
    }
}

fn print_db_update_history_json(items: &[crate::repository::UpdateHistoryRow]) {
    println!("[");
    for (idx, item) in items.iter().enumerate() {
        let comma = if idx + 1 == items.len() { "" } else { "," };
        println!(
            "  {{\"id\":{},\"software_id\":{},\"old_version\":\"{}\",\"new_version\":\"{}\",\"timestamp\":{},\"status\":\"{}\"}}{}",
            item.id,
            item.software_id,
            escape_json(&item.old_version),
            escape_json(&item.new_version),
            item.timestamp,
            escape_json(&item.status),
            comma
        );
    }
    println!("]");
}

fn print_db_update_history_table(items: &[crate::repository::UpdateHistoryRow]) {
    if items.is_empty() {
        println!("No update history entries.");
        return;
    }

    println!("id  software_id  old_version  new_version  timestamp  status");
    println!("--  -----------  -----------  -----------  ---------  ------");
    for item in items {
        println!(
            "{}  {}  {}  {}  {}  {}",
            item.id,
            item.software_id,
            item.old_version,
            item.new_version,
            item.timestamp,
            item.status
        );
    }
}

fn print_db_update_audit_summary_json(summary: &crate::repository::UpdateAuditSummary) {
    let latest = summary
        .latest_timestamp
        .map(|v| v.to_string())
        .unwrap_or_else(|| "null".to_string());
    println!(
        "{{\"total\":{},\"planned_confirmed\":{},\"planned_dry_run\":{},\"latest_timestamp\":{}}}",
        summary.total, summary.planned_confirmed, summary.planned_dry_run, latest
    );
}

fn print_db_update_audit_summary_table(summary: &crate::repository::UpdateAuditSummary) {
    let latest = summary
        .latest_timestamp
        .map(|v| v.to_string())
        .unwrap_or_else(|| "none".to_string());
    println!("total: {}", summary.total);
    println!("planned_confirmed: {}", summary.planned_confirmed);
    println!("planned_dry_run: {}", summary.planned_dry_run);
    println!("latest_timestamp: {latest}");
}

fn print_source_suggestions_json(items: &[crate::domain::SourceRecommendation]) {
    println!("[");
    for (idx, item) in items.iter().enumerate() {
        let comma = if idx + 1 == items.len() { "" } else { "," };
        let reasons = item
            .reasons
            .iter()
            .map(|r| format!("\"{}\"", escape_json(r)))
            .collect::<Vec<String>>()
            .join(", ");
        println!(
            "  {{\"software_name\":\"{}\",\"current_source\":\"{}\",\"recommended_source\":\"{}\",\"score\":{},\"reasons\":[{}]}}{}",
            escape_json(&item.software_name),
            escape_json(&item.current_source),
            escape_json(&item.recommended_source),
            item.score,
            reasons,
            comma
        );
    }
    println!("]");
}

fn print_source_suggestions_table(items: &[crate::domain::SourceRecommendation]) {
    if items.is_empty() {
        println!("No source recommendations.");
        return;
    }

    println!("software_name  current_source  recommended_source  score  reasons");
    println!("-------------  --------------  ------------------  -----  -------");
    for item in items {
        println!(
            "{}  {}  {}  {}  {}",
            item.software_name,
            item.current_source,
            item.recommended_source,
            item.score,
            item.reasons.join("|")
        );
    }
}

fn print_source_suggestions_verbose_summary(items: &[crate::domain::SourceRecommendation]) {
    let update_signal_hits = items
        .iter()
        .filter(|item| item.reasons.iter().any(|r| r.contains("update_detected")))
        .count();
    let high_confidence = items.iter().filter(|item| item.score >= 90).count();

    println!("recommendation_count: {}", items.len());
    println!("update_signal_hits: {update_signal_hits}");
    println!("high_confidence_count: {high_confidence}");
    println!("signal_mode: db_plus_update_check_best_effort");
}

#[cfg(test)]
mod tests {
    use super::{
        dispatch, map_integration_error, EXIT_INTEGRATION, EXIT_OK, EXIT_SECURITY, EXIT_USAGE,
    };
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
        let code = dispatch(&args(&[
            "update",
            "apply",
            "--id",
            "Git.Git",
            "--dry-run",
            "--confirm",
        ]))
        .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn update_apply_yes_alias_returns_ok() {
        let code = dispatch(&args(&[
            "update", "apply", "--id", "Git.Git", "--yes", "--json",
        ]))
        .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_OK);
    }

    #[test]
    fn software_list_verbose_returns_ok() {
        let code = dispatch(&args(&["software", "list", "--verbose"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_OK);
    }

    #[test]
    fn update_check_verbose_returns_ok() {
        let code = dispatch(&args(&["update", "check", "--verbose"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_OK);
    }

    #[test]
    fn config_db_list_rejects_unknown_flag() {
        let code = dispatch(&args(&["config", "db-list", "--bad-flag"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn config_history_list_rejects_unknown_flag() {
        let code = dispatch(&args(&["config", "history-list", "--bad-flag"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn config_audit_summary_rejects_unknown_flag() {
        let code = dispatch(&args(&["config", "audit-summary", "--bad-flag"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn cleanup_quarantine_missing_id_returns_usage() {
        let code = dispatch(&args(&["cleanup", "quarantine", "--dry-run"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn cleanup_quarantine_conflicting_flags_returns_usage() {
        let code = dispatch(&args(&[
            "cleanup",
            "quarantine",
            "--id",
            "Git.Git",
            "--dry-run",
            "--confirm",
        ]))
        .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn cleanup_quarantine_dry_run_json_returns_ok() {
        let code = dispatch(&args(&[
            "cleanup",
            "quarantine",
            "--id",
            "Git.Git",
            "--dry-run",
            "--json",
        ]))
        .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_OK);
    }

    #[test]
    fn source_suggest_rejects_unknown_flag() {
        let code = dispatch(&args(&["source", "suggest", "--bad-flag"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn source_suggest_verbose_returns_ok() {
        let code = dispatch(&args(&["source", "suggest", "--verbose"]))
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
        let code = map_integration_error(IntegrationError::Security(
            SecurityError::ProgramNotAllowlisted("powershell".to_string()),
        ));
        assert_eq!(code, EXIT_SECURITY);
    }
}
