# Synora Phase 1 MVP Readiness Checklist

Date baseline: 2026-02-21
Last reviewed: 2026-02-22

## Scope

Phase 1 focuses on CLI MVP stability, safety boundaries, and reproducible behavior.

## Build & Test

- [x] `cargo check` passes
- [x] `cargo test` passes
- [x] CLI contract tests are green
- [x] Integration parser tests are green

## CLI Contract

- [x] `software list [--json] [--verbose]` works
- [x] `update check [--json] [--verbose]` works
- [x] `update apply --id ... [--dry-run|--confirm|--yes] [--json]` works
- [x] `config init` works
- [x] Exit code contract enforced (`0/2/3/4/10`)

## Output Stability

- [x] `software list --json` returns stable keys (`name`, `package_id`, `version`, `source`)
- [x] `update check --json` returns stable keys (`name`, `package_id`, `installed_version`, `available_version`, `source`)
- [x] `update check` text mode prints `has_updates: true|false`
- [x] `--verbose` prints `parse_path`

## Security & Integration

- [x] Security guard validates allowlisted commands only
- [x] Integration failures map to exit code `4`
- [x] `winget` JSON mode fallback to text mode works

## Config & Paths

- [x] `SYNORA_HOME` override works
- [x] Default `.synora` root resolution works
- [x] `config init` generates `config.json` with `quarantine_dir`
- [x] CLI startup can initialize `logs/synora.log`

## Documentation

- [x] `docs/cli-spec-v0.1.md` reflects current behavior
- [x] `docs/architecture/Synora_Interface_and_Module_Specification.md` reflects current behavior
- [x] `DEVELOPMENT_LOG.md` includes latest milestones
- [x] `PROJECT_STATE.md` status snapshot is current

## Release Gate

Phase 1 MVP is ready when all checklist items are complete and validated on Windows.

## Remaining Blockers

None.
