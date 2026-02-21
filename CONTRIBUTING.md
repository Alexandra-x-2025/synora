# Contributing to Synora

Thank you for contributing.

Synora is safety-first.
Security boundaries must not be weakened.

---

# 参与 Synora

感谢你的贡献。

Synora 以安全为核心。
不得削弱安全边界。

---

## Architecture Rules

- No direct file deletion
- No HKLM registry modification by default
- No arbitrary shell execution
- All installer runs via Security Guard

---

## 架构规则

- 禁止直接删除文件
- 默认禁止 HKLM 注册表修改
- 禁止任意 shell 执行
- 所有安装操作必须通过安全守卫

---

## PR Checklist

- Safe?
- Recoverable?
- Explainable?
- Tested?

---

## PR 检查清单

- 是否安全？
- 是否可恢复？
- 是否可解释？
- 是否测试？