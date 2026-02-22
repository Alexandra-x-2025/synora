# Synora -- Cleanup Quarantine CLI Contract (Draft v0.2)

# Synora -- Cleanup Quarantine CLI 契约（草案 v0.2）

Date: 2026-02-22
Status: Draft (Phase 3, M1+M2 precheck path implemented)
Compatibility: Does not modify frozen `v0.1` contract

---

## 1. Command Shape / 命令形态

Executable: `synora`

Command:
- `synora cleanup quarantine --id <package_id> [--dry-run|--confirm] [--json] [--verbose]`

Rules:
- `--id` is required.
- Default mode is `--dry-run` when neither mode flag is provided.
- `--dry-run` and `--confirm` are mutually exclusive.
- `--json` enables machine-readable output.
- `--verbose` is text-mode diagnostic output; ignored in JSON mode.

---

## 2. Behavior Contract / 行为契约

Dry-run path:
- No file or registry mutation.
- Must persist audit intent (`quarantine_planned`) in `update_history.status`.

Confirmed path:
- Requires explicit confirmation via `--confirm`.
- Must verify safety preconditions before mutation boundary.
- Must persist lifecycle statuses:
- `quarantine_confirmed`
- `quarantine_success` or `quarantine_failed`
- optional rollback status:
- `quarantine_rollback_success`
- `quarantine_rollback_failed`

---

## 3. Output Contract / 输出契约

JSON minimum fields:
- `operation_id` (string, stable for one command execution)
- `package_id` (string)
- `requested_mode` (`dry-run` or `confirm`)
- `mode` (`plan-only` or `confirmed-execution`)
- `status` (status vocabulary above)
- `mutation_boundary_reached` (bool; M1/M2 always `false`)
- `rollback_attempted` (bool)
- `rollback_status` (`not_needed` / `success` / `failed`)
- `message` (string)

Text mode minimum summary:
- `operation_id: ...`
- `status: ...`
- `rollback_status: ...`

Verbose text additions:
- `precheck_paths: ...`
- `mutation_boundary_reached: true|false`
- `audit_rows_written: <n>`

---

## 4. Error and Exit Codes / 错误与退出码

Code mapping:
- `0`: success
- `2`: invalid usage / argument conflict / missing `--id`
- `3`: security blocked by policy guard
- `4`: integration/runtime failure
- `10`: unexpected internal error

Failure semantics:
- If failure happens before mutation boundary, rollback is `not_needed`.
- If failure happens after mutation boundary, rollback attempt is mandatory and reported.

---

## 5. Security Requirements / 安全要求

- Security Guard validates all external actions.
- Target paths must be canonical and allowlisted.
- Symbolic-link escape must be rejected.
- HIGH/CRITICAL actions must require `--confirm`.

---

## 6. Persistence Requirements / 持久化要求

Tables:
- `update_history`: lifecycle transitions
- `quarantine`: file-level candidate rows and outcomes
- `registry_backup`: required before registry mutation

Sequence requirements:
1. persist `quarantine_planned` (or `quarantine_confirmed`)
2. write pre-mutation backup evidence
3. execute mutation path
4. persist terminal status (`quarantine_success` / `quarantine_failed`)
5. if needed, persist rollback terminal status

---

## 7. Compatibility Strategy / 兼容策略

- Existing commands remain unchanged in Phase 3 draft.
- `update_history` existing statuses remain valid:
- `planned_dry_run`
- `planned_confirmed`
- New quarantine statuses are additive only.

---

## 8. Open Questions / 待决策

- Should `operation_id` use UUIDv4 or timestamp+counter strategy?
- Should confirmed mode require second factor in enterprise policy profile?
- Should rollback failure trigger dedicated alert channel in later CI/ops integration?
