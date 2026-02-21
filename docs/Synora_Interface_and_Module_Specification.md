# Synora -- Deep Interface & Module Specification

# Synora -- 深度接口与模块规范

Generated on: 2026-02-21 22:37:46

------------------------------------------------------------------------

## 1. CLI Command Contracts / CLI 命令契约

### synora list

Purpose: Enumerate installed software. Output: Table or JSON. Flags:
--json --verbose

### synora check

Purpose: Detect available updates. Flags: --json --dry-run

### synora update

Purpose: Perform safe update. Flags: --confirm --dry-run --verbose

### synora cleanup

Purpose: Quarantine orphaned files. Flags: --confirm --dry-run

------------------------------------------------------------------------

## 2. Core Module Interfaces / 核心模块接口

### Domain Module

Responsibilities: - Risk evaluation - Policy enforcement

### Service Module

Responsibilities: - Update orchestration - Cleanup planning

### Repository Module

Responsibilities: - SQLite CRUD - Transaction boundaries

### Security Guard

Responsibilities: - Parameter validation - Command whitelist - Registry
snapshot enforcement

------------------------------------------------------------------------

## 3. Cross-Module Rules / 模块交互规则

-   Domain must not access Integration directly.
-   Service coordinates Repository + Domain.
-   Security Guard validates before Integration.
