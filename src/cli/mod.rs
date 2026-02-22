use crate::integration::{IntegrationError, ParsePath};
use crate::logging::log_event;
use crate::repository::{ConfigRepository, DatabaseRepository};
use crate::security::{SecurityError, SecurityGuard};
use crate::service::{
    CleanupError, CleanupService, SoftwareService, SourceSuggestionService, UpdateService,
};
use std::time::{SystemTime, UNIX_EPOCH};

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
    let mut simulate_failure = false;
    let mut simulate_rollback_failure = false;
    let mut risk_level = "medium".to_string();

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
            "--simulate-failure" => {
                simulate_failure = true;
                idx += 1;
            }
            "--simulate-rollback-failure" => {
                simulate_rollback_failure = true;
                idx += 1;
            }
            "--risk" => {
                if idx + 1 >= args.len() {
                    eprintln!("Validation error: --risk requires a value");
                    return Ok(EXIT_USAGE);
                }
                let value = args[idx + 1].to_ascii_lowercase();
                if value != "low" && value != "medium" && value != "high" && value != "critical" {
                    eprintln!("Validation error: --risk must be one of low|medium|high|critical");
                    return Ok(EXIT_USAGE);
                }
                risk_level = value;
                idx += 2;
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
    if (simulate_failure || simulate_rollback_failure) && !confirmed {
        eprintln!("Validation error: simulation flags require --confirm");
        return Ok(EXIT_USAGE);
    }
    if simulate_rollback_failure && !simulate_failure {
        eprintln!("Validation error: --simulate-rollback-failure requires --simulate-failure");
        return Ok(EXIT_USAGE);
    }
    let guard = SecurityGuard;
    if let Err(se) = guard.validate_risk_confirmation(&risk_level, confirmed) {
        eprintln!("Security blocked: {}", format_security_error(se));
        return Ok(EXIT_SECURITY);
    }

    let service = CleanupService::default();
    let plan = match service.plan_quarantine(&package_id, confirmed, dry_run) {
        Ok(plan) => plan,
        Err(msg) => {
            eprintln!("Validation error: {msg}");
            return Ok(EXIT_USAGE);
        }
    };
    let (plan, written) =
        match service.execute_cleanup_plan(plan, simulate_failure, simulate_rollback_failure) {
            Ok(result) => result,
            Err(err) => match err {
                CleanupError::Security(se) => {
                    eprintln!("Security blocked: {}", format_security_error(se));
                    return Ok(EXIT_SECURITY);
                }
                CleanupError::Repository(re) => {
                    eprintln!("Integration failure: failed to persist cleanup plan: {re}");
                    return Ok(EXIT_INTEGRATION);
                }
            },
        };

    if as_json {
        println!(
            "{{\n  \"operation_id\": \"{}\",\n  \"package_id\": \"{}\",\n  \"requested_mode\": \"{}\",\n  \"mode\": \"{}\",\n  \"status\": \"{}\",\n  \"mutation_boundary_reached\": {},\n  \"rollback_attempted\": {},\n  \"rollback_status\": \"{}\",\n  \"message\": \"{}\"\n}}",
            escape_json(&plan.operation_id),
            escape_json(&plan.package_id),
            escape_json(&plan.requested_mode),
            escape_json(&plan.mode),
            escape_json(&plan.status),
            plan.mutation_boundary_reached,
            plan.rollback_attempted,
            escape_json(&plan.rollback_status),
            escape_json(&plan.message),
        );
    } else {
        println!("operation_id: {}", plan.operation_id);
        println!("package_id: {}", plan.package_id);
        println!("status: {}", plan.status);
        println!(
            "mutation_boundary_reached: {}",
            plan.mutation_boundary_reached
        );
        println!("rollback_status: {}", plan.rollback_status);
        println!("message: {}", plan.message);
        if verbose {
            println!("audit_rows_written: {written}");
        }
    }

    if plan.status == "quarantine_failed" {
        Ok(EXIT_INTEGRATION)
    } else {
        Ok(EXIT_OK)
    }
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
        "gate-show" => {
            let mut as_json = false;
            let mut verbose = false;
            for opt in args.iter().skip(1) {
                match opt.as_str() {
                    "--json" => as_json = true,
                    "--verbose" => verbose = true,
                    _ => {
                        print_config_help();
                        return Ok(EXIT_USAGE);
                    }
                }
            }

            let repo = ConfigRepository::default();
            match repo.load_execution_gate() {
                Ok(gate) => {
                    let config_path = repo
                        .resolved_config_path()
                        .ok()
                        .map(|p| p.display().to_string())
                        .unwrap_or_else(|| "<unknown>".to_string());
                    let config_exists = std::path::Path::new(&config_path).exists();
                    if as_json {
                        print_config_gate_json(
                            gate.real_mutation_enabled,
                            &gate.gate_version,
                            &gate.approval_record_ref,
                            Some((&config_path, config_exists, verbose)),
                        );
                    } else {
                        print_config_gate_table(
                            gate.real_mutation_enabled,
                            &gate.gate_version,
                            &gate.approval_record_ref,
                            Some((&config_path, config_exists, verbose)),
                        );
                    }
                    Ok(EXIT_OK)
                }
                Err(err) => {
                    eprintln!("Integration failure: {err}");
                    Ok(EXIT_INTEGRATION)
                }
            }
        }
        "gate-history" => {
            let mut as_json = false;
            let mut enabled_only = false;
            let mut limit: Option<usize> = None;
            let mut since_ts: Option<i64> = None;

            let mut idx = 1usize;
            while idx < args.len() {
                match args[idx].as_str() {
                    "--json" => {
                        as_json = true;
                        idx += 1;
                    }
                    "--enabled-only" => {
                        enabled_only = true;
                        idx += 1;
                    }
                    "--limit" => {
                        if idx + 1 >= args.len() {
                            eprintln!("Validation error: --limit requires a value");
                            return Ok(EXIT_USAGE);
                        }
                        let parsed = match args[idx + 1].parse::<usize>() {
                            Ok(v) if v > 0 => v,
                            _ => {
                                eprintln!("Validation error: --limit must be a positive integer");
                                return Ok(EXIT_USAGE);
                            }
                        };
                        limit = Some(parsed);
                        idx += 2;
                    }
                    "--since" => {
                        if idx + 1 >= args.len() {
                            eprintln!("Validation error: --since requires a value");
                            return Ok(EXIT_USAGE);
                        }
                        let parsed = match args[idx + 1].parse::<i64>() {
                            Ok(v) if v >= 0 => v,
                            _ => {
                                eprintln!(
                                    "Validation error: --since must be a non-negative unix timestamp"
                                );
                                return Ok(EXIT_USAGE);
                            }
                        };
                        since_ts = Some(parsed);
                        idx += 2;
                    }
                    _ => {
                        print_config_help();
                        return Ok(EXIT_USAGE);
                    }
                }
            }

            let repo = DatabaseRepository::default();
            match repo.list_gate_history_filtered(limit, enabled_only, since_ts) {
                Ok(rows) => {
                    if as_json {
                        print_gate_history_json(&rows);
                    } else {
                        print_gate_history_table(&rows);
                    }
                    Ok(EXIT_OK)
                }
                Err(err) => {
                    eprintln!("Integration failure: {err}");
                    Ok(EXIT_INTEGRATION)
                }
            }
        }
        "gate-set" => {
            let mut enable_flag: Option<bool> = None;
            let mut gate_version: Option<String> = None;
            let mut approval_record_ref = String::new();
            let mut reason = String::new();
            let mut confirm = false;
            let mut keep_record = false;
            let mut dry_run = false;
            let mut as_json = false;

            let mut idx = 1usize;
            while idx < args.len() {
                match args[idx].as_str() {
                    "--enable" => {
                        if enable_flag.is_some() {
                            eprintln!("Validation error: --enable/--disable are mutually exclusive");
                            return Ok(EXIT_USAGE);
                        }
                        enable_flag = Some(true);
                        idx += 1;
                    }
                    "--disable" => {
                        if enable_flag.is_some() {
                            eprintln!("Validation error: --enable/--disable are mutually exclusive");
                            return Ok(EXIT_USAGE);
                        }
                        enable_flag = Some(false);
                        idx += 1;
                    }
                    "--gate-version" => {
                        if idx + 1 >= args.len() {
                            eprintln!("Validation error: --gate-version requires a value");
                            return Ok(EXIT_USAGE);
                        }
                        gate_version = Some(args[idx + 1].clone());
                        idx += 2;
                    }
                    "--approval-record" => {
                        if idx + 1 >= args.len() {
                            eprintln!("Validation error: --approval-record requires a value");
                            return Ok(EXIT_USAGE);
                        }
                        approval_record_ref = args[idx + 1].clone();
                        idx += 2;
                    }
                    "--reason" => {
                        if idx + 1 >= args.len() {
                            eprintln!("Validation error: --reason requires a value");
                            return Ok(EXIT_USAGE);
                        }
                        reason = args[idx + 1].clone();
                        idx += 2;
                    }
                    "--confirm" => {
                        confirm = true;
                        idx += 1;
                    }
                    "--keep-record" => {
                        keep_record = true;
                        idx += 1;
                    }
                    "--dry-run" => {
                        dry_run = true;
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

            let Some(enabled) = enable_flag else {
                eprintln!("Validation error: one of --enable/--disable is required");
                return Ok(EXIT_USAGE);
            };
            if enabled && !confirm && !dry_run {
                eprintln!("Validation error: --confirm is required when --enable is used");
                return Ok(EXIT_USAGE);
            }
            if enabled && keep_record {
                eprintln!("Validation error: --keep-record can only be used with --disable");
                return Ok(EXIT_USAGE);
            }
            if enabled && approval_record_ref.trim().is_empty() {
                eprintln!("Validation error: --approval-record is required when --enable is used");
                return Ok(EXIT_USAGE);
            }
            if !dry_run && reason.trim().is_empty() {
                eprintln!("Validation error: --reason is required unless --dry-run is used");
                return Ok(EXIT_USAGE);
            }

            let repo = ConfigRepository::default();
            let current = match repo.load_execution_gate() {
                Ok(gate) => gate,
                Err(err) => {
                    eprintln!("Integration failure: {err}");
                    return Ok(EXIT_INTEGRATION);
                }
            };
            let final_gate_version = gate_version.unwrap_or_else(|| current.gate_version.clone());
            let final_approval_record = if enabled {
                approval_record_ref
            } else if keep_record {
                if approval_record_ref.trim().is_empty() {
                    current.approval_record_ref.clone()
                } else {
                    approval_record_ref
                }
            } else {
                String::new()
            };

            let write_path = match repo.resolved_config_path() {
                Ok(path) => path,
                Err(err) => {
                    eprintln!("Integration failure: {err}");
                    return Ok(EXIT_INTEGRATION);
                }
            };
            let gate = if dry_run {
                crate::repository::ExecutionGateConfig {
                    real_mutation_enabled: enabled,
                    gate_version: final_gate_version.clone(),
                    approval_record_ref: final_approval_record.clone(),
                }
            } else {
                if let Err(err) =
                    repo.set_execution_gate(enabled, &final_gate_version, &final_approval_record)
                {
                    eprintln!("Integration failure: {err}");
                    return Ok(EXIT_INTEGRATION);
                }
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|v| v.as_secs() as i64)
                    .unwrap_or(0);
                let db_repo = DatabaseRepository::default();
                if let Err(err) = db_repo.log_gate_change(
                    timestamp,
                    enabled,
                    &final_gate_version,
                    &final_approval_record,
                    &reason,
                ) {
                    eprintln!("Integration failure: {err}");
                    return Ok(EXIT_INTEGRATION);
                }
                match repo.load_execution_gate() {
                    Ok(gate) => gate,
                    Err(err) => {
                        eprintln!("Integration failure: {err}");
                        return Ok(EXIT_INTEGRATION);
                    }
                }
            };

            if as_json {
                print_config_gate_set_json(
                    gate.real_mutation_enabled,
                    &gate.gate_version,
                    &gate.approval_record_ref,
                    &write_path.display().to_string(),
                    dry_run,
                );
            } else {
                print_config_gate_table(
                    gate.real_mutation_enabled,
                    &gate.gate_version,
                    &gate.approval_record_ref,
                    None,
                );
                println!("dry_run: {}", dry_run);
                println!("config_path: {}", write_path.display());
            }
            Ok(EXIT_OK)
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
        SecurityError::PathTraversalDetected(path) => {
            format!("path traversal detected for '{path}'")
        }
        SecurityError::TargetOutsideAllowlist(path) => {
            format!("target path '{path}' is outside allowlist root")
        }
        SecurityError::SymbolicLinkDetected(path) => {
            format!("symbolic-link component detected at '{path}'")
        }
        SecurityError::ConfirmationRequired(risk) => {
            format!("risk '{risk}' requires explicit --confirm")
        }
        SecurityError::RealMutationDisabled => {
            "real mutation is disabled; set execution.real_mutation_enabled=true".to_string()
        }
        SecurityError::ApprovalRecordMissing => {
            "approval record is required; set execution.approval_record_ref".to_string()
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
        "Usage: synora cleanup quarantine --id <package_id> [--dry-run|--confirm] [--json] [--verbose] [--risk <low|medium|high|critical>] [--simulate-failure] [--simulate-rollback-failure]"
    );
}

fn print_config_help() {
    println!("Usage: synora config init");
    println!("   or: synora config db-list [--json]");
    println!("   or: synora config history-list [--json]");
    println!("   or: synora config audit-summary [--json]");
    println!("   or: synora config gate-show [--json] [--verbose]");
    println!(
        "   or: synora config gate-history [--json] [--enabled-only] [--limit <n>] [--since <unix_ts>]"
    );
    println!(
        "   or: synora config gate-set (--enable|--disable) [--confirm] [--approval-record <ref>] [--gate-version <version>] [--reason <text>] [--keep-record] [--dry-run] [--json]"
    );
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

fn print_config_gate_json(
    real_mutation_enabled: bool,
    gate_version: &str,
    approval_record_ref: &str,
    extra: Option<(&str, bool, bool)>,
) {
    let approval_record_present = !approval_record_ref.trim().is_empty();
    if let Some((config_path, config_exists, verbose)) = extra {
        if verbose {
            println!(
                "{{\"real_mutation_enabled\":{},\"gate_version\":\"{}\",\"approval_record_ref\":\"{}\",\"approval_record_present\":{},\"config_path\":\"{}\",\"config_exists\":{}}}",
                real_mutation_enabled,
                escape_json(gate_version),
                escape_json(approval_record_ref),
                approval_record_present,
                escape_json(config_path),
                config_exists
            );
            return;
        }
    }
    println!(
        "{{\"real_mutation_enabled\":{},\"gate_version\":\"{}\",\"approval_record_ref\":\"{}\",\"approval_record_present\":{}}}",
        real_mutation_enabled,
        escape_json(gate_version),
        escape_json(approval_record_ref),
        approval_record_present
    );
}

fn print_config_gate_table(
    real_mutation_enabled: bool,
    gate_version: &str,
    approval_record_ref: &str,
    extra: Option<(&str, bool, bool)>,
) {
    println!("real_mutation_enabled: {}", real_mutation_enabled);
    println!("gate_version: {}", gate_version);
    if approval_record_ref.trim().is_empty() {
        println!("approval_record_ref: <empty>");
        println!("approval_record_present: false");
    } else {
        println!("approval_record_ref: {}", approval_record_ref);
        println!("approval_record_present: true");
    }
    if let Some((config_path, config_exists, verbose)) = extra {
        if verbose {
            println!("config_path: {}", config_path);
            println!("config_exists: {}", config_exists);
        }
    }
}

fn print_config_gate_set_json(
    real_mutation_enabled: bool,
    gate_version: &str,
    approval_record_ref: &str,
    config_path: &str,
    dry_run: bool,
) {
    let approval_record_present = !approval_record_ref.trim().is_empty();
    println!(
        "{{\"real_mutation_enabled\":{},\"gate_version\":\"{}\",\"approval_record_ref\":\"{}\",\"approval_record_present\":{},\"config_path\":\"{}\",\"dry_run\":{}}}",
        real_mutation_enabled,
        escape_json(gate_version),
        escape_json(approval_record_ref),
        approval_record_present,
        escape_json(config_path),
        dry_run
    );
}

fn print_gate_history_json(rows: &[crate::repository::GateHistoryRow]) {
    println!("[");
    for (idx, row) in rows.iter().enumerate() {
        let comma = if idx + 1 == rows.len() { "" } else { "," };
        println!(
            "  {{\"id\":{},\"timestamp\":{},\"real_mutation_enabled\":{},\"gate_version\":\"{}\",\"approval_record_ref\":\"{}\",\"approval_record_present\":{},\"reason\":\"{}\"}}{}",
            row.id,
            row.timestamp,
            row.real_mutation_enabled,
            escape_json(&row.gate_version),
            escape_json(&row.approval_record_ref),
            !row.approval_record_ref.trim().is_empty(),
            escape_json(&row.reason),
            comma
        );
    }
    println!("]");
}

fn print_gate_history_table(rows: &[crate::repository::GateHistoryRow]) {
    if rows.is_empty() {
        println!("No gate history entries.");
        return;
    }
    println!("id  timestamp  enabled  gate_version  approval_record_present  reason");
    println!("--  ---------  -------  ------------  -----------------------  ------");
    for row in rows {
        println!(
            "{}  {}  {}  {}  {}  {}",
            row.id,
            row.timestamp,
            row.real_mutation_enabled,
            row.gate_version,
            !row.approval_record_ref.trim().is_empty(),
            row.reason
        );
    }
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
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Mutex, OnceLock};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    fn unique_test_home(prefix: &str) -> PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("{prefix}-{now}-{seq}"))
    }

    fn dispatch_isolated(args: &[String]) -> Result<i32, String> {
        let _guard = env_lock()
            .lock()
            .expect("test env lock should not be poisoned");
        let previous = std::env::var("SYNORA_HOME").ok();
        let test_home = unique_test_home("synora-cli-test-home");
        std::fs::create_dir_all(&test_home)
            .map_err(|err| format!("failed to create test home: {err}"))?;
        std::env::set_var("SYNORA_HOME", &test_home);

        let result = dispatch(args);

        match previous {
            Some(v) => std::env::set_var("SYNORA_HOME", v),
            None => std::env::remove_var("SYNORA_HOME"),
        }
        let _ = std::fs::remove_dir_all(test_home);
        result
    }

    fn args(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|v| (*v).to_string()).collect()
    }

    fn ensure_gate_disabled_for_test() {
        let code = dispatch_isolated(&args(&[
            "config",
            "gate-set",
            "--disable",
            "--reason",
            "test setup: enforce disabled gate",
            "--json",
        ]))
        .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_OK);
    }

    #[test]
    fn update_apply_missing_id_returns_usage() {
        let code = dispatch_isolated(&args(&["update", "apply"])).expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn update_apply_conflicting_flags_returns_usage() {
        let code = dispatch_isolated(&args(&[
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
        let code = dispatch_isolated(&args(&[
            "update", "apply", "--id", "Git.Git", "--yes", "--json",
        ]))
        .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_OK);
    }

    #[test]
    fn software_list_verbose_returns_ok() {
        let code = dispatch_isolated(&args(&["software", "list", "--verbose"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_OK);
    }

    #[test]
    fn update_check_verbose_returns_ok() {
        let code = dispatch_isolated(&args(&["update", "check", "--verbose"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_OK);
    }

    #[test]
    fn config_db_list_rejects_unknown_flag() {
        let code = dispatch_isolated(&args(&["config", "db-list", "--bad-flag"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn config_history_list_rejects_unknown_flag() {
        let code = dispatch_isolated(&args(&["config", "history-list", "--bad-flag"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn config_audit_summary_rejects_unknown_flag() {
        let code = dispatch_isolated(&args(&["config", "audit-summary", "--bad-flag"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn config_gate_show_rejects_unknown_flag() {
        let code = dispatch_isolated(&args(&["config", "gate-show", "--bad-flag"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn config_gate_show_json_returns_ok() {
        let code = dispatch_isolated(&args(&["config", "gate-show", "--json"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_OK);
    }

    #[test]
    fn config_gate_show_verbose_returns_ok() {
        let code = dispatch_isolated(&args(&["config", "gate-show", "--verbose"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_OK);
    }

    #[test]
    fn config_gate_history_rejects_unknown_flag() {
        let code = dispatch_isolated(&args(&["config", "gate-history", "--bad-flag"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn config_gate_history_limit_missing_value_returns_usage() {
        let code = dispatch_isolated(&args(&["config", "gate-history", "--limit"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn config_gate_history_limit_invalid_returns_usage() {
        let code = dispatch_isolated(&args(&["config", "gate-history", "--limit", "0"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn config_gate_history_since_missing_value_returns_usage() {
        let code = dispatch_isolated(&args(&["config", "gate-history", "--since"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn config_gate_history_since_invalid_returns_usage() {
        let code = dispatch_isolated(&args(&["config", "gate-history", "--since", "-1"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn config_gate_history_json_returns_ok() {
        let code = dispatch_isolated(&args(&["config", "gate-history", "--json"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_OK);
    }

    #[test]
    fn config_gate_history_enabled_only_returns_ok() {
        let code = dispatch_isolated(&args(&["config", "gate-history", "--enabled-only"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_OK);
    }

    #[test]
    fn config_gate_history_limit_returns_ok() {
        let code = dispatch_isolated(&args(&["config", "gate-history", "--limit", "10", "--json"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_OK);
    }

    #[test]
    fn config_gate_history_since_returns_ok() {
        let code = dispatch_isolated(&args(&[
            "config",
            "gate-history",
            "--enabled-only",
            "--since",
            "0",
            "--limit",
            "5",
            "--json",
        ]))
        .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_OK);
    }

    #[test]
    fn config_gate_set_rejects_unknown_flag() {
        let code = dispatch_isolated(&args(&["config", "gate-set", "--bad-flag"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn config_gate_set_requires_mode() {
        let code = dispatch_isolated(&args(&["config", "gate-set", "--json"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn config_gate_set_enable_requires_approval_record() {
        let code = dispatch_isolated(&args(&["config", "gate-set", "--enable", "--json"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn config_gate_set_enable_requires_confirm() {
        let code = dispatch_isolated(&args(&[
            "config",
            "gate-set",
            "--enable",
            "--approval-record",
            "docs/security/record.md",
            "--json",
        ]))
        .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn config_gate_set_keep_record_requires_disable() {
        let code = dispatch_isolated(&args(&[
            "config",
            "gate-set",
            "--enable",
            "--confirm",
            "--approval-record",
            "docs/security/record.md",
            "--keep-record",
            "--json",
        ]))
        .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn config_gate_set_enable_dry_run_without_confirm_returns_ok() {
        let code = dispatch_isolated(&args(&[
            "config",
            "gate-set",
            "--enable",
            "--approval-record",
            "docs/security/record.md",
            "--dry-run",
            "--json",
        ]))
        .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_OK);
    }

    #[test]
    fn config_gate_set_requires_reason_when_not_dry_run() {
        let code = dispatch_isolated(&args(&[
            "config",
            "gate-set",
            "--disable",
            "--json",
        ]))
        .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn cleanup_quarantine_missing_id_returns_usage() {
        let code = dispatch_isolated(&args(&["cleanup", "quarantine", "--dry-run"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn cleanup_quarantine_conflicting_flags_returns_usage() {
        let code = dispatch_isolated(&args(&[
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
        let code = dispatch_isolated(&args(&[
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
    fn cleanup_quarantine_confirm_json_returns_security_when_gate_disabled() {
        ensure_gate_disabled_for_test();
        let code = dispatch_isolated(&args(&[
            "cleanup",
            "quarantine",
            "--id",
            "Git.Git",
            "--confirm",
            "--json",
        ]))
        .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_SECURITY);
    }

    #[test]
    fn cleanup_quarantine_simulation_requires_confirm() {
        let code = dispatch_isolated(&args(&[
            "cleanup",
            "quarantine",
            "--id",
            "Git.Git",
            "--simulate-failure",
        ]))
        .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn cleanup_quarantine_rollback_simulation_requires_failure_flag() {
        let code = dispatch_isolated(&args(&[
            "cleanup",
            "quarantine",
            "--id",
            "Git.Git",
            "--confirm",
            "--simulate-rollback-failure",
        ]))
        .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn cleanup_quarantine_confirm_simulated_failure_returns_security_when_gate_disabled() {
        ensure_gate_disabled_for_test();
        let code = dispatch_isolated(&args(&[
            "cleanup",
            "quarantine",
            "--id",
            "Git.Git",
            "--confirm",
            "--simulate-failure",
            "--json",
        ]))
        .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_SECURITY);
    }

    #[test]
    fn cleanup_quarantine_high_risk_requires_confirm() {
        let code = dispatch_isolated(&args(&[
            "cleanup",
            "quarantine",
            "--id",
            "Git.Git",
            "--risk",
            "high",
            "--json",
        ]))
        .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_SECURITY);
    }

    #[test]
    fn cleanup_quarantine_high_risk_with_confirm_returns_security_when_gate_disabled() {
        ensure_gate_disabled_for_test();
        let code = dispatch_isolated(&args(&[
            "cleanup",
            "quarantine",
            "--id",
            "Git.Git",
            "--risk",
            "high",
            "--confirm",
            "--json",
        ]))
        .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_SECURITY);
    }

    #[test]
    fn cleanup_quarantine_traversal_target_returns_security() {
        let code = dispatch_isolated(&args(&[
            "cleanup",
            "quarantine",
            "--id",
            "../evil",
            "--confirm",
            "--json",
        ]))
        .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_SECURITY);
    }

    #[test]
    fn source_suggest_rejects_unknown_flag() {
        let code = dispatch_isolated(&args(&["source", "suggest", "--bad-flag"]))
            .expect("dispatch should return exit code");
        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn source_suggest_verbose_returns_ok() {
        let code = dispatch_isolated(&args(&["source", "suggest", "--verbose"]))
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
