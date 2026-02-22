# Synora -- Phase 3 Quarantine Implementation Gate

Date baseline: 2026-02-22
Status: Active Gate Checklist (reviewed on 2026-02-22)
Scope: Must pass before enabling real quarantine mutation path

## 1. Contract Freeze

- [x] CLI contract frozen for `cleanup quarantine` command shape
- [x] Flag conflict rules frozen (`--dry-run` vs `--confirm`)
- [x] JSON output fields frozen for machine-readable pipeline
- [x] Exit code mapping reviewed against standard (`0/2/3/4/10`)

## 2. Status Vocabulary

- [x] `update_history.status` vocabulary frozen for quarantine lifecycle
- [x] Existing statuses kept backward-compatible
- [x] Reserved statuses documented:
- [x] `quarantine_planned`
- [x] `quarantine_confirmed`
- [x] `quarantine_success`
- [x] `quarantine_failed`
- [x] `quarantine_rollback_success`
- [x] `quarantine_rollback_failed`

## 3. Rollback Boundary

- [x] Mutation boundary explicitly implemented after backup verification
- [x] Pre-boundary failures do not mutate filesystem/registry
- [x] Post-boundary failures trigger rollback attempt
- [x] Rollback outcome persisted even when rollback fails

## 4. Repository Guarantees

- [x] `registry_backup` record exists before mutation
- [x] `quarantine` records include software linkage (`software_id`)
- [x] `update_history` stage transitions append-only
- [x] Audit summary remains consistent after failure scenarios

## 5. Security Controls

- [x] Canonical path validation enforced
- [x] Path traversal blocked
- [x] Symbolic-link escape blocked
- [x] Target roots constrained by allowlist
- [x] Confirmed mode required for HIGH/CRITICAL risk actions

## 6. Test Gates

- [x] Unit tests for stage transitions and validation branches
- [x] Repository tests for status sequence persistence
- [ ] Integration tests for rollback success/failure simulation
- [x] CLI tests for conflict flags and JSON schema
- [x] Smoke commands validated:
- [x] `cargo run -- cleanup quarantine --id Git.Git --dry-run --json`
- [x] `cargo run -- cleanup quarantine --id Git.Git --confirm --json`

## 7. Release Readiness Decision

- [ ] Security review signed off
- [x] Data consistency review signed off
- [x] Backward compatibility review signed off
- [x] Go/No-Go recorded in `DEVELOPMENT_LOG.md`

## 8. Current Decision / 当前结论

- Go (simulation path): `cleanup quarantine` dry-run/confirm simulation flows are acceptable for Phase 3 development and testing.
- No-Go (real mutation path): blocked until all Security Controls are completed and security review is signed off.
