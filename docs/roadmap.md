# Roadmap

Synora evolves in structured phases.

---

# 路线图

Synora 按阶段稳步演进。

---

## Phase 0 – Bootstrap (Completed)

- Repository structure
- Documentation
- Security model definition
- CLI MVP planning

---

## 阶段 0 – 项目初始化（已完成）

- 仓库结构搭建
- 文档体系建立
- 安全模型定义
- CLI MVP 规划

---

## Phase 1 – CLI MVP

- Software discovery
- Update check
- Safe update execution
- Quarantine cleanup
- Basic registry backup
- CLI specification v0.1
- CLI command skeleton (`software list`, `update check`, `update apply`, `config init`)

Status: Completed (MVP Ready on 2026-02-22)

---

## 阶段 1 – CLI 最小可用版本

- 软件发现
- 更新检查
- 安全更新执行
- 隔离清理
- 基础注册表备份
- CLI v0.1 规范
- CLI 命令骨架（`software list`、`update check`、`update apply`、`config init`）

状态：已完成（2026-02-22 达到 MVP Ready）

---

## Phase 2 – Intelligence Layer

- Source suggestion
- Rule engine scoring refinement
- Improved retry strategy
- Enhanced logging diagnostics

Status: Completed (Week 1-4 closed on 2026-02-22)

### Phase 2 Weekly Breakdown

- Week 1: Data foundation and repository contract
- Week 2: Source suggestion pipeline (winget + metadata heuristics)
- Week 3: Rule-engine scoring and retry policy refinement
- Week 4: Logging diagnostics + hardening tests + integration review

Execution summary:
- Week 1 completed: SQLite repository baseline and `config init` schema bootstrap
- Week 2 completed: source suggestion scoring + update-signal blending + verbose diagnostics
- Week 3 completed: `update apply` plan persistence + `config history-list` audit visibility
- Week 4 completed: `config audit-summary` aggregates + confirmed-plan safety placeholders (`registry_backup` / `quarantine`)

---

## 阶段 2 – 智能增强

- 来源推荐
- 规则引擎优化
- 重试策略增强
- 日志诊断优化

状态：已完成（Week 1-4 于 2026-02-22 收口）

### 阶段 2 周计划

- 第 1 周：数据基础与 repository 契约落地
- 第 2 周：来源推荐流水线（winget + 元数据启发式）
- 第 3 周：规则引擎评分与重试策略优化
- 第 4 周：日志诊断增强 + 加固测试 + 集成评审

执行总结：
- 第 1 周完成：SQLite repository 基线与 `config init` schema 联动初始化
- 第 2 周完成：来源推荐评分、更新信号融合与 `--verbose` 诊断输出
- 第 3 周完成：`update apply` 计划持久化与 `config history-list` 审计可见
- 第 4 周完成：`config audit-summary` 聚合诊断与 confirmed 计划安全占位审计（`registry_backup` / `quarantine`）

---

## Phase 3 – Stability & Ecosystem

- Plugin-like source adapters
- Extended installer coverage
- Performance improvements
- Community contributions

Status: In progress (started on 2026-02-22)

Kickoff update:
- Added quarantine execution design draft:
- `docs/security/Synora_Quarantine_Execution_Design.md`
- Defined proposed command contract, rollback boundary, and audit status vocabulary
- Added cleanup CLI contract draft:
- `docs/architecture/Synora_Cleanup_Quarantine_CLI_Contract_Draft.md`
- Defined CLI flags, output schema, and failure semantics for Phase 3 execution path
- Added module-level implementation workplan:
- `docs/architecture/Synora_Cleanup_Quarantine_Implementation_Workplan.md`
- Defined service/repository/security/integration/cli execution order and milestones
- M1 implementation started:
- `cleanup quarantine` CLI skeleton + dry-run audit persistence (`quarantine_planned`)
- M2 implementation started:
- confirmed precheck path persists `quarantine_confirmed` and safety evidence placeholders (`registry_backup` / `quarantine`) without crossing mutation boundary
- M3 implementation started:
- confirmed path now records simulated execution success (`quarantine_success`) and marks mutation boundary reached

---

## 阶段 3 – 稳定与生态扩展

- 插件式来源扩展
- 安装器覆盖增强
- 性能优化
- 社区协作

状态：进行中（2026-02-22 启动）

启动进展：
- 已新增 quarantine 实执行设计草案：
- `docs/security/Synora_Quarantine_Execution_Design.md`
- 明确了建议命令契约、回滚边界与审计状态词汇
- 已新增 cleanup CLI 契约草案：
- `docs/architecture/Synora_Cleanup_Quarantine_CLI_Contract_Draft.md`
- 明确了 Phase 3 执行路径的参数规则、输出字段与失败语义
- 已新增模块级实施任务分解：
- `docs/architecture/Synora_Cleanup_Quarantine_Implementation_Workplan.md`
- 明确了 service/repository/security/integration/cli 的执行顺序与里程碑
- 已启动 M1 实现：
- `cleanup quarantine` CLI 骨架与 dry-run 审计持久化（`quarantine_planned`）
- 已启动 M2 实现：
- confirmed precheck 路径写入 `quarantine_confirmed` 与安全证据占位（`registry_backup` / `quarantine`），但不跨越变更边界
- 已启动 M3 实现：
- confirmed 路径记录模拟执行成功（`quarantine_success`），并标记已到达变更边界
