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
