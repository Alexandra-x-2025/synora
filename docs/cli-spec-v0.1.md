# Synora CLI Specification v0.1

Status: Frozen  
Scope: Phase 1 CLI MVP baseline

## 1. Goals

Synora CLI v0.1 provides a safety-first command interface for Windows software lifecycle tasks.

Design priorities:
- Explicit user control
- Explainable output
- Recoverable operations
- Security guard as mandatory boundary

## 2. Command Set

Executable name: `synora`

### 2.1 `synora software list`

Purpose:
- List installed software from supported sources.
- Persist local software snapshot into SQLite repository.

Options:
- `--json`: output structured JSON
- `--verbose`: include parser path details in text mode

Text mode summary:
- Prints `db_sync_count: <n>` after table output.

Exit codes:
- `0`: success
- `4`: integration failure

Failure semantics:
- If source invocation fails (for example `winget` non-zero exit), command must return `4`.

### 2.2 `synora update check`

Purpose:
- Check available updates for installed software.

Options:
- `--json`: output structured JSON
- `--verbose`: include parser path details in text mode

JSON contract (minimum keys per item):
- `name`
- `package_id`
- `installed_version`
- `available_version`
- `source`

Text mode summary:
- Must print `has_updates: true|false` after table output

Exit codes:
- `0`: success
- `4`: integration failure

Failure semantics:
- If source invocation fails (for example `winget` non-zero exit), command must return `4`.

### 2.3 `synora update apply --id <package_id> [--dry-run | --confirm] [--json]`

Purpose:
- Prepare a safe update plan for a specific package.

Behavior:
- Default behavior returns a dry-run plan.
- `--confirm` marks explicit user confirmation for high-risk path.
- Legacy `--yes` is accepted as alias of `--confirm`.
- v0.1 does not perform real installer execution yet.
- Each successful plan is persisted into SQLite `update_history` as audit record:
- `planned_dry_run` when unconfirmed
- `planned_confirmed` when `--confirm`/`--yes` is used

JSON contract (minimum keys):
- `package_id`
- `risk`
- `requested_mode`
- `mode`
- `message`

Exit codes:
- `0`: plan generated
- `2`: invalid input
- `3`: security policy violation

### 2.4 `synora config init`

Purpose:
- Create initial config file in user profile.
- Uses `SYNORA_HOME` when provided.
- Default root is user home `.synora`; fallback is current working directory `.synora` when home path is unavailable.
- Initializes SQLite schema baseline (`db/synora.db`) if not present.

Exit codes:
- `0`: success

### 2.5 `synora config db-list [--json]` (Phase 2 Read-only Utility)

Purpose:
- Inspect persisted `software` records in local SQLite repository.

Notes:
- Read-only helper for repository visibility validation.
- Does not modify system state.

### 2.6 `synora config history-list [--json]` (Phase 2 Audit Utility)

Purpose:
- Read-only listing of persisted `update_history` audit records.

Notes:
- Validates `update apply` persistence behavior for planned operations.
- Does not modify system state.

### 2.7 `synora config audit-summary [--json]` (Phase 2 Audit Utility)

Purpose:
- Read aggregated update audit metrics from SQLite `update_history`.

Fields:
- `total`
- `planned_confirmed`
- `planned_dry_run`
- `latest_timestamp` (nullable)

### 2.8 `synora source suggest [--json] [--verbose]` (Phase 2 Week 2 Prototype)

Purpose:
- Generate source recommendation candidates from persisted software snapshot.
- Blend update-check signal into recommendation score when update candidates are detected.

Output:
- Default: table with score and reasons
- `--json`: structured recommendation array
- `--verbose` (text mode): print signal summary (`recommendation_count`, `update_signal_hits`, `high_confidence_count`, `signal_mode`)

## 3. Global Behavior

### 3.1 Logging
- Log file location: `~/.synora/logs/synora.log`
- Override root directory with env var: `SYNORA_HOME`
- Minimum fields: timestamp, level, message

### 3.2 Output Modes
- Human-readable text by default
- Machine-readable JSON via `--json`

### 3.3 Error Contract

Common exit codes:
- `0`: success
- `2`: invalid usage or argument
- `3`: security blocked
- `4`: integration/runtime failure
- `10`: unexpected internal error

## 4. Security Rules (v0.1)

Aligned with AD-002 and AD-004:
- No arbitrary shell command execution
- All system interactions must go through Security Guard
- High-risk actions require explicit user confirmation
- Destructive cleanup actions remain non-destructive in MVP

## 5. Non-Goals (v0.1)

Not included in v0.1:
- Full uninstall workflow
- Real quarantine file mover
- Registry backup execution
- GUI interface

## 6. Versioning

This file defines baseline behavior for `v0.1`.
Future versions must preserve backward compatibility or document breaking changes.
