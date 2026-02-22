# API_SPEC

## 文档目的
定义 Synora 的 CLI 契约（命令、参数、输出、错误语义），作为实现与自动化测试的统一标准。

## 当前状态
- 状态：v1 Frozen（接口设计冻结版）
- 接口形态：CLI-first（JSON 机器可读 + plain 可读输出）

## 上下文输入
- 产品范围：本地软件管理 + AI 来源候选 + 安全门禁 + 审计
- 架构约束：高风险路径必须经过 guard

## 预期输出
- 命令清单与参数语义明确
- JSON 输出字段稳定
- 退出码与错误类型可预测

## 通用约定（Draft）
1. 输出模式
- 默认：plain text
- `--json`：结构化 JSON
- `--verbose`：附加调试字段

2. 退出码
- `0` success
- `2` usage/validation error
- `3` security blocked
- `4` integration failure
- `10` partial success

3. 时间字段
- 统一 Unix timestamp（秒）

## 命令契约（Draft）

### 1) 配置初始化
命令：`synora config init`

行为：
- 初始化 `config.json` 与本地数据库

JSON 输出（示例）：
```json
{
  "config_path": ".../.synora_custom/config.json",
  "db_path": ".../.synora_custom/db/synora.db",
  "initialized": true
}
```

### 2) 门禁状态查询
命令：`synora config gate-show [--json] [--verbose]`

JSON 输出字段：
- `real_mutation_enabled` boolean
- `gate_version` string
- `approval_record_ref` string
- `approval_record_present` boolean
- `config_path` string (verbose)
- `config_exists` boolean (verbose)

### 3) 门禁状态设置
命令：
- `synora config gate-set --enable --confirm --approval-record <path> --reason <text> [--json]`
- `synora config gate-set --disable --reason <text> [--keep-record] [--json]`
- `synora config gate-set ... --dry-run --json`

校验规则：
- `--enable` 必须带 `--approval-record`
- `--enable` 非 dry-run 必须带 `--confirm`
- 非 dry-run 必须带 `--reason`
- `--keep-record` 仅允许与 `--disable` 同用

### 4) 门禁历史查询
命令：`synora config gate-history [--enabled-only] [--since <ts>] [--limit <n>] [--reason-contains <kw>] [--json]`

JSON 数组字段：
- `id` number
- `timestamp` number
- `real_mutation_enabled` boolean
- `gate_version` string
- `approval_record_ref` string
- `approval_record_present` boolean
- `reason` string

### 5) 软件自动发现扫描（新增 Draft）
命令：`synora software discover scan [--json] [--verbose]`

行为：
- MVP 仅扫描 Registry uninstall keys
- 增量写入个人软件库

JSON 输出（示例）：
```json
{
  "scan_id": "discover-1771732000",
  "source": "registry",
  "total_seen": 120,
  "inserted": 18,
  "updated": 42,
  "skipped": 60,
  "duration_ms": 834
}
```

### 6) 软件列表查询
命令：`synora software list [--json] [--verbose]`

JSON 数组字段：
- `id`, `name`, `version`, `publisher`, `install_location`
- `discovery_source`, `source_confidence`, `last_seen_at`

### 7) 来源候选推荐
命令：`synora source suggest [--json] [--verbose]`

行为：
- 基于 inventory 生成来源候选
- 默认写入 `pending` 候选池

JSON 数组字段：
- `software_id`, `software_name`, `url`, `domain`, `confidence`, `reason`, `status`

### 8) 更新执行
命令：`synora update apply --id <pkg> (--yes | --dry-run) [--json]`

JSON 输出字段：
- `package_id`
- `risk`
- `confirmed`
- `dry_run`
- `requested_mode`
- `mode`
- `message`

### 9) 隔离清理执行
命令：`synora cleanup quarantine --id <pkg> [--confirm | --dry-run] [--risk <level>] [--simulate-failure] [--simulate-rollback-failure] [--json]`

关键规则：
- `risk=high` 必须 `--confirm`
- confirm 路径受 gate 约束

JSON 输出字段：
- `operation_id`
- `package_id`
- `requested_mode`
- `mode`
- `status`
- `mutation_boundary_reached`
- `rollback_attempted`
- `rollback_status`
- `message`

### 10) 审计查询
命令：
- `synora config history-list [--json]`
- `synora config audit-summary [--json]`

`audit-summary` JSON 字段：
- `total`
- `planned_confirmed`
- `planned_dry_run`
- `latest_timestamp`

### 11) 插件列表查询（Draft）
命令：`synora plugin list [--json] [--verbose]`

JSON 数组字段：
- `plugin_id`
- `name`
- `version`
- `kind`
- `enabled`
- `trust_status`
- `runtime`
- `api_compat`

### 12) 插件启停（Draft）
命令：
- `synora plugin enable --id <plugin_id> [--json]`
- `synora plugin disable --id <plugin_id> [--json]`

关键规则：
- 非 trusted 插件禁止 enable（返回 security blocked）
- 启停操作必须写审计

失败示例（校验）：
```json
{
  "error": {
    "type": "validation",
    "code": 2,
    "message": "plugin_id is required"
  }
}
```

失败示例（安全）：
```json
{
  "error": {
    "type": "security",
    "code": 3,
    "message": "plugin trust_status is untrusted"
  }
}
```

### 13) 插件执行（Draft）
命令：`synora plugin run --id <plugin_id> --action <action> [--payload <json>] [--json]`

关键规则：
- 仅允许 manifest 声明过的 action/permission
- 高风险 action 需要 `--confirm`（后续可扩展）

JSON 输出字段：
- `execution_id`
- `plugin_id`
- `action`
- `status`
- `message`

失败示例（未声明 action）：
```json
{
  "error": {
    "type": "validation",
    "code": 2,
    "message": "action is not declared in plugin manifest"
  }
}
```

失败示例（权限不足）：
```json
{
  "error": {
    "type": "security",
    "code": 3,
    "message": "permission cleanup.execute is not granted"
  }
}
```

失败示例（运行时故障）：
```json
{
  "error": {
    "type": "integration",
    "code": 4,
    "message": "plugin runtime failed to execute action"
  }
}
```

### 14) AI 软件整理分析（Draft）
命令：`synora ai analyze [--json] [--verbose]`

行为：
- 基于 inventory 与更新信号输出软件结构分析

JSON 输出字段：
- `summary`
- `categories`
- `potential_redundancy`
- `recommendations`
- `confidence`

### 15) AI 场景化安装建议（Draft）
命令：`synora ai recommend --goal <text> [--json] [--verbose]`

行为：
- 根据用户目标推荐软件组合
- 结果可写入来源候选池（pending）

JSON 输出字段：
- `goal`
- `recommended_software`
- `reason`
- `confidence`
- `risk_level`

### 16) AI 修复方案（Draft）
命令：`synora ai repair-plan --software <name> --issue <text> [--json] [--verbose]`

行为：
- 输出修复计划（plan-only）
- 不触发真实变更

JSON 输出字段：
- `target_software`
- `issue`
- `plan_steps`
- `rollback_hint`
- `risk_level`
- `confidence`

### 17) AI 修复执行（Phase 2 Draft）
命令：`synora ai repair-apply --plan-id <id> --confirm [--json]`

关键规则：
- MVP 不开放
- 开放后必须通过 gate 与审计

### 18) 任务提交（Draft）
命令：`synora job submit --type <job_type> --payload <json> [--priority <1-100>] [--schedule-at <ts>] [--json]`

行为：
- 向 `job_queue` 提交后台任务
- 默认 `status=queued`

JSON 输出字段：
- `job_id`
- `job_type`
- `status`
- `priority`
- `scheduled_at`

失败示例（校验）：
```json
{
  "error": {
    "type": "validation",
    "code": 2,
    "message": "unknown job_type"
  }
}
```

### 19) 任务列表（Draft）
命令：`synora job list [--status <status>] [--type <job_type>] [--limit <n>] [--json]`

JSON 数组字段：
- `id`
- `job_type`
- `status`
- `priority`
- `attempt_count`
- `max_attempts`
- `scheduled_at`
- `created_at`

### 20) 任务详情（Draft）
命令：`synora job show --id <job_id> [--json] [--verbose]`

JSON 输出字段：
- `id`
- `job_type`
- `payload_json`
- `status`
- `attempt_count`
- `max_attempts`
- `scheduled_at`
- `started_at`
- `finished_at`
- `last_error`

### 21) 任务重试（Draft）
命令：`synora job retry --id <job_id> [--json]`

关键规则：
- 仅允许对 `failed` 或 `deadletter` 任务重试
- 达到最大重试次数前，自动重试由 Worker 处理

JSON 输出字段：
- `job_id`
- `old_status`
- `new_status`
- `attempt_count`

失败示例（状态不允许）：
```json
{
  "error": {
    "type": "validation",
    "code": 2,
    "message": "job status does not allow manual retry"
  }
}
```

### 22) 下载开始（Draft）
命令：`synora download start --software-id <id> --url <url> [--checksum <sha256>] [--json]`

行为：
- 创建 `download.fetch` 任务并入队
- 下载后默认进入校验流程

JSON 输出字段：
- `download_task_id`
- `job_id`
- `software_id`
- `source_url`
- `status`

失败示例（安全）：
```json
{
  "error": {
    "type": "security",
    "code": 3,
    "message": "source domain is not in allowlist"
  }
}
```

### 23) 下载列表（Draft）
命令：`synora download list [--status <status>] [--software-id <id>] [--limit <n>] [--json]`

JSON 数组字段：
- `id`
- `software_id`
- `source_url`
- `status`
- `verify_status`
- `created_at`

### 24) 下载详情（Draft）
命令：`synora download show --id <download_task_id> [--json] [--verbose]`

JSON 输出字段：
- `id`
- `job_id`
- `software_id`
- `source_url`
- `target_path`
- `status`
- `bytes_total`
- `bytes_downloaded`
- `checksum_expected`
- `checksum_actual`
- `verify_status`
- `error_message`

### 25) 下载重试（Draft）
命令：`synora download retry --id <download_task_id> [--json]`

关键规则：
- 仅允许 `download_failed` 或 `verify_failed` 任务重试
- 高风险来源仍需通过安全校验

### 26) 下载校验（Draft）
命令：`synora download verify --id <download_task_id> [--json]`

行为：
- 手动触发哈希/来源校验
- 校验失败必须阻断安装流程

失败示例（校验失败）：
```json
{
  "error": {
    "type": "security",
    "code": 3,
    "message": "artifact checksum verification failed"
  }
}
```

### 27) 全局搜索查询（UI Draft）
命令（内部/未来 UI API）：`synora ui search --q <text> [--limit <n>] [--json]`

行为：
- 汇聚 actions/software/downloads/jobs/security/ai 结果
- 返回按分组排序的结果列表

JSON 输出字段：
- `query`
- `groups[]`
- `groups[].type`
- `groups[].items[]`
- `items[].title`
- `items[].subtitle`
- `items[].risk_level`
- `items[].confidence`
- `items[].action_id`

### 28) 全局搜索动作执行（UI Draft）
命令（内部/未来 UI API）：`synora ui action-run --id <action_id> [--json]`

行为：
- 执行搜索结果绑定动作
- 高风险动作仍走 confirm + gate 校验

失败示例（安全）：
```json
{
  "error": {
    "type": "security",
    "code": 3,
    "message": "action requires explicit confirmation"
  }
}
```

### 29) 仓库列表查询（Draft）
命令：`synora repo list [--kind <public|personal|candidate>] [--json]`

JSON 数组字段：
- `repo_id`
- `name`
- `kind`
- `source`
- `trust_status`
- `enabled`
- `last_sync_at`

### 30) 仓库注册（Draft）
命令：
- `synora repo add --kind personal --name <name> --source <path_or_url> [--json]`
- `synora repo add --kind public --name <name> --source <url> [--json]`

关键规则：
- `public` 仓库必须通过信任校验后才可启用
- `personal` 支持本地目录或本地文件索引

### 31) 仓库同步（Draft）
命令：`synora repo sync --id <repo_id> [--json] [--verbose]`

行为：
- 拉取或读取仓库索引
- 解析 `software.yaml`
- 写入条目快照并输出变更统计

JSON 输出字段：
- `repo_id`
- `total_entries`
- `inserted`
- `updated`
- `skipped`
- `failed`

### 32) 仓库软件检索（Draft）
命令：`synora repo package search --q <text> [--repo <repo_id>] [--limit <n>] [--json]`

JSON 数组字段：
- `package_id`
- `name`
- `version`
- `publisher`
- `repo_id`
- `risk_level`
- `install_url`
- `confidence`

### 33) 仓库条目导入/导出（Draft）
命令：
- `synora repo package import --repo <repo_id> --file <software.yaml> [--json]`
- `synora repo package export --id <package_id> --out <software.yaml> [--json]`

关键规则：
- 导入必须通过 schema 校验与 URL 安全校验
- 导出必须保留最小必填字段（name/version/install/check_update）

### 34) 社区仓库提交（Phase 2 Draft）
命令：`synora repo submit --repo <public_repo_id> --file <software.yaml> --reason <text> [--json]`

关键规则：
- MVP 不开放写入公共仓库
- Phase 2 开放后默认进入 `candidate`，需审核后发布

## 错误响应格式（JSON，Draft）

```json
{
  "error": {
    "type": "security",
    "code": 3,
    "message": "real mutation is disabled",
    "hint": "set execution.real_mutation_enabled=true after approval"
  }
}
```

`error.type` 枚举：
- `validation`
- `security`
- `integration`

## 兼容策略（Draft）
1. 新增字段只增不删
2. 旧字段语义不重定义
3. 破坏性变更通过版本号声明（`v0.x -> v0.y`）

## 字段稳定级标签（Phase 1 Freeze）
1. `stable`
- 已冻结字段；同一主版本内不得删除或重定义语义。
- 适用于核心 ID、状态、风险、审计时间戳、退出码对应语义。

2. `experimental`
- 可在 Draft 阶段演进；可能增删改，必须在变更日志中说明。
- 适用于 UI 聚合结果、AI 解释字段、候选增强指标等。

3. 标注规则
- 命令主输出默认按“核心 stable + 扩展 experimental”组织。
- `--verbose` 新增字段默认 `experimental`，冻结后再转 `stable`。

## 接口决策（Phase 1 Freeze）
1. `software discover scan` 支持 `--source registry` 显式参数；MVP 仅接受 `registry`。
2. `history-list` 在 MVP 不引入分页游标，采用 `--limit` 控制结果规模。
3. JSON 错误输出仅在 `--json` 模式强制；plain 模式保持可读文本错误。
4. 仓库同步默认采用增量模式；全量同步后置为增强能力。

## 失败示例补充（Phase 1 Freeze）
1. 参数缺失（validation）
```json
{
  "error": {
    "type": "validation",
    "code": 2,
    "message": "required argument is missing",
    "hint": "run with --help for usage"
  }
}
```

2. 门禁阻断（security）
```json
{
  "error": {
    "type": "security",
    "code": 3,
    "message": "real mutation is disabled",
    "hint": "use config gate-set with approval record"
  }
}
```

3. 外部依赖失败（integration）
```json
{
  "error": {
    "type": "integration",
    "code": 4,
    "message": "failed to read repository index",
    "hint": "check source path/network and retry"
  }
}
```

4. 部分成功（partial）
```json
{
  "result": {
    "status": "partial_success",
    "processed": 20,
    "failed": 2
  }
}
```

## Smoke 脚本草案（Phase 1 Freeze）
1. 基础初始化
- `synora config init`
- 期望：退出码 `0`，生成配置与数据库路径。

2. 门禁读写回路
- `synora config gate-show --json`
- `synora config gate-set --disable --reason "smoke baseline" --json`
- 期望：退出码 `0`，字段 `real_mutation_enabled=false`。

3. 只读主链路
- `synora software discover scan --source registry --json`
- `synora software list --json`
- `synora source suggest --json`
- 期望：退出码 `0`，返回结构化数组/对象。

4. 高风险阻断链路
- `synora cleanup quarantine --id Git.Git --confirm --json`
- 期望：在 gate 关闭时返回退出码 `3`，`error.type=security`。

5. 审计可见性
- `synora config history-list --json`
- `synora config audit-summary --json`
- 期望：存在可追溯审计记录与摘要统计字段。

## 更新规则
- 命令参数或输出变更必须同步更新：
  - `docs/DATA_MODEL.md`
  - `docs/ARCHITECTURE.md`
  - `logs/DEVELOPMENT_LOG.md`
- 当前为 Draft，可调整，不视为最终 API 冻结。

## 关联文档
- `docs/API_ERROR_CATALOG.md`
- `docs/JOB_TYPES_DRAFT.md`
- `docs/JOB_RETRY_POLICY_DRAFT.md`
- `docs/JOB_OPERATIONS_PLAYBOOK.md`
- `docs/SECURITY_BASELINE_DRAFT.md`
- `docs/SANDBOX_EXECUTION_POLICY_DRAFT.md`
- `docs/HASH_AND_SIGNATURE_POLICY_DRAFT.md`
- `docs/SOFTWARE_REPOSITORY_SYSTEM_DRAFT.md`
