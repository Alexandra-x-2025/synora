<p align="center">
  <img src="assets/logo/synora-icon.svg" width="180" />
</p>

# Synora

# Synora

Intelligent terminal-first software manager for Windows, focused on safety, transparency, and recoverability.

---

# Synora

面向 Windows 的智能终端优先软件管理工具，专注于安全性、透明性与可恢复性。

---

## ✨ Overview

Synora is a safety-first software lifecycle manager.

It helps users:

- Discover installed software
- Check updates via winget / GitHub
- Apply updates with confirmation
- Manage uninstall workflows
- Clean leftovers safely (move → quarantine)
- Backup & restore registry cleanup (HKCU only)

Synora does not prioritize automation.
It prioritizes control, explainability, and recoverability.

---

## ✨ 项目概述

Synora 是一个以安全为核心的软件生命周期管理工具。

支持：

- 已安装软件发现
- 通过 winget / GitHub 检查更新
- 手动确认更新执行
- 卸载流程管理
- 残留清理（移动至隔离区，而非直接删除）
- 注册表清理前备份与恢复（仅 HKCU）

Synora 不追求“完全自动化”，  
而追求“可控、可解释、可恢复”。

---

## 🔐 Security Philosophy

- No destructive deletion (quarantine first)
- No hidden silent installers
- No automatic registry modification
- No telemetry
- No arbitrary command execution

---

## 🔐 安全原则

- 不做不可恢复删除（优先隔离）
- 不隐藏静默安装
- 不自动修改注册表
- 不收集遥测数据
- 不允许任意命令执行

---

## 🏗 Architecture

Layered architecture:

Domain  
→ Repository  
→ Service  
→ Worker (Task Engine)  
→ Integration  
→ Security Guard  

See: `docs/architecture-overview.md`

---

## 🏗 架构

分层架构：

Domain  
→ Repository  
→ Service  
→ Worker（任务引擎）  
→ Integration  
→ Security Guard（安全守卫）

详见：`docs/architecture-overview.md`

---

## 🚀 Roadmap

v0.x – CLI MVP  
v1.x – Stable CLI  
v2.x – Extended sources & intelligence  

Current: Phase 1 (CLI MVP in progress)

CLI spec: `docs/cli-spec-v0.1.md`

---

## 🧪 CLI v0.1 Commands

`synora software list [--json]`  
`synora update check [--json]`  
`synora update apply --id <package_id> [--dry-run | --confirm] [--json]`  
`synora config init`  
`synora config gate-show [--json] [--verbose]`  
`synora config gate-history [--json]`  
`synora config gate-set (--enable|--disable) [--confirm] [--approval-record <ref>] [--gate-version <version>] [--reason <text>] [--keep-record] [--dry-run] [--json]`

Compatibility: `--yes` is still accepted as an alias of `--confirm`.

---

## 🔐 Gate Operation Quick Guide

Preview current gate:
- `cargo run -- config gate-show --json`

Preview enablement without writing:
- `cargo run -- config gate-set --enable --approval-record docs/security/Synora_Real_Mutation_Gate_Approval_Record_2026-02-22_Draft.md --dry-run --json`

Enable gate (requires explicit confirm):
- `cargo run -- config gate-set --enable --confirm --approval-record docs/security/Synora_Real_Mutation_Gate_Approval_Record_2026-02-22_Draft.md --reason "pilot enable" --json`

Disable gate and keep approval reference:
- `cargo run -- config gate-set --disable --keep-record --reason "rollback to safe default" --json`

Show gate change history:
- `cargo run -- config gate-history --json`

---

## 🦀 Rust Quick Start

Prerequisites:
- Rust toolchain installed (`cargo`)
- Windows with `winget` available (for real integration behavior)

Run:
- `cargo check`
- `cargo test`
- `cargo run -- software list --json`
- `cargo run -- update check --json`
- `cargo run -- config init`

Smoke checklist:
- `docs/testing/Synora_CLI_Smoke_Checklist.md`
- `docs/testing/Phase1_MVP_Readiness_Checklist.md`

---

## 🦀 Rust 快速开始

前置条件：
- 已安装 Rust 工具链（`cargo`）
- Windows 环境可用 `winget`（用于真实集成行为）

运行命令：
- `cargo check`
- `cargo test`
- `cargo run -- software list --json`
- `cargo run -- update check --json`
- `cargo run -- config init`

## 📁 Project Structure

```
synora/
├── README.md
├── LICENSE
├── CHANGELOG.md
├── SECURITY.md
├── CONTRIBUTING.md
├── PROJECT_STATE.md
├── ARCHITECTURE_DECISIONS.md
├── DEVELOPMENT_LOG.md
├── assets/
│   └── logo/
├── docs/
│   ├── architecture/
│   ├── security/
│   ├── testing/
│   ├── product/
│   ├── roadmap.md
│   └── architecture-overview.md
├── src/
├── tests/
└── .github/
```

---

## 🧠 Structure Rationale

Root:
- Governance files: `README`, `SECURITY`, `CONTRIBUTING`
- State and decision files: `PROJECT_STATE`, `ARCHITECTURE_DECISIONS`
- Development timeline: `DEVELOPMENT_LOG`

`docs/architecture/`:
- Core technical documents
- Design plans, interface contracts, data design, tech stack

`docs/security/`:
- Security threat model and future audit reports

`docs/testing/`:
- Testing strategy and CI/QA approach

`docs/product/`:
- Product strategy and roadmap artifacts

---

## 📁 项目结构说明

根目录放置：
- 治理类文件（`README` / `SECURITY` / `CONTRIBUTING`）
- 状态与决策文件（`PROJECT_STATE` / `ARCHITECTURE_DECISIONS`）
- 开发日志（`DEVELOPMENT_LOG`）

这些属于“项目元信息”。

`docs/` 分层：
- `architecture/`：核心技术文档（设计书、接口规范、数据设计、技术选型）
- `security/`：威胁模型与后续审计类文档
- `testing/`：测试与 CI/QA 策略
- `product/`：产品战略与路线图

---

## 🚀 路线图

v0.x – CLI 最小可用版本  
v1.x – 稳定 CLI 版本  
v2.x – 扩展来源与智能能力  

---

## 📜 License

MIT License

---

## 📜 许可证

MIT 许可证
