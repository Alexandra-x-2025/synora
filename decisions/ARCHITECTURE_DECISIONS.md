# ARCHITECTURE_DECISIONS

## 文档目的
记录关键架构决策（ADR）及其理由。

## 当前状态
- 状态：v1 Frozen（设计冻结）
- ADR：已创建（001-005）

## 上下文输入
- 当前尚未进入详细架构设计

## 预期输出
- 每个关键决策可追溯
- 未来变更可对照历史决策

## ADR 规范决策（Phase 1 Freeze）
- 编号规则：`ADR-XXX`（三位递增，示例：`ADR-001`、`ADR-002`）。
- 生效规则：每条 ADR 必须有 `Status`，且只能通过新增 ADR 或 `Superseded by` 进行替代，禁止静默改写。
- 模板字段固定为：
  - `Title`
  - `Date`
  - `Status`（Proposed / Accepted / Superseded / Deprecated）
  - `Context`
  - `Decision`
  - `Consequences`
  - `Alternatives Considered`
  - `Related Docs`

## ADR-001
### Title
CLI-first + Local-first 作为 V1 主交付形态
### Date
2026-02-22
### Status
Accepted
### Context
项目初期需要快速验证“软件管理闭环 + 安全门禁 + 审计追溯”，同时降低分布式复杂度。
### Decision
V1 采用 CLI-first、Local-first：核心流程在本机闭环执行，云端能力非必需。
### Consequences
- 优点：实现路径短，风险可控，易于审计。
- 代价：多端协同与远程调度延后到后续阶段。
### Alternatives Considered
- Web-first：前期交互友好，但后端复杂度更高。
- Cloud-first：可扩展性强，但与本地安全门禁目标冲突。
### Related Docs
- `docs/PRODUCT_SPEC.md`
- `docs/ROADMAP.md`
- `docs/FINAL_ARCHITECTURE.md`

## ADR-002
### Title
执行门禁默认关闭，真实变更需显式审批与确认
### Date
2026-02-22
### Status
Accepted
### Context
项目含下载、安装、清理、修复等高风险动作，必须满足安全可控与责任可追溯。
### Decision
默认 `real_mutation_enabled=false`；真实变更需审批记录、确认动作、风险分级通过后执行。
### Consequences
- 优点：误操作与供应链风险显著下降。
- 代价：操作路径更长，MVP 体验略重。
### Alternatives Considered
- 默认启用真实变更：体验快，但安全不可接受。
- 仅二次确认无审批记录：可追溯性不足。
### Related Docs
- `SECURITY.md`
- `docs/SANDBOX_EXECUTION_POLICY_DRAFT.md`
- `docs/HASH_AND_SIGNATURE_POLICY_DRAFT.md`

## ADR-003
### Title
插件系统分阶段落地：V1/V2 原生插件，V3 再评估 WASM
### Date
2026-02-22
### Status
Accepted
### Context
插件是扩展来源与能力的核心，但安全模型、兼容性、运行时复杂度需渐进控制。
### Decision
V1/V2 先采用受控原生插件模型，配合签名、权限矩阵与生命周期治理；WASM 放到 V3 评估。
### Consequences
- 优点：先建立治理模型与稳定接口，再扩展沙箱执行能力。
- 代价：早期跨语言插件生态受限。
### Alternatives Considered
- 直接 WASM-first：架构先进，但前期复杂度过高。
- 不做插件：短期简单，长期扩展能力不足。
### Related Docs
- `docs/PLUGIN_SYSTEM.md`
- `docs/PLUGIN_PERMISSION_MATRIX.md`
- `docs/PLUGIN_LIFECYCLE_SEQUENCE.md`

## ADR-004
### Title
2026-02-22 设计冻结生效
### Date
2026-02-22
### Status
Accepted
### Context
设计冻结清单 A 区（待决策）与 B 区（待补充）已全部完成，需要形成正式冻结点，作为后续实现与变更评审基线。
### Decision
自 2026-02-22 起，以下文档进入冻结基线：`docs/PRODUCT_SPEC.md`、`docs/FINAL_ARCHITECTURE.md`、`docs/API_SPEC.md`、`docs/DATA_MODEL.md`、`docs/TECH_STACK.md`、`docs/ROADMAP.md`、`SECURITY.md`。后续变更需通过 ADR 或冻结清单追加流程。
### Consequences
- 优点：实现阶段输入稳定，减少返工与语义漂移。
- 代价：新增需求需走变更流程，短期灵活性下降。
### Alternatives Considered
- 继续保持 Draft：灵活但易失控，冻结目标无法达成。
- 仅冻结部分文档：会导致跨文档不一致风险。
### Related Docs
- `docs/DESIGN_FREEZE_CHECKLIST.md`
- `docs/PRODUCT_SPEC.md`
- `docs/ROADMAP.md`

## ADR-005
### Title
v0.2 CLI Baseline 冻结（Source/Update 主链路）
### Date
2026-02-22
### Status
Accepted
### Context
`cargo check` 与 `cargo test` 已通过，CLI 主链路已形成可重复回归能力；需要明确实现基线范围与非范围，避免后续实现偏移。
### Decision
冻结 `v0.2 CLI baseline`，包含以下范围：
- `source` 生命周期：`suggest`、`review`、`review-bulk`、`list`、`apply-approved`、`registry-list`、`registry-disable`、`registry-enable`。
- `update` 生命周期：`check`、`apply(dry-run/confirm)`、`history`。
- 门禁与审计：`update apply --confirm` 复用 execution gate，`update_operation_history` 记录执行与回滚结构化字段。
- 回归基线：`docs/CLI_SMOKE_TESTS.md` 作为当前阶段 smoke 清单。
### Consequences
- 优点：实现边界清晰，可进入增量迭代与缺陷修复阶段。
- 代价：超出基线的新能力（例如真实下载/安装执行、外部源在线探测）需走新增 ADR 或明确变更流程。
### Alternatives Considered
- 不冻结直接继续扩展：短期速度快，但容易导致范围漂移与回归不可控。
- 仅冻结 `source` 不冻结 `update`：会造成更新链路验收标准不完整。
### Related Docs
- `README.md`
- `docs/CLI_SMOKE_TESTS.md`
- `docs/API_SPEC.md`
- `logs/DEVELOPMENT_LOG.md`

## 更新规则
- 关键设计决策必须新增 ADR 条目。
- ADR 一旦生效，不可静默覆盖，只能追加修订。
