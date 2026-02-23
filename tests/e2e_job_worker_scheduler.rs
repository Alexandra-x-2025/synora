use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection};
use serde_json::Value;

fn unique_home() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let pid = std::process::id();
    let dir = std::env::temp_dir().join(format!("synora-e2e-{pid}-{nanos}"));
    fs::create_dir_all(&dir).expect("failed to create temp home");
    dir
}

fn run_synora(home: &Path, args: &[&str]) -> Output {
    let exe = env!("CARGO_BIN_EXE_synora");
    Command::new(exe)
        .args(args)
        .env("SYNORA_HOME", home)
        .output()
        .expect("failed to run synora")
}

fn stdout_json(output: &Output) -> Value {
    let s = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str::<Value>(s.trim()).expect("stdout should be valid json")
}

fn db_path(home: &Path) -> PathBuf {
    home.join("db").join("synora.db")
}

#[test]
fn worker_run_success_download_fetch() {
    let home = unique_home();

    let out = run_synora(
        &home,
        &[
            "job",
            "submit",
            "--type",
            "download.fetch",
            "--priority",
            "100",
            "--payload",
            r#"{"package_id":"public_default.sample"}"#,
            "--json",
        ],
    );
    assert!(out.status.success(), "submit failed: {:?}", out);

    let out = run_synora(&home, &["job", "worker-run", "--once", "--json"]);
    assert!(out.status.success(), "worker-run failed: {:?}", out);
    let payload = stdout_json(&out);
    assert_eq!(payload["picked"], true);
    assert_eq!(payload["new_status"], "success");
    assert_eq!(payload["job_type"], "download.fetch");

    let out = run_synora(
        &home,
        &["job", "list", "--json", "--status", "success", "--limit", "5"],
    );
    assert!(out.status.success(), "job list failed: {:?}", out);
    let listed = stdout_json(&out);
    assert!(listed.as_array().is_some_and(|arr| !arr.is_empty()));
}

#[test]
fn worker_run_failure_retrying_then_scheduler_requeue() {
    let home = unique_home();

    let out = run_synora(
        &home,
        &[
            "job",
            "submit",
            "--type",
            "download.verify",
            "--priority",
            "100",
            "--payload",
            r#"{"job_id":"sched-test","simulate_failure":true}"#,
            "--json",
        ],
    );
    assert!(out.status.success(), "submit failed: {:?}", out);

    let out = run_synora(&home, &["job", "worker-run", "--once", "--json"]);
    assert!(out.status.success(), "worker-run failed: {:?}", out);
    let payload = stdout_json(&out);
    assert_eq!(payload["new_status"], "retrying");
    assert_eq!(payload["backoff_seconds"], 5);
    let job_id = payload["job_id"].as_i64().expect("job_id should be i64");

    let conn = Connection::open(db_path(&home)).expect("open db");
    conn.execute(
        "UPDATE job_queue SET scheduled_at = ?1 WHERE id = ?2",
        params![0_i64, job_id],
    )
    .expect("force schedule_at");

    let out = run_synora(&home, &["job", "scheduler-run", "--json", "--limit", "10"]);
    assert!(out.status.success(), "scheduler-run failed: {:?}", out);
    let scheduled = stdout_json(&out);
    assert_eq!(scheduled["updated"], 1);

    let out = run_synora(
        &home,
        &["job", "list", "--json", "--status", "queued", "--limit", "10"],
    );
    assert!(out.status.success(), "job list queued failed: {:?}", out);
    let listed = stdout_json(&out);
    let found = listed.as_array().is_some_and(|arr| {
        arr.iter().any(|it| it["id"] == job_id && it["status"] == "queued")
    });
    assert!(found, "queued job should exist after scheduler-run");
}

#[test]
fn worker_run_deadletter_after_max_attempts() {
    let home = unique_home();

    let out = run_synora(
        &home,
        &[
            "job",
            "submit",
            "--type",
            "download.verify",
            "--priority",
            "100",
            "--payload",
            r#"{"job_id":"dlq-test","simulate_failure":true}"#,
            "--json",
        ],
    );
    assert!(out.status.success(), "submit failed: {:?}", out);

    let out = run_synora(&home, &["job", "worker-run", "--once", "--json"]);
    assert!(out.status.success(), "worker-run #1 failed: {:?}", out);
    let first = stdout_json(&out);
    let job_id = first["job_id"].as_i64().expect("job_id should be i64");
    assert_eq!(first["new_status"], "retrying");

    let conn = Connection::open(db_path(&home)).expect("open db");
    conn.execute(
        "UPDATE job_queue SET attempt_count = 2, status = 'retrying', scheduled_at = ?1 WHERE id = ?2",
        params![0_i64, job_id],
    )
    .expect("force attempt_count");

    let out = run_synora(&home, &["job", "scheduler-run", "--json", "--limit", "10"]);
    assert!(out.status.success(), "scheduler-run failed: {:?}", out);

    let out = run_synora(&home, &["job", "worker-run", "--once", "--json"]);
    assert!(out.status.success(), "worker-run #final failed: {:?}", out);
    let final_run = stdout_json(&out);
    assert_eq!(final_run["job_id"], job_id);
    assert_eq!(final_run["new_status"], "deadletter");
    assert_eq!(final_run["deadlettered"], true);
    assert_eq!(final_run["backoff_seconds"], 0);
}

#[test]
fn scheduler_run_rejects_zero_limit() {
    let home = unique_home();
    let out = run_synora(&home, &["job", "scheduler-run", "--json", "--limit", "0"]);
    assert_eq!(out.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("--limit must be >= 1"),
        "unexpected stderr: {stderr}"
    );
}
