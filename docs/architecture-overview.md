# Architecture Overview

Synora is designed as a safety-first, terminal-oriented software lifecycle manager.

The architecture emphasizes:

- Clear responsibility boundaries
- Strict security guardrails
- Explainability of decisions
- Recoverability of destructive actions

---

# 架构概览

Synora 是一个以安全为核心、终端优先的软件生命周期管理工具。

架构设计强调：

- 清晰的职责边界
- 严格的安全约束
- 决策可解释
- 破坏性操作可恢复

---

## Layered Design

Synora follows a layered architecture:

Domain  
→ Repository  
→ Service  
→ Worker (Task Engine)  
→ Integration  
→ Security Guard  

---

## 分层结构

Synora 采用分层架构：

Domain（领域层）  
→ Repository（数据访问层）  
→ Service（业务逻辑层）  
→ Worker（任务引擎）  
→ Integration（系统交互层）  
→ Security Guard（安全守卫）

---

## Domain Layer

Pure logic only.

- No IO
- No database access
- No system calls
- Contains models, scoring logic, risk classification

---

## Domain 层

纯逻辑层。

- 不进行 IO
- 不访问数据库
- 不进行系统调用
- 包含模型、评分逻辑、风险分类

---

## Repository Layer

Responsible for:

- SQLite interaction
- Data persistence
- Transaction control

---

## Repository 层

负责：

- SQLite 数据交互
- 数据持久化
- 事务控制

---

## Service Layer

Coordinates domain logic and repository access.

- Update workflow
- Cleanup planning
- Registry backup logic
- Source resolution

---

## Service 层

协调领域逻辑与数据访问。

- 更新流程
- 清理计划
- 注册表备份
- 来源解析

---

## Worker (Task Engine)

Handles asynchronous operations:

- Update execution
- Retry policy
- Concurrency control
- Progress tracking
- Cancellation

---

## Worker（任务引擎）

处理异步操作：

- 更新执行
- 重试策略
- 并发控制
- 进度追踪
- 任务取消

---

## Integration Layer

Interacts with:

- winget
- GitHub API
- Windows Registry
- Filesystem
- Installer binaries

All interactions must pass Security Guard.

---

## Integration 层

与以下系统交互：

- winget
- GitHub API
- Windows 注册表
- 文件系统
- 安装器程序

所有交互必须经过 Security Guard。

---

## Security Guard

The final safety boundary.

Ensures:

- No arbitrary command execution
- Installer parameter whitelist
- No direct file deletion
- Registry backup enforcement

---

## Security Guard（安全守卫）

最终安全边界。

确保：

- 禁止任意命令执行
- 安装器参数白名单
- 禁止直接删除文件
- 强制注册表备份