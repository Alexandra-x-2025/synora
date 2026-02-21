# Synora – Architecture Decisions

This document records major architectural decisions.
These decisions are stable unless explicitly revised.

---

# Synora – 架构决策记录

本文件记录关键架构决策。
除非明确修订，否则视为稳定决策。

---

## AD-001: Windows-First Architecture

### Decision
Synora is designed as a Windows-first software manager.

### Rationale
- Deep integration with Windows Registry
- winget integration
- Installer execution
- Windows-specific cleanup logic

Cross-platform support is not a primary goal at this stage.

---

## AD-001：Windows 优先架构

### 决策
Synora 以 Windows 平台为优先目标。

### 原因
- 深度集成 Windows 注册表
- 集成 winget
- 安装器执行
- Windows 专属清理逻辑

当前阶段不追求跨平台。

---

## AD-002: Safety-First Philosophy

### Decision
All destructive operations must be reversible.

### Implementation Rules
- No direct file deletion
- Move to quarantine first
- Registry deletion requires backup
- High-risk operations require confirmation

---

## AD-002：安全优先原则

### 决策
所有破坏性操作必须可恢复。

### 实现规则
- 禁止直接删除文件
- 必须移动至隔离区
- 注册表删除必须备份
- 高风险操作必须确认

---

## AD-003: Layered Architecture

### Decision
Synora follows a strict layered design:

Domain  
→ Repository  
→ Service  
→ Worker  
→ Integration  
→ Security Guard  

### Rationale
- Clear separation of concerns
- Testability
- Maintainability
- Prevents unsafe cross-layer access

---

## AD-003：分层架构

### 决策
Synora 采用严格分层结构：

Domain  
→ Repository  
→ Service  
→ Worker  
→ Integration  
→ Security Guard  

### 原因
- 职责清晰
- 易测试
- 易维护
- 防止跨层安全漏洞

---

## AD-004: Security Guard Mandatory

### Decision
All system-level operations must pass Security Guard.

### Scope
- Installer execution
- Registry deletion
- File cleanup
- Command invocation

No direct shell execution allowed.

---

## AD-004：强制安全守卫

### 决策
所有系统级操作必须通过 Security Guard。

### 范围
- 安装器执行
- 注册表删除
- 文件清理
- 命令调用

禁止直接 shell 执行。

---

## AD-005: Terminal-First Interface

### Decision
Synora will prioritize CLI before GUI.

### Rationale
- Faster MVP delivery
- Power-user focus
- Easier testing
- Lower architectural complexity

GUI may be added later.

---

## AD-005：终端优先策略

### 决策
Synora 优先实现 CLI。

### 原因
- 更快推出 MVP
- 面向高级用户
- 易测试
- 架构复杂度低

GUI 未来可扩展。

---

## AD-006: Code Storage & Tooling Strategy

### Decision
- Code stored on Windows filesystem
- AI tooling runs in WSL
- Windows handles execution and debugging

### Rationale
Avoid Windows network port conflicts and preserve native behavior.

---

## AD-006：代码与工具链策略

### 决策
- 代码放在 Windows 文件系统
- AI 工具运行在 WSL
- Windows 负责执行与调试

### 原因
避免 Windows 端口冲突，同时保持原生行为一致性。