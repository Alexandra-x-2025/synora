# Synora Phase 1 MVP Readiness Checklist

Date baseline: 2026-02-21

## Scope

Phase 1 focuses on CLI MVP stability, safety boundaries, and reproducible behavior.

## Build & Test

- [ ] `cargo check` passes
- [ ] `cargo test` passes
- [ ] CLI contract tests are green
- [ ] Integration parser tests are green

## CLI Contract

- [ ] `software list [--json] [--verbose]` works
- [ ] `update check [--json] [--verbose]` works
- [ ] `update apply --id ... [--dry-run|--confirm|--yes] [--json]` works
- [ ] `config init` works
- [ ] Exit code contract enforced (`0/2/3/4/10`)

## Output Stability

- [ ] `software list --json` returns stable keys (`name`, `package_id`, `version`, `source`)
- [ ] `update check --json` returns stable keys (`name`, `package_id`, `installed_version`, `available_version`, `source`)
- [ ] `update check` text mode prints `has_updates: true|false`
- [ ] `--verbose` prints `parse_path`

## Security & Integration

- [ ] Security guard validates allowlisted commands only
- [ ] Integration failures map to exit code `4`
- [ ] `winget` JSON mode fallback to text mode works

## Config & Paths

- [ ] `SYNORA_HOME` override works
- [ ] Default `.synora` root resolution works
- [ ] `config init` generates `config.json` with `quarantine_dir`
- [ ] CLI startup can initialize `logs/synora.log`

## Documentation

- [ ] `docs/cli-spec-v0.1.md` reflects current behavior
- [ ] `docs/architecture/Synora_Interface_and_Module_Specification.md` reflects current behavior
- [ ] `DEVELOPMENT_LOG.md` includes latest milestones
- [ ] `PROJECT_STATE.md` status snapshot is current

## Release Gate

Phase 1 MVP is ready when all checklist items are complete and validated on Windows.
