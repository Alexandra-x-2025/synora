# UI_MAP

## 文档目的
定义界面信息架构与页面映射。

## 当前状态
- 状态：v1 Frozen（设计冻结版）
- UI 规划：CLI-first + Raycast 风格全局搜索入口

## 上下文输入
- 当前以 CLI 为主，后续可扩展桌面/Web UI
- 设计目标：统一“一个入口触达全部动作”

## 预期输出
- 页面/视图层级清晰
- 关键入口与导航明确

## UI 映射决策（Phase 1 Freeze）
1. 首版 UI 形态：桌面优先（CLI + 桌面入口），Web 控制台后置。
2. 插件命令结果：MVP 支持插件结果展示，但执行仍受权限与门禁约束。
3. 默认排序策略：置信度优先，其次最近使用，风险为硬约束不越级。

## 冻结信息架构（Phase 1 Freeze）
1. 顶层信息架构
- `Global Search`（主入口，类似 Raycast）
- `Software`（软件库与详情）
- `Downloads`（下载任务与产物）
- `Jobs`（任务队列监控）
- `Security`（Gate、审计、风险告警）
- `Plugins`（插件管理）
- `AI Assistant`（Analyze/Recommend/Repair Plan）

2. 全局搜索面板
- 触发：`Ctrl+K` / `Cmd+K`（平台适配）
- 输入区：支持自然语言与命令关键字
- 结果区分组：
- `Actions`：可直接执行的动作
- `Software`：软件项与快捷操作
- `Downloads`：下载任务与状态
- `Jobs`：后台任务与重试入口
- `Security`：告警与门禁操作
- `AI Suggestions`：推荐与修复方案

3. 结果项结构
- `title`
- `subtitle`
- `type`
- `risk_level`
- `confidence`
- `shortcut_hint`
- `action_id`

4. 详情视图映射
- `Software Detail`
- `Download Task Detail`
- `Job Detail`
- `Audit Event Detail`
- `Plugin Detail`

5. 交互反馈
- 高风险动作显示二次确认浮层
- 安全阻断显示原因 + 修复建议
- 可重试失败项直接给出 `Retry` 快捷操作

## 更新规则
- 新增视图时必须更新映射。
- 与 UX_FLOW 保持一致。

## 关联文档
- `docs/UI_GLOBAL_SEARCH_DRAFT.md`
- `docs/UI_SHORTCUTS_DRAFT.md`
- `docs/UI_COMPONENT_SPEC_DRAFT.md`
- `docs/UI_INTERACTION_COPY_DRAFT.md`
