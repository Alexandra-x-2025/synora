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

## Expected Outcomes

1. Build succeeds.
2. Tests pass.
3. JSON array output (`[]` or populated list).
4. Text output plus `parse_path: ...`.
5. JSON array output (`[]` or populated list) with stable fields.
6. Text output plus `has_updates: true|false` and `parse_path: ...`.
7. JSON object output with `requested_mode: "confirm"`.
8. Fails with usage validation and exit code `2`.
9. Prints config path under `.synora` (or under `SYNORA_HOME` if set).
