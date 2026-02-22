# Synora CLI Smoke Checklist

Date baseline: 2026-02-21

## Purpose

Quick regression checks for CLI contract and integration behavior.

## Commands

1. `cargo check`
2. `cargo test`
3. `cargo run -- software list --json`
4. `cargo run -- software list --verbose`
5. `cargo run -- update check --json`
6. `cargo run -- update check --verbose`
7. `cargo run -- update apply --id Git.Git --yes --json`
8. `cargo run -- update apply --id Git.Git --dry-run --confirm`
9. `cargo run -- config init`
10. `cargo run -- config db-list --json`
11. `cargo run -- config history-list --json`
12. `cargo run -- config audit-summary --json`
13. `cargo run -- source suggest --json`
14. `cargo run -- cleanup quarantine --id Git.Git --dry-run --json`
15. `cargo run -- cleanup quarantine --id Git.Git --confirm --json`
16. `cargo run -- cleanup quarantine --id Git.Git --confirm --simulate-failure --json`

## Expected Outcomes

1. Build succeeds.
2. Tests pass.
3. JSON array output (`[]` or populated list).
4. Text output plus `db_sync_count: <n>` and `parse_path: ...`.
5. JSON array output (`[]` or populated list) with stable fields.
6. Text output plus `has_updates: true|false` and `parse_path: ...`.
7. JSON object output with `requested_mode: "confirm"`.
8. Fails with usage validation and exit code `2`.
9. Prints config path under `.synora` (or under `SYNORA_HOME` if set).
10. Returns JSON array of persisted software records (can be empty).
11. Returns JSON array of persisted update history records (can be empty).
12. Returns JSON object with audit aggregates (`total`, `planned_confirmed`, `planned_dry_run`, `latest_timestamp`).
13. Returns JSON array of source recommendations (can be empty).
14. Returns JSON object for cleanup dry-run with `status: "quarantine_planned"` and rollback fields.
15. Returns JSON object for cleanup confirm simulated execution with `status: "quarantine_success"` and `mutation_boundary_reached: true`.
16. Returns JSON payload with failure statuses and exits with code `4`; history should include rollback status row.
