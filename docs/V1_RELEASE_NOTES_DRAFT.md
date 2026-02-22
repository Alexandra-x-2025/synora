# V1_RELEASE_NOTES_DRAFT

## 版本信息
- 版本：v1.0.0-draft
- 日期：2026-02-22
- 状态：Draft（发布前复核中）

## 变更摘要
1. 配置与门禁
- `config init/gate-show/gate-set/gate-history` 完整可用。
- 真实变更路径必须通过 gate 与审批记录约束。

2. 软件发现与来源治理
- `software discover scan/history/list`：支持增量同步、active/inactive 语义与扫描历史。
- `source suggest/list/review/review-bulk/apply-approved/registry-*`：支持候选审核、执行来源启停、批量操作。

3. 更新与清理执行链路
- `update check/apply/history`：支持 dry-run/confirm、execution-ticket、回滚结构化审计。
- `cleanup apply/history`：与 update 对齐的 confirm + gate + ticket + rollback 语义。

4. 下载与校验
- `download start/list/show/retry/verify/history`：支持哈希/签名/来源策略结构化状态，失败阻断。

5. 仓库与包检索
- `repo list/add/remove/sync` 与 `package search` 已可用。

6. AI 与 UI 入口（plan-only + simulated）
- AI：`ai analyze/recommend/repair-plan`（plan-only，不触发真实变更）。
- UI：`ui search` 聚合只读搜索、`ui action-run` 模拟执行与高风险 confirm 拦截。

7. 任务队列（最小闭环）
- `job submit/list/retry`。
- `job deadletter-list/replay-deadletter` 运维闭环。

## 错误码契约
- `0`：成功
- `2`：参数校验/用法错误
- `3`：安全阻断（如高风险未 confirm、gate 未开启）
- `4`：集成失败（模拟失败/数据库/IO 等）

## 已知限制
1. worker/scheduler 仍为模拟闭环
- `job_queue` 当前未接入真实异步 worker。
- `download/update/cleanup/ui action-run` 的实际执行仍以模拟路径为主。

2. AI 能力为规则模板驱动
- 当前 AI 输出为 plan-only 模式下的结构化建议，不调用外部模型服务。

3. 平台范围
- 发现链路以 Windows Registry 为核心，跨平台能力后续增强。

## 升级与兼容性
- 启动时会自动执行 SQLite 表结构补齐（新增列/新增表）。
- 历史数据兼容：旧记录字段（如 rollback/execution_ticket）采用默认值回填，不影响查询。

## 回滚指引（最小）
1. 配置回滚
- 关闭真实变更门禁：
  - `cargo run -- config gate-set --disable --reason "rollback" --json`

2. 执行来源回滚
- 停用执行来源：
  - `cargo run -- source registry-disable --json --status active --limit 100`

3. 数据快照
- 发布前备份：`.synora_custom/db/synora.db`
- 问题定位优先使用：
  - `update history`
  - `cleanup history`
  - `download history`
  - `job list/deadletter-list`

## 发布前必跑
- `powershell -ExecutionPolicy Bypass -File .\scripts\smoke_phase8.ps1`
- 按 `docs/RELEASE_READINESS_CHECKLIST.md` 全量打勾后才允许发布。

