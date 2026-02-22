# Synora -- Security Sign-off Checklist (Phase 3)

Date baseline: 2026-02-22
Status: Pending Sign-off
Scope: Required before real cleanup mutation go-live

## 1. Review Scope Confirmation

- [ ] Scope confirmed: this sign-off covers real cleanup mutation path only
- [ ] Scope confirmed: simulation path remains available for development/testing
- [ ] Version/tag of reviewed code recorded

## 2. Policy and Threat Alignment

- [ ] Threat model reviewed against current implementation:
- [ ] Malicious installer injection
- [ ] Registry tampering
- [ ] Orphan cleanup overreach
- [ ] Privilege escalation attempt
- [ ] Policy requirements mapped to controls:
- [ ] allowlist boundaries
- [ ] confirmation gates
- [ ] rollback semantics

## 3. Security Controls Evidence

- [ ] Canonical path normalization evidence attached
- [ ] Path traversal blocking evidence attached
- [ ] Symbolic-link escape blocking evidence attached
- [ ] Target allowlist root enforcement evidence attached
- [ ] HIGH/CRITICAL confirmation gate evidence attached

## 4. Rollback and Failure Handling

- [ ] Failure path writes `quarantine_failed`
- [ ] Rollback success path writes `quarantine_rollback_success`
- [ ] Rollback failure path writes `quarantine_rollback_failed`
- [ ] Exit code behavior reviewed for failed operations (`4`)

## 5. Audit and Data Integrity

- [ ] `update_history` transition sequence reviewed (append-only)
- [ ] `quarantine` rows linked to `software_id`
- [ ] `registry_backup` evidence present before mutation boundary
- [ ] `config history-list` and `config audit-summary` outputs reviewed for consistency

## 6. Test Evidence

- [ ] `cargo test` report archived (all tests passing)
- [ ] CLI smoke evidence archived:
- [ ] `cleanup quarantine --id <id> --dry-run --json`
- [ ] `cleanup quarantine --id <id> --confirm --json`
- [ ] `cleanup quarantine --id <id> --confirm --simulate-failure --json`
- [ ] `cleanup quarantine --id <id> --confirm --simulate-failure --simulate-rollback-failure --json`
- [ ] Negative security cases archived (traversal and high-risk-no-confirm)

## 7. Decision

- [ ] Security sign-off: Approved
- [ ] Security sign-off: Rejected
- [ ] Decision rationale recorded in `DEVELOPMENT_LOG.md`

Signer:
- Name:
- Date:
- Decision:
- Notes:
