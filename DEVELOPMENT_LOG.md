# Synora – Development Log

This document tracks the chronological evolution of Synora.

It records:
- Major milestones
- Architectural shifts
- Development phases
- Important fixes
- Behavioral changes

---

# Synora – 开发日志

本文件记录 Synora 的时间线演进。

用于记录：
- 重要里程碑
- 架构变更
- 开发阶段
- 关键修复
- 行为变更

---

## 2025-12-XX – Project Initialization

### English

- Project name finalized: Synora
- Repository created
- MIT license added
- Topics configured
- Bilingual documentation structure established
- Architecture documentation created
- Security-first philosophy defined
- Logo system designed (Deep Blue Shield edition)
- SVG brand asset generated
- Windows-first strategy confirmed
- AI tooling separated via WSL

### 中文

- 项目名称确定：Synora
- 创建 GitHub 仓库
- 添加 MIT 许可证
- 配置 topics
- 建立双语文档体系
- 完成架构文档
- 确立安全优先理念
- 设计品牌 Logo（深蓝盾牌版）
- 生成 SVG 品牌资产
- 确认 Windows 优先策略
- AI 工具链与 Windows 执行环境分离（WSL）

---

## Phase 0 – Bootstrap Complete

### English

Documentation phase completed.

Project now contains:

- README
- SECURITY
- CONTRIBUTING
- CHANGELOG
- PROJECT_STATE
- ARCHITECTURE_DECISIONS
- DEVELOPMENT_LOG
- docs/architecture-overview
- docs/roadmap
- Official logo assets

Ready to enter CLI development phase.

### 中文

文档阶段完成。

当前项目包含：

- README
- SECURITY
- CONTRIBUTING
- CHANGELOG
- PROJECT_STATE
- ARCHITECTURE_DECISIONS
- DEVELOPMENT_LOG
- docs/architecture-overview
- docs/roadmap
- 官方 Logo 资源

准备进入 CLI 开发阶段。

---

## Upcoming – Phase 1 CLI MVP

### Planned Scope (English)

- Software listing
- Update check
- Safe update stub
- Basic logging
- Initial configuration file

### 计划范围（中文）

- 软件列表
- 更新检测
- 安全更新占位实现
- 基础日志系统
- 初始配置文件结构

---

## 2026-02-21 – CLI Specification v0.1 Started

### English

- Entered Phase 1 implementation track
- Added `docs/cli-spec-v0.1.md` as baseline contract
- Initialized Python package structure under `src/synora`
- Implemented layered modules:
- Domain models and risk classifier
- Repository config initializer
- Service coordinators for software and updates
- Worker retry engine
- Integration winget client with guarded command execution
- Security Guard allowlist policy
- Added CLI entrypoint with v0.1 commands:
- `software list`
- `update check`
- `update apply --id ... [--yes]` (plan-only in v0.1)
- `config init`
- Added basic unit tests for security policy and CLI argument flow

### 中文

- 进入 Phase 1 实施阶段
- 新增 `docs/cli-spec-v0.1.md` 作为规范基线
- 在 `src/synora` 初始化 Python 包结构
- 完成分层模块最小实现：
- Domain 模型与风险分类
- Repository 配置初始化
- Service 软件与更新协调
- Worker 重试引擎
- Integration 的 winget 客户端（通过安全守卫执行）
- Security Guard 白名单策略
- 增加 CLI v0.1 命令入口：
- `software list`
- `update check`
- `update apply --id ... [--yes]`（v0.1 仅计划输出）
- `config init`
- 增加基础单元测试（安全策略与 CLI 参数流程）

---

## 2026-02-21 – CLI v0.1 Interface Freeze Review

### English

- Replaced generated interface draft with frozen contract document:
- `docs/architecture/Synora_Interface_and_Module_Specification.md`
- Unified command contract with implementation:
- `software list`
- `update check`
- `update apply --id ... [--dry-run | --confirm] [--json]`
- `config init`
- Kept backward compatibility:
- `--yes` remains alias of `--confirm`
- Marked `docs/cli-spec-v0.1.md` as `Frozen`
- Added CLI tests for confirm path and alias compatibility

### 中文

- 将自动生成版接口草案替换为冻结契约文档：
- `docs/architecture/Synora_Interface_and_Module_Specification.md`
- 统一文档与实现命令契约：
- `software list`
- `update check`
- `update apply --id ... [--dry-run | --confirm] [--json]`
- `config init`
- 保留向后兼容：
- `--yes` 继续作为 `--confirm` 别名
- 将 `docs/cli-spec-v0.1.md` 状态标记为 `Frozen`
- 增加 CLI 测试覆盖确认路径与别名兼容性

---

## 2026-02-21 – CLI Freeze Hardening Pass

### English

- Enforced integration failure contract:
- `winget` non-zero exit now raises integration error instead of silent empty output
- CLI now maps parser errors consistently to exit code `2`
- Made `update apply` mode intent observable in JSON:
- added `requested_mode`
- added `dry_run`
- Added contract-level tests:
- integration failure returns `4`
- JSON output contains frozen keys
- conflicting flags and missing required args return `2`

### 中文

- 强化集成失败契约：
- `winget` 非 0 退出不再静默为空结果，改为集成错误
- CLI 解析错误统一映射为退出码 `2`
- 让 `update apply` 模式语义可观测：
- 新增 `requested_mode`
- 新增 `dry_run`
- 增加契约级测试：
- 集成失败返回 `4`
- JSON 输出包含冻结字段
- 模式参数冲突与缺失参数返回 `2`

---

## 2026-02-21 – Repository Structure Standardization

### English

- Reorganized docs into domain folders:
- `docs/architecture/`
- `docs/security/`
- `docs/testing/`
- `docs/product/`
- Moved existing specification files into their grouped folders
- Added missing top-level state file: `PROJECT_STATE.md`
- Added placeholder architecture plan documents:
- `docs/architecture/Synora_Final_Design_and_Development_Plan.md`
- `docs/architecture/Synora_Enterprise_Architecture_Master_Plan.md`
- Synced README with:
- updated CLI frozen command syntax

---

## 2026-02-22 – Source Suggestion Signal Upgrade

### English

- Upgraded `source suggest` scoring pipeline:
- Added update-check signal blending (when update candidates are detected, recommendation score is boosted with explicit reasons)
- Kept failure posture safe:
- If update probing is unavailable, recommendation still falls back to repository-only scoring
- Added unit test for score boost behavior under update signal
- Synced `docs/cli-spec-v0.1.md` to reflect the new signal model
- Added `source suggest --verbose` text diagnostics for recommendation signal visibility
- Connected `update apply` plan flow to SQLite `update_history` audit persistence (`planned_dry_run` / `planned_confirmed`)
- Added read-only CLI visibility command: `config history-list [--json]` for update audit inspection
- Added read-only aggregate command: `config audit-summary [--json]` for update audit diagnostics
- Connected confirmed `update apply` plans to placeholder `registry_backup` and `quarantine` audit rows

### 中文

- 升级 `source suggest` 评分链路：
- 引入更新信号融合（检测到可更新候选时，提升推荐分并给出明确原因）
- 保持安全降级策略：
- 当更新探测不可用时，仍回退到仅基于本地仓库的推荐
- 新增更新信号加分行为的单元测试
- 同步 `docs/cli-spec-v0.1.md` 反映新评分模型
- project structure tree
- structure rationale

### 中文

- 按领域完成文档目录重组：
- `docs/architecture/`
- `docs/security/`
- `docs/testing/`
- `docs/product/`
- 将既有规范文件迁移到对应分组目录
- 新增顶层状态文件：`PROJECT_STATE.md`
- 补充架构规划占位文档：
- `docs/architecture/Synora_Final_Design_and_Development_Plan.md`
- `docs/architecture/Synora_Enterprise_Architecture_Master_Plan.md`
- 同步 README：
- 更新冻结后的 CLI 命令语法
- 增加项目结构树
- 增加目录分层解释

---

## 2026-02-21 – Architecture Decision Status Governance

### English

- Added architecture decision status taxonomy in `ARCHITECTURE_DECISIONS.md`:
- `Active`
- `Superseded`
- `Deprecated`
- Added explicit `Status: Active` field to AD-001 through AD-006
- Established a future-safe governance pattern for decision evolution

### 中文

- 在 `ARCHITECTURE_DECISIONS.md` 增加架构决策状态体系：
- `Active`
- `Superseded`
- `Deprecated`
- 为 AD-001 至 AD-006 增加显式状态字段：`Status: Active`
- 建立后续可演进的决策治理模式

---

## 2026-02-21 – Phase 1 Rust Bootstrap Started

### English

- Completed structure freeze confirmation with option A
- Initialized Rust project scaffold:
- `Cargo.toml`
- `src/main.rs`
- layered module folders:
- `src/cli/`
- `src/domain/`
- `src/service/`
- `src/repository/`
- `src/integration/`
- `src/security/`
- Archived previous Python prototype to:
- `legacy/python/`

### 中文

- 完成结构冻结确认，采用方案 A
- 初始化 Rust 工程骨架：
- `Cargo.toml`
- `src/main.rs`
- 分层模块目录：
- `src/cli/`
- `src/domain/`
- `src/service/`
- `src/repository/`
- `src/integration/`
- `src/security/`
- 将先前 Python 原型归档至：
- `legacy/python/`

---

## 2026-02-21 – Rust CLI Wiring Pass (Phase 1)

### English

- Wired Rust CLI dispatch and exit-code contract in `src/cli/mod.rs`
- Implemented v0.1 command paths:
- `software list [--json]`
- `update check [--json]`
- `update apply --id <package_id> [--dry-run|--confirm|--yes] [--json]`
- `config init`
- Connected layered modules:
- `service` -> `integration` -> `security`
- Added integration and security error mapping to frozen exit codes (`2/3/4/10`)
- Kept winget execution as explicit placeholder on Windows (non-Windows returns empty list)

### 中文

- 在 `src/cli/mod.rs` 完成 Rust CLI 分发与退出码契约接线
- 实现 v0.1 命令路径：
- `software list [--json]`
- `update check [--json]`
- `update apply --id <package_id> [--dry-run|--confirm|--yes] [--json]`
- `config init`
- 打通分层模块调用链：
- `service` -> `integration` -> `security`
- 增加集成与安全错误到冻结退出码（`2/3/4/10`）的映射
- 保持 winget Windows 侧执行为明确占位（非 Windows 返回空列表）

---

## 2026-02-21 – Rust Software Discovery Integration (Step 1)

### English

- Integrated real `winget list` execution path in Rust integration layer
- Added JSON parsing pipeline for multiple winget response shapes:
- `Sources -> Packages`
- `Data`
- top-level array fallback
- Added parser tests in `src/integration/mod.rs` for shape compatibility
- Kept `update check` adapter as explicit placeholder for next step
- Added runtime fallback chain:
- try `winget list --output json` first
- fallback to tabular output parsing if JSON mode is unavailable

### 中文

- 在 Rust 集成层接入真实 `winget list` 执行路径
- 增加对多种 winget JSON 结构的解析：
- `Sources -> Packages`
- `Data`
- 顶层数组兜底
- 在 `src/integration/mod.rs` 增加解析测试，保障结构兼容
- `update check` 适配器保持占位，待下一步实现
- 增加运行时回退链路：
- 先尝试 `winget list --output json`
- 若 JSON 模式不可用，自动回退文本表格解析

---

## 2026-02-21 – Rust Update Check Integration (Step 2)

### English

- Integrated real `winget upgrade` execution path
- Implemented JSON-first + text-fallback parsing strategy for update checks
- Added upgrade-specific parsers:
- JSON parser with `Available` / `AvailableVersion` compatibility
- tabular parser with installed-to-available version mapping
- Added parser tests for both JSON and tabular upgrade output shapes

### 中文

- 接入真实 `winget upgrade` 执行路径
- 为更新检测实现“JSON 优先 + 文本回退”解析策略
- 增加升级场景专用解析器：
- JSON 解析兼容 `Available` / `AvailableVersion`
- 文本表格解析支持已安装版本到可用版本映射
- 增加升级 JSON 与文本形态的解析测试

---

## 2026-02-21 – Rust CLI Contract Tests (Step 3)

### English

- Added CLI contract unit tests in `src/cli/mod.rs`
- Covered frozen behaviors:
- missing `--id` returns `EXIT_USAGE (2)`
- conflicting `--dry-run` and `--confirm` returns `EXIT_USAGE (2)`
- `--yes` alias remains backward compatible
- integration errors map to `EXIT_INTEGRATION (4)`
- security errors map to `EXIT_SECURITY (3)`

### 中文

- 在 `src/cli/mod.rs` 增加 CLI 契约单元测试
- 覆盖冻结行为：
- 缺失 `--id` 返回 `EXIT_USAGE (2)`
- `--dry-run` 与 `--confirm` 冲突返回 `EXIT_USAGE (2)`
- `--yes` 别名保持向后兼容
- 集成错误映射到 `EXIT_INTEGRATION (4)`
- 安全错误映射到 `EXIT_SECURITY (3)`

---

## 2026-02-21 – JSON Output Contract Stabilization

### English

- Separated update DTO from software DTO in Rust domain layer
- `update check --json` now emits stable upgrade fields:
- `name`
- `package_id`
- `installed_version`
- `available_version`
- `source`
- Updated upgrade parsers and tests to preserve installed/available version semantics
- Synced JSON field contract into CLI spec and interface specification

### 中文

- 在 Rust Domain 层将更新 DTO 与软件 DTO 解耦
- `update check --json` 现在输出稳定升级字段：
- `name`
- `package_id`
- `installed_version`
- `available_version`
- `source`
- 更新升级解析器与测试，保持已安装/可用版本语义
- 已将 JSON 字段契约同步到 CLI 规范与接口规范文档

---

## 2026-02-21 – CLI Observability and Smoke Baseline

### English

- Added `--verbose` support for:
- `software list`
- `update check`
- Verbose text mode now prints parser path (`json/text_fallback/unsupported_platform`)
- `update check` text mode now prints summary flag: `has_updates: true|false`
- Added smoke regression checklist:
- `docs/testing/Synora_CLI_Smoke_Checklist.md`
- Added CLI tests for verbose command paths

### 中文

- 为以下命令增加 `--verbose` 支持：
- `software list`
- `update check`
- verbose 文本模式新增解析路径输出（`json/text_fallback/unsupported_platform`）
- `update check` 文本模式新增汇总标识：`has_updates: true|false`
- 新增 smoke 回归清单：
- `docs/testing/Synora_CLI_Smoke_Checklist.md`
- 增加 CLI verbose 命令路径测试

---

## 2026-02-21 – Config and Logging Path Alignment (Rust)

### English

- Added Rust path strategy module: `src/paths/mod.rs`
- Implemented `SYNORA_HOME`-aware home resolution and fallback behavior
- Added Rust logging module: `src/logging/mod.rs`
- CLI startup now initializes log file path (`logs/synora.log`) in resolved Synora home
- Updated `config init` to use shared path strategy and include `quarantine_dir`
- Added unit tests for path resolution and repository config initialization

### 中文

- 新增 Rust 路径策略模块：`src/paths/mod.rs`
- 实现 `SYNORA_HOME` 感知的根目录解析与回退策略
- 新增 Rust 日志模块：`src/logging/mod.rs`
- CLI 启动时初始化日志路径（解析后的 Synora 根目录下 `logs/synora.log`）
- `config init` 改为使用统一路径策略，并写入 `quarantine_dir`
- 增加路径解析与配置初始化单元测试

---

## 2026-02-21 – Phase 1 Release Readiness Docs Sync

### English

- Updated `README.md` with Rust quick-start commands
- Added testing references for smoke and readiness validation
- Expanded `CHANGELOG.md` unreleased entries with Rust CLI milestones
- Added `docs/testing/Phase1_MVP_Readiness_Checklist.md`

### 中文

- 在 `README.md` 增加 Rust 快速开始命令
- 增加 smoke 与就绪验证文档引用
- 扩展 `CHANGELOG.md` 的未发布条目，覆盖 Rust CLI 里程碑
- 新增 `docs/testing/Phase1_MVP_Readiness_Checklist.md`

---

## 2026-02-22 – Phase Transition: Phase 1 Complete, Phase 2 Planning Started

### English

- Confirmed Phase 1 MVP readiness on Windows validation runs
- Updated roadmap status:
- Phase 1 marked as completed
- Phase 2 marked as planning
- Added Phase 2 weekly breakdown (Week 1 to Week 4) in `docs/roadmap.md`
- Updated project state baseline date and current phase in `PROJECT_STATE.md`
- Set immediate execution focus to Phase 2 Week 1 (schema + repository contract)

### 中文

- 基于 Windows 验证结果确认 Phase 1 达到 MVP Ready
- 更新路线图状态：
- Phase 1 标记为完成
- Phase 2 标记为规划中
- 在 `docs/roadmap.md` 增加 Phase 2 周计划（第 1 至第 4 周）
- 在 `PROJECT_STATE.md` 更新基准日期与当前阶段
- 将近期执行重点设为 Phase 2 第 1 周（schema + repository 契约）

---

## 2026-02-22 – Phase 2 Week 1 Implementation: SQLite Repository Baseline

### English

- Added SQLite dependency (`rusqlite` with bundled runtime)
- Implemented `DatabaseRepository` in Rust repository layer:
- Schema bootstrap for core tables:
- `software`
- `update_history`
- `quarantine`
- `registry_backup`
- Added repository contract methods:
- software upsert/list
- update history logging
- quarantine logging
- registry backup logging
- Updated `config init` to bootstrap both config file and SQLite schema
- Added repository unit tests for schema init and write/list roundtrip

### 中文

- 新增 SQLite 依赖（`rusqlite`，bundled 运行时）
- 在 Rust repository 层实现 `DatabaseRepository`：
- 核心表 schema 初始化：
- `software`
- `update_history`
- `quarantine`
- `registry_backup`
- 增加 repository 契约方法：
- 软件 upsert / 列表查询
- 更新历史写入
- 隔离记录写入
- 注册表备份记录写入
- `config init` 改为联动初始化配置与 SQLite schema
- 增加 repository 单元测试（schema 初始化 + 写入/查询回环）

---

## 2026-02-22 – Phase 2 Week 1 Visibility Hook: DB Read-only CLI

### English

- Added read-only CLI entry: `synora config db-list [--json]`
- Command reads software entries from SQLite repository for local verification
- Synced interface and CLI spec docs with Phase 2 utility command

### 中文

- 新增只读 CLI 入口：`synora config db-list [--json]`
- 命令可读取 SQLite repository 中的软件记录，便于本地验证
- 已将 Phase 2 工具命令同步到接口规范与 CLI 规范文档

---

## 2026-02-22 – Phase 2 Week 1 Repository Sync Hook

### English

- `software list` now upserts discovered entries into SQLite `software` table
- Added text-mode sync summary: `db_sync_count: <n>`
- Added service-layer snapshot sync helper for repository wiring
- Updated smoke checklist and interface/CLI specs for sync behavior

### 中文

- `software list` 现在会将发现结果 upsert 到 SQLite `software` 表
- 新增文本模式同步汇总：`db_sync_count: <n>`
- 在 service 层增加仓储快照同步辅助方法
- 已同步更新 smoke 清单与接口/CLI 规范

---

## 2026-02-22 – Phase 2 Week 2 Kickoff: Source Suggestion Prototype

### English

- Added prototype command: `synora source suggest [--json]`
- Implemented initial recommendation scoring in service layer
- Recommendation output includes:
- software name
- current source
- recommended source
- score
- reasons
- Added unit tests for basic scoring behavior
- Synced CLI spec, interface spec, and smoke checklist

### 中文

- 新增原型命令：`synora source suggest [--json]`
- 在 service 层实现首版来源推荐评分逻辑
- 推荐输出包含：
- 软件名
- 当前来源
- 推荐来源
- 分数
- 理由
- 增加评分逻辑基础单元测试
- 已同步 CLI 规范、接口规范与 smoke 清单

---

## Logging Rules

Every significant change must be recorded.

Examples:

- New module introduction
- Major refactor
- Security policy change
- CLI command redesign
- Release milestone

---

## 记录规则

每一次重大变更都必须写入本日志。

例如：

- 新模块引入
- 重大重构
- 安全策略变更
- CLI 命令重设计
- 版本发布里程碑

---

## 2026-02-22 – Phase 2 Weekly Closure (Week 1-4)

### English

- Closed Phase 2 weekly execution loop (Week 1 through Week 4)
- Confirmed delivery of:
- Repository baseline and schema bootstrap
- Source suggestion with update-signal blending and diagnostics
- Update planning persistence into audit tables
- Read-only audit visibility commands (`history-list`, `audit-summary`)
- Promoted project status toward Phase 3 preparation

### 中文

- 完成 Phase 2 周计划收口（Week 1 至 Week 4）
- 已确认交付：
- Repository 基线与 schema 初始化
- 来源推荐（更新信号融合 + 诊断输出）
- 更新计划持久化到审计表
- 只读审计可见命令（`history-list`、`audit-summary`）
- 项目状态已推进到 Phase 3 准备阶段

---

## 2026-02-22 – Phase 3 Kickoff: Quarantine Execution Draft

### English

- Added `docs/security/Synora_Quarantine_Execution_Design.md`
- Defined future execution contract for real quarantine operations
- Defined mutation boundary and rollback semantics
- Defined planned status vocabulary for audit persistence

### 中文

- 新增 `docs/security/Synora_Quarantine_Execution_Design.md`
- 定义了未来 quarantine 实执行契约
- 定义了变更边界与回滚语义
- 定义了审计持久化所需状态词汇

---

## 2026-02-22 – Phase 3 Gate Setup: Status Mapping & Implementation Checklist

### English

- Added explicit `update_history.status` mapping table in quarantine design draft
- Added Phase 3 implementation gate checklist:
- `docs/testing/Phase3_Quarantine_Implementation_Gate.md`
- Synced database ER document with active/reserved status vocabulary

### 中文

- 在 quarantine 设计草案中补充 `update_history.status` 映射表
- 新增 Phase 3 实施前门禁清单：
- `docs/testing/Phase3_Quarantine_Implementation_Gate.md`
- 同步数据库 ER 文档的已实现/预留状态词汇

---

## 2026-02-22 – Phase 3 CLI Draft: Cleanup Quarantine Contract

### English

- Added `docs/architecture/Synora_Cleanup_Quarantine_CLI_Contract_Draft.md`
- Defined draft command contract for:
- `synora cleanup quarantine --id <package_id> [--dry-run|--confirm] [--json] [--verbose]`
- Defined draft output schema (`operation_id`, rollback fields, status lifecycle)
- Linked draft references into interface specification and roadmap kickoff notes

### 中文

- 新增 `docs/architecture/Synora_Cleanup_Quarantine_CLI_Contract_Draft.md`
- 定义草案命令契约：
- `synora cleanup quarantine --id <package_id> [--dry-run|--confirm] [--json] [--verbose]`
- 定义草案输出结构（`operation_id`、回滚字段、状态生命周期）
- 已在接口总规范与路线图启动记录中建立草案引用

---

## 2026-02-22 – Phase 3 Workplan: Module-Level Implementation Breakdown

### English

- Added `docs/architecture/Synora_Cleanup_Quarantine_Implementation_Workplan.md`
- Split implementation tasks by layer:
- CLI
- Service
- Repository
- Security
- Integration
- Defined milestone sequence (M1-M4) and completion criteria

### 中文

- 新增 `docs/architecture/Synora_Cleanup_Quarantine_Implementation_Workplan.md`
- 按层拆分实施任务：
- CLI
- Service
- Repository
- Security
- Integration
- 定义里程碑顺序（M1-M4）与完成标准

---

## 2026-02-22 – Phase 3 M1: Cleanup Quarantine CLI Skeleton

### English

- Added runtime command path:
- `synora cleanup quarantine --id <package_id> [--dry-run|--confirm] [--json] [--verbose]`
- Implemented M1 behavior:
- dry-run audit persistence with `update_history.status = quarantine_planned`
- Added service-layer cleanup plan object and persistence helper
- Added CLI contract tests for missing id, conflicting flags, and dry-run JSON success

### 中文

- 新增运行时命令路径：
- `synora cleanup quarantine --id <package_id> [--dry-run|--confirm] [--json] [--verbose]`
- 完成 M1 行为：
- dry-run 审计持久化（`update_history.status = quarantine_planned`）
- 新增 service 层 cleanup 计划对象与持久化辅助逻辑
- 增加 CLI 契约测试：缺失 id、参数冲突、dry-run JSON 成功路径

---

## 2026-02-22 – Phase 3 M2: Confirm Precheck Audit Path

### English

- Extended `cleanup quarantine --confirm` into precheck-audit path (still no real mutation)
- Persisted `quarantine_confirmed` status into `update_history`
- Persisted safety evidence placeholders into:
- `registry_backup`
- `quarantine`
- Added test coverage for confirmed precheck persistence path

### 中文

- 将 `cleanup quarantine --confirm` 扩展为 precheck 审计路径（仍不做真实变更）
- 持久化 `quarantine_confirmed` 到 `update_history`
- 持久化安全证据占位到：
- `registry_backup`
- `quarantine`
- 增加 confirmed precheck 持久化路径的测试覆盖

---

## 2026-02-22 – Phase 3 M3: Confirm Simulated Execution Path

### English

- Promoted confirmed cleanup flow from precheck-only to simulated execution stage
- Confirmed path now:
- persists `quarantine_confirmed`
- records simulated success `quarantine_success`
- sets `mutation_boundary_reached = true`
- Kept safety posture:
- still no real filesystem/registry mutation in runtime

### 中文

- 将 confirmed cleanup 流程从仅 precheck 提升到模拟执行阶段
- confirmed 路径现在会：
- 持久化 `quarantine_confirmed`
- 记录模拟成功状态 `quarantine_success`
- 设置 `mutation_boundary_reached = true`
- 保持安全策略：
- 运行时仍不做真实文件/注册表变更

---

## 2026-02-22 – Phase 3 M4: Failure and Rollback Simulation Path

### English

- Added simulation flags for confirmed cleanup flow:
- `--simulate-failure`
- `--simulate-rollback-failure`
- Added audit persistence for failure lifecycle:
- `quarantine_failed`
- `quarantine_rollback_success`
- `quarantine_rollback_failed`
- Failed cleanup operations now map to exit code `4` while still emitting machine-readable output

### 中文

- 为 confirmed cleanup 流程增加模拟参数：
- `--simulate-failure`
- `--simulate-rollback-failure`
- 增加失败生命周期审计持久化：
- `quarantine_failed`
- `quarantine_rollback_success`
- `quarantine_rollback_failed`
- 失败操作映射为退出码 `4`，同时保持可机读输出

---

## 2026-02-22 – Phase 3 Gate Review Decision (Go/No-Go)

### English

- Reviewed `docs/testing/Phase3_Quarantine_Implementation_Gate.md`
- Marked contract, persistence, rollback simulation, and smoke evidence as satisfied
- Decision:
- Go for simulation path and continued Phase 3 development
- No-Go for real filesystem/registry mutation path until security controls are fully implemented and signed off

### 中文

- 已完成 `docs/testing/Phase3_Quarantine_Implementation_Gate.md` 阶段评审
- 已勾选契约、持久化、回滚模拟与 smoke 证据相关项
- 结论：
- 对模拟路径与 Phase 3 持续开发给出 Go
- 对真实文件/注册表变更路径给出 No-Go（需先完成并签署安全控制项）

---

## 2026-02-22 – Phase 3 Security Control Progress: Target Path Validation

### English

- Added cleanup target path validation in `SecurityGuard`:
- canonical normalization
- parent traversal rejection
- allowlist root constraint
- Connected cleanup execution to security validation path and mapped violations to exit code `3`
- Added tests for traversal rejection at security and CLI levels

### 中文

- 在 `SecurityGuard` 中新增 cleanup 目标路径校验：
- canonical 归一化
- 上级目录穿越拦截
- allowlist 根目录约束
- 将 cleanup 执行接入安全校验路径，并将违规映射为退出码 `3`
- 新增安全层与 CLI 层的路径穿越拒绝测试

---

## 2026-02-22 – Phase 3 Security Control Progress: Symbolic-Link Escape Blocking

### English

- Added symbolic-link component detection in cleanup target validation
- Cleanup path now rejects existing symlink escape paths with security exit `3`
- Added security-layer symlink rejection test (unix target)

### 中文

- 在 cleanup 目标路径校验中新增 symbolic-link 组件检测
- cleanup 路径现在会拒绝 symlink escape 路径，并返回安全退出码 `3`
- 新增安全层 symlink 拒绝测试（unix 目标）

---

## 2026-02-22 – Phase 3 Security Control Progress: High/Critical Confirmation Gate

### English

- Added cleanup risk gate via `--risk <low|medium|high|critical>` option
- Enforced rule: `high`/`critical` risk requires explicit `--confirm`
- Violations map to security exit code `3`
- Added CLI tests for high-risk blocked and confirmed-pass scenarios

### 中文

- 为 cleanup 增加风险门禁参数：`--risk <low|medium|high|critical>`
- 强制规则：`high`/`critical` 风险必须显式 `--confirm`
- 违规映射到安全退出码 `3`
- 新增 CLI 测试覆盖高风险阻断与确认放行场景

---

## 2026-02-22 – Phase 3 Documentation Closure: Gate Ready for Security Sign-off

### English

- Updated Phase 3 gate status to `Ready for Security Sign-off`
- Synced project state and roadmap with current gate readiness
- Kept real-mutation go-live blocked until security sign-off is complete

### 中文

- 将 Phase 3 门禁状态更新为 `Ready for Security Sign-off`
- 同步项目状态与路线图，反映当前门禁就绪状态
- 继续保持真实变更上线阻断，直至 security 签署完成

---

## 2026-02-22 – Phase 3 Security Sign-off Checklist Added

### English

- Added `docs/security/Synora_Security_Signoff_Checklist.md`
- Linked gate blocking item to the checklist document
- Standardized sign-off evidence collection format before real mutation go-live

### 中文

- 新增 `docs/security/Synora_Security_Signoff_Checklist.md`
- 将门禁阻塞项关联到该签署清单
- 在真实变更上线前统一签署证据收集格式

---

## 2026-02-22 – Phase 3 Security Sign-off Pre-Filled Draft

### English

- Added pre-filled review draft:
- `docs/security/Synora_Security_Signoff_Checklist_2026-02-22_Draft.md`
- Marked evidence-backed items as checked
- Kept approval and signer fields pending for formal security decision

### 中文

- 新增预填评审草案：
- `docs/security/Synora_Security_Signoff_Checklist_2026-02-22_Draft.md`
- 已将有证据支撑的条目标记为完成
- 审批与签字字段保留待正式安全决策填写

---

## 2026-02-22 – Phase 3 Approval Record Template Added

### English

- Added single-page approval template:
- `docs/security/Synora_Security_Signoff_Approval_Record_Template.md`
- Designed for direct reuse in PR description or release notes

### 中文

- 新增单页审批模板：
- `docs/security/Synora_Security_Signoff_Approval_Record_Template.md`
- 可直接复用到 PR 描述或发布说明

---

## 2026-02-22 – Phase 3 Pre-Filled Approval Record Draft Added

### English

- Added pre-filled approval record draft:
- `docs/security/Synora_Security_Signoff_Approval_Record_2026-02-22_Draft.md`
- Included current evidence summary and pending decision placeholders

### 中文

- 新增预填审批记录草案：
- `docs/security/Synora_Security_Signoff_Approval_Record_2026-02-22_Draft.md`
- 包含当前证据摘要与待签署决策占位字段

---

## 2026-02-22 – Phase 3 Design Book Alignment (M1-M4 Snapshot Synced)

### English

- Updated quarantine execution design to reflect real runtime state:
- simulation path implemented, real mutation path still release-gated
- Added explicit go-live criteria for real mutation enablement
- Added implementation status matrix to cleanup CLI contract draft
- Synced open decisions with release gate switch strategy

### 中文

- 更新 quarantine 执行设计书，反映当前真实实现状态：
- 模拟路径已落地，真实变更路径仍处于发布门禁控制
- 新增真实变更启用的上线准入条件
- 在 cleanup CLI 契约草案中增加实现状态矩阵
- 将待决策项同步到发布开关策略

---

## 2026-02-22 – Real Mutation Gate Strategy Draft Added

### English

- Added:
- `docs/security/Synora_Real_Mutation_Gate_Strategy_Draft.md`
- Proposed three-layer gate model:
- policy sign-off gate
- runtime config gate
- command confirmation/risk gate
- Kept real mutation default-off and auditable-by-design

### 中文

- 新增：
- `docs/security/Synora_Real_Mutation_Gate_Strategy_Draft.md`
- 提出三层门禁模型：
- 策略签署门禁
- 运行时配置门禁
- 命令确认/风险门禁
- 保持真实变更默认关闭，并确保全程可审计

---

## 2026-02-22 – Real Mutation Gate Approval Pack Added

### English

- Added approval artifacts for real mutation gate decision:
- `docs/security/Synora_Real_Mutation_Gate_Approval_Record_Template.md`
- `docs/security/Synora_Real_Mutation_Gate_Approval_Record_2026-02-22_Draft.md`
- Linked strategy draft to approval artifacts for direct PR/release use

### 中文

- 新增真实变更开关审批工件：
- `docs/security/Synora_Real_Mutation_Gate_Approval_Record_Template.md`
- `docs/security/Synora_Real_Mutation_Gate_Approval_Record_2026-02-22_Draft.md`
- 已在策略草案中关联审批工件，便于直接用于 PR/发布审批

---

## 2026-02-22 – Real Mutation Gate Final Sign-off Pack Added

### English

- Added:
- `docs/security/Synora_Real_Mutation_Gate_Final_Signoff_Pack.md`
- Included copy-ready `Approved` / `Rejected` decision blocks for PR/release notes

### 中文

- 新增：
- `docs/security/Synora_Real_Mutation_Gate_Final_Signoff_Pack.md`
- 提供可直接粘贴到 PR/发布说明的 `Approved` / `Rejected` 决策模板

---

## 2026-02-22 – Runtime Capability Boundary Note Added

### English

- Added:
- `docs/security/Synora_Runtime_Capability_Boundary_2026-02-22.md`
- Explicitly separated:
- currently available simulation/audit capabilities
- gated real mutation capabilities

### 中文

- 新增：
- `docs/security/Synora_Runtime_Capability_Boundary_2026-02-22.md`
- 明确区分：
- 当前可用的 simulation/audit 能力
- 仍受门禁控制的真实变更能力

---

## 2026-02-22 – State and Design Entry Links Aligned

### English

- Added direct boundary-note link to `PROJECT_STATE.md`
- Added quick links in quarantine execution design for:
- runtime boundary note
- final sign-off pack

### 中文

- 在 `PROJECT_STATE.md` 增加运行边界说明直达链接
- 在 quarantine 执行设计书增加快速入口：
- 运行边界说明
- 最终签署决策包

---

## 2026-02-22 – Push Workflow Docs-Only Fast Path Added

### English

- Updated `docs/Push_Workflow_Standard.md`
- Added docs-only fast path to skip unnecessary cargo checks for markdown-only pushes

### 中文

- 更新 `docs/Push_Workflow_Standard.md`
- 增加 docs-only 快速流程，文档提交可跳过不必要的 cargo 检查

---

## 2026-02-22 – Push Workflow Assistant Templates Added

### English

- Extended `docs/Push_Workflow_Standard.md` with:
- assistant mode selection rule (`code` vs `docs-only`)
- fixed output templates for both push modes
- Standardized future assistant push responses with deterministic structure

### 中文

- 扩展 `docs/Push_Workflow_Standard.md`：
- 新增助手模式选择规则（`code` 与 `docs-only`）
- 新增两类固定输出模板
- 统一后续助手推送输出结构，减少临时判断

---

## 2026-02-22 – Real Mutation Runtime Gate Wired (Default-Off)

### English

- Added runtime gate checks before `cleanup quarantine --confirm` execution:
- `execution.real_mutation_enabled` must be `true`
- `execution.approval_record_ref` must be non-empty
- Added security errors and CLI mapping for blocked confirm path
- Updated default `config init` payload with execution gate fields

### 中文

- 在 `cleanup quarantine --confirm` 执行前接入运行时门禁校验：
- `execution.real_mutation_enabled` 必须为 `true`
- `execution.approval_record_ref` 必须非空
- 新增对应安全错误与 CLI 阻断映射
- 更新 `config init` 默认配置，加入 execution 门禁字段

---

## 2026-02-22 – Gate Visibility Command and Windows Config Compatibility

### English

- Added `synora config gate-show [--json]` for runtime gate visibility.
- Fixed `config init` JSON generation to avoid Windows path escaping issues.
- Added backward-compatible fallback parsing for legacy malformed config content.

### 中文

- 新增 `synora config gate-show [--json]`，用于查看运行时门禁状态。
- 修复 `config init` JSON 生成，避免 Windows 路径转义导致配置非法。
- 增加旧配置兼容解析，避免历史 malformed 配置触发集成错误。

---

## 2026-02-22 – Gate Control Command Added (`config gate-set`)

### English

- Added `synora config gate-set` to write execution gate state via CLI.
- Enforced command contract:
- `--enable`/`--disable` mutually exclusive and required
- `--approval-record` required when enabling gate
- Added CLI and repository tests for gate-set validation and persistence.

### 中文

- 新增 `synora config gate-set`，可通过 CLI 写入 execution 门禁配置。
- 强化参数契约：
- `--enable`/`--disable` 必须二选一
- 启用门禁时必须提供 `--approval-record`
- 补充 CLI 与 repository 测试，覆盖参数校验与持久化行为。

---

## 2026-02-22 – Gate-Set Safety Confirmation and Keep-Record Semantics

### English

- Hardened `config gate-set` contract:
- `--enable` now requires explicit `--confirm`
- `--keep-record` is valid only with `--disable`
- Added state visibility sync in `PROJECT_STATE.md` for gate control plane

### 中文

- 强化 `config gate-set` 契约：
- 启用门禁时新增强制 `--confirm`
- `--keep-record` 仅允许与 `--disable` 配合
- 在 `PROJECT_STATE.md` 同步门禁控制面可见性状态

---

## 2026-02-22 – Gate Dry-Run Preview and Gate-Show Verbose Diagnostics

### English

- Added `--dry-run` to `config gate-set` for non-persistent preview.
- Added `--verbose` to `config gate-show` for config path/existence diagnostics.
- Added README quick guide for gate operations.

### 中文

- 为 `config gate-set` 增加 `--dry-run` 预览能力（不落盘）。
- 为 `config gate-show` 增加 `--verbose` 诊断输出（配置路径/存在性）。
- 在 `README.md` 增加 gate 操作快速指引。

---

## 2026-02-22 – Gate Reason Audit and History Query Added

### English

- Added `--reason` contract for persistent `config gate-set` writes.
- Added `config gate-history [--json]` for gate change audit visibility.
- Persisted gate change records into SQLite `gate_history` table.

### 中文

- 为持久化 `config gate-set` 写入新增 `--reason` 契约。
- 新增 `config gate-history [--json]`，用于查看门禁变更审计轨迹。
- 门禁变更记录已持久化至 SQLite `gate_history` 表。

---

## 2026-02-22 – Gate History Filters Added (`--enabled-only`, `--limit`)

### English

- Extended `config gate-history` with:
- `--enabled-only` to filter for enable-state records only
- `--limit <n>` to return latest bounded rows (positive integer)
- Added repository-side filtered query path and CLI validation tests.
- Updated CLI spec, interface spec, README, and smoke checklist to match.

### 中文

- 扩展 `config gate-history`：
- `--enabled-only` 仅查看启用状态记录
- `--limit <n>` 限定返回最新 N 条（正整数）
- 增加 repository 侧过滤查询与 CLI 参数校验测试。
- 已同步更新 CLI 规格、接口规范、README 与冒烟清单。

---

## 2026-02-22 – Gate History Time Filter Added (`--since`)

### English

- Added `--since <unix_ts>` to `config gate-history`.
- Supports combined filtering with `--enabled-only` and `--limit`.
- Added CLI validation tests for missing/invalid `--since` and combined success path.

### 中文

- 为 `config gate-history` 新增 `--since <unix_ts>`。
- 支持与 `--enabled-only`、`--limit` 组合过滤。
- 补充 CLI 参数校验测试（缺值/非法值）及组合成功路径测试。
