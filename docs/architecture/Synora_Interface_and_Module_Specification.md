# Synora -- Interface & Module Specification (Frozen v0.1)

# Synora -- 接口与模块规范（冻结版 v0.1）

Last updated: 2026-02-21
Status: Frozen for v0.1

---

## 1. CLI Contract / CLI 契约

Executable: `synora`

### 1.1 `synora software list [--json] [--verbose]`

Purpose:
- Enumerate installed software.
- Persist repository snapshot for discovered software entries.

Output:
- Default: human-readable table
- `--json`: machine-readable JSON array
- `--verbose`: append parser path in text mode

Text summary contract:
- print `db_sync_count: <n>` after table output

### 1.2 `synora update check [--json] [--verbose]`

Purpose:
- Detect available updates.

Output:
- Default: human-readable table
- `--json`: machine-readable JSON array
- `--verbose`: append parser path in text mode

JSON minimum fields:
- `name`
- `package_id`
- `installed_version`
- `available_version`
- `source`

Text summary contract:
- print `has_updates: true|false` after table output

### 1.3 `synora update apply --id <package_id> [--dry-run | --confirm] [--json]`

Purpose:
- Create or confirm update plan for one package.

Rules:
- Default mode is plan-only (`--dry-run` semantics).
- `--confirm` marks explicit user confirmation for high-risk path.
- v0.1 remains plan-only execution (no real installer run yet).
- `--json` returns structured plan payload.
- Each successful plan must persist an audit event in `update_history`:
- `planned_dry_run` for unconfirmed plans
- `planned_confirmed` for confirmed plans
- Confirmed plans also persist placeholder safety audit rows in `registry_backup` and `quarantine` tables.

JSON minimum fields:
- `package_id`
- `risk`
- `requested_mode`
- `mode`
- `message`

Compatibility:
- Legacy flag `--yes` is accepted as alias of `--confirm`.

### 1.4 `synora config init`

Purpose:
- Initialize local configuration file.
- Bootstrap local database schema baseline when missing.

Path rules:
- Use `SYNORA_HOME` when provided.
- Default root is user home `.synora`.
- Fallback to working directory `.synora` if home root is unavailable.

### 1.5 `synora config db-list [--json]` (Phase 2 utility)

Purpose:
- Read-only listing of persisted software entries from SQLite repository.

### 1.6 `synora config history-list [--json]` (Phase 2 utility)

Purpose:
- Read-only listing of persisted `update_history` audit events from SQLite repository.

### 1.7 `synora config audit-summary [--json]` (Phase 2 utility)

Purpose:
- Read-only aggregate metrics from `update_history` for operational diagnostics.

### 1.8 `synora config gate-show [--json]` (Phase 3 utility)

Purpose:
- Read-only visibility of real mutation gate config (`execution.*`).

### 1.9 `synora config gate-set (--enable|--disable) [--confirm] [--approval-record <ref>] [--gate-version <version>] [--keep-record] [--json]` (Phase 3 utility)

Purpose:
- Write execution gate config (`execution.*`) in a controlled CLI path.

### 1.10 `synora source suggest [--json] [--verbose]` (Phase 2 Week 2 prototype)

Purpose:
- Produce source recommendation candidates with score and reasons.
- Blend persisted snapshot and update-check signal for scoring.

---

## 2. Error Code Contract / 错误码契约

- `0`: success
- `2`: invalid usage / invalid argument
- `3`: security policy blocked
- `4`: integration/runtime failure
- `10`: unexpected internal failure

Integration rule:
- External command non-zero exit must map to `4` (not silent empty result).

---

## 3. Module Boundaries / 模块边界

### 3.1 Domain

Responsibilities:
- Data models
- Risk classification

Constraints:
- No IO
- No system calls

### 3.2 Repository

Responsibilities:
- Local config/data persistence
- Future SQLite access layer

### 3.3 Service

Responsibilities:
- Workflow orchestration
- Policy coordination across Domain + Integration

### 3.4 Worker

Responsibilities:
- Retry execution
- Future concurrency and cancellation hooks

### 3.5 Integration

Responsibilities:
- External system adapters (winget, registry, filesystem)

### 3.6 Security Guard

Responsibilities:
- Command allowlist enforcement
- Pre-execution validation
- Final mandatory boundary for system-level actions

---

## 4. Cross-Module Rules / 跨模块规则

- Domain cannot call Integration directly.
- Service coordinates Domain + Repository + Integration.
- Integration must route risky system actions via Security Guard.
- CLI layer does not bypass Service or Security Guard.

---

## 5. Stability Rules / 稳定性规则

For `v0.1`:
- Command names and exit codes are frozen.
- Breaking changes require new spec version section.
- New flags must remain backward compatible.

---

## 6. Phase 3 Draft References / Phase 3 草案引用

- `docs/security/Synora_Quarantine_Execution_Design.md`
- `docs/architecture/Synora_Cleanup_Quarantine_CLI_Contract_Draft.md`
