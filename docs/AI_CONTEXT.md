# AI_CONTEXT

## 文档目的
为 AI 协作提供稳定上下文，确保文档与设计推进方向一致。

## 当前状态
- 状态：v0.2 Draft（继续设计中，未冻结）
- 当前阶段：需求与架构设计深化（不冻结、不实现）

## 项目目标摘要
- 项目：Synora
- 定位：AI 驱动的软件操作系统管理器（Windows first, CLI-first, local-first）
- 核心主线：软件发现、来源推荐、安全门禁、可审计执行
- 新增主线：插件系统（先设计，后分阶段落地）
- 新增主线：AI 管理助手（analyze / recommend / repair-plan）
- 新增主线：软件仓库系统（public/personal/candidate + software.yaml）

## AI 必读顺序
1. `README.md`
2. `docs/PRODUCT_SPEC.md`
3. `docs/FINAL_ARCHITECTURE.md`
4. `docs/ARCHITECTURE.md`
5. `docs/PLUGIN_SYSTEM.md`
6. `docs/SOFTWARE_REPOSITORY_SYSTEM_DRAFT.md`
7. `docs/API_SPEC.md`
8. `docs/DATA_MODEL.md`
9. `docs/ROADMAP.md`

## 协作规则
1. 当前只推进设计文档，不冻结基线。
2. 文档变更必须保持跨文档一致。
3. 任何高风险功能默认加安全约束与审计要求。
4. 插件相关设计必须明确“不能绕过 gate”。
5. AI 修复相关设计必须明确“先计划、后受控执行”。

## 禁止项
1. 未经明确确认，不进入真实变更实现。
2. 不在文档间引入互相冲突的范围定义。
3. 不把 Draft 结论写成 Frozen/Final。

## 最小同步集合
- `docs/PRODUCT_SPEC.md`
- `docs/ARCHITECTURE.md`
- `docs/API_SPEC.md`
- `docs/DATA_MODEL.md`
- `docs/ROADMAP.md`
- `docs/PLUGIN_SYSTEM.md`
- `logs/DEVELOPMENT_LOG.md`

## 更新规则
- AI 协作规则变更必须同步 `README.md` 与 `CONTRIBUTING.md`。
