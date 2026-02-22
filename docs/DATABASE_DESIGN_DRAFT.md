# DATABASE_DESIGN_DRAFT

## 文档目的
给出 Synora 当前阶段的“临时可落库”数据库方案，支撑后续实现与测试联调。

## 当前状态
- 状态：v0.1 Draft（临时基线，未冻结）
- 数据库：SQLite（local-first）
- 对应初始化脚本：`docs/sql/V001__init_schema_draft.sql`

## 设计原则
1. 先可用：覆盖当前产品与架构中已确定的功能路径。
2. 安全优先：高风险流程必须能被审计追踪。
3. 可演进：枚举与字段保留扩展空间，后续通过 migration 演进。
4. 可查询：关键列表与历史路径必须有索引。

## 逻辑分域
1. 资产域
- `software_inventory`
- `source_candidate`

2. 执行审计域
- `update_history`
- `cleanup_history`
- `audit_event`

3. 门禁治理域
- `gate_history`

4. 插件扩展域
- `plugin_registry`
- `plugin_execution_history`

5. AI 助手域
- `ai_insight_history`
- `repair_plan_history`

6. 后台任务域
- `job_queue`
- `scheduler_task`

7. 下载域
- `download_task_history`
- `download_artifact`
- 策略文档：`docs/DOWNLOAD_CACHE_POLICY_DRAFT.md`

8. 仓库域
- `repository_registry`
- `repository_package`
- `repository_submission`（Phase 2）
- 设计文档：`docs/SOFTWARE_REPOSITORY_SYSTEM_DRAFT.md`

## 关键约束（临时）
1. 所有时间字段用 Unix timestamp（秒）。
2. 所有状态字段用 `CHECK` 约束保持词汇一致。
3. `software_inventory` 作为候选源、更新历史主关联对象。
4. `source_candidate`、`update_history` 对 `software_inventory` 使用外键。
5. 插件执行与 AI 输出均需可审计记录。
6. 扫描/下载/更新/修复执行统一走任务队列。

## 当前不做
1. 不引入复杂 join 表（保持单表可读性优先）。
2. 不做分区/归档（后续按数据量再加）。
3. 不做全文检索（后续可加 FTS）。

## 后续迁移方向
1. 增加 `schema_migrations` 与 migration runner。
2. 为 `audit_event.payload_json` 逐步抽结构化字段。
3. 增加 AI 结果去重与版本控制字段。

## 更新规则
- 数据库结构变更必须同步：
  - `docs/DATA_MODEL.md`
  - `docs/API_SPEC.md`
  - `docs/ARCHITECTURE.md`
  - `logs/DEVELOPMENT_LOG.md`
