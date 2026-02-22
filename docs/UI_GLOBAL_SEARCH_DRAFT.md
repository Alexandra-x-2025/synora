# UI_GLOBAL_SEARCH_DRAFT

## 文档目的
定义 Synora 全局搜索（Command Palette）设计，作为 UI 主入口规范。

## 当前状态
- 状态：v0.1 Draft（未冻结）
- 参考风格：Raycast（快速检索 + 快捷执行）

## 目标（Draft）
1. 一个入口触达全部核心能力。
2. 搜索即操作：尽量减少层级跳转。
3. 高风险动作仍受安全门禁约束。

## 输入类型（Draft）
1. 命令关键词（如 `download list`）
2. 实体关键词（软件名、任务 ID、插件名）
3. 意图型自然语言（如“视频剪辑工具”“查看失败任务”）

## 结果分组（Draft）
1. `Actions`
2. `Software`
3. `Downloads`
4. `Jobs`
5. `Security`
6. `AI Suggestions`

## 排序策略（Draft）
1. 默认权重：精确匹配 > 前缀匹配 > 语义匹配
2. 风险显示：高风险结果显示 `risk_level`
3. 个性化（Phase 2）：最近使用与历史成功率加权

## 快捷键与交互（Draft）
1. 打开面板：`Ctrl/Cmd + K`
2. 上下移动：`↑ / ↓`
3. 执行动作：`Enter`
4. 查看详情：`Tab`
5. 退出：`Esc`
6. 详细键位规范见：`docs/UI_SHORTCUTS_DRAFT.md`

## 安全与执行边界（Draft）
1. 搜索结果触发动作仍走 API/Service/Queue/Worker。
2. 高风险动作触发二次确认。
3. 安全阻断必须返回可操作提示。

## 搜索入口决策（Phase 1 Freeze）
1. 多语言策略：MVP 优先支持中英混输检索。
2. 快捷动作策略：MVP 不开放用户自定义快捷动作，Phase 2 再开放。
3. 参数编辑策略：MVP 不支持结果内联参数编辑，采用详情页/命令参数输入。

## 更新规则
- 设计变更必须同步：
  - `docs/UI_MAP.md`
  - `docs/UX_FLOW.md`
  - `docs/API_SPEC.md`
  - `docs/PRODUCT_SPEC.md`
  - `docs/UI_SHORTCUTS_DRAFT.md`
