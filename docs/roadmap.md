# ROADMAP

## 文档目的
定义阶段目标、里程碑与完成标准，并与产品范围保持一致。

## 当前状态
- 状态：Phase 8 completed（稳定化与发布收口）
- 来源：同步自 `docs/PRODUCT_SPEC.md` 与 `docs/FEATURES.md`

## 执行进度快照（2026-02-22）
- Phase 1-7：已完成并通过 CLI 回归验证
- Phase 8 Step 1：已完成（发布清单 + 一键回归脚本）
- Phase 8 Step 2：已完成（job submit/list/retry 最小队列闭环）
- Phase 8 Step 3：已完成（deadletter-list / replay-deadletter 运维闭环）
- Phase 8 Step 4：已完成（发布说明与最终门禁收口）
- Phase 8 Step 5：已完成（一键回归通过 + Go/No-Go 判定为 Go）

## 实现基线里程碑
1. 2026-02-22：`v0.2 CLI baseline frozen`（Source/Update 主链路可运行）
- 对应决策：`ADR-005`（`decisions/ARCHITECTURE_DECISIONS.md`）
- 覆盖能力：`source` 生命周期、`update check/apply/history`、gate 门禁与执行审计。

## 上下文输入
- 当前阶段：产品与功能分层草拟
- 当前约束：暂不进入实现排期与任务拆解

## 预期输出
- 形成清晰阶段顺序（MVP -> Phase 2 -> 后续）
- 功能层级与阶段目标一一对应

## 阶段决策（Phase 1 Freeze）
1. MVP 截止条件：以稳定性门禁为准（满足 PRODUCT_SPEC DoD + 安全基线）。
2. 更新检测默认模式：MVP 默认手动触发，轻量定时作为可选项。
3. Phase 2 投入边界：仅做能力增强，不新增默认开启的高风险自动执行。
4. Discovery Phase 2 扩展顺序：`Program Files` -> `Start Menu` -> `Portable`。
5. 队列后端路线：V1/V2 使用 SQLite 本地队列，Phase 3 评估 Redis/NATS 迁移。

## 冻结阶段计划（Phase 1 Freeze）
1. Phase 1（MVP）
- 一键安装/升级/卸载能力
- 可更新软件检测（手动触发或轻量定时）
- 下载源基础安全校验（签名/来源/白名单）
- 软件体检、清理、修复最小安全子集
- 软件自动发现并生成个人软件库（Software Discovery）
- AI 软件整理分析（Analyze）
- AI 场景化安装建议（Recommend）
- AI 修复方案（Repair Plan，计划模式）
- 本地任务队列（SQLite）与 Worker 基础框架
- Scheduler 基础能力（定时入队）
- 下载模块 MVP（受控下载 + 校验 + 审计）
- 全局搜索 UI 主入口（Raycast 风格）
- 软件仓库系统 MVP（公共仓库只读 + 个人仓库管理）
- 安全门禁与审计链路可用
- Discovery 范围：Registry-only（已决策）

2. Phase 2（增强）
- 软件展示与搜索增强（分类、筛选、可读性优化）
- 来源推荐能力增强（排序与解释信息）
- 自动发现结果归并与置信度策略优化
- 插件系统（官方插件）首批落地：GitHub / JetBrains / Python / Node
- AI 修复受控执行（Repair Apply，需 confirm + gate）
- 扫描/下载/更新全异步化（queue + worker 完整链路）
- 下载增强（多源镜像、断点续传优化、调度策略）
- 全局搜索排序/召回增强（上下文感知、历史偏好）
- 社区仓库贡献流程（software.yaml 提交/审核/发布）
- AI 辅助仓库条目生成与质量评分

3. Phase 3（后续）
- 评估是否引入榜单体系
- 评估是否引入评分生态
- 仅在不偏离系统治理定位前提下推进
- 评估 Rust WASM 插件沙箱与受控插件市场
- 评估并迁移到分布式队列后端（如 Redis/NATS）

4. 暂不做（当前阶段）
- 重运营化的软件榜单体系
- 面向消费分发的评分生态

## 时间窗与验收口径（Phase 1 Freeze）
1. Phase 1-2：T0 至 T0+2 周
- 验收：配置/门禁/审计闭环可用，核心命令退出码稳定。

2. Phase 3-4：T0+3 周 至 T0+5 周
- 验收：发现入库链路稳定；仓库同步与检索可用；冲突记录可追溯。

3. Phase 5-6：T0+6 周 至 T0+8 周
- 验收：下载校验失败阻断生效；高风险执行严格受 gate 控制；回滚链路可审计。

4. Phase 7-8：T0+9 周 至 T0+12 周
- 验收：AI plan-only 与全局搜索入口联通；发布门禁与稳定性指标满足 MVP DoD。

## V1 到终局里程碑映射（Draft）
一句话愿景：`Windows 的 Raycast + Homebrew + AI 安全编排层`

1. 入口层（Raycast）
- V1：全局搜索可触达核心动作（下载/任务/安全/AI）
- V2：搜索排序与召回增强（上下文、历史偏好）
- V3：多入口统一（CLI + Desktop UI + 可选 Web 控制台）

2. 供给层（Homebrew）
- V1：公共仓库只读 + 个人仓库可写 + `software.yaml` 导入导出
- V2：社区提交流程（submit/review/publish）与质量评分
- V3：多仓库治理与插件化仓库适配（组织级策略）

3. 控制层（AI 安全编排）
- V1：AI analyze/recommend/repair-plan + gate + 审计闭环
- V2：受控 `repair-apply`（confirm + gate + queue + worker）
- V3：策略化自动化编排（在审批与审计约束下）

## 执行版开发顺序（Phase 1-Phase 8）
说明：该顺序按“依赖关系”而非“功能热度”排列，避免返工。

1. Phase 1：基线冻结与工程骨架
- 目标：冻结 V1 文档输入，建立可编译工程骨架。
- 关键范围：`FINAL_ARCHITECTURE`、`API_SPEC`、`DATA_MODEL`、`TECH_STACK` 对齐。
- 退出标准：`cargo check` 通过；核心命令入口可运行。

2. Phase 2：存储与审计底座
- 目标：落地 SQLite + config + gate + audit 基础能力。
- 关键范围：`config init/gate-show/gate-set/gate-history`。
- 退出标准：门禁与审计查询闭环可用，退出码语义稳定。

3. Phase 3：发现与个人软件库
- 目标：完成 Registry-only 自动发现并写入 inventory。
- 关键范围：`software discover scan`、`software list`、增量更新与去重。
- 退出标准：扫描/入库/查询链路稳定，回归测试通过。

4. Phase 4：仓库系统 MVP（供给层）
- 目标：完成公共仓库只读 + 个人仓库可写 + `software.yaml` 导入导出。
- 关键范围：`repo list/add/sync/package search/import/export`。
- 退出标准：仓库优先级规则（personal > trusted public）生效。

5. Phase 5：下载与校验链路
- 目标：下载任务、哈希校验、来源校验、失败阻断。
- 关键范围：`download start/list/show/retry/verify`。
- 退出标准：校验失败必阻断后续执行，审计字段完整。

6. Phase 6：执行链路与安全门禁
- 目标：update/cleanup confirm 链路接入 queue/worker/security。
- 关键范围：confirm + gate + ticket + rollback。
- 退出标准：高风险误执行为 0，安全阻断路径全部可测。

7. Phase 7：AI 助手与全局搜索入口（入口层）
- 目标：接入 AI plan-only 能力并可通过全局搜索触达。
- 关键范围：`ai analyze/recommend/repair-plan` + `ui search/action-run`。
- 退出标准：全局搜索可触达核心动作，AI 输出具备 reason/confidence/risk。

8. Phase 8：稳定化与 V1 发布
- 目标：性能、重试、运维、发布门禁收口。
- 关键范围：重试策略、死信处理、安全基线、发布检查清单。
- 退出标准：达到 PRODUCT_SPEC 的 MVP DoD，可发布 V1。

## 更新规则
- 阶段边界变更时，必须同步更新：
  - `docs/PRODUCT_SPEC.md`
  - `docs/FEATURES.md`
  - `logs/DEVELOPMENT_LOG.md`
- 本文档记录“阶段目标”，不记录“实现细节任务”。
