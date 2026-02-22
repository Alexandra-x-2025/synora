# Synora -- Phase 3 Quarantine Implementation Gate

Date baseline: 2026-02-22
Status: Active Gate Checklist
Scope: Must pass before enabling real quarantine mutation path

## 1. Contract Freeze

- [ ] CLI contract frozen for `cleanup quarantine` command shape
- [ ] Flag conflict rules frozen (`--dry-run` vs `--confirm`)
- [ ] JSON output fields frozen for machine-readable pipeline
- [ ] Exit code mapping reviewed against standard (`0/2/3/4/10`)

## 2. Status Vocabulary

- [ ] `update_history.status` vocabulary frozen for quarantine lifecycle
- [ ] Existing statuses kept backward-compatible
- [ ] Reserved statuses documented:
- [ ] `quarantine_planned`
- [ ] `quarantine_confirmed`
- [ ] `quarantine_success`
- [ ] `quarantine_failed`
- [ ] `quarantine_rollback_success`
- [ ] `quarantine_rollback_failed`

## 3. Rollback Boundary

- [ ] Mutation boundary explicitly implemented after backup verification
- [ ] Pre-boundary failures do not mutate filesystem/registry
- [ ] Post-boundary failures trigger rollback attempt
- [ ] Rollback outcome persisted even when rollback fails

## 4. Repository Guarantees

- [ ] `registry_backup` record exists before mutation
- [ ] `quarantine` records include software linkage (`software_id`)
- [ ] `update_history` stage transitions append-only
- [ ] Audit summary remains consistent after failure scenarios

## 5. Security Controls

- [ ] Canonical path validation enforced
- [ ] Path traversal blocked
- [ ] Symbolic-link escape blocked
- [ ] Target roots constrained by allowlist
- [ ] Confirmed mode required for HIGH/CRITICAL risk actions

## 6. Test Gates

- [ ] Unit tests for stage transitions and validation branches
- [ ] Repository tests for status sequence persistence
- [ ] Integration tests for rollback success/failure simulation
- [ ] CLI tests for conflict flags and JSON schema
- [ ] Smoke commands validated:
- [ ] `cargo run -- cleanup quarantine --id Git.Git --dry-run --json`
- [ ] `cargo run -- cleanup quarantine --id Git.Git --confirm --json`

## 7. Release Readiness Decision

- [ ] Security review signed off
- [ ] Data consistency review signed off
- [ ] Backward compatibility review signed off
- [ ] Go/No-Go recorded in `DEVELOPMENT_LOG.md`
