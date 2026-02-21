# Synora -- Database ER Design

# Synora -- 数据库 ER 设计

Generated on: 2026-02-21 22:37:46

------------------------------------------------------------------------

## Core Tables / 核心数据表

software(id, name, version, source, install_path, risk_level)

update_history(id, software_id, old_version, new_version, timestamp,
status)

quarantine(id, file_path, original_location, timestamp, reason)

registry_backup(id, hive, key_path, backup_blob, timestamp)

------------------------------------------------------------------------

## Relationships / 关系

software 1:N update_history software 1:N quarantine

------------------------------------------------------------------------

## Integrity Rules / 完整性规则

-   All updates logged.
-   Cleanup must reference software_id.
-   Registry deletion requires backup entry.
