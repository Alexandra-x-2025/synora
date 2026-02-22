use std::env;
use std::fs;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::path::{Path, PathBuf};

use clap::{Args, Parser, Subcommand};
use rusqlite::types::Value;
use rusqlite::{params, params_from_iter, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
enum CliError {
    #[error("validation error: {0}")]
    Usage(String),
    #[error("security blocked: {0}")]
    #[allow(dead_code)]
    Security(String),
    #[error("integration failure: {0}")]
    Integration(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("db error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("config error: {0}")]
    Config(String),
}

impl CliError {
    fn code(&self) -> i32 {
        match self {
            CliError::Usage(_) => 2,
            CliError::Security(_) => 3,
            CliError::Integration(_) | CliError::Io(_) | CliError::Db(_) | CliError::Json(_) => 4,
            CliError::Config(_) => 4,
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "synora", version, about = "Synora CLI skeleton")]
struct Cli {
    #[command(subcommand)]
    command: TopCommand,
}

#[derive(Debug, Subcommand)]
enum TopCommand {
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
    Software {
        #[command(subcommand)]
        command: SoftwareCommand,
    },
    Source {
        #[command(subcommand)]
        command: SourceCommand,
    },
    Update {
        #[command(subcommand)]
        command: UpdateCommand,
    },
    Ai {
        #[command(subcommand)]
        command: AiCommand,
    },
    Ui {
        #[command(subcommand)]
        command: UiCommand,
    },
    Job {
        #[command(subcommand)]
        command: JobCommand,
    },
    Cleanup {
        #[command(subcommand)]
        command: CleanupCommand,
    },
    Repo {
        #[command(subcommand)]
        command: RepoCommand,
    },
    Package {
        #[command(subcommand)]
        command: PackageCommand,
    },
    Download {
        #[command(subcommand)]
        command: DownloadCommand,
    },
}

#[derive(Debug, Subcommand)]
enum ConfigCommand {
    Init(OutputArgs),
    GateShow(OutputArgs),
    GateSet(GateSetArgs),
    GateHistory(GateHistoryArgs),
}

#[derive(Debug, Subcommand)]
enum SoftwareCommand {
    Discover {
        #[command(subcommand)]
        command: DiscoverCommand,
    },
    List(SoftwareListArgs),
}

#[derive(Debug, Subcommand)]
enum DiscoverCommand {
    Scan(OutputArgs),
    History(DiscoverHistoryArgs),
}

#[derive(Debug, Subcommand)]
enum SourceCommand {
    Suggest(SourceSuggestArgs),
    Review(SourceReviewArgs),
    ReviewBulk(SourceReviewBulkArgs),
    List(SourceListArgs),
    ApplyApproved(SourceApplyApprovedArgs),
    RegistryList(SourceRegistryListArgs),
    RegistryDisable(SourceRegistryDisableArgs),
    RegistryEnable(SourceRegistryEnableArgs),
}

#[derive(Debug, Subcommand)]
enum UpdateCommand {
    Check(UpdateCheckArgs),
    Apply(UpdateApplyArgs),
    History(UpdateHistoryArgs),
}

#[derive(Debug, Subcommand)]
enum AiCommand {
    Analyze(AiAnalyzeArgs),
    Recommend(AiRecommendArgs),
    RepairPlan(AiRepairPlanArgs),
}

#[derive(Debug, Subcommand)]
enum UiCommand {
    Search(UiSearchArgs),
    ActionRun(UiActionRunArgs),
}

#[derive(Debug, Subcommand)]
enum JobCommand {
    Submit(JobSubmitArgs),
    List(JobListArgs),
    Retry(JobRetryArgs),
    DeadletterList(JobDeadletterListArgs),
    ReplayDeadletter(JobReplayDeadletterArgs),
    WorkerRun(JobWorkerRunArgs),
}

#[derive(Debug, Subcommand)]
enum CleanupCommand {
    Apply(CleanupApplyArgs),
    History(CleanupHistoryArgs),
}

#[derive(Debug, Subcommand)]
enum RepoCommand {
    List(RepoListArgs),
    Add(RepoAddArgs),
    Remove(RepoRemoveArgs),
    Sync(RepoSyncArgs),
}

#[derive(Debug, Subcommand)]
enum PackageCommand {
    Search(PackageSearchArgs),
}

#[derive(Debug, Subcommand)]
enum DownloadCommand {
    Start(DownloadStartArgs),
    List(DownloadListArgs),
    Show(DownloadShowArgs),
    Retry(DownloadRetryArgs),
    Verify(DownloadVerifyArgs),
    History(DownloadHistoryArgs),
}

#[derive(Debug, Clone, Args)]
struct OutputArgs {
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct GateSetArgs {
    #[arg(long)]
    enable: bool,
    #[arg(long)]
    disable: bool,
    #[arg(long)]
    confirm: bool,
    #[arg(long)]
    approval_record: Option<String>,
    #[arg(long)]
    gate_version: Option<String>,
    #[arg(long)]
    reason: Option<String>,
    #[arg(long)]
    keep_record: bool,
    #[arg(long)]
    dry_run: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct GateHistoryArgs {
    #[arg(long)]
    enabled_only: bool,
    #[arg(long)]
    since: Option<i64>,
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    reason_contains: Option<String>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct SourceSuggestArgs {
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    min_confidence: Option<i64>,
    #[arg(long)]
    domain: Option<String>,
    #[arg(long)]
    contains: Option<String>,
    #[arg(long)]
    status: Option<String>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct SoftwareListArgs {
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    offset: Option<u32>,
    #[arg(long)]
    contains: Option<String>,
    #[arg(long)]
    active_only: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct SourceReviewArgs {
    #[arg(long)]
    candidate_id: i64,
    #[arg(long)]
    approve: bool,
    #[arg(long)]
    reject: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct SourceReviewBulkArgs {
    #[arg(long)]
    approve: bool,
    #[arg(long)]
    reject: bool,
    #[arg(long)]
    status: Option<String>,
    #[arg(long)]
    domain: Option<String>,
    #[arg(long)]
    contains: Option<String>,
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct SourceListArgs {
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    offset: Option<u32>,
    #[arg(long)]
    status: Option<String>,
    #[arg(long)]
    domain: Option<String>,
    #[arg(long)]
    contains: Option<String>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct SourceApplyApprovedArgs {
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    domain: Option<String>,
    #[arg(long)]
    contains: Option<String>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct SourceRegistryListArgs {
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    offset: Option<u32>,
    #[arg(long)]
    status: Option<String>,
    #[arg(long)]
    domain: Option<String>,
    #[arg(long)]
    contains: Option<String>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct SourceRegistryDisableArgs {
    #[arg(long)]
    candidate_id: Option<i64>,
    #[arg(long)]
    status: Option<String>,
    #[arg(long)]
    domain: Option<String>,
    #[arg(long)]
    contains: Option<String>,
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct SourceRegistryEnableArgs {
    #[arg(long)]
    candidate_id: Option<i64>,
    #[arg(long)]
    status: Option<String>,
    #[arg(long)]
    domain: Option<String>,
    #[arg(long)]
    contains: Option<String>,
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct DiscoverHistoryArgs {
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    offset: Option<u32>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct UpdateCheckArgs {
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    offset: Option<u32>,
    #[arg(long)]
    domain: Option<String>,
    #[arg(long)]
    contains: Option<String>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct UpdateApplyArgs {
    #[arg(long)]
    candidate_id: i64,
    #[arg(long)]
    dry_run: bool,
    #[arg(long)]
    confirm: bool,
    #[arg(long)]
    execution_ticket: Option<String>,
    #[arg(long)]
    simulate_failure: bool,
    #[arg(long)]
    simulate_rollback_failure: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct UpdateHistoryArgs {
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    offset: Option<u32>,
    #[arg(long)]
    status: Option<String>,
    #[arg(long)]
    mode: Option<String>,
    #[arg(long)]
    candidate_id: Option<i64>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct CleanupApplyArgs {
    #[arg(long)]
    software_id: i64,
    #[arg(long)]
    dry_run: bool,
    #[arg(long)]
    confirm: bool,
    #[arg(long)]
    execution_ticket: Option<String>,
    #[arg(long)]
    simulate_failure: bool,
    #[arg(long)]
    simulate_rollback_failure: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct CleanupHistoryArgs {
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    offset: Option<u32>,
    #[arg(long)]
    status: Option<String>,
    #[arg(long)]
    mode: Option<String>,
    #[arg(long)]
    software_id: Option<i64>,
    #[arg(long)]
    execution_ticket: Option<String>,
    #[arg(long)]
    rollback_status: Option<String>,
    #[arg(long)]
    contains: Option<String>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct AiRepairPlanArgs {
    #[arg(long)]
    software: String,
    #[arg(long)]
    issue: String,
    #[arg(long)]
    verbose: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct AiAnalyzeArgs {
    #[arg(long)]
    verbose: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct AiRecommendArgs {
    #[arg(long)]
    goal: String,
    #[arg(long)]
    verbose: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct UiSearchArgs {
    #[arg(long)]
    q: String,
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct UiActionRunArgs {
    #[arg(long)]
    id: String,
    #[arg(long)]
    confirm: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct JobSubmitArgs {
    #[arg(long = "type")]
    job_type: String,
    #[arg(long)]
    payload: String,
    #[arg(long)]
    priority: Option<i64>,
    #[arg(long)]
    schedule_at: Option<i64>,
    #[arg(long)]
    simulate_failed: bool,
    #[arg(long)]
    simulate_deadletter: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct JobListArgs {
    #[arg(long)]
    status: Option<String>,
    #[arg(long = "type")]
    job_type: Option<String>,
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    offset: Option<u32>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct JobRetryArgs {
    #[arg(long)]
    id: i64,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct JobDeadletterListArgs {
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    offset: Option<u32>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct JobReplayDeadletterArgs {
    #[arg(long)]
    id: Option<i64>,
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct JobWorkerRunArgs {
    #[arg(long)]
    once: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct RepoListArgs {
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    offset: Option<u32>,
    #[arg(long)]
    status: Option<String>,
    #[arg(long)]
    kind: Option<String>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct RepoAddArgs {
    #[arg(long)]
    name: String,
    #[arg(long)]
    url: String,
    #[arg(long)]
    kind: String,
    #[arg(long)]
    priority: Option<i64>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct RepoRemoveArgs {
    #[arg(long)]
    repo_key: String,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct RepoSyncArgs {
    #[arg(long)]
    repo_key: Option<String>,
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct PackageSearchArgs {
    #[arg(long)]
    contains: Option<String>,
    #[arg(long)]
    repo_key: Option<String>,
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    offset: Option<u32>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct DownloadStartArgs {
    #[arg(long)]
    package_id: String,
    #[arg(long)]
    dry_run: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct DownloadListArgs {
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    offset: Option<u32>,
    #[arg(long)]
    status: Option<String>,
    #[arg(long)]
    verification_status: Option<String>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct DownloadShowArgs {
    #[arg(long)]
    job_id: String,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct DownloadRetryArgs {
    #[arg(long)]
    job_id: String,
    #[arg(long)]
    dry_run: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct DownloadVerifyArgs {
    #[arg(long)]
    job_id: String,
    #[arg(long)]
    simulate_failure: bool,
    #[arg(long)]
    simulate_hash_failure: bool,
    #[arg(long)]
    simulate_signature_failure: bool,
    #[arg(long)]
    simulate_source_policy_failure: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Args)]
struct DownloadHistoryArgs {
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    offset: Option<u32>,
    #[arg(long)]
    status: Option<String>,
    #[arg(long)]
    failure_type: Option<String>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppConfig {
    execution: ExecutionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExecutionConfig {
    real_mutation_enabled: bool,
    gate_version: String,
    approval_record_ref: String,
}

#[derive(Debug, Clone, Deserialize)]
struct DiscoveredSoftware {
    name: String,
    version: String,
    publisher: String,
    install_location: String,
    discovery_source: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            execution: ExecutionConfig {
                real_mutation_enabled: false,
                gate_version: "phase3-draft-v1".to_string(),
                approval_record_ref: String::new(),
            },
        }
    }
}

pub fn run() -> i32 {
    match run_impl() {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("Error: {err}");
            err.code()
        }
    }
}

fn run_impl() -> Result<(), CliError> {
    let cli = Cli::parse();
    match cli.command {
        TopCommand::Config { command } => handle_config(command),
        TopCommand::Software { command } => handle_software(command),
        TopCommand::Source { command } => handle_source(command),
        TopCommand::Update { command } => handle_update(command),
        TopCommand::Ai { command } => handle_ai(command),
        TopCommand::Ui { command } => handle_ui(command),
        TopCommand::Job { command } => handle_job(command),
        TopCommand::Cleanup { command } => handle_cleanup(command),
        TopCommand::Repo { command } => handle_repo(command),
        TopCommand::Package { command } => handle_package(command),
        TopCommand::Download { command } => handle_download(command),
    }
}

fn handle_config(command: ConfigCommand) -> Result<(), CliError> {
    match command {
        ConfigCommand::Init(args) => config_init(args.json),
        ConfigCommand::GateShow(args) => config_gate_show(args.json),
        ConfigCommand::GateSet(args) => config_gate_set(args),
        ConfigCommand::GateHistory(args) => config_gate_history(args),
    }
}

fn handle_software(command: SoftwareCommand) -> Result<(), CliError> {
    match command {
        SoftwareCommand::Discover { command } => match command {
            DiscoverCommand::Scan(args) => software_discover_scan(args.json),
            DiscoverCommand::History(args) => software_discover_history(args),
        },
        SoftwareCommand::List(args) => software_list(args),
    }
}

fn handle_source(command: SourceCommand) -> Result<(), CliError> {
    match command {
        SourceCommand::Suggest(args) => source_suggest(args),
        SourceCommand::Review(args) => source_review(args),
        SourceCommand::ReviewBulk(args) => source_review_bulk(args),
        SourceCommand::List(args) => source_list(args),
        SourceCommand::ApplyApproved(args) => source_apply_approved(args),
        SourceCommand::RegistryList(args) => source_registry_list(args),
        SourceCommand::RegistryDisable(args) => source_registry_disable(args),
        SourceCommand::RegistryEnable(args) => source_registry_enable(args),
    }
}

fn handle_update(command: UpdateCommand) -> Result<(), CliError> {
    match command {
        UpdateCommand::Check(args) => update_check(args),
        UpdateCommand::Apply(args) => update_apply(args),
        UpdateCommand::History(args) => update_history(args),
    }
}

fn handle_ai(command: AiCommand) -> Result<(), CliError> {
    match command {
        AiCommand::Analyze(args) => ai_analyze(args),
        AiCommand::Recommend(args) => ai_recommend(args),
        AiCommand::RepairPlan(args) => ai_repair_plan(args),
    }
}

fn handle_ui(command: UiCommand) -> Result<(), CliError> {
    match command {
        UiCommand::Search(args) => ui_search(args),
        UiCommand::ActionRun(args) => ui_action_run(args),
    }
}

fn handle_job(command: JobCommand) -> Result<(), CliError> {
    match command {
        JobCommand::Submit(args) => job_submit(args),
        JobCommand::List(args) => job_list(args),
        JobCommand::Retry(args) => job_retry(args),
        JobCommand::DeadletterList(args) => job_deadletter_list(args),
        JobCommand::ReplayDeadletter(args) => job_replay_deadletter(args),
        JobCommand::WorkerRun(args) => job_worker_run(args),
    }
}

fn handle_cleanup(command: CleanupCommand) -> Result<(), CliError> {
    match command {
        CleanupCommand::Apply(args) => cleanup_apply(args),
        CleanupCommand::History(args) => cleanup_history(args),
    }
}

fn handle_repo(command: RepoCommand) -> Result<(), CliError> {
    match command {
        RepoCommand::List(args) => repo_list(args),
        RepoCommand::Add(args) => repo_add(args),
        RepoCommand::Remove(args) => repo_remove(args),
        RepoCommand::Sync(args) => repo_sync(args),
    }
}

fn handle_package(command: PackageCommand) -> Result<(), CliError> {
    match command {
        PackageCommand::Search(args) => package_search(args),
    }
}

fn handle_download(command: DownloadCommand) -> Result<(), CliError> {
    match command {
        DownloadCommand::Start(args) => download_start(args),
        DownloadCommand::List(args) => download_list(args),
        DownloadCommand::Show(args) => download_show(args),
        DownloadCommand::Retry(args) => download_retry(args),
        DownloadCommand::Verify(args) => download_verify(args),
        DownloadCommand::History(args) => download_history(args),
    }
}

fn software_discover_scan(as_json: bool) -> Result<(), CliError> {
    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let discovered = discover_registry_software()?;
    let total_seen = discovered.len() as i64;
    let mut inserted = 0_i64;
    let mut updated = 0_i64;
    let mut reactivated = 0_i64;
    let mut deactivated = 0_i64;
    let mut skipped = 0_i64;
    let now = unix_ts();
    let mut seen_fingerprints: HashSet<String> = HashSet::new();

    for item in discovered {
        let fingerprint = make_fingerprint(&item.name, &item.publisher, &item.install_location);
        if fingerprint.is_empty() {
            skipped += 1;
            continue;
        }
        seen_fingerprints.insert(fingerprint.clone());

        let existing_active: Option<i64> = conn
            .query_row(
                "SELECT is_active FROM software_inventory WHERE fingerprint = ?1",
                params![&fingerprint],
                |r| r.get::<_, i64>(0),
            )
            .optional()?;

        conn.execute(
            r#"
            INSERT INTO software_inventory
            (name, version, publisher, install_location, discovery_source, source_confidence, first_seen_at, last_seen_at, is_active, fingerprint)
            VALUES (?1, ?2, ?3, ?4, ?5, 80, ?6, ?6, 1, ?7)
            ON CONFLICT(fingerprint) DO UPDATE SET
                name=excluded.name,
                version=excluded.version,
                publisher=excluded.publisher,
                install_location=excluded.install_location,
                discovery_source=excluded.discovery_source,
                source_confidence=excluded.source_confidence,
                last_seen_at=excluded.last_seen_at,
                is_active=1
            "#,
            params![
                item.name,
                item.version,
                item.publisher,
                item.install_location,
                item.discovery_source,
                now,
                fingerprint
            ],
        )?;

        match existing_active {
            None => inserted += 1,
            Some(0) => {
                updated += 1;
                reactivated += 1;
            }
            Some(_) => updated += 1,
        }
    }

    let mut stmt = conn.prepare(
        "SELECT id, fingerprint FROM software_inventory WHERE discovery_source = 'registry' AND is_active = 1",
    )?;
    let existing_rows = stmt
        .query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?)))?
        .collect::<Result<Vec<_>, _>>()?;
    for (id, fingerprint) in existing_rows {
        if !seen_fingerprints.contains(&fingerprint) {
            conn.execute(
                "UPDATE software_inventory SET is_active = 0 WHERE id = ?1",
                params![id],
            )?;
            deactivated += 1;
        }
    }

    let active_after: i64 = conn.query_row(
        "SELECT COUNT(1) FROM software_inventory WHERE discovery_source = 'registry' AND is_active = 1",
        [],
        |r| r.get(0),
    )?;

    let scan_id = next_operation_id("discover", total_seen);

    conn.execute(
        r#"
        INSERT INTO software_discovery_history
        (scan_id, ts, source, total_seen, inserted, updated, reactivated, deactivated, skipped, active_after)
        VALUES (?1, ?2, 'registry', ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        "#,
        params![
            scan_id,
            now,
            total_seen,
            inserted,
            updated,
            reactivated,
            deactivated,
            skipped,
            active_after
        ],
    )?;

    let payload = json!({
        "scan_id": scan_id,
        "source": "registry",
        "total_seen": total_seen,
        "inserted": inserted,
        "updated": updated,
        "reactivated": reactivated,
        "deactivated": deactivated,
        "active_after": active_after,
        "skipped": skipped,
        "duration_ms": 0
    });
    print_payload(as_json, payload, "Discovery scan finished.")
}

fn software_discover_history(args: DiscoverHistoryArgs) -> Result<(), CliError> {
    let limit = i64::from(args.limit.unwrap_or(100));
    let offset = i64::from(args.offset.unwrap_or(0));
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let mut stmt = conn.prepare(
        r#"
        SELECT scan_id, ts, source, total_seen, inserted, updated, reactivated, deactivated, skipped, active_after
        FROM software_discovery_history
        ORDER BY id DESC
        LIMIT ?1 OFFSET ?2
        "#,
    )?;
    let rows = stmt.query_map(params![limit, offset], |row| {
        Ok(json!({
            "scan_id": row.get::<_, String>(0)?,
            "timestamp": row.get::<_, i64>(1)?,
            "source": row.get::<_, String>(2)?,
            "total_seen": row.get::<_, i64>(3)?,
            "inserted": row.get::<_, i64>(4)?,
            "updated": row.get::<_, i64>(5)?,
            "reactivated": row.get::<_, i64>(6)?,
            "deactivated": row.get::<_, i64>(7)?,
            "skipped": row.get::<_, i64>(8)?,
            "active_after": row.get::<_, i64>(9)?
        }))
    })?;
    let payload: Vec<serde_json::Value> = rows.collect::<Result<Vec<_>, _>>()?;
    if payload.is_empty() {
        return print_payload(args.json, json!([]), "No discover history entries found.");
    }
    print_payload(args.json, json!(payload), "Discover history listed.")
}

fn software_list(args: SoftwareListArgs) -> Result<(), CliError> {
    let limit = i64::from(args.limit.unwrap_or(200));
    let offset = i64::from(args.offset.unwrap_or(0));
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let mut sql = String::from(
        r#"
        SELECT id, name, version, publisher, install_location, discovery_source, source_confidence, first_seen_at, last_seen_at, is_active
        FROM software_inventory
        "#,
    );

    let mut clauses: Vec<String> = Vec::new();
    let mut values: Vec<Value> = Vec::new();
    if args.active_only {
        clauses.push("is_active = 1".to_string());
    }
    if let Some(contains) = args.contains.clone() {
        clauses.push("(name LIKE ? OR publisher LIKE ? OR version LIKE ?)".to_string());
        let like = format!("%{contains}%");
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like));
    }
    if !clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&clauses.join(" AND "));
    }
    sql.push_str(" ORDER BY name ASC LIMIT ? OFFSET ?");
    values.push(Value::Integer(limit));
    values.push(Value::Integer(offset));

    let mut stmt = conn.prepare(&sql)?;

    let rows = stmt.query_map(params_from_iter(values.iter()), |row| {
        Ok(json!({
            "id": row.get::<_, i64>(0)?,
            "name": row.get::<_, String>(1)?,
            "version": row.get::<_, String>(2)?,
            "publisher": row.get::<_, String>(3)?,
            "install_location": row.get::<_, String>(4)?,
            "discovery_source": row.get::<_, String>(5)?,
            "source_confidence": row.get::<_, i64>(6)?,
            "first_seen_at": row.get::<_, i64>(7)?,
            "last_seen_at": row.get::<_, i64>(8)?,
            "is_active": row.get::<_, i64>(9)? == 1
        }))
    })?;
    let payload: Vec<serde_json::Value> = rows.collect::<Result<Vec<_>, _>>()?;
    if payload.is_empty() {
        return print_payload(args.json, json!([]), "No software entries found.");
    }
    print_payload(args.json, json!(payload), "Software entries listed.")
}

fn source_suggest(args: SourceSuggestArgs) -> Result<(), CliError> {
    let limit = i64::from(args.limit.unwrap_or(50));
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }
    let min_confidence = args.min_confidence.unwrap_or(0);
    if let Some(status) = args.status.as_deref() {
        match status {
            "pending" | "approved" | "rejected" => {}
            _ => {
                return Err(CliError::Usage(
                    "--status must be one of: pending, approved, rejected".to_string(),
                ));
            }
        }
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let mut stmt = conn.prepare(
        r#"
        SELECT id, name, publisher
        FROM software_inventory
        WHERE is_active = 1
        ORDER BY id ASC
        "#,
    )?;

    let mut rows = stmt.query([])?;
    let now = unix_ts();
    let mut generated = 0_i64;
    let mut upserted = 0_i64;

    while let Some(row) = rows.next()? {
        let software_id: i64 = row.get(0)?;
        let name: String = row.get(1)?;
        let publisher: String = row.get(2)?;

        let candidates = build_source_candidates(&name, &publisher);
        generated += candidates.len() as i64;

        for c in candidates {
            conn.execute(
                r#"
                INSERT INTO source_candidate
                (software_id, software_name, url, domain, confidence, reason, status, created_at)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'pending', ?7)
                ON CONFLICT(software_id, url) DO UPDATE SET
                    confidence=excluded.confidence,
                    reason=excluded.reason
                "#,
                params![
                    software_id,
                    name,
                    c.url,
                    c.domain,
                    c.confidence,
                    c.reason,
                    now
                ],
            )?;
            upserted += 1;
        }
    }

    let mut cte_clauses: Vec<String> = vec!["confidence >= ?".to_string()];
    let mut values: Vec<Value> = vec![Value::Integer(min_confidence)];
    if let Some(domain) = args.domain.clone() {
        cte_clauses.push("domain = ?".to_string());
        values.push(Value::Text(domain));
    }
    if let Some(contains) = args.contains.clone() {
        cte_clauses.push("(software_name LIKE ? OR url LIKE ? OR reason LIKE ?)".to_string());
        let like = format!("%{contains}%");
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like));
    }
    if let Some(status) = args.status.clone() {
        cte_clauses.push("status = ?".to_string());
        values.push(Value::Text(status));
    }

    let mut sql = String::from(
        r#"
        WITH ranked AS (
            SELECT
                id,
                software_id,
                software_name,
                url,
                domain,
                confidence,
                reason,
                status,
                ROW_NUMBER() OVER (
                    PARTITION BY software_id
                    ORDER BY confidence DESC, id ASC
                ) AS rn
            FROM source_candidate
        "#,
    );
    if !cte_clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&cte_clauses.join(" AND "));
    }
    sql.push_str(
        r#"
        )
        SELECT id, software_id, software_name, url, domain, confidence, reason, status
        FROM ranked
        WHERE rn = 1
        ORDER BY confidence DESC, software_name ASC LIMIT ?
        "#,
    );
    values.push(Value::Integer(limit));

    let mut out_stmt = conn.prepare(&sql)?;
    let out_rows = out_stmt.query_map(params_from_iter(values.iter()), |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, i64>(5)?,
            row.get::<_, String>(6)?,
            row.get::<_, String>(7)?,
        ))
    })?;

    let raw_items: Vec<(i64, i64, String, String, String, i64, String, String)> =
        out_rows.collect::<Result<Vec<_>, _>>()?;

    let mut domain_counts: HashMap<String, usize> = HashMap::new();
    let mut items: Vec<serde_json::Value> = Vec::new();
    const MAX_PER_DOMAIN: usize = 25;
    for (candidate_id, software_id, software_name, url, domain, confidence, reason, status) in raw_items {
        let count = domain_counts.entry(domain.clone()).or_insert(0);
        if *count >= MAX_PER_DOMAIN {
            continue;
        }
        *count += 1;
        items.push(json!({
            "candidate_id": candidate_id,
            "software_id": software_id,
            "software_name": software_name,
            "url": url,
            "domain": domain,
            "confidence": confidence,
            "reason": reason,
            "status": status
        }));
    }

    if args.json {
        println!("{}", serde_json::to_string_pretty(&items)?);
    } else {
        println!(
            "Source suggest finished: generated={generated}, upserted={upserted}, shown={}",
            items.len()
        );
        println!(
            "Filters: limit={limit}, min_confidence={min_confidence}, domain={:?}, contains={:?}, status={:?}",
            args.domain, args.contains, args.status
        );
    }
    Ok(())
}

fn source_review(args: SourceReviewArgs) -> Result<(), CliError> {
    if args.approve == args.reject {
        return Err(CliError::Usage(
            "exactly one of --approve or --reject must be set".to_string(),
        ));
    }

    let status = if args.approve { "approved" } else { "rejected" };
    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let changed = conn.execute(
        "UPDATE source_candidate SET status = ?1 WHERE id = ?2",
        params![status, args.candidate_id],
    )?;

    if changed == 0 {
        return Err(CliError::Usage(format!(
            "candidate_id {} not found",
            args.candidate_id
        )));
    }

    let payload = json!({
        "candidate_id": args.candidate_id,
        "status": status,
        "updated": true
    });
    print_payload(args.json, payload, "Source candidate reviewed.")
}

fn source_review_bulk(args: SourceReviewBulkArgs) -> Result<(), CliError> {
    if args.approve == args.reject {
        return Err(CliError::Usage(
            "exactly one of --approve or --reject must be set".to_string(),
        ));
    }
    if let Some(status) = args.status.as_deref() {
        match status {
            "pending" | "approved" | "rejected" => {}
            _ => {
                return Err(CliError::Usage(
                    "--status must be one of: pending, approved, rejected".to_string(),
                ));
            }
        }
    }

    let target_status = if args.approve { "approved" } else { "rejected" };
    let limit = i64::from(args.limit.unwrap_or(100));
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let mut sql = String::from("SELECT id FROM source_candidate");
    let mut clauses: Vec<String> = Vec::new();
    let mut values: Vec<Value> = Vec::new();

    if let Some(status) = args.status.clone() {
        clauses.push("status = ?".to_string());
        values.push(Value::Text(status));
    }
    if let Some(domain) = args.domain.clone() {
        clauses.push("domain = ?".to_string());
        values.push(Value::Text(domain));
    }
    if let Some(contains) = args.contains.clone() {
        clauses.push("(software_name LIKE ? OR url LIKE ? OR reason LIKE ?)".to_string());
        let like = format!("%{contains}%");
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like));
    }
    if !clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&clauses.join(" AND "));
    }
    sql.push_str(" ORDER BY id DESC LIMIT ?");
    values.push(Value::Integer(limit));

    let mut stmt = conn.prepare(&sql)?;
    let ids = stmt
        .query_map(params_from_iter(values.iter()), |row| row.get::<_, i64>(0))?
        .collect::<Result<Vec<_>, _>>()?;

    let mut changed = 0_i64;
    for id in &ids {
        changed += conn.execute(
            "UPDATE source_candidate SET status = ?1 WHERE id = ?2",
            params![target_status, id],
        )? as i64;
    }

    let payload = json!({
        "target_status": target_status,
        "matched": ids.len(),
        "updated": changed,
        "limit": limit
    });
    print_payload(args.json, payload, "Source candidates reviewed in bulk.")
}

fn source_list(args: SourceListArgs) -> Result<(), CliError> {
    let limit = i64::from(args.limit.unwrap_or(100));
    let offset = i64::from(args.offset.unwrap_or(0));
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }
    if let Some(status) = args.status.as_deref() {
        match status {
            "pending" | "approved" | "rejected" => {}
            _ => {
                return Err(CliError::Usage(
                    "--status must be one of: pending, approved, rejected".to_string(),
                ));
            }
        }
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let mut sql = String::from(
        r#"
        SELECT id, software_id, software_name, url, domain, confidence, reason, status, created_at
        FROM source_candidate
        "#,
    );
    let mut clauses: Vec<String> = Vec::new();
    let mut values: Vec<Value> = Vec::new();

    if let Some(status) = args.status.clone() {
        clauses.push("status = ?".to_string());
        values.push(Value::Text(status));
    }
    if let Some(domain) = args.domain.clone() {
        clauses.push("domain = ?".to_string());
        values.push(Value::Text(domain));
    }
    if let Some(contains) = args.contains.clone() {
        clauses.push("(software_name LIKE ? OR url LIKE ? OR reason LIKE ?)".to_string());
        let like = format!("%{contains}%");
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like));
    }
    if !clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&clauses.join(" AND "));
    }
    sql.push_str(" ORDER BY id DESC LIMIT ? OFFSET ?");
    values.push(Value::Integer(limit));
    values.push(Value::Integer(offset));

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params_from_iter(values.iter()), |row| {
        Ok(json!({
            "candidate_id": row.get::<_, i64>(0)?,
            "software_id": row.get::<_, i64>(1)?,
            "software_name": row.get::<_, String>(2)?,
            "url": row.get::<_, String>(3)?,
            "domain": row.get::<_, String>(4)?,
            "confidence": row.get::<_, i64>(5)?,
            "reason": row.get::<_, String>(6)?,
            "status": row.get::<_, String>(7)?,
            "created_at": row.get::<_, i64>(8)?
        }))
    })?;

    let payload: Vec<serde_json::Value> = rows.collect::<Result<Vec<_>, _>>()?;
    if payload.is_empty() {
        return print_payload(args.json, json!([]), "No source candidates found.");
    }
    print_payload(args.json, json!(payload), "Source candidates listed.")
}

fn source_apply_approved(args: SourceApplyApprovedArgs) -> Result<(), CliError> {
    let limit = i64::from(args.limit.unwrap_or(100));
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let mut sql = String::from(
        r#"
        SELECT id, software_id, software_name, url, domain, confidence, reason
        FROM source_candidate
        WHERE status = 'approved'
        "#,
    );
    let mut values: Vec<Value> = Vec::new();
    if let Some(domain) = args.domain.clone() {
        sql.push_str(" AND domain = ?");
        values.push(Value::Text(domain));
    }
    if let Some(contains) = args.contains.clone() {
        sql.push_str(" AND (software_name LIKE ? OR url LIKE ? OR reason LIKE ?)");
        let like = format!("%{contains}%");
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like));
    }
    sql.push_str(" ORDER BY id DESC LIMIT ?");
    values.push(Value::Integer(limit));

    let mut stmt = conn.prepare(&sql)?;
    let candidates = stmt
        .query_map(params_from_iter(values.iter()), |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, i64>(5)?,
                row.get::<_, String>(6)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    let applied_at = unix_ts();
    let mut inserted = 0_i64;
    let mut updated = 0_i64;
    let mut items: Vec<serde_json::Value> = Vec::new();

    for (candidate_id, software_id, software_name, url, domain, confidence, reason) in candidates {
        let exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM source_registry WHERE candidate_id = ?1)",
            params![candidate_id],
            |r| r.get::<_, i64>(0).map(|v| v == 1),
        )?;

        conn.execute(
            r#"
            INSERT INTO source_registry
            (candidate_id, software_id, software_name, url, domain, confidence, reason, status, applied_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'active', ?8)
            ON CONFLICT(candidate_id) DO UPDATE SET
                software_id=excluded.software_id,
                software_name=excluded.software_name,
                url=excluded.url,
                domain=excluded.domain,
                confidence=excluded.confidence,
                reason=excluded.reason,
                status='active',
                applied_at=excluded.applied_at
            "#,
            params![
                candidate_id,
                software_id,
                software_name,
                url,
                domain,
                confidence,
                reason,
                applied_at
            ],
        )?;

        if exists {
            updated += 1;
        } else {
            inserted += 1;
        }

        items.push(json!({
            "candidate_id": candidate_id,
            "software_id": software_id,
            "software_name": software_name,
            "url": url,
            "domain": domain,
            "confidence": confidence,
            "status": "active"
        }));
    }

    let payload = json!({
        "matched": items.len(),
        "inserted": inserted,
        "updated": updated,
        "applied_at": applied_at,
        "items": items
    });
    print_payload(args.json, payload, "Approved sources applied.")
}

fn source_registry_list(args: SourceRegistryListArgs) -> Result<(), CliError> {
    let limit = i64::from(args.limit.unwrap_or(100));
    let offset = i64::from(args.offset.unwrap_or(0));
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }
    if let Some(status) = args.status.as_deref() {
        validate_registry_status(status)?;
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let mut sql = String::from(
        r#"
        SELECT id, candidate_id, software_id, software_name, url, domain, confidence, reason, status, applied_at
        FROM source_registry
        "#,
    );
    let mut clauses: Vec<String> = Vec::new();
    let mut values: Vec<Value> = Vec::new();

    if let Some(status) = args.status.clone() {
        clauses.push("status = ?".to_string());
        values.push(Value::Text(status));
    }
    if let Some(domain) = args.domain.clone() {
        clauses.push("domain = ?".to_string());
        values.push(Value::Text(domain));
    }
    if let Some(contains) = args.contains.clone() {
        clauses.push("(software_name LIKE ? OR url LIKE ? OR reason LIKE ?)".to_string());
        let like = format!("%{contains}%");
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like));
    }
    if !clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&clauses.join(" AND "));
    }
    sql.push_str(" ORDER BY id DESC LIMIT ? OFFSET ?");
    values.push(Value::Integer(limit));
    values.push(Value::Integer(offset));

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params_from_iter(values.iter()), |row| {
        Ok(json!({
            "registry_id": row.get::<_, i64>(0)?,
            "candidate_id": row.get::<_, i64>(1)?,
            "software_id": row.get::<_, i64>(2)?,
            "software_name": row.get::<_, String>(3)?,
            "url": row.get::<_, String>(4)?,
            "domain": row.get::<_, String>(5)?,
            "confidence": row.get::<_, i64>(6)?,
            "reason": row.get::<_, String>(7)?,
            "status": row.get::<_, String>(8)?,
            "applied_at": row.get::<_, i64>(9)?
        }))
    })?;
    let payload: Vec<serde_json::Value> = rows.collect::<Result<Vec<_>, _>>()?;
    if payload.is_empty() {
        return print_payload(args.json, json!([]), "No registry sources found.");
    }
    print_payload(args.json, json!(payload), "Registry sources listed.")
}

fn source_registry_disable(args: SourceRegistryDisableArgs) -> Result<(), CliError> {
    let limit = i64::from(args.limit.unwrap_or(100));
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }
    if let Some(status) = args.status.as_deref() {
        validate_registry_status(status)?;
    }
    if args.candidate_id.is_none() && args.domain.is_none() && args.contains.is_none() && args.status.is_none() {
        return Err(CliError::Usage(
            "at least one selector is required: --candidate-id / --status / --domain / --contains".to_string(),
        ));
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let mut sql = String::from("SELECT id, candidate_id FROM source_registry");
    let mut clauses: Vec<String> = Vec::new();
    let mut values: Vec<Value> = Vec::new();

    if let Some(candidate_id) = args.candidate_id {
        clauses.push("candidate_id = ?".to_string());
        values.push(Value::Integer(candidate_id));
    }
    if let Some(status) = args.status.clone() {
        clauses.push("status = ?".to_string());
        values.push(Value::Text(status));
    } else {
        clauses.push("status = 'active'".to_string());
    }
    if let Some(domain) = args.domain.clone() {
        clauses.push("domain = ?".to_string());
        values.push(Value::Text(domain));
    }
    if let Some(contains) = args.contains.clone() {
        clauses.push("(software_name LIKE ? OR url LIKE ? OR reason LIKE ?)".to_string());
        let like = format!("%{contains}%");
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like));
    }
    if !clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&clauses.join(" AND "));
    }
    sql.push_str(" ORDER BY id DESC LIMIT ?");
    values.push(Value::Integer(limit));

    let mut stmt = conn.prepare(&sql)?;
    let targets = stmt
        .query_map(params_from_iter(values.iter()), |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    let mut changed = 0_i64;
    let mut candidate_ids: Vec<i64> = Vec::new();
    for (registry_id, candidate_id) in &targets {
        changed += conn.execute(
            "UPDATE source_registry SET status = 'disabled' WHERE id = ?1",
            params![registry_id],
        )? as i64;
        candidate_ids.push(*candidate_id);
    }

    let payload = json!({
        "matched": targets.len(),
        "updated": changed,
        "candidate_ids": candidate_ids
    });
    print_payload(args.json, payload, "Registry sources disabled.")
}

fn source_registry_enable(args: SourceRegistryEnableArgs) -> Result<(), CliError> {
    let limit = i64::from(args.limit.unwrap_or(100));
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }
    if let Some(status) = args.status.as_deref() {
        validate_registry_status(status)?;
    }
    if args.candidate_id.is_none() && args.domain.is_none() && args.contains.is_none() && args.status.is_none() {
        return Err(CliError::Usage(
            "at least one selector is required: --candidate-id / --status / --domain / --contains".to_string(),
        ));
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let mut sql = String::from("SELECT id, candidate_id FROM source_registry");
    let mut clauses: Vec<String> = Vec::new();
    let mut values: Vec<Value> = Vec::new();

    if let Some(candidate_id) = args.candidate_id {
        clauses.push("candidate_id = ?".to_string());
        values.push(Value::Integer(candidate_id));
    }
    if let Some(status) = args.status.clone() {
        clauses.push("status = ?".to_string());
        values.push(Value::Text(status));
    } else {
        clauses.push("status = 'disabled'".to_string());
    }
    if let Some(domain) = args.domain.clone() {
        clauses.push("domain = ?".to_string());
        values.push(Value::Text(domain));
    }
    if let Some(contains) = args.contains.clone() {
        clauses.push("(software_name LIKE ? OR url LIKE ? OR reason LIKE ?)".to_string());
        let like = format!("%{contains}%");
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like));
    }
    if !clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&clauses.join(" AND "));
    }
    sql.push_str(" ORDER BY id DESC LIMIT ?");
    values.push(Value::Integer(limit));

    let mut stmt = conn.prepare(&sql)?;
    let targets = stmt
        .query_map(params_from_iter(values.iter()), |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    let mut changed = 0_i64;
    let mut candidate_ids: Vec<i64> = Vec::new();
    for (registry_id, candidate_id) in &targets {
        changed += conn.execute(
            "UPDATE source_registry SET status = 'active' WHERE id = ?1",
            params![registry_id],
        )? as i64;
        candidate_ids.push(*candidate_id);
    }

    let payload = json!({
        "matched": targets.len(),
        "updated": changed,
        "candidate_ids": candidate_ids
    });
    print_payload(args.json, payload, "Registry sources enabled.")
}

fn update_check(args: UpdateCheckArgs) -> Result<(), CliError> {
    let limit = i64::from(args.limit.unwrap_or(100));
    let offset = i64::from(args.offset.unwrap_or(0));
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let mut sql = String::from(
        r#"
        SELECT candidate_id, software_id, software_name, url, domain, confidence, reason, applied_at
        FROM source_registry
        WHERE status = 'active'
        "#,
    );
    let mut values: Vec<Value> = Vec::new();

    if let Some(domain) = args.domain.clone() {
        sql.push_str(" AND domain = ?");
        values.push(Value::Text(domain));
    }
    if let Some(contains) = args.contains.clone() {
        sql.push_str(" AND (software_name LIKE ? OR url LIKE ? OR reason LIKE ?)");
        let like = format!("%{contains}%");
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like));
    }
    sql.push_str(" ORDER BY confidence DESC, candidate_id DESC LIMIT ? OFFSET ?");
    values.push(Value::Integer(limit));
    values.push(Value::Integer(offset));

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params_from_iter(values.iter()), |row| {
        let confidence = row.get::<_, i64>(5)?;
        let risk_level = if confidence >= 70 { "low" } else { "medium" };
        let recommendation = if confidence >= 70 {
            "review_and_apply"
        } else {
            "review_source_first"
        };
        Ok(json!({
            "candidate_id": row.get::<_, i64>(0)?,
            "software_id": row.get::<_, i64>(1)?,
            "software_name": row.get::<_, String>(2)?,
            "source_url": row.get::<_, String>(3)?,
            "source_domain": row.get::<_, String>(4)?,
            "confidence": confidence,
            "reason": row.get::<_, String>(6)?,
            "applied_at": row.get::<_, i64>(7)?,
            "update_available": true,
            "risk_level": risk_level,
            "recommendation": recommendation
        }))
    })?;
    let payload: Vec<serde_json::Value> = rows.collect::<Result<Vec<_>, _>>()?;
    if payload.is_empty() {
        return print_payload(args.json, json!([]), "No active update sources found.");
    }
    print_payload(args.json, json!(payload), "Update check completed.")
}

fn update_apply(args: UpdateApplyArgs) -> Result<(), CliError> {
    validate_update_apply_flags(&args)?;

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let selected = conn.query_row(
        r#"
        SELECT candidate_id, software_id, software_name, url, domain, confidence, reason
        FROM source_registry
        WHERE candidate_id = ?1 AND status = 'active'
        "#,
        params![args.candidate_id],
        |row| {
            Ok(json!({
                "candidate_id": row.get::<_, i64>(0)?,
                "software_id": row.get::<_, i64>(1)?,
                "software_name": row.get::<_, String>(2)?,
                "source_url": row.get::<_, String>(3)?,
                "source_domain": row.get::<_, String>(4)?,
                "confidence": row.get::<_, i64>(5)?,
                "reason": row.get::<_, String>(6)?
            }))
        },
    );

    let target = match selected {
        Ok(v) => v,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            return Err(CliError::Usage(format!(
                "active source not found for --candidate-id {}",
                args.candidate_id
            )));
        }
        Err(e) => return Err(CliError::Db(e)),
    };

    let operation_id = next_operation_id("update", args.candidate_id);
    let mode = if args.dry_run { "dry_run" } else { "confirmed_execution" };
    let execution_ticket = args.execution_ticket.clone().unwrap_or_default();

    if args.dry_run {
        let payload = json!({
            "operation_id": operation_id,
            "mode": mode,
            "status": "planned",
            "mutation_boundary_reached": false,
            "rollback_attempted": false,
            "execution_ticket": "",
            "target": target,
            "message": "update apply dry-run planned successfully"
        });
        return print_payload(args.json, payload, "Update apply dry-run planned.");
    }

    ensure_real_mutation_gate_enabled()?;

    let started_at = unix_ts();
    conn.execute(
        r#"
        INSERT INTO update_operation_history
        (operation_id, ts, candidate_id, mode, status, message, rollback_attempted, rollback_status, execution_ticket)
        VALUES (?1, ?2, ?3, ?4, 'started', 'real execution started', 0, 'not_needed', ?5)
        "#,
        params![operation_id, started_at, args.candidate_id, mode, execution_ticket],
    )?;

    let completed_at = unix_ts();
    if args.simulate_failure {
        let (rollback_status, final_message) = simulated_failure_outcome(
            args.simulate_rollback_failure,
            "simulated execution failed; rollback succeeded",
            "simulated execution failed; rollback failed",
        );

        conn.execute(
            r#"
            UPDATE update_operation_history
            SET ts = ?1, status = 'failed', message = ?2, rollback_attempted = 1, rollback_status = ?3
            WHERE operation_id = ?4
            "#,
            params![completed_at, final_message, rollback_status, operation_id],
        )?;

        let payload = json!({
            "operation_id": operation_id,
            "mode": mode,
            "status": "update_failed",
            "mutation_boundary_reached": true,
            "rollback_attempted": true,
            "rollback_status": rollback_status,
            "execution_ticket": execution_ticket,
            "target": target,
            "message": final_message
        });
        if args.json {
            println!("{}", serde_json::to_string_pretty(&payload)?);
        }
        return Err(CliError::Integration(
            "update apply confirmed execution failed (simulated)".to_string(),
        ));
    }

    conn.execute(
        r#"
        UPDATE update_operation_history
        SET ts = ?1, status = 'succeeded', message = 'simulated real mutation succeeded', rollback_attempted = 0, rollback_status = 'not_needed'
        WHERE operation_id = ?2
        "#,
        params![completed_at, operation_id],
    )?;

    let payload = json!({
        "operation_id": operation_id,
        "mode": mode,
        "status": "update_success",
        "mutation_boundary_reached": true,
        "rollback_attempted": false,
        "execution_ticket": execution_ticket,
        "target": target,
        "message": "update apply confirmed execution succeeded (simulated)"
    });
    print_payload(args.json, payload, "Update apply confirmed execution succeeded.")
}

fn update_history(args: UpdateHistoryArgs) -> Result<(), CliError> {
    let limit = i64::from(args.limit.unwrap_or(100));
    let offset = i64::from(args.offset.unwrap_or(0));
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }
    if let Some(mode) = args.mode.as_deref() {
        match mode {
            "dry_run" | "confirmed_execution" => {}
            _ => {
                return Err(CliError::Usage(
                    "--mode must be one of: dry_run, confirmed_execution".to_string(),
                ));
            }
        }
    }
    if let Some(status) = args.status.as_deref() {
        match status {
            "started" | "succeeded" | "failed" => {}
            _ => {
                return Err(CliError::Usage(
                    "--status must be one of: started, succeeded, failed".to_string(),
                ));
            }
        }
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let mut sql = String::from(
        r#"
        SELECT id, operation_id, ts, candidate_id, mode, status, message, rollback_attempted, rollback_status, execution_ticket
        FROM update_operation_history
        "#,
    );
    let mut clauses: Vec<String> = Vec::new();
    let mut values: Vec<Value> = Vec::new();

    if let Some(status) = args.status.clone() {
        clauses.push("status = ?".to_string());
        values.push(Value::Text(status));
    }
    if let Some(mode) = args.mode.clone() {
        clauses.push("mode = ?".to_string());
        values.push(Value::Text(mode));
    }
    if let Some(candidate_id) = args.candidate_id {
        clauses.push("candidate_id = ?".to_string());
        values.push(Value::Integer(candidate_id));
    }
    if !clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&clauses.join(" AND "));
    }
    sql.push_str(" ORDER BY id DESC LIMIT ? OFFSET ?");
    values.push(Value::Integer(limit));
    values.push(Value::Integer(offset));

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params_from_iter(values.iter()), |row| {
        Ok(json!({
            "id": row.get::<_, i64>(0)?,
            "operation_id": row.get::<_, String>(1)?,
            "timestamp": row.get::<_, i64>(2)?,
            "candidate_id": row.get::<_, i64>(3)?,
            "mode": row.get::<_, String>(4)?,
            "status": row.get::<_, String>(5)?,
            "message": row.get::<_, String>(6)?,
            "rollback_attempted": row.get::<_, i64>(7)? == 1,
            "rollback_status": row.get::<_, String>(8)?,
            "execution_ticket": row.get::<_, String>(9)?
        }))
    })?;
    let payload: Vec<serde_json::Value> = rows.collect::<Result<Vec<_>, _>>()?;
    if payload.is_empty() {
        return print_payload(args.json, json!([]), "No update history entries found.");
    }
    print_payload(args.json, json!(payload), "Update history listed.")
}

fn cleanup_apply(args: CleanupApplyArgs) -> Result<(), CliError> {
    validate_cleanup_apply_flags(&args)?;

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let selected = conn.query_row(
        r#"
        SELECT id, name, version, publisher, is_active
        FROM software_inventory
        WHERE id = ?1
        "#,
        params![args.software_id],
        |row| {
            Ok(json!({
                "software_id": row.get::<_, i64>(0)?,
                "software_name": row.get::<_, String>(1)?,
                "software_version": row.get::<_, String>(2)?,
                "software_publisher": row.get::<_, String>(3)?,
                "is_active": row.get::<_, i64>(4)? == 1
            }))
        },
    );

    let target = match selected {
        Ok(v) => v,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            return Err(CliError::Usage(format!(
                "software not found for --software-id {}",
                args.software_id
            )));
        }
        Err(e) => return Err(CliError::Db(e)),
    };

    let operation_id = next_operation_id("cleanup", args.software_id);
    let mode = if args.dry_run {
        "dry_run"
    } else {
        "confirmed_execution"
    };
    let execution_ticket = args.execution_ticket.clone().unwrap_or_default();

    if args.dry_run {
        let payload = json!({
            "operation_id": operation_id,
            "mode": mode,
            "status": "planned",
            "mutation_boundary_reached": false,
            "rollback_attempted": false,
            "execution_ticket": "",
            "target": target,
            "message": "cleanup apply dry-run planned successfully"
        });
        return print_payload(args.json, payload, "Cleanup apply dry-run planned.");
    }

    ensure_real_mutation_gate_enabled()?;

    let started_at = unix_ts();
    conn.execute(
        r#"
        INSERT INTO cleanup_operation_history
        (operation_id, ts, software_id, mode, status, message, rollback_attempted, rollback_status, execution_ticket)
        VALUES (?1, ?2, ?3, ?4, 'started', 'real execution started', 0, 'not_needed', ?5)
        "#,
        params![operation_id, started_at, args.software_id, mode, execution_ticket],
    )?;

    let completed_at = unix_ts();
    if args.simulate_failure {
        let (rollback_status, final_message) = simulated_failure_outcome(
            args.simulate_rollback_failure,
            "simulated cleanup failed; rollback succeeded",
            "simulated cleanup failed; rollback failed",
        );

        conn.execute(
            r#"
            UPDATE cleanup_operation_history
            SET ts = ?1, status = 'failed', message = ?2, rollback_attempted = 1, rollback_status = ?3
            WHERE operation_id = ?4
            "#,
            params![completed_at, final_message, rollback_status, operation_id],
        )?;

        let payload = json!({
            "operation_id": operation_id,
            "mode": mode,
            "status": "cleanup_failed",
            "mutation_boundary_reached": true,
            "rollback_attempted": true,
            "rollback_status": rollback_status,
            "execution_ticket": execution_ticket,
            "target": target,
            "message": final_message
        });
        if args.json {
            println!("{}", serde_json::to_string_pretty(&payload)?);
        }
        return Err(CliError::Integration(
            "cleanup apply confirmed execution failed (simulated)".to_string(),
        ));
    }

    conn.execute(
        r#"
        UPDATE cleanup_operation_history
        SET ts = ?1, status = 'succeeded', message = 'simulated cleanup mutation succeeded', rollback_attempted = 0, rollback_status = 'not_needed'
        WHERE operation_id = ?2
        "#,
        params![completed_at, operation_id],
    )?;

    let payload = json!({
        "operation_id": operation_id,
        "mode": mode,
        "status": "cleanup_success",
        "mutation_boundary_reached": true,
        "rollback_attempted": false,
        "execution_ticket": execution_ticket,
        "target": target,
        "message": "cleanup apply confirmed execution succeeded (simulated)"
    });
    print_payload(args.json, payload, "Cleanup apply confirmed execution succeeded.")
}

fn cleanup_history(args: CleanupHistoryArgs) -> Result<(), CliError> {
    let limit = i64::from(args.limit.unwrap_or(100));
    let offset = i64::from(args.offset.unwrap_or(0));
    validate_cleanup_history_filters(&args, limit)?;

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let mut sql = String::from(
        r#"
        SELECT id, operation_id, ts, software_id, mode, status, message, rollback_attempted, rollback_status, execution_ticket
        FROM cleanup_operation_history
        "#,
    );
    let mut clauses: Vec<String> = Vec::new();
    let mut values: Vec<Value> = Vec::new();

    if let Some(status) = args.status.clone() {
        clauses.push("status = ?".to_string());
        values.push(Value::Text(status));
    }
    if let Some(mode) = args.mode.clone() {
        clauses.push("mode = ?".to_string());
        values.push(Value::Text(mode));
    }
    if let Some(software_id) = args.software_id {
        clauses.push("software_id = ?".to_string());
        values.push(Value::Integer(software_id));
    }
    if let Some(execution_ticket) = args.execution_ticket.clone() {
        clauses.push("execution_ticket = ?".to_string());
        values.push(Value::Text(execution_ticket));
    }
    if let Some(rollback_status) = args.rollback_status.clone() {
        clauses.push("rollback_status = ?".to_string());
        values.push(Value::Text(rollback_status));
    }
    if let Some(contains) = args.contains.clone() {
        clauses.push("(operation_id LIKE ? OR message LIKE ? OR execution_ticket LIKE ?)".to_string());
        let like = format!("%{contains}%");
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like));
    }
    if !clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&clauses.join(" AND "));
    }
    sql.push_str(" ORDER BY id DESC LIMIT ? OFFSET ?");
    values.push(Value::Integer(limit));
    values.push(Value::Integer(offset));

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params_from_iter(values.iter()), |row| {
        Ok(json!({
            "id": row.get::<_, i64>(0)?,
            "operation_id": row.get::<_, String>(1)?,
            "timestamp": row.get::<_, i64>(2)?,
            "software_id": row.get::<_, i64>(3)?,
            "mode": row.get::<_, String>(4)?,
            "status": row.get::<_, String>(5)?,
            "message": row.get::<_, String>(6)?,
            "rollback_attempted": row.get::<_, i64>(7)? == 1,
            "rollback_status": row.get::<_, String>(8)?,
            "execution_ticket": row.get::<_, String>(9)?
        }))
    })?;
    let payload: Vec<serde_json::Value> = rows.collect::<Result<Vec<_>, _>>()?;
    if payload.is_empty() {
        return print_payload(args.json, json!([]), "No cleanup history entries found.");
    }
    print_payload(args.json, json!(payload), "Cleanup history listed.")
}

fn validate_cleanup_history_filters(args: &CleanupHistoryArgs, limit: i64) -> Result<(), CliError> {
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }
    if let Some(mode) = args.mode.as_deref() {
        match mode {
            "dry_run" | "confirmed_execution" => {}
            _ => {
                return Err(CliError::Usage(
                    "--mode must be one of: dry_run, confirmed_execution".to_string(),
                ));
            }
        }
    }
    if let Some(status) = args.status.as_deref() {
        match status {
            "started" | "succeeded" | "failed" => {}
            _ => {
                return Err(CliError::Usage(
                    "--status must be one of: started, succeeded, failed".to_string(),
                ));
            }
        }
    }
    if let Some(rollback_status) = args.rollback_status.as_deref() {
        match rollback_status {
            "not_needed" | "not_recorded" | "success" | "failed" => {}
            _ => {
                return Err(CliError::Usage(
                    "--rollback-status must be one of: not_needed, not_recorded, success, failed".to_string(),
                ));
            }
        }
    }
    Ok(())
}

fn validate_ai_repair_plan_args(args: &AiRepairPlanArgs) -> Result<(), CliError> {
    if args.software.trim().is_empty() {
        return Err(CliError::Usage("--software is required".to_string()));
    }
    if args.issue.trim().is_empty() {
        return Err(CliError::Usage("--issue is required".to_string()));
    }
    Ok(())
}

fn validate_ai_recommend_args(args: &AiRecommendArgs) -> Result<(), CliError> {
    if args.goal.trim().is_empty() {
        return Err(CliError::Usage("--goal is required".to_string()));
    }
    Ok(())
}

fn ai_analyze(args: AiAnalyzeArgs) -> Result<(), CliError> {
    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let total_active: i64 = conn.query_row(
        "SELECT COUNT(1) FROM software_inventory WHERE is_active = 1",
        [],
        |r| r.get(0),
    )?;
    let total_publishers: i64 = conn.query_row(
        "SELECT COUNT(DISTINCT publisher) FROM software_inventory WHERE is_active = 1",
        [],
        |r| r.get(0),
    )?;

    let mut publisher_stmt = conn.prepare(
        r#"
        SELECT publisher, COUNT(1) AS c
        FROM software_inventory
        WHERE is_active = 1
        GROUP BY publisher
        ORDER BY c DESC, publisher ASC
        LIMIT 5
        "#,
    )?;
    let top_publishers: Vec<serde_json::Value> = publisher_stmt
        .query_map([], |row| {
            Ok(json!({
                "publisher": row.get::<_, String>(0)?,
                "count": row.get::<_, i64>(1)?
            }))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    let mut redundancy_stmt = conn.prepare(
        r#"
        SELECT name, COUNT(1) AS c
        FROM software_inventory
        WHERE is_active = 1
        GROUP BY name
        HAVING c >= 2
        ORDER BY c DESC, name ASC
        LIMIT 20
        "#,
    )?;
    let potential_redundancy: Vec<serde_json::Value> = redundancy_stmt
        .query_map([], |row| {
            Ok(json!({
                "software_name": row.get::<_, String>(0)?,
                "instances": row.get::<_, i64>(1)?
            }))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    let mut recommendations = Vec::new();
    if !potential_redundancy.is_empty() {
        recommendations.push(json!({
            "action": "review_duplicates",
            "reason": "multiple active entries detected for same software name",
            "risk_level": "low",
            "confidence": 80
        }));
    }
    if total_active > 100 {
        recommendations.push(json!({
            "action": "prioritize_update_groups",
            "reason": "large active software set increases maintenance overhead",
            "risk_level": "medium",
            "confidence": 74
        }));
    }
    if recommendations.is_empty() {
        recommendations.push(json!({
            "action": "maintain_current_baseline",
            "reason": "no immediate redundancy pressure detected",
            "risk_level": "low",
            "confidence": 68
        }));
    }

    let confidence = if potential_redundancy.is_empty() { 72 } else { 81 };
    let analysis_id = next_operation_id("ai-analyze", unix_ts());
    let created_at = unix_ts();
    conn.execute(
        r#"
        INSERT INTO ai_analyze_history
        (analysis_id, ts, total_active, total_publishers, confidence, summary_json)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#,
        params![
            analysis_id,
            created_at,
            total_active,
            total_publishers,
            confidence,
            serde_json::to_string(&json!({
                "top_publishers": top_publishers,
                "potential_redundancy": potential_redundancy,
                "recommendations": recommendations
            }))?
        ],
    )?;

    let payload = json!({
        "analysis_id": analysis_id,
        "summary": {
            "total_active": total_active,
            "total_publishers": total_publishers
        },
        "categories": {
            "top_publishers": top_publishers
        },
        "potential_redundancy": potential_redundancy,
        "recommendations": recommendations,
        "confidence": confidence,
        "mode": "plan_only",
        "created_at": created_at
    });
    if args.verbose && !args.json {
        println!("AI analyze generated in plan-only mode.");
    }
    print_payload(args.json, payload, "AI analyze generated.")
}

fn ai_recommend(args: AiRecommendArgs) -> Result<(), CliError> {
    validate_ai_recommend_args(&args)?;

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let goal = args.goal.trim();
    let goal_l = goal.to_lowercase();
    let (recommended_software, risk_level, confidence, reason) = if goal_l.contains("video")
        || goal_l.contains("剪辑")
    {
        (
            vec![
                json!({"name": "DaVinci Resolve", "category": "editor"}),
                json!({"name": "OBS Studio", "category": "capture"}),
                json!({"name": "HandBrake", "category": "transcode"})
            ],
            "medium",
            79,
            "goal matched media production workflow template",
        )
    } else if goal_l.contains("rust") || goal_l.contains("开发") || goal_l.contains("program") {
        (
            vec![
                json!({"name": "RustRover", "category": "ide"}),
                json!({"name": "Visual Studio Code", "category": "editor"}),
                json!({"name": "Docker Desktop", "category": "runtime"})
            ],
            "low",
            77,
            "goal matched software development workflow template",
        )
    } else {
        (
            vec![
                json!({"name": "Visual Studio Code", "category": "general"}),
                json!({"name": "7-Zip", "category": "utility"}),
                json!({"name": "PowerToys", "category": "productivity"})
            ],
            "low",
            66,
            "goal matched generic productivity fallback template",
        )
    };

    let recommendation_id = next_operation_id("ai-recommend", unix_ts());
    let created_at = unix_ts();
    conn.execute(
        r#"
        INSERT INTO ai_recommend_history
        (recommendation_id, ts, goal, risk_level, confidence, reason, recommendation_json)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        "#,
        params![
            recommendation_id,
            created_at,
            goal,
            risk_level,
            confidence,
            reason,
            serde_json::to_string(&recommended_software)?
        ],
    )?;

    let payload = json!({
        "recommendation_id": recommendation_id,
        "goal": goal,
        "recommended_software": recommended_software,
        "reason": reason,
        "confidence": confidence,
        "risk_level": risk_level,
        "mode": "plan_only",
        "created_at": created_at
    });
    if args.verbose && !args.json {
        println!("AI recommend generated in plan-only mode.");
    }
    print_payload(args.json, payload, "AI recommendation generated.")
}

fn ai_repair_plan(args: AiRepairPlanArgs) -> Result<(), CliError> {
    validate_ai_repair_plan_args(&args)?;

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let software_like = format!("%{}%", args.software.trim());
    let target = conn
        .query_row(
            r#"
            SELECT id, name, version, publisher, is_active
            FROM software_inventory
            WHERE name LIKE ?1
            ORDER BY is_active DESC, last_seen_at DESC, id DESC
            LIMIT 1
            "#,
            params![software_like],
            |row| {
                Ok(json!({
                    "software_id": row.get::<_, i64>(0)?,
                    "software_name": row.get::<_, String>(1)?,
                    "software_version": row.get::<_, String>(2)?,
                    "software_publisher": row.get::<_, String>(3)?,
                    "is_active": row.get::<_, i64>(4)? == 1
                }))
            },
        )
        .optional()?;

    let issue_l = args.issue.to_lowercase();
    let risk_level = if issue_l.contains("crash")
        || issue_l.contains("data")
        || issue_l.contains("corrupt")
    {
        "high"
    } else {
        "medium"
    };
    let confidence = if target.is_some() { 78 } else { 64 };
    let reason = if target.is_some() {
        "plan generated from issue text and matched local software inventory"
    } else {
        "plan generated from issue text without exact local software match"
    };

    let plan_steps = vec![
        json!({"step": 1, "action": "Collect logs and reproduce issue", "reason": "confirm failure pattern before changes"}),
        json!({"step": 2, "action": "Check current version and known compatibility notes", "reason": "avoid blind reinstall"}),
        json!({"step": 3, "action": "Run safe repair action (cache reset/settings reset)", "reason": "prefer low-risk remediation first"}),
        json!({"step": 4, "action": "If unresolved, plan controlled reinstall with rollback point", "reason": "contain impact and preserve recovery path"})
    ];

    let created_at = unix_ts();
    let plan_id = next_operation_id("repair-plan", created_at);
    let target_software_id = target
        .as_ref()
        .and_then(|v| v.get("software_id"))
        .and_then(|v| v.as_i64());
    conn.execute(
        r#"
        INSERT INTO ai_repair_plan_history
        (plan_id, ts, software_query, issue_text, target_software_id, risk_level, confidence, reason, plan_json)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        "#,
        params![
            plan_id,
            created_at,
            args.software.trim(),
            args.issue.trim(),
            target_software_id,
            risk_level,
            confidence,
            reason,
            serde_json::to_string(&plan_steps)?
        ],
    )?;

    let payload = json!({
        "plan_id": plan_id,
        "target_software": args.software.trim(),
        "matched_target": target,
        "issue": args.issue.trim(),
        "plan_steps": plan_steps,
        "rollback_hint": "create restore point or backup config before reinstall",
        "risk_level": risk_level,
        "confidence": confidence,
        "reason": reason,
        "mode": "plan_only",
        "created_at": created_at
    });
    if args.verbose && !args.json {
        println!("AI repair plan generated in plan-only mode.");
    }
    print_payload(args.json, payload, "AI repair plan generated.")
}

fn ui_search(args: UiSearchArgs) -> Result<(), CliError> {
    let limit = i64::from(args.limit.unwrap_or(8));
    let query = validate_ui_search_args(&args.q, limit)?;

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;
    let like = format!("%{query}%");

    let mut groups: Vec<serde_json::Value> = Vec::new();

    let mut software_stmt = conn.prepare(
        r#"
        SELECT id, name, version, publisher
        FROM software_inventory
        WHERE is_active = 1 AND (name LIKE ?1 OR publisher LIKE ?1)
        ORDER BY last_seen_at DESC, id DESC
        LIMIT ?2
        "#,
    )?;
    let software_items: Vec<serde_json::Value> = software_stmt
        .query_map(params![&like, limit], |row| {
            Ok(json!({
                "title": row.get::<_, String>(1)?,
                "subtitle": format!("{} | {} | {}", row.get::<_, String>(2)?, row.get::<_, String>(3)?, "active"),
                "risk_level": "low",
                "confidence": 80,
                "action_id": format!("software.show:{}", row.get::<_, i64>(0)?)
            }))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    if !software_items.is_empty() {
        groups.push(json!({"type": "software", "items": software_items}));
    }

    let mut source_stmt = conn.prepare(
        r#"
        SELECT candidate_id, software_name, domain, confidence, url
        FROM source_registry
        WHERE status = 'active' AND (software_name LIKE ?1 OR domain LIKE ?1 OR url LIKE ?1)
        ORDER BY confidence DESC, candidate_id DESC
        LIMIT ?2
        "#,
    )?;
    let source_items: Vec<serde_json::Value> = source_stmt
        .query_map(params![&like, limit], |row| {
            Ok(json!({
                "title": row.get::<_, String>(1)?,
                "subtitle": format!("{} | {}", row.get::<_, String>(2)?, row.get::<_, String>(4)?),
                "risk_level": "low",
                "confidence": row.get::<_, i64>(3)?,
                "action_id": format!("source.registry:{}", row.get::<_, i64>(0)?)
            }))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    if !source_items.is_empty() {
        groups.push(json!({"type": "source", "items": source_items}));
    }

    let mut update_stmt = conn.prepare(
        r#"
        SELECT operation_id, status, message
        FROM update_operation_history
        WHERE operation_id LIKE ?1 OR message LIKE ?1
        ORDER BY id DESC
        LIMIT ?2
        "#,
    )?;
    let update_items: Vec<serde_json::Value> = update_stmt
        .query_map(params![&like, limit], |row| {
            let status = row.get::<_, String>(1)?;
            let (risk_level, confidence) = match status.as_str() {
                "failed" => ("high", 72),
                "succeeded" => ("low", 76),
                _ => ("medium", 60),
            };
            Ok(json!({
                "title": format!("Update {}", status),
                "subtitle": row.get::<_, String>(2)?,
                "risk_level": risk_level,
                "confidence": confidence,
                "action_id": format!("update.history:{}", row.get::<_, String>(0)?)
            }))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    if !update_items.is_empty() {
        groups.push(json!({"type": "update", "items": update_items}));
    }

    let mut download_stmt = conn.prepare(
        r#"
        SELECT job_id, status, package_id, message
        FROM download_job_history
        WHERE job_id LIKE ?1 OR package_id LIKE ?1 OR message LIKE ?1
        ORDER BY id DESC
        LIMIT ?2
        "#,
    )?;
    let download_items: Vec<serde_json::Value> = download_stmt
        .query_map(params![&like, limit], |row| {
            let status = row.get::<_, String>(1)?;
            let (risk_level, confidence) = match status.as_str() {
                "failed" => ("high", 70),
                "verified" => ("low", 78),
                _ => ("low", 65),
            };
            Ok(json!({
                "title": format!("Download {}", status),
                "subtitle": format!("{} | {}", row.get::<_, String>(2)?, row.get::<_, String>(3)?),
                "risk_level": risk_level,
                "confidence": confidence,
                "action_id": format!("download.show:{}", row.get::<_, String>(0)?)
            }))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    if !download_items.is_empty() {
        groups.push(json!({"type": "download", "items": download_items}));
    }

    let mut ai_items: Vec<serde_json::Value> = Vec::new();
    let mut ai_repair_stmt = conn.prepare(
        r#"
        SELECT plan_id, software_query, issue_text, risk_level, confidence
        FROM ai_repair_plan_history
        WHERE software_query LIKE ?1 OR issue_text LIKE ?1 OR reason LIKE ?1
        ORDER BY id DESC
        LIMIT ?2
        "#,
    )?;
    ai_items.extend(
        ai_repair_stmt
            .query_map(params![&like, limit], |row| {
                Ok(json!({
                    "title": format!("AI Repair Plan {}", row.get::<_, String>(1)?),
                    "subtitle": row.get::<_, String>(2)?,
                    "risk_level": row.get::<_, String>(3)?,
                    "confidence": row.get::<_, i64>(4)?,
                    "action_id": format!("ai.repair-plan:{}", row.get::<_, String>(0)?)
                }))
            })?
            .collect::<Result<Vec<_>, _>>()?,
    );

    let mut ai_recommend_stmt = conn.prepare(
        r#"
        SELECT recommendation_id, goal, reason, risk_level, confidence
        FROM ai_recommend_history
        WHERE goal LIKE ?1 OR reason LIKE ?1
        ORDER BY id DESC
        LIMIT ?2
        "#,
    )?;
    ai_items.extend(
        ai_recommend_stmt
            .query_map(params![&like, limit], |row| {
                Ok(json!({
                    "title": format!("AI Recommend {}", row.get::<_, String>(1)?),
                    "subtitle": row.get::<_, String>(2)?,
                    "risk_level": row.get::<_, String>(3)?,
                    "confidence": row.get::<_, i64>(4)?,
                    "action_id": format!("ai.recommend:{}", row.get::<_, String>(0)?)
                }))
            })?
            .collect::<Result<Vec<_>, _>>()?,
    );
    if !ai_items.is_empty() {
        groups.push(json!({"type": "ai", "items": ai_items}));
    }

    let payload = json!({
        "query": query,
        "groups": groups
    });
    print_payload(args.json, payload, "UI search completed.")
}

fn ui_action_run(args: UiActionRunArgs) -> Result<(), CliError> {
    let action_id = validate_ui_action_id(&args.id)?;
    let risk_level = if action_id.starts_with("cleanup.")
        || action_id.starts_with("update.")
        || action_id.starts_with("download.retry")
    {
        "high"
    } else if action_id.starts_with("download.") || action_id.starts_with("source.") {
        "medium"
    } else {
        "low"
    };

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;
    let event_id = next_operation_id("ui-action", unix_ts());
    let now = unix_ts();

    if risk_level == "high" && !args.confirm {
        conn.execute(
            r#"
            INSERT INTO ui_action_history (event_id, ts, action_id, risk_level, status, message)
            VALUES (?1, ?2, ?3, ?4, 'blocked', 'action requires explicit confirmation')
            "#,
            params![event_id, now, action_id, risk_level],
        )?;
        return Err(CliError::Security(
            "action requires explicit confirmation".to_string(),
        ));
    }

    conn.execute(
        r#"
        INSERT INTO ui_action_history (event_id, ts, action_id, risk_level, status, message)
        VALUES (?1, ?2, ?3, ?4, 'executed_simulated', 'ui action executed in simulated mode')
        "#,
        params![event_id, now, action_id, risk_level],
    )?;

    let payload = json!({
        "event_id": event_id,
        "action_id": action_id,
        "risk_level": risk_level,
        "status": "executed_simulated",
        "message": "ui action executed in simulated mode"
    });
    print_payload(args.json, payload, "UI action executed in simulated mode.")
}

fn validate_ui_search_args<'a>(q: &'a str, limit: i64) -> Result<&'a str, CliError> {
    let query = q.trim();
    if query.is_empty() {
        return Err(CliError::Usage("--q is required".to_string()));
    }
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }
    Ok(query)
}

fn validate_ui_action_id(id: &str) -> Result<&str, CliError> {
    let action_id = id.trim();
    if action_id.is_empty() {
        return Err(CliError::Usage("--id is required".to_string()));
    }
    Ok(action_id)
}

fn validate_job_type(job_type: &str) -> Result<(), CliError> {
    match job_type {
        "discover.scan"
        | "source.suggest"
        | "update.check"
        | "update.apply"
        | "cleanup.apply"
        | "download.fetch"
        | "download.verify"
        | "ai.analyze"
        | "ai.recommend"
        | "ai.repair-plan" => Ok(()),
        _ => Err(CliError::Usage("unknown job_type".to_string())),
    }
}

fn validate_job_status(status: &str) -> Result<(), CliError> {
    match status {
        "queued" | "running" | "success" | "failed" | "retrying" | "deadletter" => Ok(()),
        _ => Err(CliError::Usage(
            "--status must be one of: queued, running, success, failed, retrying, deadletter"
                .to_string(),
        )),
    }
}

fn job_submit(args: JobSubmitArgs) -> Result<(), CliError> {
    validate_job_type(args.job_type.trim())?;
    let payload_value: serde_json::Value = serde_json::from_str(args.payload.trim())
        .map_err(|e| CliError::Usage(format!("--payload must be valid JSON: {e}")))?;
    let priority = args.priority.unwrap_or(50);
    if !(1..=100).contains(&priority) {
        return Err(CliError::Usage("--priority must be in [1,100]".to_string()));
    }
    if args.simulate_failed && args.simulate_deadletter {
        return Err(CliError::Usage(
            "only one of --simulate-failed or --simulate-deadletter can be set".to_string(),
        ));
    }

    let now = unix_ts();
    let scheduled_at = args.schedule_at.unwrap_or(now);
    let max_attempts = 3_i64;
    let (status, attempt_count, last_error) = if args.simulate_deadletter {
        (
            "deadletter",
            max_attempts,
            "simulated deadletter injected at submit",
        )
    } else if args.simulate_failed {
        ("failed", 1_i64, "simulated failure injected at submit")
    } else {
        ("queued", 0_i64, "")
    };

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;
    conn.execute(
        r#"
        INSERT INTO job_queue
        (job_type, payload_json, status, priority, attempt_count, max_attempts, scheduled_at, created_at, started_at, finished_at, last_error)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, NULL, ?9)
        "#,
        params![
            args.job_type.trim(),
            serde_json::to_string(&payload_value)?,
            status,
            priority,
            attempt_count,
            max_attempts,
            scheduled_at,
            now,
            last_error
        ],
    )?;
    let job_id = conn.last_insert_rowid();

    let payload = json!({
        "job_id": job_id,
        "job_type": args.job_type.trim(),
        "status": status,
        "priority": priority,
        "scheduled_at": scheduled_at
    });
    print_payload(args.json, payload, "Job submitted.")
}

fn job_list(args: JobListArgs) -> Result<(), CliError> {
    let limit = i64::from(args.limit.unwrap_or(100));
    let offset = i64::from(args.offset.unwrap_or(0));
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }
    if let Some(status) = args.status.as_deref() {
        validate_job_status(status)?;
    }
    if let Some(job_type) = args.job_type.as_deref() {
        validate_job_type(job_type)?;
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let mut sql = String::from(
        "SELECT id, job_type, status, priority, attempt_count, max_attempts, scheduled_at, created_at FROM job_queue",
    );
    let mut clauses: Vec<String> = Vec::new();
    let mut values: Vec<Value> = Vec::new();
    if let Some(status) = args.status {
        clauses.push("status = ?".to_string());
        values.push(Value::Text(status));
    }
    if let Some(job_type) = args.job_type {
        clauses.push("job_type = ?".to_string());
        values.push(Value::Text(job_type));
    }
    if !clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&clauses.join(" AND "));
    }
    sql.push_str(" ORDER BY id DESC LIMIT ? OFFSET ?");
    values.push(Value::Integer(limit));
    values.push(Value::Integer(offset));

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params_from_iter(values.iter()), |row| {
        Ok(json!({
            "id": row.get::<_, i64>(0)?,
            "job_type": row.get::<_, String>(1)?,
            "status": row.get::<_, String>(2)?,
            "priority": row.get::<_, i64>(3)?,
            "attempt_count": row.get::<_, i64>(4)?,
            "max_attempts": row.get::<_, i64>(5)?,
            "scheduled_at": row.get::<_, i64>(6)?,
            "created_at": row.get::<_, i64>(7)?
        }))
    })?;
    let payload: Vec<serde_json::Value> = rows.collect::<Result<Vec<_>, _>>()?;
    if payload.is_empty() {
        return print_payload(args.json, json!([]), "No jobs found.");
    }
    print_payload(args.json, json!(payload), "Jobs listed.")
}

fn job_retry(args: JobRetryArgs) -> Result<(), CliError> {
    if args.id <= 0 {
        return Err(CliError::Usage("--id must be >= 1".to_string()));
    }
    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let found = conn
        .query_row(
            "SELECT status, attempt_count, max_attempts FROM job_queue WHERE id = ?1",
            params![args.id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                ))
            },
        )
        .optional()?;
    let (old_status, attempt_count, max_attempts) = match found {
        Some(v) => v,
        None => return Err(CliError::Usage(format!("job {} not found", args.id))),
    };
    if old_status != "failed" && old_status != "deadletter" {
        return Err(CliError::Usage(
            "job status does not allow manual retry".to_string(),
        ));
    }

    let (new_status, next_attempt_count) = if attempt_count >= max_attempts {
        ("deadletter", attempt_count)
    } else {
        ("queued", attempt_count + 1)
    };
    let now = unix_ts();
    let message = if new_status == "queued" {
        ""
    } else {
        "max attempts reached; kept in deadletter"
    };
    conn.execute(
        r#"
        UPDATE job_queue
        SET status = ?1, attempt_count = ?2, scheduled_at = ?3, last_error = ?4
        WHERE id = ?5
        "#,
        params![new_status, next_attempt_count, now, message, args.id],
    )?;

    let payload = json!({
        "job_id": args.id,
        "old_status": old_status,
        "new_status": new_status,
        "attempt_count": next_attempt_count
    });
    print_payload(args.json, payload, "Job retry processed.")
}

fn job_deadletter_list(args: JobDeadletterListArgs) -> Result<(), CliError> {
    job_list(JobListArgs {
        status: Some("deadletter".to_string()),
        job_type: None,
        limit: args.limit,
        offset: args.offset,
        json: args.json,
    })
}

fn job_replay_deadletter(args: JobReplayDeadletterArgs) -> Result<(), CliError> {
    let limit = i64::from(args.limit.unwrap_or(10));
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }
    if let Some(id) = args.id {
        if id <= 0 {
            return Err(CliError::Usage("--id must be >= 1".to_string()));
        }
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let targets: Vec<i64> = if let Some(id) = args.id {
        let status = conn
            .query_row(
                "SELECT status FROM job_queue WHERE id = ?1",
                params![id],
                |row| row.get::<_, String>(0),
            )
            .optional()?;
        match status {
            None => return Err(CliError::Usage(format!("job {} not found", id))),
            Some(s) if s != "deadletter" => {
                return Err(CliError::Usage(
                    "job status does not allow deadletter replay".to_string(),
                ));
            }
            Some(_) => vec![id],
        }
    } else {
        let mut stmt = conn.prepare(
            "SELECT id FROM job_queue WHERE status = 'deadletter' ORDER BY id DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit], |row| row.get::<_, i64>(0))?;
        let ids = rows.collect::<Result<Vec<_>, _>>()?;
        ids
    };

    let mut updated = 0_i64;
    let now = unix_ts();
    for id in &targets {
        updated += conn.execute(
            r#"
            UPDATE job_queue
            SET status = 'queued',
                attempt_count = 0,
                scheduled_at = ?1,
                last_error = ''
            WHERE id = ?2 AND status = 'deadletter'
            "#,
            params![now, id],
        )? as i64;
    }

    let payload = json!({
        "matched": targets.len(),
        "updated": updated,
        "job_ids": targets
    });
    print_payload(args.json, payload, "Deadletter jobs replayed.")
}

#[derive(Debug, Clone)]
struct ClaimedJob {
    id: i64,
    job_type: String,
    payload_json: String,
    priority: i64,
    attempt_count: i64,
    max_attempts: i64,
    scheduled_at: i64,
}

fn validate_job_worker_run_args(args: &JobWorkerRunArgs) -> Result<(), CliError> {
    if !args.once {
        return Err(CliError::Usage(
            "--once is currently required for job worker-run in this phase".to_string(),
        ));
    }
    Ok(())
}

fn compute_worker_retry_schedule(
    ok: bool,
    deadlettered: bool,
    attempt_after: i64,
    finished_at: i64,
    scheduled_at_before: i64,
) -> (i64, i64) {
    if ok {
        return (0, scheduled_at_before);
    }
    if deadlettered {
        return (0, scheduled_at_before);
    }
    let backoff_seconds = 5_i64 * attempt_after;
    let next_scheduled_at = finished_at.saturating_add(backoff_seconds);
    (backoff_seconds, next_scheduled_at)
}

fn claim_next_queued_job(conn: &mut Connection, now: i64) -> Result<Option<ClaimedJob>, CliError> {
    let tx = conn.transaction()?;
    let picked = tx
        .query_row(
            r#"
            SELECT id, job_type, payload_json, priority, attempt_count, max_attempts, scheduled_at
            FROM job_queue
            WHERE status = 'queued' AND scheduled_at <= ?1
            ORDER BY priority DESC, scheduled_at ASC, id ASC
            LIMIT 1
            "#,
            params![now],
            |row| {
                Ok(ClaimedJob {
                    id: row.get::<_, i64>(0)?,
                    job_type: row.get::<_, String>(1)?,
                    payload_json: row.get::<_, String>(2)?,
                    priority: row.get::<_, i64>(3)?,
                    attempt_count: row.get::<_, i64>(4)?,
                    max_attempts: row.get::<_, i64>(5)?,
                    scheduled_at: row.get::<_, i64>(6)?,
                })
            },
        )
        .optional()?;

    if let Some(job) = &picked {
        let changed = tx.execute(
            r#"
            UPDATE job_queue
            SET status = 'running',
                started_at = ?1,
                finished_at = NULL,
                last_error = ''
            WHERE id = ?2 AND status = 'queued'
            "#,
            params![now, job.id],
        )?;
        if changed != 1 {
            tx.rollback()?;
            return Ok(None);
        }
    }

    tx.commit()?;
    Ok(picked)
}

fn execute_worker_job(job: &ClaimedJob) -> (bool, String) {
    match job.job_type.as_str() {
        "download.fetch" => (true, "worker executed download.fetch (simulated)".to_string()),
        "download.verify" => {
            let parsed = serde_json::from_str::<serde_json::Value>(&job.payload_json);
            match parsed {
                Err(e) => (
                    false,
                    format!("download.verify payload parse failed: {e}"),
                ),
                Ok(payload) => {
                    let has_job_id = payload.get("job_id").and_then(|v| v.as_str()).is_some();
                    let simulate_failure = payload
                        .get("simulate_failure")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);
                    if !has_job_id {
                        (false, "download.verify payload missing job_id".to_string())
                    } else if simulate_failure {
                        (
                            false,
                            "worker simulated failure for download.verify".to_string(),
                        )
                    } else {
                        (true, "worker executed download.verify (simulated)".to_string())
                    }
                }
            }
        }
        _ => (
            false,
            format!("worker unsupported job_type: {}", job.job_type),
        ),
    }
}

fn job_worker_run(args: JobWorkerRunArgs) -> Result<(), CliError> {
    validate_job_worker_run_args(&args)?;

    let db_file = db_path()?;
    init_db(&db_file)?;
    let mut conn = Connection::open(db_file)?;
    let started_at = unix_ts();

    let claimed = claim_next_queued_job(&mut conn, started_at)?;
    let Some(job) = claimed else {
        let payload = json!({
            "mode": "once",
            "picked": false,
            "message": "no queued job available"
        });
        return print_payload(args.json, payload, "No queued job available.");
    };

    let (ok, message) = execute_worker_job(&job);
    let attempt_after = job.attempt_count + 1;
    let finished_at = unix_ts();
    let (new_status, deadlettered, error_text) = if ok {
        ("success", false, String::new())
    } else if attempt_after >= job.max_attempts {
        ("deadletter", true, message.clone())
    } else {
        ("queued", false, message.clone())
    };
    let (backoff_seconds, next_scheduled_at) = compute_worker_retry_schedule(
        ok,
        deadlettered,
        attempt_after,
        finished_at,
        job.scheduled_at,
    );
    conn.execute(
        r#"
        UPDATE job_queue
        SET status = ?1,
            attempt_count = ?2,
            scheduled_at = ?3,
            finished_at = ?4,
            last_error = ?5
        WHERE id = ?6
        "#,
        params![
            new_status,
            attempt_after,
            next_scheduled_at,
            finished_at,
            error_text,
            job.id
        ],
    )?;

    let payload = json!({
        "mode": "once",
        "picked": true,
        "job_id": job.id,
        "job_type": job.job_type,
        "priority": job.priority,
        "scheduled_at": job.scheduled_at,
        "old_status": "queued",
        "new_status": new_status,
        "attempt_count": attempt_after,
        "max_attempts": job.max_attempts,
        "next_scheduled_at": next_scheduled_at,
        "backoff_seconds": backoff_seconds,
        "started_at": started_at,
        "finished_at": finished_at,
        "duration_ms": (finished_at - started_at) * 1000,
        "deadlettered": deadlettered,
        "message": if ok { "worker execution completed" } else if deadlettered { "worker execution failed and moved to deadletter" } else { "worker execution failed" },
        "error": if ok { "" } else { message.as_str() }
    });
    print_payload(args.json, payload, "Worker run finished.")
}

fn repo_list(args: RepoListArgs) -> Result<(), CliError> {
    let limit = i64::from(args.limit.unwrap_or(100));
    let offset = i64::from(args.offset.unwrap_or(0));
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }
    if let Some(status) = args.status.as_deref() {
        match status {
            "active" | "disabled" => {}
            _ => {
                return Err(CliError::Usage(
                    "--status must be one of: active, disabled".to_string(),
                ));
            }
        }
    }
    if let Some(kind) = args.kind.as_deref() {
        match kind {
            "public" | "personal" => {}
            _ => {
                return Err(CliError::Usage(
                    "--kind must be one of: public, personal".to_string(),
                ));
            }
        }
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let mut sql = String::from(
        "SELECT id, repo_key, name, url, kind, status, priority, created_at, updated_at FROM repo_registry",
    );
    let mut clauses: Vec<String> = Vec::new();
    let mut values: Vec<Value> = Vec::new();

    if let Some(status) = args.status.clone() {
        clauses.push("status = ?".to_string());
        values.push(Value::Text(status));
    }
    if let Some(kind) = args.kind.clone() {
        clauses.push("kind = ?".to_string());
        values.push(Value::Text(kind));
    }
    if !clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&clauses.join(" AND "));
    }
    sql.push_str(" ORDER BY priority DESC, id ASC LIMIT ? OFFSET ?");
    values.push(Value::Integer(limit));
    values.push(Value::Integer(offset));

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params_from_iter(values.iter()), |row| {
        Ok(json!({
            "id": row.get::<_, i64>(0)?,
            "repo_key": row.get::<_, String>(1)?,
            "name": row.get::<_, String>(2)?,
            "url": row.get::<_, String>(3)?,
            "kind": row.get::<_, String>(4)?,
            "status": row.get::<_, String>(5)?,
            "priority": row.get::<_, i64>(6)?,
            "created_at": row.get::<_, i64>(7)?,
            "updated_at": row.get::<_, i64>(8)?
        }))
    })?;
    let payload: Vec<serde_json::Value> = rows.collect::<Result<Vec<_>, _>>()?;
    if payload.is_empty() {
        return print_payload(args.json, json!([]), "No repositories found.");
    }
    print_payload(args.json, json!(payload), "Repositories listed.")
}

fn repo_add(args: RepoAddArgs) -> Result<(), CliError> {
    match args.kind.as_str() {
        "public" | "personal" => {}
        _ => {
            return Err(CliError::Usage(
                "--kind must be one of: public, personal".to_string(),
            ));
        }
    }
    if args.name.trim().is_empty() || args.url.trim().is_empty() {
        return Err(CliError::Usage("--name and --url are required".to_string()));
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let now = unix_ts();
    let repo_key = format!(
        "{}-{}-{}",
        args.kind,
        next_operation_id("repo", now).replace(':', "-"),
        args.name
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect::<String>()
            .to_lowercase()
    );
    let priority = args.priority.unwrap_or(if args.kind == "personal" { 200 } else { 100 });

    conn.execute(
        r#"
        INSERT INTO repo_registry
        (repo_key, name, url, kind, status, priority, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, 'active', ?5, ?6, ?6)
        "#,
        params![repo_key, args.name, args.url, args.kind, priority, now],
    )?;

    let payload = json!({
        "repo_key": repo_key,
        "name": args.name,
        "url": args.url,
        "kind": args.kind,
        "status": "active",
        "priority": priority
    });
    print_payload(args.json, payload, "Repository added.")
}

fn repo_remove(args: RepoRemoveArgs) -> Result<(), CliError> {
    if args.repo_key == "public-default" || args.repo_key == "personal-local" {
        return Err(CliError::Usage(
            "default repositories cannot be removed".to_string(),
        ));
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let changed = conn.execute(
        "DELETE FROM repo_registry WHERE repo_key = ?1",
        params![args.repo_key],
    )?;
    if changed == 0 {
        return Err(CliError::Usage(format!(
            "repo_key {} not found",
            args.repo_key
        )));
    }

    let payload = json!({
        "repo_key": args.repo_key,
        "removed": true
    });
    print_payload(args.json, payload, "Repository removed.")
}

fn repo_sync(args: RepoSyncArgs) -> Result<(), CliError> {
    let limit = i64::from(args.limit.unwrap_or(50));
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;
    let now = unix_ts();
    let sync_id = next_operation_id("repo-sync", now);

    let mut sql = String::from(
        "SELECT repo_key, name, url FROM repo_registry WHERE status = 'active'",
    );
    let mut values: Vec<Value> = Vec::new();
    if let Some(repo_key) = args.repo_key.clone() {
        sql.push_str(" AND repo_key = ?");
        values.push(Value::Text(repo_key));
    }
    sql.push_str(" ORDER BY priority DESC, id ASC LIMIT ?");
    values.push(Value::Integer(limit));

    let mut stmt = conn.prepare(&sql)?;
    let repos = stmt
        .query_map(params_from_iter(values.iter()), |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    let mut repo_results: Vec<serde_json::Value> = Vec::new();
    let mut synced = 0_i64;
    for (repo_key, name, url) in repos {
        let package_id = format!("{}.sample", repo_key.replace('-', "_"));
        let package_name = format!("Sample package from {name}");
        let version = "0.1.0".to_string();

        conn.execute(
            r#"
            INSERT INTO repo_package_index
            (repo_key, package_id, package_name, version, source_url, status, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, 'active', ?6)
            ON CONFLICT(repo_key, package_id) DO UPDATE SET
                package_name=excluded.package_name,
                version=excluded.version,
                source_url=excluded.source_url,
                status='active',
                updated_at=excluded.updated_at
            "#,
            params![repo_key, package_id, package_name, version, url, now],
        )?;

        conn.execute(
            r#"
            INSERT INTO repo_sync_history
            (sync_id, ts, repo_key, status, message, packages_upserted)
            VALUES (?1, ?2, ?3, 'succeeded', 'phase4 simulated sync succeeded', 1)
            "#,
            params![sync_id, now, repo_key],
        )?;

        repo_results.push(json!({
            "repo_key": repo_key,
            "packages_upserted": 1,
            "status": "succeeded"
        }));
        synced += 1;
    }

    let payload = json!({
        "sync_id": sync_id,
        "timestamp": now,
        "repos_synced": synced,
        "repos": repo_results
    });
    print_payload(args.json, payload, "Repository sync completed.")
}

fn package_search(args: PackageSearchArgs) -> Result<(), CliError> {
    let limit = i64::from(args.limit.unwrap_or(100));
    let offset = i64::from(args.offset.unwrap_or(0));
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let mut sql = String::from(
        "SELECT repo_key, package_id, package_name, version, source_url, status, updated_at FROM repo_package_index",
    );
    let mut clauses: Vec<String> = Vec::new();
    let mut values: Vec<Value> = Vec::new();

    clauses.push("status = 'active'".to_string());
    if let Some(repo_key) = args.repo_key.clone() {
        clauses.push("repo_key = ?".to_string());
        values.push(Value::Text(repo_key));
    }
    if let Some(contains) = args.contains.clone() {
        clauses.push("(package_id LIKE ? OR package_name LIKE ?)".to_string());
        let like = format!("%{contains}%");
        values.push(Value::Text(like.clone()));
        values.push(Value::Text(like));
    }
    if !clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&clauses.join(" AND "));
    }
    sql.push_str(" ORDER BY updated_at DESC, repo_key ASC LIMIT ? OFFSET ?");
    values.push(Value::Integer(limit));
    values.push(Value::Integer(offset));

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params_from_iter(values.iter()), |row| {
        Ok(json!({
            "repo_key": row.get::<_, String>(0)?,
            "package_id": row.get::<_, String>(1)?,
            "package_name": row.get::<_, String>(2)?,
            "version": row.get::<_, String>(3)?,
            "source_url": row.get::<_, String>(4)?,
            "status": row.get::<_, String>(5)?,
            "updated_at": row.get::<_, i64>(6)?
        }))
    })?;
    let payload: Vec<serde_json::Value> = rows.collect::<Result<Vec<_>, _>>()?;
    if payload.is_empty() {
        return print_payload(args.json, json!([]), "No packages found.");
    }
    print_payload(args.json, json!(payload), "Packages listed.")
}

fn download_start(args: DownloadStartArgs) -> Result<(), CliError> {
    if args.package_id.trim().is_empty() {
        return Err(CliError::Usage("--package-id is required".to_string()));
    }
    if !args.dry_run {
        return Err(CliError::Usage(
            "--dry-run is currently required for download start in this phase".to_string(),
        ));
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let pkg = conn.query_row(
        r#"
        SELECT repo_key, package_id, package_name, source_url, version
        FROM repo_package_index
        WHERE package_id = ?1 AND status = 'active'
        ORDER BY updated_at DESC, id DESC
        LIMIT 1
        "#,
        params![args.package_id],
        |row| {
            Ok(json!({
                "repo_key": row.get::<_, String>(0)?,
                "package_id": row.get::<_, String>(1)?,
                "package_name": row.get::<_, String>(2)?,
                "source_url": row.get::<_, String>(3)?,
                "version": row.get::<_, String>(4)?
            }))
        },
    );

    let target = match pkg {
        Ok(v) => v,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            return Err(CliError::Usage(format!(
                "active package not found for --package-id {}",
                args.package_id
            )));
        }
        Err(e) => return Err(CliError::Db(e)),
    };

    let now = unix_ts();
    let job_id = next_operation_id("download", now);
    conn.execute(
        r#"
        INSERT INTO download_job_history
        (job_id, ts, package_id, repo_key, source_url, mode, status, verification_status, hash_status, signature_status, source_policy_status, message)
        VALUES (?1, ?2, ?3, ?4, ?5, 'dry_run', 'planned', 'not_started', 'not_started', 'not_started', 'not_started', 'download dry-run planned')
        "#,
        params![
            job_id,
            now,
            args.package_id,
            target.get("repo_key").and_then(|v| v.as_str()).unwrap_or(""),
            target.get("source_url").and_then(|v| v.as_str()).unwrap_or("")
        ],
    )?;

    let payload = json!({
        "job_id": job_id,
        "mode": "dry_run",
        "status": "planned",
        "verification_status": "not_started",
        "hash_status": "not_started",
        "signature_status": "not_started",
        "source_policy_status": "not_started",
        "target": target,
        "message": "download start dry-run planned successfully"
    });
    print_payload(args.json, payload, "Download start dry-run planned.")
}

fn download_list(args: DownloadListArgs) -> Result<(), CliError> {
    let limit = i64::from(args.limit.unwrap_or(100));
    let offset = i64::from(args.offset.unwrap_or(0));
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }
    if let Some(status) = args.status.as_deref() {
        match status {
            "planned" | "queued" | "downloaded" | "verified" | "failed" => {}
            _ => {
                return Err(CliError::Usage(
                    "--status must be one of: planned, queued, downloaded, verified, failed".to_string(),
                ));
            }
        }
    }
    if let Some(vs) = args.verification_status.as_deref() {
        match vs {
            "not_started" | "passed" | "failed" => {}
            _ => {
                return Err(CliError::Usage(
                    "--verification-status must be one of: not_started, passed, failed".to_string(),
                ));
            }
        }
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let mut sql = String::from(
        "SELECT id, job_id, ts, package_id, repo_key, source_url, mode, status, verification_status, hash_status, signature_status, source_policy_status, message FROM download_job_history",
    );
    let mut clauses: Vec<String> = Vec::new();
    let mut values: Vec<Value> = Vec::new();
    if let Some(status) = args.status.clone() {
        clauses.push("status = ?".to_string());
        values.push(Value::Text(status));
    }
    if let Some(vs) = args.verification_status.clone() {
        clauses.push("verification_status = ?".to_string());
        values.push(Value::Text(vs));
    }
    if !clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&clauses.join(" AND "));
    }
    sql.push_str(" ORDER BY id DESC LIMIT ? OFFSET ?");
    values.push(Value::Integer(limit));
    values.push(Value::Integer(offset));

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params_from_iter(values.iter()), |row| {
        Ok(json!({
            "id": row.get::<_, i64>(0)?,
            "job_id": row.get::<_, String>(1)?,
            "timestamp": row.get::<_, i64>(2)?,
            "package_id": row.get::<_, String>(3)?,
            "repo_key": row.get::<_, String>(4)?,
            "source_url": row.get::<_, String>(5)?,
            "mode": row.get::<_, String>(6)?,
            "status": row.get::<_, String>(7)?,
            "verification_status": row.get::<_, String>(8)?,
            "hash_status": row.get::<_, String>(9)?,
            "signature_status": row.get::<_, String>(10)?,
            "source_policy_status": row.get::<_, String>(11)?,
            "message": row.get::<_, String>(12)?
        }))
    })?;
    let payload: Vec<serde_json::Value> = rows.collect::<Result<Vec<_>, _>>()?;
    if payload.is_empty() {
        return print_payload(args.json, json!([]), "No download jobs found.");
    }
    print_payload(args.json, json!(payload), "Download jobs listed.")
}

fn download_show(args: DownloadShowArgs) -> Result<(), CliError> {
    if args.job_id.trim().is_empty() {
        return Err(CliError::Usage("--job-id is required".to_string()));
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let payload = conn
        .query_row(
            "SELECT id, job_id, ts, package_id, repo_key, source_url, mode, status, verification_status, hash_status, signature_status, source_policy_status, message
             FROM download_job_history
             WHERE job_id = ?1",
            params![args.job_id],
            |row| {
                Ok(json!({
                    "id": row.get::<_, i64>(0)?,
                    "job_id": row.get::<_, String>(1)?,
                    "timestamp": row.get::<_, i64>(2)?,
                    "package_id": row.get::<_, String>(3)?,
                    "repo_key": row.get::<_, String>(4)?,
                    "source_url": row.get::<_, String>(5)?,
                    "mode": row.get::<_, String>(6)?,
                    "status": row.get::<_, String>(7)?,
                    "verification_status": row.get::<_, String>(8)?,
                    "hash_status": row.get::<_, String>(9)?,
                    "signature_status": row.get::<_, String>(10)?,
                    "source_policy_status": row.get::<_, String>(11)?,
                    "message": row.get::<_, String>(12)?
                }))
            },
        )
        .optional()?;

    match payload {
        Some(v) => print_payload(args.json, v, "Download job shown."),
        None => Err(CliError::Usage(format!("job_id {} not found", args.job_id))),
    }
}

fn download_retry(args: DownloadRetryArgs) -> Result<(), CliError> {
    if args.job_id.trim().is_empty() {
        return Err(CliError::Usage("--job-id is required".to_string()));
    }
    if !args.dry_run {
        return Err(CliError::Usage(
            "--dry-run is currently required for download retry in this phase".to_string(),
        ));
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let previous = conn
        .query_row(
            "SELECT package_id, repo_key, source_url, status FROM download_job_history WHERE job_id = ?1",
            params![args.job_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            },
        )
        .optional()?;
    let (package_id, repo_key, source_url, previous_status) = match previous {
        Some(v) => v,
        None => return Err(CliError::Usage(format!("job_id {} not found", args.job_id))),
    };
    if previous_status != "failed" {
        return Err(CliError::Usage(
            "download retry only supports failed jobs".to_string(),
        ));
    }

    let now = unix_ts();
    let retry_job_id = next_operation_id("download", now);
    let message = format!("download retry planned from {}", args.job_id);
    conn.execute(
        r#"
        INSERT INTO download_job_history
        (job_id, ts, package_id, repo_key, source_url, mode, status, verification_status, hash_status, signature_status, source_policy_status, message)
        VALUES (?1, ?2, ?3, ?4, ?5, 'dry_run', 'planned', 'not_started', 'not_started', 'not_started', 'not_started', ?6)
        "#,
        params![retry_job_id, now, package_id, repo_key, source_url, message],
    )?;

    let payload = json!({
        "old_job_id": args.job_id,
        "new_job_id": retry_job_id,
        "status": "planned",
        "verification_status": "not_started",
        "hash_status": "not_started",
        "signature_status": "not_started",
        "source_policy_status": "not_started",
        "message": "download retry dry-run planned successfully"
    });
    print_payload(args.json, payload, "Download retry dry-run planned.")
}

fn download_verify(args: DownloadVerifyArgs) -> Result<(), CliError> {
    if args.job_id.trim().is_empty() {
        return Err(CliError::Usage("--job-id is required".to_string()));
    }
    let failure_flags = [
        args.simulate_failure,
        args.simulate_hash_failure,
        args.simulate_signature_failure,
        args.simulate_source_policy_failure,
    ]
    .into_iter()
    .filter(|v| *v)
    .count();
    if failure_flags > 1 {
        return Err(CliError::Usage(
            "only one simulate failure flag can be set".to_string(),
        ));
    }
    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM download_job_history WHERE job_id = ?1)",
        params![args.job_id],
        |r| r.get::<_, i64>(0).map(|v| v == 1),
    )?;
    if !exists {
        return Err(CliError::Usage(format!("job_id {} not found", args.job_id)));
    }

    let now = unix_ts();
    if args.simulate_failure {
        conn.execute(
            "UPDATE download_job_history
             SET ts=?1, status='failed', verification_status='failed',
                 hash_status='failed', signature_status='failed', source_policy_status='failed',
                 message='download verify failed (simulated)'
             WHERE job_id=?2",
            params![now, args.job_id],
        )?;
        let payload = json!({
            "job_id": args.job_id,
            "status": "failed",
            "verification_status": "failed",
            "hash_status": "failed",
            "signature_status": "failed",
            "source_policy_status": "failed",
            "message": "download verify failed (simulated)"
        });
        if args.json {
            println!("{}", serde_json::to_string_pretty(&payload)?);
        }
        return Err(CliError::Integration(
            "download verify failed (simulated)".to_string(),
        ));
    }
    if args.simulate_hash_failure {
        conn.execute(
            "UPDATE download_job_history
             SET ts=?1, status='failed', verification_status='failed',
                 hash_status='failed', signature_status='passed', source_policy_status='passed',
                 message='download verify failed: hash mismatch (simulated)'
             WHERE job_id=?2",
            params![now, args.job_id],
        )?;
        let payload = json!({
            "job_id": args.job_id,
            "status": "failed",
            "verification_status": "failed",
            "hash_status": "failed",
            "signature_status": "passed",
            "source_policy_status": "passed",
            "message": "download verify failed: hash mismatch (simulated)"
        });
        if args.json {
            println!("{}", serde_json::to_string_pretty(&payload)?);
        }
        return Err(CliError::Integration(
            "download verify failed: hash mismatch (simulated)".to_string(),
        ));
    }
    if args.simulate_signature_failure {
        conn.execute(
            "UPDATE download_job_history
             SET ts=?1, status='failed', verification_status='failed',
                 hash_status='passed', signature_status='failed', source_policy_status='passed',
                 message='download verify failed: signature invalid (simulated)'
             WHERE job_id=?2",
            params![now, args.job_id],
        )?;
        let payload = json!({
            "job_id": args.job_id,
            "status": "failed",
            "verification_status": "failed",
            "hash_status": "passed",
            "signature_status": "failed",
            "source_policy_status": "passed",
            "message": "download verify failed: signature invalid (simulated)"
        });
        if args.json {
            println!("{}", serde_json::to_string_pretty(&payload)?);
        }
        return Err(CliError::Integration(
            "download verify failed: signature invalid (simulated)".to_string(),
        ));
    }
    if args.simulate_source_policy_failure {
        conn.execute(
            "UPDATE download_job_history
             SET ts=?1, status='failed', verification_status='failed',
                 hash_status='passed', signature_status='passed', source_policy_status='failed',
                 message='download verify failed: source policy blocked (simulated)'
             WHERE job_id=?2",
            params![now, args.job_id],
        )?;
        let payload = json!({
            "job_id": args.job_id,
            "status": "failed",
            "verification_status": "failed",
            "hash_status": "passed",
            "signature_status": "passed",
            "source_policy_status": "failed",
            "message": "download verify failed: source policy blocked (simulated)"
        });
        if args.json {
            println!("{}", serde_json::to_string_pretty(&payload)?);
        }
        return Err(CliError::Integration(
            "download verify failed: source policy blocked (simulated)".to_string(),
        ));
    }

    conn.execute(
        "UPDATE download_job_history
         SET ts=?1, status='verified', verification_status='passed',
             hash_status='passed', signature_status='passed', source_policy_status='passed',
             message='download verify passed (simulated)'
         WHERE job_id=?2",
        params![now, args.job_id],
    )?;

    let payload = json!({
        "job_id": args.job_id,
        "status": "verified",
        "verification_status": "passed",
        "hash_status": "passed",
        "signature_status": "passed",
        "source_policy_status": "passed",
        "message": "download verify passed (simulated)"
    });
    print_payload(args.json, payload, "Download verify succeeded.")
}

fn download_history(args: DownloadHistoryArgs) -> Result<(), CliError> {
    let limit = i64::from(args.limit.unwrap_or(100));
    let offset = i64::from(args.offset.unwrap_or(0));
    if limit <= 0 {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }
    if let Some(status) = args.status.as_deref() {
        match status {
            "planned" | "queued" | "downloaded" | "verified" | "failed" => {}
            _ => {
                return Err(CliError::Usage(
                    "--status must be one of: planned, queued, downloaded, verified, failed".to_string(),
                ));
            }
        }
    }
    if let Some(ft) = args.failure_type.as_deref() {
        match ft {
            "hash" | "signature" | "source_policy" | "generic" => {}
            _ => {
                return Err(CliError::Usage(
                    "--failure-type must be one of: hash, signature, source_policy, generic".to_string(),
                ));
            }
        }
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let mut sql = String::from(
        "SELECT id, job_id, ts, package_id, repo_key, mode, status, verification_status, hash_status, signature_status, source_policy_status, message FROM download_job_history",
    );
    let mut clauses: Vec<String> = Vec::new();
    let mut values: Vec<Value> = Vec::new();

    if let Some(status) = args.status.clone() {
        clauses.push("status = ?".to_string());
        values.push(Value::Text(status));
    }
    if let Some(ft) = args.failure_type.clone() {
        clauses.push("status = 'failed'".to_string());
        let cond = match ft.as_str() {
            "hash" => "hash_status = 'failed'",
            "signature" => "signature_status = 'failed'",
            "source_policy" => "source_policy_status = 'failed'",
            _ => "hash_status = 'not_started' AND signature_status = 'not_started' AND source_policy_status = 'not_started'",
        };
        clauses.push(cond.to_string());
    }
    if !clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&clauses.join(" AND "));
    }
    sql.push_str(" ORDER BY id DESC LIMIT ? OFFSET ?");
    values.push(Value::Integer(limit));
    values.push(Value::Integer(offset));

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params_from_iter(values.iter()), |row| {
        Ok(json!({
            "id": row.get::<_, i64>(0)?,
            "job_id": row.get::<_, String>(1)?,
            "timestamp": row.get::<_, i64>(2)?,
            "package_id": row.get::<_, String>(3)?,
            "repo_key": row.get::<_, String>(4)?,
            "mode": row.get::<_, String>(5)?,
            "status": row.get::<_, String>(6)?,
            "verification_status": row.get::<_, String>(7)?,
            "hash_status": row.get::<_, String>(8)?,
            "signature_status": row.get::<_, String>(9)?,
            "source_policy_status": row.get::<_, String>(10)?,
            "message": row.get::<_, String>(11)?
        }))
    })?;
    let entries: Vec<serde_json::Value> = rows.collect::<Result<Vec<_>, _>>()?;

    let stats = json!({
        "failed_hash": count_download_failures(&conn, "hash")?,
        "failed_signature": count_download_failures(&conn, "signature")?,
        "failed_source_policy": count_download_failures(&conn, "source_policy")?,
        "failed_generic": count_download_failures(&conn, "generic")?
    });

    let payload = json!({
        "entries": entries,
        "stats": stats
    });
    print_payload(args.json, payload, "Download history listed.")
}

fn count_download_failures(conn: &Connection, failure_type: &str) -> Result<i64, CliError> {
    let condition = match failure_type {
        "hash" => "status = 'failed' AND hash_status = 'failed'",
        "signature" => "status = 'failed' AND signature_status = 'failed'",
        "source_policy" => "status = 'failed' AND source_policy_status = 'failed'",
        _ => "status = 'failed' AND hash_status = 'not_started' AND signature_status = 'not_started' AND source_policy_status = 'not_started'",
    };
    let sql = format!("SELECT COUNT(1) FROM download_job_history WHERE {condition}");
    let n: i64 = conn.query_row(&sql, [], |r| r.get(0))?;
    Ok(n)
}

fn validate_registry_status(status: &str) -> Result<(), CliError> {
    match status {
        "active" | "disabled" => Ok(()),
        _ => Err(CliError::Usage(
            "--status must be one of: active, disabled".to_string(),
        )),
    }
}

fn validate_update_apply_flags(args: &UpdateApplyArgs) -> Result<(), CliError> {
    validate_confirmed_execution_flags(
        args.dry_run,
        args.confirm,
        args.execution_ticket.as_deref(),
        args.simulate_failure,
        args.simulate_rollback_failure,
    )
}

fn validate_cleanup_apply_flags(args: &CleanupApplyArgs) -> Result<(), CliError> {
    validate_confirmed_execution_flags(
        args.dry_run,
        args.confirm,
        args.execution_ticket.as_deref(),
        args.simulate_failure,
        args.simulate_rollback_failure,
    )
}

fn validate_confirmed_execution_flags(
    dry_run: bool,
    confirm: bool,
    execution_ticket: Option<&str>,
    simulate_failure: bool,
    simulate_rollback_failure: bool,
) -> Result<(), CliError> {
    if dry_run && confirm {
        return Err(CliError::Usage(
            "--confirm is not needed when --dry-run is used".to_string(),
        ));
    }
    if !dry_run && !confirm {
        return Err(CliError::Usage(
            "--confirm is required when --dry-run is not used".to_string(),
        ));
    }
    if simulate_rollback_failure && !simulate_failure {
        return Err(CliError::Usage(
            "--simulate-rollback-failure requires --simulate-failure".to_string(),
        ));
    }
    if dry_run && (simulate_failure || simulate_rollback_failure) {
        return Err(CliError::Usage(
            "simulation flags are only valid with confirmed execution".to_string(),
        ));
    }
    if dry_run && execution_ticket.is_some() {
        return Err(CliError::Usage(
            "--execution-ticket is only valid with confirmed execution".to_string(),
        ));
    }
    if confirm {
        let valid = execution_ticket.map(|v| !v.trim().is_empty()).unwrap_or(false);
        if !valid {
            return Err(CliError::Usage(
                "--execution-ticket is required when --confirm is used".to_string(),
            ));
        }
    }
    Ok(())
}

fn ensure_real_mutation_gate_enabled() -> Result<(), CliError> {
    let config = load_config()?;
    if !config.execution.real_mutation_enabled {
        return Err(CliError::Security(
            "real mutation is disabled; set execution.real_mutation_enabled=true".to_string(),
        ));
    }
    if config.execution.approval_record_ref.trim().is_empty() {
        return Err(CliError::Security(
            "approval record is required for real mutation".to_string(),
        ));
    }
    Ok(())
}

fn simulated_failure_outcome(
    simulate_rollback_failure: bool,
    rollback_success_message: &str,
    rollback_failed_message: &str,
) -> (&'static str, String) {
    if simulate_rollback_failure {
        ("failed", rollback_failed_message.to_string())
    } else {
        ("success", rollback_success_message.to_string())
    }
}

fn config_init(as_json: bool) -> Result<(), CliError> {
    let config_path = config_path()?;
    if !config_path.exists() {
        let text = serde_json::to_string_pretty(&AppConfig::default())?;
        fs::write(&config_path, text)?;
    }

    let db_path = db_path()?;
    init_db(&db_path)?;

    let payload = json!({
        "config_path": config_path.to_string_lossy(),
        "db_path": db_path.to_string_lossy(),
        "initialized": true
    });
    print_payload(as_json, payload, "Config and database initialized.")
}

fn config_gate_show(as_json: bool) -> Result<(), CliError> {
    let config = load_config()?;
    let payload = json!({
        "real_mutation_enabled": config.execution.real_mutation_enabled,
        "gate_version": config.execution.gate_version,
        "approval_record_ref": config.execution.approval_record_ref,
        "approval_record_present": !config.execution.approval_record_ref.trim().is_empty()
    });
    print_payload(as_json, payload, "Gate state displayed.")
}

fn config_gate_set(args: GateSetArgs) -> Result<(), CliError> {
    validate_gate_set(&args)?;
    let mut config = load_config()?;
    let current = config.execution.clone();

    let new_enabled = args.enable;
    let new_gate_version = args
        .gate_version
        .clone()
        .unwrap_or_else(|| current.gate_version.clone());
    let new_approval_record = if args.enable {
        args.approval_record
            .clone()
            .ok_or_else(|| CliError::Usage("--approval-record is required when --enable is used".to_string()))?
    } else if args.keep_record {
        current.approval_record_ref
    } else {
        String::new()
    };

    if !args.dry_run {
        config.execution.real_mutation_enabled = new_enabled;
        config.execution.gate_version = new_gate_version.clone();
        config.execution.approval_record_ref = new_approval_record.clone();
        save_config(&config)?;

        let db_file = db_path()?;
        init_db(&db_file)?;
        let conn = Connection::open(db_file)?;
        let reason = args
            .reason
            .clone()
            .ok_or_else(|| CliError::Usage("--reason is required unless --dry-run is used".to_string()))?;
        conn.execute(
            "INSERT INTO gate_history (ts, real_mutation_enabled, gate_version, approval_record_ref, reason)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                unix_ts(),
                if new_enabled { 1 } else { 0 },
                new_gate_version.clone(),
                new_approval_record.clone(),
                reason
            ],
        )?;
    }

    let payload = json!({
        "real_mutation_enabled": new_enabled,
        "gate_version": new_gate_version,
        "approval_record_ref": new_approval_record,
        "approval_record_present": !new_approval_record.trim().is_empty(),
        "config_path": config_path()?.to_string_lossy(),
        "dry_run": args.dry_run
    });
    print_payload(args.json, payload, "Gate state updated.")
}

fn config_gate_history(args: GateHistoryArgs) -> Result<(), CliError> {
    if args.limit == Some(0) {
        return Err(CliError::Usage("--limit must be >= 1".to_string()));
    }

    let db_file = db_path()?;
    init_db(&db_file)?;
    let conn = Connection::open(db_file)?;

    let mut sql = String::from(
        "SELECT id, ts, real_mutation_enabled, gate_version, approval_record_ref, reason \
         FROM gate_history",
    );
    let mut clauses: Vec<String> = Vec::new();
    let mut values: Vec<Value> = Vec::new();

    if args.enabled_only {
        clauses.push("real_mutation_enabled = 1".to_string());
    }
    if let Some(since) = args.since {
        clauses.push("ts >= ?".to_string());
        values.push(Value::Integer(since));
    }
    if let Some(reason_like) = args.reason_contains.clone() {
        clauses.push("reason LIKE ?".to_string());
        values.push(Value::Text(format!("%{reason_like}%")));
    }
    if !clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&clauses.join(" AND "));
    }
    sql.push_str(" ORDER BY id DESC LIMIT ?");
    values.push(Value::Integer(i64::from(args.limit.unwrap_or(100))));

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params_from_iter(values.iter()), |row| {
        let approval_record_ref: String = row.get(4)?;
        Ok(json!({
            "id": row.get::<_, i64>(0)?,
            "timestamp": row.get::<_, i64>(1)?,
            "real_mutation_enabled": row.get::<_, i64>(2)? == 1,
            "gate_version": row.get::<_, String>(3)?,
            "approval_record_ref": approval_record_ref,
            "approval_record_present": !approval_record_ref.trim().is_empty(),
            "reason": row.get::<_, String>(5)?
        }))
    })?;

    let out: Result<Vec<_>, rusqlite::Error> = rows.collect();
    let out = out.map_err(CliError::Db)?;
    if args.json {
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        println!("gate_history_count: {}", out.len());
    }
    Ok(())
}

fn validate_gate_set(args: &GateSetArgs) -> Result<(), CliError> {
    if args.enable == args.disable {
        return Err(CliError::Usage(
            "exactly one of --enable or --disable must be set".to_string(),
        ));
    }
    if args.keep_record && !args.disable {
        return Err(CliError::Usage(
            "--keep-record is only valid with --disable".to_string(),
        ));
    }
    if args.enable && args.approval_record.is_none() {
        return Err(CliError::Usage(
            "--approval-record is required when --enable is used".to_string(),
        ));
    }
    if args.enable && !args.dry_run && !args.confirm {
        return Err(CliError::Usage(
            "--confirm is required when --enable is used".to_string(),
        ));
    }
    if !args.dry_run && args.reason.is_none() {
        return Err(CliError::Usage(
            "--reason is required unless --dry-run is used".to_string(),
        ));
    }
    Ok(())
}

fn load_config() -> Result<AppConfig, CliError> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(AppConfig::default());
    }

    let raw = fs::read_to_string(&path)?;
    serde_json::from_str(&raw)
        .map_err(|e| CliError::Config(format!("failed to parse {}: {e}", path.display())))
}

fn save_config(config: &AppConfig) -> Result<(), CliError> {
    let path = config_path()?;
    let raw = serde_json::to_string_pretty(config)?;
    fs::write(path, raw)?;
    Ok(())
}

fn config_path() -> Result<PathBuf, CliError> {
    Ok(synora_home()?.join("config.json"))
}

fn db_path() -> Result<PathBuf, CliError> {
    let home = synora_home()?;
    let db_dir = home.join("db");
    fs::create_dir_all(&db_dir)?;
    Ok(db_dir.join("synora.db"))
}

fn init_db(path: &Path) -> Result<(), CliError> {
    let conn = Connection::open(path)?;
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS software_inventory (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            version TEXT NOT NULL DEFAULT '',
            publisher TEXT NOT NULL DEFAULT '',
            install_location TEXT NOT NULL DEFAULT '',
            discovery_source TEXT NOT NULL,
            source_confidence INTEGER NOT NULL DEFAULT 50,
            first_seen_at INTEGER NOT NULL,
            last_seen_at INTEGER NOT NULL,
            is_active INTEGER NOT NULL DEFAULT 1,
            fingerprint TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS software_discovery_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            scan_id TEXT NOT NULL UNIQUE,
            ts INTEGER NOT NULL,
            source TEXT NOT NULL,
            total_seen INTEGER NOT NULL,
            inserted INTEGER NOT NULL,
            updated INTEGER NOT NULL,
            reactivated INTEGER NOT NULL DEFAULT 0,
            deactivated INTEGER NOT NULL DEFAULT 0,
            skipped INTEGER NOT NULL,
            active_after INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS repo_registry (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_key TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            url TEXT NOT NULL,
            kind TEXT NOT NULL,
            status TEXT NOT NULL,
            priority INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS repo_package_index (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_key TEXT NOT NULL,
            package_id TEXT NOT NULL,
            package_name TEXT NOT NULL,
            version TEXT NOT NULL,
            source_url TEXT NOT NULL,
            status TEXT NOT NULL,
            updated_at INTEGER NOT NULL,
            UNIQUE(repo_key, package_id)
        );

        CREATE TABLE IF NOT EXISTS repo_sync_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sync_id TEXT NOT NULL,
            ts INTEGER NOT NULL,
            repo_key TEXT NOT NULL,
            status TEXT NOT NULL,
            message TEXT NOT NULL,
            packages_upserted INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS download_job_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            job_id TEXT NOT NULL UNIQUE,
            ts INTEGER NOT NULL,
            package_id TEXT NOT NULL,
            repo_key TEXT NOT NULL,
            source_url TEXT NOT NULL,
            mode TEXT NOT NULL,
            status TEXT NOT NULL,
            verification_status TEXT NOT NULL,
            hash_status TEXT NOT NULL DEFAULT 'not_started',
            signature_status TEXT NOT NULL DEFAULT 'not_started',
            source_policy_status TEXT NOT NULL DEFAULT 'not_started',
            message TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS ai_repair_plan_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            plan_id TEXT NOT NULL UNIQUE,
            ts INTEGER NOT NULL,
            software_query TEXT NOT NULL,
            issue_text TEXT NOT NULL,
            target_software_id INTEGER,
            risk_level TEXT NOT NULL,
            confidence INTEGER NOT NULL,
            reason TEXT NOT NULL,
            plan_json TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS ai_analyze_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            analysis_id TEXT NOT NULL UNIQUE,
            ts INTEGER NOT NULL,
            total_active INTEGER NOT NULL,
            total_publishers INTEGER NOT NULL,
            confidence INTEGER NOT NULL,
            summary_json TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS ai_recommend_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            recommendation_id TEXT NOT NULL UNIQUE,
            ts INTEGER NOT NULL,
            goal TEXT NOT NULL,
            risk_level TEXT NOT NULL,
            confidence INTEGER NOT NULL,
            reason TEXT NOT NULL,
            recommendation_json TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS ui_action_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            event_id TEXT NOT NULL UNIQUE,
            ts INTEGER NOT NULL,
            action_id TEXT NOT NULL,
            risk_level TEXT NOT NULL,
            status TEXT NOT NULL,
            message TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS job_queue (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            job_type TEXT NOT NULL,
            payload_json TEXT NOT NULL,
            status TEXT NOT NULL,
            priority INTEGER NOT NULL,
            attempt_count INTEGER NOT NULL,
            max_attempts INTEGER NOT NULL,
            scheduled_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            started_at INTEGER,
            finished_at INTEGER,
            last_error TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS source_candidate (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            software_id INTEGER NOT NULL,
            software_name TEXT NOT NULL,
            url TEXT NOT NULL,
            domain TEXT NOT NULL,
            confidence INTEGER NOT NULL,
            reason TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            UNIQUE(software_id, url)
        );

        CREATE TABLE IF NOT EXISTS source_registry (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            candidate_id INTEGER NOT NULL UNIQUE,
            software_id INTEGER NOT NULL,
            software_name TEXT NOT NULL,
            url TEXT NOT NULL,
            domain TEXT NOT NULL,
            confidence INTEGER NOT NULL,
            reason TEXT NOT NULL,
            status TEXT NOT NULL,
            applied_at INTEGER NOT NULL,
            UNIQUE(software_id, url)
        );

        CREATE TABLE IF NOT EXISTS update_operation_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            operation_id TEXT NOT NULL UNIQUE,
            ts INTEGER NOT NULL,
            candidate_id INTEGER NOT NULL,
            mode TEXT NOT NULL,
            status TEXT NOT NULL,
            message TEXT NOT NULL,
            rollback_attempted INTEGER NOT NULL DEFAULT 0,
            rollback_status TEXT NOT NULL DEFAULT 'not_recorded',
            execution_ticket TEXT NOT NULL DEFAULT ''
        );

        CREATE TABLE IF NOT EXISTS cleanup_operation_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            operation_id TEXT NOT NULL UNIQUE,
            ts INTEGER NOT NULL,
            software_id INTEGER NOT NULL,
            mode TEXT NOT NULL,
            status TEXT NOT NULL,
            message TEXT NOT NULL,
            rollback_attempted INTEGER NOT NULL DEFAULT 0,
            rollback_status TEXT NOT NULL DEFAULT 'not_recorded',
            execution_ticket TEXT NOT NULL DEFAULT ''
        );

        CREATE TABLE IF NOT EXISTS gate_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ts INTEGER NOT NULL,
            real_mutation_enabled INTEGER NOT NULL,
            gate_version TEXT NOT NULL,
            approval_record_ref TEXT NOT NULL,
            reason TEXT NOT NULL
        );
        "#,
    )?;
    ensure_update_history_columns(&conn)?;
    ensure_cleanup_history_columns(&conn)?;
    ensure_download_history_columns(&conn)?;
    ensure_default_repositories(&conn)?;
    Ok(())
}

fn ensure_update_history_columns(conn: &Connection) -> Result<(), CliError> {
    let mut stmt = conn.prepare("PRAGMA table_info(update_operation_history)")?;
    let rows = stmt.query_map([], |row| row.get::<_, String>(1))?;
    let columns: Vec<String> = rows.collect::<Result<Vec<_>, _>>()?;

    if !columns.iter().any(|c| c == "rollback_attempted") {
        conn.execute(
            "ALTER TABLE update_operation_history ADD COLUMN rollback_attempted INTEGER NOT NULL DEFAULT 0",
            [],
        )?;
    }
    if !columns.iter().any(|c| c == "rollback_status") {
        conn.execute(
            "ALTER TABLE update_operation_history ADD COLUMN rollback_status TEXT NOT NULL DEFAULT 'not_recorded'",
            [],
        )?;
    }
    if !columns.iter().any(|c| c == "execution_ticket") {
        conn.execute(
            "ALTER TABLE update_operation_history ADD COLUMN execution_ticket TEXT NOT NULL DEFAULT ''",
            [],
        )?;
    }
    Ok(())
}

fn ensure_default_repositories(conn: &Connection) -> Result<(), CliError> {
    let now = unix_ts();
    conn.execute(
        r#"
        INSERT OR IGNORE INTO repo_registry
        (repo_key, name, url, kind, status, priority, created_at, updated_at)
        VALUES
        ('public-default', 'Synora Public Repository', 'https://repo.synora.local/public-index.yaml', 'public', 'active', 100, ?1, ?1)
        "#,
        params![now],
    )?;
    conn.execute(
        r#"
        INSERT OR IGNORE INTO repo_registry
        (repo_key, name, url, kind, status, priority, created_at, updated_at)
        VALUES
        ('personal-local', 'Synora Personal Repository', 'file://.synora_custom/repos/personal/software.yaml', 'personal', 'active', 200, ?1, ?1)
        "#,
        params![now],
    )?;
    Ok(())
}

fn ensure_cleanup_history_columns(conn: &Connection) -> Result<(), CliError> {
    let mut stmt = conn.prepare("PRAGMA table_info(cleanup_operation_history)")?;
    let rows = stmt.query_map([], |row| row.get::<_, String>(1))?;
    let columns: Vec<String> = rows.collect::<Result<Vec<_>, _>>()?;

    if !columns.iter().any(|c| c == "rollback_attempted") {
        conn.execute(
            "ALTER TABLE cleanup_operation_history ADD COLUMN rollback_attempted INTEGER NOT NULL DEFAULT 0",
            [],
        )?;
    }
    if !columns.iter().any(|c| c == "rollback_status") {
        conn.execute(
            "ALTER TABLE cleanup_operation_history ADD COLUMN rollback_status TEXT NOT NULL DEFAULT 'not_recorded'",
            [],
        )?;
    }
    if !columns.iter().any(|c| c == "execution_ticket") {
        conn.execute(
            "ALTER TABLE cleanup_operation_history ADD COLUMN execution_ticket TEXT NOT NULL DEFAULT ''",
            [],
        )?;
    }
    Ok(())
}

fn ensure_download_history_columns(conn: &Connection) -> Result<(), CliError> {
    let mut stmt = conn.prepare("PRAGMA table_info(download_job_history)")?;
    let rows = stmt.query_map([], |row| row.get::<_, String>(1))?;
    let columns: Vec<String> = rows.collect::<Result<Vec<_>, _>>()?;

    if !columns.iter().any(|c| c == "hash_status") {
        conn.execute(
            "ALTER TABLE download_job_history ADD COLUMN hash_status TEXT NOT NULL DEFAULT 'not_started'",
            [],
        )?;
    }
    if !columns.iter().any(|c| c == "signature_status") {
        conn.execute(
            "ALTER TABLE download_job_history ADD COLUMN signature_status TEXT NOT NULL DEFAULT 'not_started'",
            [],
        )?;
    }
    if !columns.iter().any(|c| c == "source_policy_status") {
        conn.execute(
            "ALTER TABLE download_job_history ADD COLUMN source_policy_status TEXT NOT NULL DEFAULT 'not_started'",
            [],
        )?;
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct SourceCandidateDraft {
    url: String,
    domain: String,
    confidence: i64,
    reason: String,
}

fn build_source_candidates(name: &str, publisher: &str) -> Vec<SourceCandidateDraft> {
    let mut out = Vec::new();
    let q = encode_query(name);
    out.push(SourceCandidateDraft {
        url: format!("https://winget.run/search?query={q}"),
        domain: "winget.run".to_string(),
        confidence: 55,
        reason: "default winget search heuristic".to_string(),
    });

    let pub_l = publisher.to_lowercase();
    let name_l = name.to_lowercase();
    if pub_l.contains("microsoft") || name_l.contains("visual studio") || name_l.contains(".net") {
        out.push(SourceCandidateDraft {
            url: "https://github.com/microsoft/winget-pkgs".to_string(),
            domain: "github.com".to_string(),
            confidence: 72,
            reason: "publisher/name matched microsoft heuristic".to_string(),
        });
    } else if pub_l.contains("jetbrains") {
        out.push(SourceCandidateDraft {
            url: "https://www.jetbrains.com/toolbox-app/".to_string(),
            domain: "jetbrains.com".to_string(),
            confidence: 70,
            reason: "publisher matched jetbrains heuristic".to_string(),
        });
    } else if name_l.contains("git") {
        out.push(SourceCandidateDraft {
            url: "https://git-scm.com/download/win".to_string(),
            domain: "git-scm.com".to_string(),
            confidence: 68,
            reason: "name matched git heuristic".to_string(),
        });
    }

    out
}

fn encode_query(s: &str) -> String {
    let mut out = String::new();
    for b in s.as_bytes() {
        let c = *b as char;
        if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' {
            out.push(c);
        } else if c == ' ' {
            out.push('+');
        } else {
            out.push_str(&format!("%{:02X}", *b));
        }
    }
    out
}

fn make_fingerprint(name: &str, publisher: &str, install_location: &str) -> String {
    let name = normalize(name);
    let publisher = normalize(publisher);
    let install_location = normalize(install_location);
    if name.is_empty() {
        return String::new();
    }
    format!("{name}|{publisher}|{install_location}")
}

fn normalize(value: &str) -> String {
    value.trim().to_lowercase()
}

#[cfg(target_os = "windows")]
fn discover_registry_software() -> Result<Vec<DiscoveredSoftware>, CliError> {
    let script = r#"
$ErrorActionPreference = 'Stop'
$paths = @(
  'HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*',
  'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*'
)

$items = foreach ($p in $paths) {
  Get-ItemProperty -Path $p -ErrorAction SilentlyContinue |
    Where-Object { $_.DisplayName -and $_.DisplayName.Trim() -ne '' } |
    ForEach-Object {
      [PSCustomObject]@{
        name = [string]$_.DisplayName
        version = [string]$_.DisplayVersion
        publisher = [string]$_.Publisher
        install_location = [string]$_.InstallLocation
        discovery_source = 'registry'
      }
    }
}

$items | ConvertTo-Json -Compress
"#;

    let output = std::process::Command::new("powershell")
        .arg("-NoProfile")
        .arg("-NonInteractive")
        .arg("-Command")
        .arg(script)
        .output()
        .map_err(|e| CliError::Integration(format!("failed to launch powershell: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(CliError::Integration(format!(
            "registry discovery failed: {}",
            stderr.trim()
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if stdout.is_empty() || stdout == "null" {
        return Ok(Vec::new());
    }

    let value: serde_json::Value = serde_json::from_str(&stdout)
        .map_err(|e| CliError::Integration(format!("invalid discovery json: {e}")))?;
    if value.is_array() {
        serde_json::from_value(value)
            .map_err(|e| CliError::Integration(format!("invalid discovery shape: {e}")))
    } else {
        let one: DiscoveredSoftware = serde_json::from_value(value)
            .map_err(|e| CliError::Integration(format!("invalid discovery row: {e}")))?;
        Ok(vec![one])
    }
}

#[cfg(not(target_os = "windows"))]
fn discover_registry_software() -> Result<Vec<DiscoveredSoftware>, CliError> {
    Ok(Vec::new())
}

fn synora_home() -> Result<PathBuf, CliError> {
    if let Ok(value) = env::var("SYNORA_HOME") {
        let path = PathBuf::from(value);
        fs::create_dir_all(&path)?;
        return Ok(path);
    }

    let cwd = env::current_dir()?;
    let path = cwd.join(".synora_custom");
    fs::create_dir_all(&path)?;
    Ok(path)
}

fn unix_ts() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now();
    let dur = now
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    dur.as_secs() as i64
}

fn next_operation_id(prefix: &str, target_id: i64) -> String {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0))
        .as_nanos();
    format!("{prefix}-{nanos}-{seq}-{target_id}")
}

fn print_payload(as_json: bool, payload: serde_json::Value, plain: &str) -> Result<(), CliError> {
    if as_json {
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else {
        println!("{plain}");
    }
    Ok(())
}

#[allow(dead_code)]
fn _is_within(root: &Path, target: &Path) -> bool {
    let root = root.components().collect::<Vec<_>>();
    let target = target.components().collect::<Vec<_>>();
    target.starts_with(&root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_operation_id_should_be_unique() {
        let a = next_operation_id("update", 180);
        let b = next_operation_id("update", 180);
        assert_ne!(a, b);
    }

    #[test]
    fn validate_registry_status_rejects_unknown() {
        let err = validate_registry_status("unknown").unwrap_err();
        assert!(matches!(err, CliError::Usage(_)));
    }

    #[test]
    fn validate_update_apply_flags_rejects_invalid_combinations() {
        let err = validate_update_apply_flags(&UpdateApplyArgs {
            candidate_id: 1,
            dry_run: true,
            confirm: true,
            execution_ticket: None,
            simulate_failure: false,
            simulate_rollback_failure: false,
            json: true,
        })
        .unwrap_err();
        assert!(matches!(err, CliError::Usage(_)));

        let err = validate_update_apply_flags(&UpdateApplyArgs {
            candidate_id: 1,
            dry_run: false,
            confirm: true,
            execution_ticket: None,
            simulate_failure: false,
            simulate_rollback_failure: true,
            json: true,
        })
        .unwrap_err();
        assert!(matches!(err, CliError::Usage(_)));

        let err = validate_update_apply_flags(&UpdateApplyArgs {
            candidate_id: 1,
            dry_run: false,
            confirm: true,
            execution_ticket: None,
            simulate_failure: false,
            simulate_rollback_failure: false,
            json: true,
        })
        .unwrap_err();
        assert!(matches!(err, CliError::Usage(_)));
    }

    #[test]
    fn validate_cleanup_apply_flags_rejects_invalid_combinations() {
        let err = validate_cleanup_apply_flags(&CleanupApplyArgs {
            software_id: 1,
            dry_run: true,
            confirm: true,
            execution_ticket: None,
            simulate_failure: false,
            simulate_rollback_failure: false,
            json: true,
        })
        .unwrap_err();
        assert!(matches!(err, CliError::Usage(_)));

        let err = validate_cleanup_apply_flags(&CleanupApplyArgs {
            software_id: 1,
            dry_run: false,
            confirm: true,
            execution_ticket: None,
            simulate_failure: false,
            simulate_rollback_failure: false,
            json: true,
        })
        .unwrap_err();
        assert!(matches!(err, CliError::Usage(_)));
    }

    #[test]
    fn validate_cleanup_history_filters_rejects_unknown_rollback_status() {
        let err = validate_cleanup_history_filters(
            &CleanupHistoryArgs {
                limit: Some(10),
                offset: Some(0),
                status: None,
                mode: None,
                software_id: None,
                execution_ticket: None,
                rollback_status: Some("unknown".to_string()),
                contains: None,
                json: true,
            },
            10,
        )
        .unwrap_err();
        assert!(matches!(err, CliError::Usage(_)));
    }

    #[test]
    fn validate_ai_repair_plan_args_rejects_empty_fields() {
        let err = validate_ai_repair_plan_args(&AiRepairPlanArgs {
            software: " ".to_string(),
            issue: "crash on start".to_string(),
            verbose: false,
            json: true,
        })
        .unwrap_err();
        assert!(matches!(err, CliError::Usage(_)));

        let err = validate_ai_repair_plan_args(&AiRepairPlanArgs {
            software: "VS Code".to_string(),
            issue: " ".to_string(),
            verbose: false,
            json: true,
        })
        .unwrap_err();
        assert!(matches!(err, CliError::Usage(_)));
    }

    #[test]
    fn validate_ai_recommend_args_rejects_empty_goal() {
        let err = validate_ai_recommend_args(&AiRecommendArgs {
            goal: " ".to_string(),
            verbose: false,
            json: true,
        })
        .unwrap_err();
        assert!(matches!(err, CliError::Usage(_)));
    }

    #[test]
    fn validate_ui_search_args_rejects_empty_query() {
        let err = validate_ui_search_args("  ", 5).unwrap_err();
        assert!(matches!(err, CliError::Usage(_)));
    }

    #[test]
    fn validate_ui_action_id_rejects_empty() {
        let err = validate_ui_action_id("  ").unwrap_err();
        assert!(matches!(err, CliError::Usage(_)));
    }

    #[test]
    fn validate_job_type_rejects_unknown() {
        let err = validate_job_type("unknown.type").unwrap_err();
        assert!(matches!(err, CliError::Usage(_)));
    }

    #[test]
    fn validate_job_worker_run_args_requires_once() {
        let err = validate_job_worker_run_args(&JobWorkerRunArgs {
            once: false,
            json: true,
        })
        .unwrap_err();
        assert!(matches!(err, CliError::Usage(_)));
    }

    #[test]
    fn compute_worker_retry_schedule_deadletter_has_no_backoff() {
        let (backoff, next_at) = compute_worker_retry_schedule(false, true, 3, 1000, 900);
        assert_eq!(backoff, 0);
        assert_eq!(next_at, 900);
    }
}
