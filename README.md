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