# Synora -- Cleanup Quarantine Implementation Workplan (Phase 3)

# Synora -- Cleanup Quarantine 实施任务分解（Phase 3）

Date: 2026-02-22
Status: Draft Workplan
Depends on:
- `docs/security/Synora_Quarantine_Execution_Design.md`
- `docs/architecture/Synora_Cleanup_Quarantine_CLI_Contract_Draft.md`
- `docs/testing/Phase3_Quarantine_Implementation_Gate.md`

---

## 1. Objective / 目标

English:
- Convert Phase 3 quarantine contract draft into implementation-ready tasks by module.
- Keep delivery incremental: dry-run first, confirmed mutation after gates.

中文:
- 将 Phase 3 quarantine 契约草案拆解为可执行模块任务。
- 采用增量交付：先 dry-run，再进入 confirmed 实执行。

---

## 2. Work Breakdown by Module / 按模块分解

### 2.1 CLI Layer (`src/cli/`)

Tasks:
- Add command route: `cleanup quarantine`.
- Add argument parsing for `--id`, `--dry-run`, `--confirm`, `--json`, `--verbose`.
- Enforce conflict rule between `--dry-run` and `--confirm`.
- Emit stable output contract fields.

Acceptance:
- Unknown flags return exit `2`.
- Missing `--id` returns exit `2`.
- JSON fields match draft schema.

### 2.2 Service Layer (`src/service/`)

Tasks:
- Add `CleanupService` orchestration entrypoint.
- Implement stage transitions:
- `quarantine_planned`
- `quarantine_confirmed`
- `quarantine_success` / `quarantine_failed`
- `quarantine_rollback_success` / `quarantine_rollback_failed`
- Generate stable `operation_id`.
- Record rollback attempt semantics.

Acceptance:
- Stage transitions are append-only in persistence.
- Mutation boundary semantics match design doc.

### 2.3 Repository Layer (`src/repository/`)

Tasks:
- Add repository APIs for quarantine lifecycle writes.
- Add query helpers for operation-level audit retrieval.
- Ensure writes remain transactional where needed.

Acceptance:
- `update_history` sequence persists correctly across success/failure.
- `quarantine` rows always linked by `software_id`.
- `registry_backup` evidence written before mutation stage.

### 2.4 Security Layer (`src/security/`)

Tasks:
- Add canonical path checks and traversal rejection helpers.
- Add symbolic-link escape checks.
- Add risk-level gating rule for confirmed mode.

Acceptance:
- Security violations map to exit `3`.
- Path policy decisions are deterministic and testable.

### 2.5 Integration Layer (`src/integration/`)

Tasks:
- Add filesystem adapter for quarantine staging operations.
- Add registry adapter hooks for backup/restore placeholders.
- Surface errors with clear stage context for service rollback handling.

Acceptance:
- Integration failures map to exit `4`.
- Adapter errors include enough context for audit logs.

### 2.6 Testing Layer (`src/* tests + docs/testing`)

Tasks:
- Add CLI contract tests for new command and flag behavior.
- Add service tests for stage machine and rollback branches.
- Add repository tests for persistence sequence and summary consistency.
- Add integration simulation tests for rollback success/failure.
- Extend smoke checklist commands when implementation begins.

Acceptance:
- Phase 3 gate checklist can be checked item-by-item with evidence.

---

## 3. Milestones / 里程碑

M1 (Dry-run foundation):
- CLI parsing + dry-run orchestration + `quarantine_planned` persistence.

M2 (Confirmed prechecks):
- Security prechecks + backup evidence writes + boundary markers.

M3 (Confirmed execution):
- Mutation path + success/failure transitions + rollback framework.

M4 (Hardening):
- Full tests + docs sync + gate checklist closure review.

---

## 4. Recommended Implementation Order / 建议实施顺序

1. CLI parser skeleton and exit-code contract.
2. Service stage orchestration without real mutation.
3. Repository operation helpers and sequence assertions.
4. Security path validation and risk gates.
5. Integration adapter hooks and rollback wiring.
6. Test hardening and gate checklist closure.

---

## 5. Definition of Done / 完成定义

- Gate checklist in `docs/testing/Phase3_Quarantine_Implementation_Gate.md` fully checked.
- CLI output contract stable and documented.
- Rollback behavior tested for both success and failure paths.
- `DEVELOPMENT_LOG.md` contains Go/No-Go decision record.
