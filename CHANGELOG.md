# Changelog

All notable changes will be documented here.

---

# 更新日志

所有重要变更将记录于此。

---

## [Unreleased]

### Added
- Project bootstrap
- Documentation structure
- Rust CLI bootstrap (`Cargo.toml`, layered `src/` modules)
- CLI command wiring for `software list`, `update check`, `update apply`, `config init`
- Winget integration for list/check with JSON-first and text-fallback parsing
- CLI contract tests and integration parser tests
- `SYNORA_HOME`-aware path strategy and Rust logging bootstrap
- Smoke checklist and Phase 1 readiness checklist under `docs/testing/`

---

## [Unreleased]

### 新增
- 项目初始化
- 文档结构建立
- Rust CLI 初始化（`Cargo.toml` 与分层 `src/` 模块）
- 命令接线：`software list`、`update check`、`update apply`、`config init`
- Winget 集成（列表/更新检测）与 JSON 优先、文本回退解析
- CLI 契约测试与集成解析测试
- 支持 `SYNORA_HOME` 的路径策略与 Rust 日志初始化
- `docs/testing/` 下新增 smoke 与 Phase 1 就绪检查清单
