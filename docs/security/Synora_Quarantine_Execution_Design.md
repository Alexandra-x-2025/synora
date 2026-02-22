# Synora -- Quarantine Execution Design (Draft v0.1)

# Synora -- Quarantine 实执行设计（草案 v0.1）

Date: 2026-02-22
Status: Draft for Phase 3 Entry
Scope: Design only, no runtime behavior change in v0.1

---

## 1. Goals / 目标

English:
- Define a safe execution path for real quarantine actions in future versions.
- Keep strong rollback boundaries before destructive operations.
- Ensure every action is auditable in SQLite.

中文:
- 为未来版本的 quarantine 实执行定义安全路径。
- 在破坏性操作前保持明确回滚边界。
- 确保每一步动作都可在 SQLite 中审计。

---

## 2. Non-Goals / 非目标

English:
- No real file move/delete in current v0.1 runtime.
- No privilege escalation implementation in this draft.
- No GUI workflow in this phase.

中文:
- 当前 v0.1 运行时不做真实文件移动/删除。
- 本草案不实现提权逻辑。
- 本阶段不涉及 GUI 流程。

---

## 3. Proposed Execution Contract (Future v0.2+) / 建议执行契约（未来 v0.2+）

Proposed command shape:
- `synora cleanup quarantine --id <package_id> [--dry-run|--confirm] [--json]`

Contract:
- Default mode is `--dry-run`.
- `--confirm` required for real mutation.
- `--dry-run` and `--confirm` are mutually exclusive.
- Exit code contract reuses existing standard:
- `0` success
- `2` usage/validation
- `3` security blocked
- `4` integration/runtime failure
- `10` unexpected internal

---

## 4. State Machine / 状态机

Stages:
1. `plan_created`
2. `backup_verified`
3. `quarantine_written`
4. `cleanup_committed`
5. `rollback_applied` (if any failure after mutation boundary)

Mutation boundary:
- Boundary starts only after `backup_verified`.
- Any failure after boundary must trigger rollback attempt and log outcome.

---

## 5. Data Persistence Rules / 数据持久化规则

Required records:
- `update_history`: append operation status transitions.
- `registry_backup`: must exist before any registry mutation.
- `quarantine`: one row per file operation candidate.

Planned status vocabulary:
- `quarantine_planned`
- `quarantine_confirmed`
- `quarantine_success`
- `quarantine_failed`
- `quarantine_rollback_success`
- `quarantine_rollback_failed`

Status mapping into existing `update_history.status`:

| Category | Status Value | Source | Notes |
| --- | --- | --- | --- |
| Update Plan | `planned_dry_run` | Existing | v0.1 `update apply --dry-run` |
| Update Plan | `planned_confirmed` | Existing | v0.1 `update apply --confirm/--yes` |
| Quarantine Plan | `quarantine_planned` | Reserved | Phase 3 dry-run write path |
| Quarantine Confirm | `quarantine_confirmed` | Reserved | Phase 3 confirmed execution start |
| Quarantine Result | `quarantine_success` | Reserved | Mutation completed successfully |
| Quarantine Result | `quarantine_failed` | Reserved | Mutation failed |
| Rollback Result | `quarantine_rollback_success` | Reserved | Rollback completed |
| Rollback Result | `quarantine_rollback_failed` | Reserved | Rollback failed, requires operator attention |

---

## 6. Security Boundaries / 安全边界

Security Guard policy:
- Allowlist target roots only (for example install roots and configured quarantine root).
- Reject path traversal and non-canonical paths.
- Reject symbolic-link escape.
- Require explicit confirmation for HIGH/CRITICAL paths.

Risk mapping:
- LOW: metadata-only audit
- MEDIUM: staging and copy checks
- HIGH: registry-related mutation
- CRITICAL: system-level cleanup affecting protected paths

---

## 7. Failure Semantics / 失败语义

Failure policy:
- Before mutation boundary: fail-fast, no rollback needed.
- After mutation boundary: best-effort rollback required.
- Rollback failure must not overwrite original failure; both should be logged.

Output requirement:
- Text mode prints summary with rollback result when relevant.
- JSON mode includes:
- `operation_id`
- `stage`
- `status`
- `rollback_attempted`
- `rollback_status`

---

## 8. Testing Entry Criteria / 测试准入标准

Required before implementation:
- Unit tests for stage transitions and validation paths.
- Repository tests for status sequence persistence.
- Integration tests for simulated rollback success/failure.
- CLI contract tests for conflicting flags and machine-readable fields.

Suggested smoke additions:
- `cargo run -- cleanup quarantine --id Git.Git --dry-run --json`
- `cargo run -- cleanup quarantine --id Git.Git --confirm --json`

---

## 9. Open Decisions / 待决策项

- Should quarantine action use package-level lock to prevent concurrent mutation?
- Should rollback be mandatory-sync or background with durable retry queue?
- Should `registry_backup.backup_blob` move to structured JSON schema versioning?

---

## 10. Adoption Plan / 采用计划

Phase 3 Step 1:
- Freeze execution contract and status vocabulary.

Phase 3 Step 2:
- Implement dry-run only with full audit persistence.

Phase 3 Step 3:
- Implement confirmed mutation path with rollback hooks.

Phase 3 Step 4:
- Add hardening tests and release gate checklist.
