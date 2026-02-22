# DEVELOPMENT_LOG

## 文档目的
记录项目演进时间线与关键变更。

## 当前状态
- 状态：占位日志

## 上下文输入
- 当前阶段仅进行模板搭建

## 预期输出
- 关键操作可追踪
- 变更上下文可复盘

## 日志规范决策（Phase 1 Freeze）
- 日志粒度：按“设计决策批次 / 文档冻结批次 / 实现里程碑”记录，不记录零散微调。
- 记录格式：统一使用日期段落 + 扁平要点，至少包含“变更范围、影响文档、状态结论”。
- 责任标注：冻结类记录默认写明“你（确认）+ 我（落文）”。

## 里程碑记录规范（Phase 1 Freeze）
1. 每条里程碑必须包含：`变更范围`、`影响文档/模块`、`状态结论`。
2. 里程碑按“批次”记录，不拆成过细操作日志。
3. 冻结相关里程碑必须附责任标注：`你（确认）+ 我（落文）`。

## 发布记录规范（Phase 1 Freeze）
1. 每个发布记录必须包含：`版本号`、`发布日期`、`变更摘要`、`风险说明`、`回滚方式`。
2. 发布记录来源于已合并内容，不记录计划项。
3. 发生发布阻断时，必须追加“阻断原因 + 解除条件”。

## 更新规则
- 每次阶段性变更后必须追加记录。
- 日志采用追加写，不覆盖历史条目。

---

## 2026-02-22
- 完成文档统一模板化（固定结构）。
- 明确当前仅占位，不进入软件详细设计。

## 2026-02-22
- 完成产品文档草稿对齐：`docs/PRODUCT_SPEC.md`、`docs/FEATURES.md`、`docs/ROADMAP.md`。
- 明确功能分层：MVP 必做 / Phase 2 后置 / 暂不做。
- 保持状态为 Draft，暂不进入实现设计与冻结。

## 2026-02-22
- 将“软件自动发现（Software Discovery）并生成个人软件库”同步到 `docs/FEATURES.md` 与 `docs/ROADMAP.md`。
- 保持状态为 Draft，未进入实现设计。

## 2026-02-22
- 新增并拍板 `docs/Discovery_Scope_Decision_One_Pager.md`。
- Discovery 范围确定为 MVP 使用 Registry-only（HKLM/HKCU Uninstall）。
- Multi-source（Program Files/Start Menu）明确后置到 Phase 2。

## 2026-02-22
- 将 `docs/ARCHITECTURE.md` 从占位升级为 v0.2 Draft，补齐模块边界、关键数据流、状态机与失败语义。
- 将 `docs/DATA_MODEL.md` 从占位升级为 v0.2 Draft，补齐核心实体、字段、约束、关系与迁移策略。
- 将 `docs/API_SPEC.md` 从占位升级为 v0.2 Draft，补齐 CLI 契约、输出字段、错误语义与兼容策略。
- 同步 `docs/PRODUCT_SPEC.md` 与 `docs/ROADMAP.md`：Discovery MVP 范围明确为 Registry-only，Phase 2 讨论多源扩展。
- 本轮仍保持 Draft，不做冻结。

## 2026-02-22
- 新增 `docs/PLUGIN_SYSTEM.md`（插件系统 1 页设计草案）。
- 在 `docs/PRODUCT_SPEC.md` 增加插件系统目标、类型、分阶段策略与安全边界。
- 同步 `docs/ARCHITECTURE.md`：增加 Plugin Runtime 模块与插件调用流程。
- 同步 `docs/DATA_MODEL.md`：增加 `plugin_registry` 与 `plugin_execution_history` 草案表。
- 同步 `docs/API_SPEC.md`：增加 `plugin list/enable/disable/run` 命令契约草案。
- 同步 `docs/FEATURES.md`、`docs/ROADMAP.md`、`docs/TECH_STACK.md`、`docs/AI_CONTEXT.md`、`SECURITY.md`。
- 本轮保持 Draft，不冻结。

## 2026-02-22
- 完成插件系统三项基础设计（Draft）：Manifest 规范、权限模型、生命周期与错误码映射。
- `docs/PLUGIN_SYSTEM.md`：新增 Manifest 字段规则、权限矩阵、状态迁移与错误码映射。
- `docs/API_SPEC.md`：补充插件命令失败示例（validation/security/integration）。
- `docs/DATA_MODEL.md`：补充插件 runtime/api_compat/actions/result_code 字段与状态词汇。
- `docs/SECURITY.md`：补充插件权限策略与供应链策略。
- `docs/ARCHITECTURE.md`：补充插件执行前置校验清单。
- 保持 Draft，不冻结。

## 2026-02-22
- 新增 `docs/PLUGIN_MANIFEST_SCHEMA.md`：插件 manifest JSON Schema Draft v1.0。
- 新增 `docs/PLUGIN_PERMISSION_MATRIX.md`：插件类型与权限矩阵、分级与授权规则。
- 新增 `docs/PLUGIN_LIFECYCLE_SEQUENCE.md`：生命周期状态转换与成功/失败时序。
- 更新 `docs/PLUGIN_SYSTEM.md`：补充关联文档入口。
- 本轮继续保持 Draft，不冻结。

## 2026-02-22
- 根据新需求升级 AI 模块为“AI 管理助手”（Draft）。
- `docs/PRODUCT_SPEC.md`：新增三类能力（软件整理分析、场景化安装建议、修复方案）与执行边界。
- `docs/FEATURES.md`、`docs/ROADMAP.md`：加入 AI 管理助手分阶段落地。
- `docs/ARCHITECTURE.md`：新增 `integrations.ai_assistant` 模块与 Flow E/F/G。
- `docs/API_SPEC.md`：新增 `ai analyze / ai recommend / ai repair-plan` 命令草案，并预留 `ai repair-apply`（Phase 2）。
- `docs/DATA_MODEL.md`：新增 `ai_insight_history`、`repair_plan_history` 草案表。
- `SECURITY.md`、`docs/AI_CONTEXT.md`：补充 AI 修复安全边界（plan-only 默认，执行需 gate）。
- 本轮保持 Draft，不冻结。

## 2026-02-22
- 新增临时数据库设计文档 `docs/DATABASE_DESIGN_DRAFT.md`。
- 新增临时初始化脚本 `docs/sql/V001__init_schema_draft.sql`（SQLite）。
- `docs/DATA_MODEL.md` 升级为 v0.3 Draft，并对齐 SQL 基线入口。
- 增补 `plugin_registry.lifecycle_status` 字段约束，统一插件生命周期语义。
- 本轮为暂时性数据库设计，未冻结。

## 2026-02-22
- 架构升级为分层模型：UI / API / Service / Worker / Storage（Draft）。
- `docs/ARCHITECTURE.md`：新增目标目录结构（api/core/services/workers/queue/scheduler/plugins/ai/integration/security/audit/db/contracts）。
- `docs/ARCHITECTURE.md`：新增 queue/scheduler 模块职责与任务状态机。
- `docs/ROADMAP.md`：将本地任务队列与 scheduler 纳入 MVP，分布式队列迁移纳入 Phase 3 评估。
- `docs/DATA_MODEL.md`：新增 `job_queue`、`scheduler_task` 草案实体。
- `docs/sql/V001__init_schema_draft.sql`：同步新增队列表与索引。
- `docs/DATABASE_DESIGN_DRAFT.md`：新增后台任务域说明。
- 本轮保持 Draft，不冻结。

## 2026-02-22
- 新增 `docs/JOB_TYPES_DRAFT.md`：定义首批 job_type 与 payload_json 契约（discover/source/update/cleanup/ai）。
- `docs/ARCHITECTURE.md`：补充任务类型契约引用入口。
- `docs/DATA_MODEL.md`：在临时数据库基线中加入任务类型契约引用。
- 本轮保持 Draft，不冻结。

## 2026-02-22
- `docs/API_SPEC.md` 新增任务队列命令契约：`job submit/list/show/retry`。
- 新增 `docs/JOB_RETRY_POLICY_DRAFT.md`：定义重试退避、死信触发、人工介入与安全规则。
- `docs/JOB_TYPES_DRAFT.md` 增加重试策略关联文档入口。
- `docs/ARCHITECTURE.md` 增加重试策略引用入口。
- 本轮保持 Draft，不冻结。

## 2026-02-22
- 新增 `docs/JOB_OPERATIONS_PLAYBOOK.md`：任务队列运维与排障手册。
- 新增 `docs/API_ERROR_CATALOG.md`：统一错误码与错误目录草案。
- `docs/API_SPEC.md` 增加关联文档入口（错误目录、任务类型、重试策略、运维手册）。
- `docs/ARCHITECTURE.md` 增加任务运维手册入口。
- 本轮保持 Draft，不冻结。

## 2026-02-22
- 新增下载模块设计（Draft），并完成跨文档同步。
- `docs/PRODUCT_SPEC.md`：加入下载模块目标、MVP 边界与验收条目。
- `docs/ARCHITECTURE.md`：加入 `integration.download` 与下载任务 Flow H。
- `docs/SECURITY.md`：加入下载模块安全边界（白名单/哈希/路径防护/审计）。
- `docs/JOB_TYPES_DRAFT.md`：新增 `download.fetch`、`download.verify`、`download.cleanup`。
- `docs/DATA_MODEL.md`：新增 `download_task_history`、`download_artifact`。
- `docs/sql/V001__init_schema_draft.sql`：同步新增下载相关表与索引。
- `docs/API_SPEC.md`：新增 `download start/list/show/retry/verify` 命令契约。
- `docs/FEATURES.md`、`docs/ROADMAP.md`、`docs/DATABASE_DESIGN_DRAFT.md`、`docs/API_ERROR_CATALOG.md` 同步下载模块条目。
- 本轮保持 Draft，不冻结。

## 2026-02-22
- 新增 `docs/DOWNLOAD_SOURCE_POLICY_DRAFT.md`：下载来源信任模型、白名单规则、风险分级与拦截策略。
- 新增 `docs/DOWNLOAD_CACHE_POLICY_DRAFT.md`：下载缓存保留、复用与清理策略。
- `SECURITY.md` 增加下载来源策略入口引用。
- `docs/ARCHITECTURE.md` 增加下载来源/缓存策略入口引用。
- `docs/DATABASE_DESIGN_DRAFT.md` 增加下载缓存策略入口引用。
- 本轮保持 Draft，不冻结。

## 2026-02-22
- 新增 `docs/SECURITY_BASELINE_DRAFT.md`：定义 6 项最小安全基线（hash、sandbox、来源、签名、门禁、审计）。
- 新增 `docs/SANDBOX_EXECUTION_POLICY_DRAFT.md`：高风险执行受限策略（权限/路径/进程/网络/资源）。
- 新增 `docs/HASH_AND_SIGNATURE_POLICY_DRAFT.md`：hash+签名联合校验策略。
- `SECURITY.md`：增加安全基线、sandbox、hash+签名策略入口。
- `docs/ARCHITECTURE.md`、`docs/API_SPEC.md`：增加上述安全策略引用入口。
- `docs/API_ERROR_CATALOG.md`：新增签名相关错误目录（signature_invalid/signature_missing）。
- 本轮保持 Draft，不冻结。

## 2026-02-22
- 完成 UI 全局搜索（Raycast 风格）设计草案并跨文档同步。
- `docs/UI_MAP.md`：从占位升级为全局搜索信息架构草案。
- `docs/UX_FLOW.md`：补全全局搜索主流程、异常流程、确认/回退流程。
- `docs/PRODUCT_SPEC.md`：加入全局搜索模块目标、MVP 范围与验收条目。
- `docs/ROADMAP.md`：将全局搜索纳入 Phase 1，排序增强纳入 Phase 2。
- `docs/API_SPEC.md`：新增 `ui search` 与 `ui action-run` 草案接口。
- 新增 `docs/UI_GLOBAL_SEARCH_DRAFT.md` 作为专门设计文档。
- 本轮保持 Draft，不冻结。

## 2026-02-22
- 新增 `docs/UI_COMPONENT_SPEC_DRAFT.md`：全局搜索组件规范、状态机与数据结构草案。
- 新增 `docs/UI_INTERACTION_COPY_DRAFT.md`：全局搜索与安全流程文案规范草案。
- `docs/UI_MAP.md` 与 `docs/UX_FLOW.md` 增加 UI 组件/文案规范入口引用。
- 本轮保持 Draft，不冻结。

## 2026-02-22
- 新增 `docs/UI_SEARCH_RANKING_DRAFT.md`：全局搜索排序模型、权重策略与可解释性输出草案。
- 新增 `docs/UI_SHORTCUTS_DRAFT.md`：全局快捷键分层规则、冲突策略与安全确认约束草案。
- `docs/UI_GLOBAL_SEARCH_DRAFT.md`、`docs/UI_MAP.md`、`docs/UX_FLOW.md` 同步新增快捷键规范引用。
- 本轮保持 Draft，不冻结。

## 2026-02-22
- 新增 `docs/SOFTWARE_REPOSITORY_SYSTEM_DRAFT.md`：软件仓库系统草案（公共仓库 + 个人仓库 + 候选池 + AI 参与）。
- `docs/PRODUCT_SPEC.md`：加入“软件仓库系统”范围、MVP/Phase 2 分层与能力边界。
- `docs/FEATURES.md`、`docs/ROADMAP.md`：同步仓库系统阶段目标。
- `docs/ARCHITECTURE.md`：新增 `integration.repository` 模块与仓库同步 Flow I。
- `docs/API_SPEC.md`：新增 `repo list/add/sync/package search/import/export/submit` 命令草案。
- `docs/DATA_MODEL.md`、`docs/DATABASE_DESIGN_DRAFT.md`、`docs/sql/V001__init_schema_draft.sql`：新增仓库域数据模型与临时表。
- `SECURITY.md`、`docs/API_ERROR_CATALOG.md`：补充仓库系统安全边界与错误目录。
- `docs/AI_CONTEXT.md`：加入仓库系统主线与阅读顺序。
- 本轮保持 Draft，不冻结。

## 2026-02-22
- 新增 `docs/FINAL_ARCHITECTURE.md`：Synora 最终架构基线（v1.0）主入口文档。
- 明确最终分层：UI -> API -> Service -> Queue/Scheduler -> Worker -> Integration -> Security/Audit -> Storage。
- 固化核心链路：发现、仓库、补链、下载、执行、AI 六条主流程。
- `docs/AI_CONTEXT.md`：将 `docs/FINAL_ARCHITECTURE.md` 纳入 AI 必读顺序。

## 2026-02-22
- `README.md`：新增终极愿景（Windows 的 Raycast + Homebrew + AI 安全编排层）与三条主线说明。
- `docs/PRODUCT_SPEC.md`：新增“终极版本愿景”章节，明确入口层/供给层/控制层分解。

## 2026-02-22
- `docs/ROADMAP.md`：新增“V1 到终局里程碑映射（Draft）”。
- 将三层愿景拆分为分阶段目标：
  - 入口层（Raycast）：V1 可达 -> V2 排序增强 -> V3 多入口统一
  - 供给层（Homebrew）：V1 仓库基础 -> V2 社区提交流程 -> V3 多仓库治理
  - 控制层（AI 安全编排）：V1 建议与门禁 -> V2 受控执行 -> V3 策略化编排

## 2026-02-22
- `docs/TECH_STACK.md`：升级为 v0.3 Draft，形成可执行技术选型基线。
- 明确 V1 主选：Rust + clap + serde + SQLite/rusqlite + Registry/winget + 本地队列 + YAML 仓库格式。
- 明确 AI 与插件策略：AI provider 抽象优先、插件先可控后开放（V2 原生/V3 WASM 评估）。
- 增补依赖策略、平台支持策略、暂缓项与新增待决策项。

## 2026-02-22
- 新增 `docs/CARGO_DEPENDENCY_BASELINE_DRAFT.md`：技术选型到 `Cargo.toml` 的依赖映射草案。
- 依赖按模块分组：CLI/存储/下载校验/Windows 集成/并发日志。
- 明确实施顺序与暂缓项（tokio/tracing/wasmtime/分布式队列）。
- `docs/TECH_STACK.md` 更新同步规则，纳入依赖基线文档引用。

## 2026-02-22
- `Cargo.toml` 落地 Step 1 最小依赖集：
  - `clap`、`serde`、`serde_json`、`serde_yaml`、`thiserror`、`rusqlite(bundled)`
- 目标：先打通 CLI 契约、序列化与本地存储基础能力，暂不引入并发与下载依赖。

## 2026-02-22
- `docs/ROADMAP.md`：新增“执行版开发顺序（Phase 1-Phase 8）”。
- 将 V1 实施路径固定为 8 个阶段：基线 -> 存储审计 -> 发现 -> 仓库 -> 下载校验 -> 执行门禁 -> AI+搜索 -> 稳定发布。
- 每个阶段新增目标范围与退出标准，用于后续周度推进与验收。

## 2026-02-22
- 新增 `docs/DESIGN_FREEZE_CHECKLIST.md`：设计冻结前总清单（待决策 + 待补充 + 冻结 Gate）。
- 汇总当前所有文档未完成项，并统一状态字段（未开始/进行中/已完成）。
- 明确冻结判定条件与主文档冻结更新范围。

## 2026-02-22
- 完成顶层决策第一组清空：
  - `docs/PRODUCT_SPEC.md`：冻结 8 项顶层产品决策。
  - `docs/ROADMAP.md`：冻结 MVP 截止口径、更新默认模式、Phase 2 边界、Discovery 扩展顺序、队列路线。
  - `docs/TECH_STACK.md`：冻结插件加载方式、WASM 运行时、签名链路、AI provider 优先级、签名覆盖范围。
  - `SECURITY.md`：冻结门禁策略、风险分级、安全签署流程、插件信任模型。
- `docs/DESIGN_FREEZE_CHECKLIST.md`：A 区对应项（2/17/18/22）状态更新为“已完成”，并补充日期与负责人。

## 2026-02-22
- 完成实现前关键决策第二组清空：
  - `docs/API_SPEC.md`：冻结 discover source 参数、history 分页策略、plain/json 错误输出策略、repo sync 默认增量策略。
  - `docs/ARCHITECTURE.md`：冻结去重主键策略、stale 阈值（30 天）、AI linker 离线 rule-first 策略。
  - `docs/DATA_MODEL.md`：冻结 inventory 去重键、cleanup 强关联方案、audit payload 列策略（Phase 1 保持 JSON）。
- `docs/DESIGN_FREEZE_CHECKLIST.md`：A 区对应项（4/5/7）状态更新为“已完成”，并补充日期与负责人。

## 2026-02-22
- 完成安全冻结第三组清空：
  - `docs/SANDBOX_EXECUTION_POLICY_DRAFT.md`：冻结 Windows sandbox 选型（Job Object + 受限令牌）、网络策略位置、策略版本存储位置。
  - `docs/SECURITY_BASELINE_DRAFT.md`：冻结 MVP sandbox 形态、Phase 2 签名最小覆盖范围、审计保留归档策略。
  - `docs/HASH_AND_SIGNATURE_POLICY_DRAFT.md`：冻结签名缺失默认阻断、发布者白名单层级、多算法路线。
- `docs/DESIGN_FREEZE_CHECKLIST.md`：A 区对应项（11/19/20）状态更新为“已完成”，并补充日期与负责人。

## 2026-02-22
- 完成仓库与下载策略第四组清空：
  - `docs/SOFTWARE_REPOSITORY_SYSTEM_DRAFT.md`：冻结公共仓库单源策略、`software.yaml` schema_version、双人审核策略。
  - `docs/DOWNLOAD_SOURCE_POLICY_DRAFT.md`：冻结企业覆盖优先级、子域继承默认关闭、一次性临时授权策略。
  - `docs/DOWNLOAD_CACHE_POLICY_DRAFT.md`：冻结风险分层保留期、手动固定保留策略、`download cache stats` 命令纳入 V1。
- `docs/DESIGN_FREEZE_CHECKLIST.md`：A 区对应项（8/9/21）状态更新为“已完成”，并补充日期与负责人。

## 2026-02-22
- 完成 A 区剩余待决策项清空（第八组）：
- `README.md`：冻结项目一句话定位与首个冻结范围。
- `decisions/ARCHITECTURE_DECISIONS.md`：冻结 ADR 编号规则与模板字段。
- `docs/FEATURES.md`：冻结更新检测默认模式、体检/清理/修复边界、Phase 2 搜索增强粒度、Discovery 冲突处理策略。
- `logs/DEVELOPMENT_LOG.md`：冻结日志粒度与里程碑记录格式。
- `docs/DESIGN_FREEZE_CHECKLIST.md`：A 区对应项（1/3/10/28）更新为“已完成”。

## 2026-02-22
- 完成插件冻结第五组清空：
  - `docs/PLUGIN_SYSTEM.md`：冻结插件路线（V1/V2 原生、V3 WASM）、签名信任模型、权限粒度、Phase 2 签名强制策略。
  - `docs/PLUGIN_PERMISSION_MATRIX.md`：冻结命令级权限、临时授权 token、企业策略覆盖层。
  - `docs/PLUGIN_LIFECYCLE_SEQUENCE.md`：冻结自动 blocked 阈值、恢复审批要求、批量执行重试策略。
- `docs/DESIGN_FREEZE_CHECKLIST.md`：A 区对应项（14/15/16）状态更新为“已完成”，并补充日期与负责人。

## 2026-02-22
- 完成队列与依赖冻结第六组清空：
  - `docs/JOB_RETRY_POLICY_DRAFT.md`：冻结按 job_type 覆盖重试、jitter、批量死信回放策略。
  - `docs/JOB_TYPES_DRAFT.md`：冻结幂等键、fan-out、`download.verify` 强制串联策略。
  - `docs/CARGO_DEPENDENCY_BASELINE_DRAFT.md`：冻结 V1 `tokio` 执行模型、`windows` 覆盖范围、`rusqlite bundled` 策略。
- `docs/DESIGN_FREEZE_CHECKLIST.md`：A 区对应项（6/12/13）状态更新为“已完成”，并补充日期与负责人。

## 2026-02-22
- 完成 UI/UX 冻结第七组清空：
  - `docs/UI_GLOBAL_SEARCH_DRAFT.md`：冻结中英混输、快捷动作开放策略、参数编辑策略。
  - `docs/UI_MAP.md`：冻结首版 UI 形态、插件结果展示策略、默认排序优先级。
  - `docs/UI_SEARCH_RANKING_DRAFT.md`：冻结手动置顶、场景化排序、风险惩罚分策略。
  - `docs/UI_SHORTCUTS_DRAFT.md`：冻结 MVP 快捷键开放边界（不开放自定义/vim/双击 Ctrl）。
  - `docs/UX_FLOW.md`：冻结默认排序、查询混合规则、高频快捷键集合。
- `docs/DESIGN_FREEZE_CHECKLIST.md`：A 区对应项（23/24/25/26/27）状态更新为“已完成”，并补充日期与负责人。

## 2026-02-22
- 完成 B 区待补充第一组清空：
- `README.md`：补齐项目概述、技术栈摘要、快速启动、目录结构说明。
- `decisions/ARCHITECTURE_DECISIONS.md`：补齐 ADR-001/002/003。
- `logs/DEVELOPMENT_LOG.md`：补齐里程碑记录规范与发布记录规范。
- `docs/DESIGN_FREEZE_CHECKLIST.md`：B 区对应项（1/3/13）更新为“已完成”。

## 2026-02-22
- 完成 B 区待补充第二组清空：
- `SECURITY.md`：将 11 组安全草案条目落为 Phase 1 正式策略（原则、威胁、漏洞流程、发布门禁、插件/AI/下载/sandbox/仓库边界）。
- `docs/API_SPEC.md`：补充字段稳定级标签（stable/experimental）、通用失败示例、CLI smoke 脚本草案。
- `docs/DESIGN_FREEZE_CHECKLIST.md`：B 区对应项（2/4）更新为“已完成”。

## 2026-02-22
- 完成 B 区待补充第三组清空：
- `docs/ARCHITECTURE.md`：补齐系统时序图（discover/suggest/execute）、包级依赖图、失败注入测试点。
- `docs/DATA_MODEL.md`：补齐 ER 图（Mermaid）、TTL/归档策略、性能基线与索引基线。
- `docs/DESIGN_FREEZE_CHECKLIST.md`：B 区对应项（5/6）更新为“已完成”。

## 2026-02-22
- 完成 B 区待补充第四组清空：
- `docs/FEATURES.md`：将 MVP/Phase2/暂不做清单由草稿态转为冻结态。
- `docs/PRODUCT_SPEC.md`：将目标用户/问题定义/价值主张/MVP 边界条目统一转为冻结表达。
- `docs/ROADMAP.md`：补齐执行版时间窗与验收口径，并转为冻结态。
- `docs/DESIGN_FREEZE_CHECKLIST.md`：B 区对应项（7/8/9）更新为“已完成”。
- `docs/PRODUCT_SPEC.md`：追加清理“草稿”残留标记，统一为冻结术语。

## 2026-02-22
- 完成 B 区待补充第五组清空：
- `docs/TECH_STACK.md`：补齐冻结落实清单（依赖对齐、Windows API 封装、插件运行时 PoC 结论、AI provider 抽象）。
- `docs/UI_MAP.md`：补齐信息架构、搜索面板、结果结构、详情映射、交互反馈冻结化。
- `docs/UX_FLOW.md`：补齐主流程 A-D 与异常/确认/回退流程冻结化。
- `docs/DESIGN_FREEZE_CHECKLIST.md`：B 区对应项（10/11/12）更新为“已完成”。

## 2026-02-22
- 设计冻结正式生效：
- `decisions/ARCHITECTURE_DECISIONS.md`：新增 `ADR-004`（设计冻结生效）。
- 主文档状态统一更新：`docs/API_SPEC.md`、`docs/DATA_MODEL.md`、`docs/TECH_STACK.md`、`docs/ROADMAP.md`、`docs/PRODUCT_SPEC.md`、`SECURITY.md` 为 Frozen，`docs/FINAL_ARCHITECTURE.md` 维持 Final Baseline。
- `docs/DESIGN_FREEZE_CHECKLIST.md`：当前状态更新为 `v1 Frozen`，记录 Gate 满足与冻结日期。

## 2026-02-22
- 实现阶段启动（Phase 1 骨架）：
- `src/main.rs`：切换到 CLI 入口执行模型（基于退出码返回）。
- `src/cli/mod.rs`：新增最小可运行命令骨架（`config init`、`config gate-show`、`software discover scan`、`software list`、`source suggest`）。
- `config init` 已落地：自动创建 `.synora_custom/config.json` 与 `.synora_custom/db/synora.db`，并初始化最小 `gate_history` 表。
- 当前验证受环境网络限制：`cargo check` 无法访问 crates index（`Could not resolve host: index.crates.io`）。

## 2026-02-22
- 实现阶段推进（Phase 2 配置与门禁命令）：
- `src/cli/mod.rs`：新增 `config gate-set` 与 `config gate-history`。
- `gate-set`：实现 `--enable/--disable/--confirm/--approval-record/--reason/--keep-record/--dry-run` 规则校验与落盘逻辑。
- `gate-history`：实现 `--enabled-only/--since/--limit/--reason-contains` 过滤查询（SQLite）。
- CLI 退出码映射补齐：usage=2、security=3、integration=4。

## 2026-02-22
- 实现阶段推进（Phase 3 发现入库骨架）：
- `software discover scan`：接入 Windows Registry-only 发现（PowerShell 查询 HKLM/HKCU uninstall）并写入 SQLite `software_inventory`。
- `software list`：改为读取数据库真实库存，不再固定返回空数组。
- 数据库初始化扩展：新增 `software_inventory` 表并保留 `gate_history`。

## 2026-02-22
- 实现阶段推进（Phase 4 来源建议与可筛选输出）：
- `source suggest`：从 `software_inventory` 生成候选来源并写入 `source_candidate`（幂等 upsert）。
- 输出策略升级：默认按“每个软件最高置信度候选”返回，避免同软件多条候选刷屏。
- 新增过滤参数：`--limit`、`--min-confidence`、`--domain`、`--contains`。
- 结果分布保护：增加单域名返回上限，降低单一域名在结果中的垄断。

## 2026-02-22
- 实现阶段推进（Phase 5 查询能力补强）：
- `source suggest`：新增 `--status` 过滤（`pending|approved|rejected`）。
- `software list`：新增分页与检索参数（`--limit`、`--offset`、`--contains`、`--active-only`）。

## 2026-02-22
- 实现阶段推进（Phase 6 来源审核闭环）：
- `source suggest`：输出新增 `candidate_id`，用于后续审核操作。
- `source review`：新增候选审核命令（`--candidate-id` + `--approve|--reject`），可将候选标记为 `approved/rejected`。

## 2026-02-22
- 缺陷修复（来源审核可见性）：
- 修复 `source suggest` 重算时覆盖审核状态的问题（不再把已审核候选重置为 `pending`）。
- 修复 `--status` 过滤语义：过滤条件提前到排名前执行，`approved/rejected` 结果可稳定查询。

## 2026-02-22
- 实现阶段推进（Phase 7 审核台全量查询）：
- 新增 `source list` 子命令，提供不压缩的候选全量视图。
- 支持筛选与分页：`--status`、`--domain`、`--contains`、`--limit`、`--offset`。
- 默认按 `candidate_id DESC` 排序，便于优先处理最新候选。

## 2026-02-22
- 实现阶段推进（Phase 8 批量审核）：
- 新增 `source review-bulk` 子命令，支持按筛选条件批量 `approve/reject`。
- 支持参数：`--status`、`--domain`、`--contains`、`--limit`、`--approve|--reject`。

## 2026-02-22
- 实现阶段推进（Phase 9 已审核来源落库）：
- 新增 `source apply-approved` 子命令，将 `approved` 候选写入 `source_registry` 执行来源表。
- 支持筛选参数：`--domain`、`--contains`、`--limit`。
- 落库策略为幂等 upsert，并返回 `matched/inserted/updated` 统计。

## 2026-02-22
- 实现阶段推进（Phase 10 执行来源管理）：
- 新增 `source registry-list` 子命令（执行来源全量视图，支持 `--status/--domain/--contains/--limit/--offset`）。
- 新增 `source registry-disable` 子命令（按 `candidate_id` 或筛选条件批量停用，默认仅作用于 `active`）。

## 2026-02-22
- 实现阶段推进（Phase 11 执行来源恢复）：
- 新增 `source registry-enable` 子命令（按 `candidate_id` 或筛选条件批量恢复，默认仅作用于 `disabled`）。

## 2026-02-22
- 实现阶段推进（Phase 12 更新检查入口）：
- 新增 `update check` 子命令，读取 `source_registry(status=active)` 输出更新建议清单。
- 支持筛选与分页：`--domain`、`--contains`、`--limit`、`--offset`。

## 2026-02-22
- 实现阶段推进（Phase 13 更新执行草案）：
- 新增 `update apply` 子命令（当前仅支持 `--dry-run`），可按 `--candidate-id` 生成计划执行结果。
- 验证规则：未启用 `--dry-run` 直接拒绝；`--dry-run` 下禁止 `--confirm`。

## 2026-02-22
- 实现阶段推进（Phase 14 更新执行门禁与审计）：
- `update apply` 打通 `--confirm` 路径，复用 gate 配置进行真实执行门禁校验（real mutation + approval record）。
- 新增 `update_operation_history` 审计表，记录 `operation_id/candidate_id/mode/status/message/ts`。
- 维持 dry-run 与 confirm 双路径输出。

## 2026-02-22
- 实现阶段推进（Phase 15 更新审计查询）：
- 新增 `update history` 子命令，支持 `--limit/--offset/--status/--mode/--candidate-id` 过滤查询。

## 2026-02-22
- 实现阶段推进（Phase 16 更新失败与回滚审计）：
- `update apply --confirm` 新增失败模拟能力：`--simulate-failure`、`--simulate-rollback-failure`。
- 失败路径会写入 `update_operation_history(status=failed)`，并在 `message` 记录回滚结果（success/failed）。

## 2026-02-22
- 缺陷修复（update operation_id 冲突）：
- 将 `update apply` 的 `operation_id` 从秒级时间戳改为“纳秒时间 + 进程内序号 + target_id”，修复连续执行时 UNIQUE 冲突问题。

## 2026-02-22
- 实现阶段推进（Phase 17 更新回滚字段结构化）：
- `update_operation_history` 新增结构化字段 `rollback_attempted` 与 `rollback_status`（含旧库兼容迁移）。
- `update history` 输出增加上述字段，不再只能从 `message` 推断回滚状态。

## 2026-02-22
- 质量收尾（测试与文档）：
- `src/cli/mod.rs`：新增最小单测（operation_id 唯一性、registry status 校验、update apply 参数组合校验）。
- 新增 `docs/CLI_SMOKE_TESTS.md`：沉淀 PowerShell 回归命令清单。
- `README.md`：更新为实现阶段状态，并补充当前可用命令矩阵。

## 2026-02-22
- Phase 3 落地增强（发现入库同步语义）：
- `software discover scan` 新增结构化统计：`reactivated`、`deactivated`、`active_after`。
- 扫描后自动将“本次未发现且此前为 active 的 registry 记录”标记为 inactive，实现增量同步闭环。

## 2026-02-22
- Phase 3 落地收口（扫描历史与唯一标识）：
- `software discover scan` 的 `scan_id` 升级为高唯一 ID（纳秒时间 + 序号）。
- 新增 `software discover history` 子命令，持久化并查询每次扫描统计（seen/inserted/updated/reactivated/deactivated/skipped/active_after）。

## 2026-02-22
- Phase 4 启动（仓库系统 MVP - Step 1）：
- 新增 `repo list` 子命令，支持 `--limit/--offset/--status/--kind` 过滤与分页。
- 数据库新增 `repo_registry` 表，并在初始化时写入默认仓库（public/personal）。

## 2026-02-22
- Phase 4 推进（仓库系统 MVP - Step 2）：
- 新增 `repo add` 与 `repo remove` 子命令，支持仓库条目增删。
- 安全保护：默认仓库（`public-default`、`personal-local`）禁止删除。

## 2026-02-22
- Phase 4 推进（仓库系统 MVP - Step 3）：
- 新增 `repo sync` 子命令，将 active 仓库同步到 `repo_package_index`，并写入 `repo_sync_history`。
- 当前为 phase4 模拟同步实现（可复现、可审计），为下一步 `package search` 提供索引输入。

## 2026-02-22
- Phase 4 推进（仓库系统 MVP - Step 4）：
- 新增 `package search` 子命令，支持 `--contains`、`--repo-key`、`--limit`、`--offset`。
- 检索数据来源为 `repo_package_index(status=active)`。

## 2026-02-22
- Phase 5 启动（下载与校验链路 - Step 1）：
- 新增 `download start`（当前仅 dry-run 计划模式）。
- 新增 `download list`（支持状态/校验状态过滤与分页）。
- 新增 `download verify`（模拟校验通过/失败）并写入 `download_job_history`。

## 2026-02-22
- Phase 5 推进（下载与校验链路 - Step 2）：
- `download_job_history` 增加结构化校验字段：`hash_status`、`signature_status`、`source_policy_status`（含旧库兼容迁移）。
- `download verify` 支持模拟特定失败类型：`--simulate-hash-failure`、`--simulate-signature-failure`、`--simulate-source-policy-failure`。

## 2026-02-22
- Phase 5 推进（下载与校验链路 - Step 3）：
- 新增 `download history` 子命令，支持 `--status`、`--failure-type`、`--limit`、`--offset`。
- `download history` 输出新增统计汇总：`failed_hash`、`failed_signature`、`failed_source_policy`、`failed_generic`。

## 2026-02-22
- Phase 5 推进（下载与校验链路 - Step 4）：
- 新增 `download show` 子命令，支持按 `--job-id` 查看单条下载任务详情（含结构化校验字段）。
- 新增 `download retry` 子命令，支持对 `failed` 任务生成新的重试任务（当前阶段仅 `--dry-run`）。

## 2026-02-22
- Phase 6 启动（执行链路与安全门禁 - Step 1）：
- `update apply --confirm` 新增强制执行票据参数 `--execution-ticket`（confirmed 路径必填）。
- `update_operation_history` 新增 `execution_ticket` 字段并完成兼容迁移。
- `update history` 输出新增 `execution_ticket`，支持审计时关联执行票据。

## 2026-02-22
- Phase 6 推进（执行链路与安全门禁 - Step 2）：
- 新增 `cleanup apply` 与 `cleanup history`，复用 `dry-run/confirm + gate + execution-ticket + rollback` 安全语义。
- 新增 `cleanup_operation_history` 审计表（含 `rollback_attempted`、`rollback_status`、`execution_ticket`）。

## 2026-02-22
- Phase 6 推进（执行链路与安全门禁 - Step 3）：
- 抽取通用执行 helper：统一 `update/cleanup` 的 confirm 参数校验、门禁检查与失败回滚结果生成。
- 降低 `update apply` 与 `cleanup apply` 重复逻辑，保持 CLI 行为与错误码契约不变。

## 2026-02-22
- Phase 6 推进（执行链路与安全门禁 - Step 4）：
- `cleanup history` 新增审计细粒度筛选：`--execution-ticket`、`--rollback-status`、`--contains`。
- 支持按票据与回滚结果进行追溯检索，补齐执行证据链查询能力。

## 2026-02-22
- Phase 7 启动（AI 助手与全局搜索入口 - Step 1）：
- 新增 `ai repair-plan`（plan-only）命令：输入 `--software`、`--issue`，输出 `plan_steps/rollback_hint/risk_level/confidence/reason`。
- 新增 `ai_repair_plan_history` 审计表，记录生成的修复计划元数据与计划 JSON。

## 2026-02-22
- Phase 7 推进（AI 助手与全局搜索入口 - Step 2）：
- 新增 `ai analyze`（plan-only）命令：基于本地 inventory 输出结构化分析、冗余提示与建议。
- 新增 `ai recommend`（plan-only）命令：基于 `--goal` 输出场景化软件建议（含 reason/confidence/risk_level）。
- 新增审计表：`ai_analyze_history`、`ai_recommend_history`。

## 2026-02-22
- Phase 7 推进（AI 助手与全局搜索入口 - Step 3）：
- 新增 `ui search` 命令，按查询词聚合 `software/source/update/download/ai` 只读结果分组。
- 输出结构对齐 API 草案：`query` + `groups[].type/items[]`，并包含 `title/subtitle/risk_level/confidence/action_id`。

## 2026-02-22
- Phase 7 推进（AI 助手与全局搜索入口 - Step 4）：
- 新增 `ui action-run` 命令（模拟执行），按 `action_id` 风险分级进行确认拦截。
- 高风险动作未携带 `--confirm` 返回 security(3)；执行事件写入 `ui_action_history` 审计表。

## 2026-02-22
- Phase 8 启动（稳定化与发布收口 - Step 1）：
- 新增 `scripts/smoke_phase8.ps1` 一键回归脚本，覆盖 build + 核心链路 + 预期失败校验。
- 新增 `docs/RELEASE_READINESS_CHECKLIST.md` 发布门禁清单，统一检查错误码契约、安全门禁、审计覆盖与 MVP DoD。

## 2026-02-22
- Phase 8 推进（稳定化与发布收口 - Step 2）：
- 新增最小任务队列命令：`job submit/list/retry`，并落地 `job_queue` 表。
- `job retry` 仅允许 `failed/deadletter`，并按 `attempt_count/max_attempts` 处理 `queued/deadletter` 转移。
- `job submit` 增加 `--simulate-failed/--simulate-deadletter` 便于本地回归无 worker 场景。

## 2026-02-22
- Phase 8 推进（稳定化与发布收口 - Step 3）：
- 新增死信运维命令：`job deadletter-list` 与 `job replay-deadletter`。
- `replay-deadletter` 支持单条 `--id` 或批量 `--limit` 回放，回放后任务重置为 `queued` 且 `attempt_count=0`。

## 2026-02-22
- Phase 8 推进（稳定化与发布收口 - Step 4）：
- `docs/roadmap.md` 更新为 Phase 8 执行中状态，并补充执行进度快照。
- 新增 `docs/V1_RELEASE_NOTES_DRAFT.md`：整理本版变更摘要、错误码契约、已知限制、回滚与发布前检查指引。

## 2026-02-22
- Phase 8 推进（稳定化与发布收口 - Step 5）：
- `docs/roadmap.md` 执行快照中将 Phase 8 Step 4 标记为“已完成”。
- 新增 `docs/V1_GO_NO_GO.md`：发布最终决策单页（Go/No-Go、阻断项、签署栏）。

## 2026-02-22
- Phase 8 收口（稳定化与发布收口 - Final）：
- `scripts/smoke_phase8.ps1` 在 Windows PowerShell 全流程通过，并输出 `[phase8-smoke] completed`。
- 修复并确认脚本稳定性问题：job payload JSON 引号传递、expected-fail 空参数场景（改为缺值参数语义）。
- 更新发布文档状态：`docs/RELEASE_READINESS_CHECKLIST.md` 全项通过、`docs/V1_GO_NO_GO.md` 判定为 Go、`docs/ROADMAP.md` 状态更新为 Phase 8 completed。
