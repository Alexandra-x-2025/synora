# Security Policy

Synora performs system-level operations.
Security is a first-class design constraint.

---

# 安全策略

Synora 涉及系统级操作。
安全性是核心设计约束。

---

## Principles

- Quarantine instead of delete
- Backup before registry deletion
- HKCU only by default
- Installer execution whitelist
- Redacted logging
- No telemetry

---

## 原则

- 使用隔离区而非直接删除
- 删除注册表前必须备份
- 默认仅支持 HKCU
- 安装器执行白名单机制
- 日志自动脱敏
- 无遥测

---

## Threat Model

Mitigated threats:

- Arbitrary command execution
- Accidental file deletion
- Registry corruption
- Data leakage via logs

Out of scope:

- Compromised upstream installers
- OS-level vulnerabilities

---

## 威胁模型

已防御：

- 任意命令执行
- 文件误删
- 注册表损坏
- 日志泄露

不在范围内：

- 上游安装包被污染
- 操作系统漏洞

---

## Reporting a Vulnerability

Do not disclose publicly.

Email: your-email@example.com

---

## 漏洞报告

请勿公开披露。

通过邮箱联系维护者。