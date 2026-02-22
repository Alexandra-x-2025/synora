# JOB_TYPES_DRAFT

## 文档目的
定义 Synora 后台任务队列的首批 `job_type` 与 `payload_json` 结构，作为 Service 入队与 Worker 执行的统一契约。

## 当前状态
- 状态：v0.1 Draft（未冻结）
- 适用范围：SQLite 本地队列（`job_queue`）

## 通用任务包结构（Draft）
```json
{
  "job_id": 123,
  "job_type": "discovery.scan.registry",
  "priority": 80,
  "payload_json": {},
  "attempt_count": 0,
  "max_attempts": 3,
  "scheduled_at": 1771733000
}
```

## 任务类型清单（Phase 1 Draft）

### 1) `discovery.scan.registry`
用途：执行 Registry-only 软件扫描

`payload_json`：
```json
{
  "source": "registry",
  "include_hklm": true,
  "include_hkcu": true,
  "scan_reason": "manual|scheduled"
}
```

### 2) `source.suggest.enrich`
用途：为指定软件或批量软件生成来源候选

`payload_json`：
```json
{
  "software_ids": [1, 2, 3],
  "mode": "batch|single",
  "max_candidates_per_software": 5
}
```

### 3) `update.check.batch`
用途：批量更新检测（不执行变更）

`payload_json`：
```json
{
  "software_ids": [1, 2, 3],
  "provider": "winget",
  "include_prerelease": false
}
```

### 4) `update.apply.package`
用途：受控更新执行任务

`payload_json`：
```json
{
  "package_id": "Git.Git",
  "requested_mode": "dry-run|confirm",
  "risk_level": "low|medium|high",
  "ticket_id": "optional-ticket-id",
  "confirm": false
}
```

### 5) `cleanup.quarantine`
用途：隔离清理任务

`payload_json`：
```json
{
  "package_id": "Git.Git",
  "requested_mode": "dry-run|confirm",
  "risk_level": "low|medium|high",
  "simulate_failure": false,
  "simulate_rollback_failure": false
}
```

### 6) `ai.analyze`
用途：AI 软件整理分析

`payload_json`：
```json
{
  "scope": "all|subset",
  "software_ids": [1, 2],
  "include_update_signal": true
}
```

### 7) `ai.recommend`
用途：AI 场景化安装建议

`payload_json`：
```json
{
  "goal": "video editing",
  "constraints": {
    "prefer_open_source": false,
    "max_recommendations": 5
  }
}
```

### 8) `ai.repair.plan`
用途：AI 修复方案生成（plan-only）

`payload_json`：
```json
{
  "target_software": "Google Chrome",
  "issue": "crash on startup",
  "collect_diagnostics": true
}
```

### 9) `download.fetch`
用途：下载文件到本地缓存目录

`payload_json`：
```json
{
  "software_id": 1,
  "source_url": "https://example.com/file.exe",
  "target_filename": "file.exe",
  "checksum_sha256": "optional",
  "allow_untrusted_source": false
}
```

### 10) `download.verify`
用途：校验下载产物（哈希/来源）

`payload_json`：
```json
{
  "download_task_id": 101,
  "artifact_path": "C:/.../downloads/file.exe",
  "checksum_sha256": "expected-sha256",
  "domain": "example.com"
}
```

### 11) `download.cleanup`
用途：清理过期下载产物

`payload_json`：
```json
{
  "older_than_days": 30,
  "only_unreferenced": true
}
```

## Scheduler 任务模板（Draft）
1. `daily_update_check`
- `job_type`: `update.check.batch`
- 推荐 cron：`0 3 * * *`

2. `daily_registry_scan`
- `job_type`: `discovery.scan.registry`
- 推荐 cron：`0 2 * * *`

## 优先级建议（Draft）
- 90-100：安全/故障修复相关
- 70-89：用户交互触发任务
- 40-69：日常批处理
- 1-39：低优先级后台任务

## 安全校验要求（Draft）
1. `update.apply.package` 与 `cleanup.quarantine` 在 Worker 执行前必须做 gate 校验。
2. `risk_level=high` 的任务必须带 confirm 并验证审批链路。
3. AI 任务默认只读/建议型，不直接触发真实变更。

## 任务类型决策（Phase 1 Freeze）
1. 队列层支持任务幂等键（idempotency key），默认作用域为 `job_type + payload_fingerprint`。
2. 批量任务采用 fan-out 子任务拆分（主任务负责聚合结果）。
3. 下载任务强制串联 `download.verify`，未校验通过不得进入后续安装链路。

## 更新规则
- 任务类型变更必须同步：
  - `docs/ARCHITECTURE.md`
  - `docs/API_SPEC.md`
  - `docs/DATA_MODEL.md`
  - `docs/sql/V001__init_schema_draft.sql`

## 关联文档
- `docs/JOB_RETRY_POLICY_DRAFT.md`
