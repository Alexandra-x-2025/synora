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
13. `cargo run -- config gate-show --json`
14. `cargo run -- config gate-set --enable --approval-record docs/security/Synora_Real_Mutation_Gate_Approval_Record_2026-02-22_Draft.md --json`
15. `cargo run -- config gate-show --json`
16. `cargo run -- source suggest --json`
17. `cargo run -- cleanup quarantine --id Git.Git --dry-run --json`
18. `cargo run -- cleanup quarantine --id Git.Git --confirm --json`
19. `cargo run -- cleanup quarantine --id Git.Git --confirm --simulate-failure --json`

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
13. Returns JSON gate snapshot with `real_mutation_enabled` and `gate_version`.
14. Returns JSON with `real_mutation_enabled: true` and `approval_record_present: true`.
15. Returns updated gate JSON snapshot.
16. Returns JSON array of source recommendations (can be empty).
17. Returns JSON object for cleanup dry-run with `status: "quarantine_planned"` and rollback fields.
18. Returns JSON object for confirm path after gate enablement.
19. Returns failure JSON payload with integration exit `4` when simulated failure is requested.
