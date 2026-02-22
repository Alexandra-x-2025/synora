# FINAL_ARCHITECTURE

## 文档目的
给出 Synora 的最终架构基线（当前阶段），统一后续实现、测试与文档对齐入口。

## 当前状态
- 状态：v1.0 Final Baseline（设计基线）
- 适用范围：Windows-first、CLI-first、Local-first
- 说明：此文档作为“主架构入口”，细节分解仍由专项 Draft 文档承载

## 架构目标
1. 提供统一的软件治理入口：发现、仓库、下载、更新、清理、审计。
2. 强制安全闭环：高风险操作必须经过 confirm + gate + 审计。
3. 支持 AI 增强但不让 AI 越权：AI 默认只建议，不直接做真实变更。
4. 保持可演进：MVP 本地单机架构可平滑扩展到插件化与分布式队列。

## 最终分层架构

```text
UI Layer (CLI now / GUI later)
  -> API Layer (contracts + validation + exit codes)
    -> Service Layer (use-case orchestration)
      -> Queue/Scheduler Layer (job lifecycle)
        -> Worker Layer (async execution)
          -> Integration Layer
             (discovery / package_manager / download / repository / ai / plugins)
            -> Security & Audit Layer (guard + policy + evidence)
              -> Storage Layer (SQLite + config.json + artifacts)
```

## 最终模块边界
1. `api`
- 命令契约、参数校验、输出格式（plain/json/verbose）、退出码映射。

2. `services`
- 编排业务流程，拆分为同步路径与异步任务，不直接执行高耗时动作。

3. `queue` + `scheduler`
- 统一任务生命周期（queued/running/success/failed/retrying/deadletter）。
- 支持重试、死信、定时入队。

4. `workers`
- 异步执行器，消费任务并回写状态与审计。

5. `integration.discovery`
- 软件自动发现（MVP：Registry-only）。

6. `integration.repository`
- 软件仓库系统（public/personal/candidate）。
- 解析与校验 `software.yaml`，支持仓库同步与检索。

7. `integration.download`
- 下载、哈希校验、来源校验、产物落盘。

8. `integration.package_manager`
- 与 winget 等包管理器交互（MVP 可保留模拟执行边界）。

9. `ai`
- `ai_linker`：来源候选补全。
- `ai_assistant`：analyze/recommend/repair-plan。

10. `plugins`
- 插件清单、权限、执行生命周期（不绕过安全门禁）。

11. `security`
- 风险分级、确认策略、真实变更门禁、路径/来源/签名策略。

12. `audit`
- 统一事件写入与可追溯证据。

13. `db`
- SQLite 结构化存储 + `config.json` 门禁配置。

14. `contracts`
- DTO、状态词汇、错误码共享定义。

## 核心业务链路（最终基线）
1. 发现链路：`software discover scan` -> inventory -> audit。
2. 仓库链路：`repo sync/search/import` -> repository tables -> audit。
3. 补链链路：`source suggest` -> candidate pool -> review。
4. 下载链路：`download start` -> fetch -> verify -> artifact/audit。
5. 执行链路：`update/cleanup confirm` -> queue -> worker -> guard -> audit。
6. AI 链路：`ai analyze/recommend/repair-plan` -> insight history -> audit。

## 安全控制点（最终基线）
1. 高风险操作必须显式 `--confirm`。
2. 真实变更必须满足 gate 开启与审批记录存在。
3. 非 trusted 仓库/插件不得进入真实执行链路。
4. 校验失败（hash/signature/source）必须阻断安装/执行。
5. 所有关键动作必须有结构化审计记录。

## 数据与存储基线
1. 核心实体域：
- inventory/source/update/cleanup/gate/audit
- plugin/ai/job/scheduler/download/repository
2. 基线脚本：
- `docs/sql/V001__init_schema_draft.sql`
3. 数据模型主文档：
- `docs/DATA_MODEL.md`

## 代码结构基线

```text
src
 ├ api
 ├ core
 ├ services
 ├ workers
 ├ queue
 ├ scheduler
 ├ plugins
 ├ ai
 ├ integration
 ├ security
 ├ audit
 ├ db
 └ contracts
```

## 非功能目标（最终基线）
1. 安全：高风险误执行为 0。
2. 可观测：关键流程 100% 可审计。
3. 可靠性：核心命令退出码一致率 >= 99%。
4. 可演进：MVP 不锁死后续 GUI/分布式队列/插件沙箱路线。

## MVP 与后续边界
1. MVP（当前实现目标）
- Registry-only 发现
- 公共仓库只读 + 个人仓库可写
- AI 只做建议/候选
- 下载可控、执行受门禁

2. Phase 2
- 社区仓库提交审核
- AI 修复受控执行
- 搜索排序个性化增强

3. Phase 3
- WASM 插件沙箱评估
- 分布式队列迁移评估

## 与现有文档关系
1. 主架构入口：`docs/FINAL_ARCHITECTURE.md`（本文件）
2. 详细架构拆分：`docs/ARCHITECTURE.md`
3. 产品范围：`docs/PRODUCT_SPEC.md`
4. 接口契约：`docs/API_SPEC.md`
5. 数据模型：`docs/DATA_MODEL.md`
6. 安全边界：`SECURITY.md`

## 更新规则
1. 架构级决策先改本文件，再同步下游文档。
2. 下游文档不得与本文件冲突；冲突以本文件为准。
3. 若发生破坏性调整，必须在 `ARCHITECTURE_DECISIONS.md` 追加 ADR。
